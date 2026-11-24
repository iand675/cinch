# Text.Zip

Read stored, deflated and Zip64 zip archives with CRC-32 verification and write stored or deflated archives.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Zip`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Zip.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Checksum](Text.Checksum.md), [Text.Zlib](Text.Zlib.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Adler32](Text.Adler32.md), [Text.Binary](Text.Binary.md), [Text.Checksum](Text.Checksum.md), [Text.Crc64](Text.Crc64.md), [Text.Zip](Text.Zip.md), [Text.Zlib](Text.Zlib.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-5a697041726368697665"></a>

## `ZipArchive`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ZipArchive</span></code></pre>

A parsed archive: its entries in directory order and its comment.

[Source](/reference-sources/Text.Zip.cinch) · line 75

<a id="type-5a6970456e747279"></a>

## `ZipEntry`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ZipEntry</span></code></pre>

A central directory entry: name, comment, extra fields, creator version, flags, method, MS-DOS time
and date, CRC-32, compressed and uncompressed sizes, external attributes, local header offset and
the modification time in Unix seconds.

[Source](/reference-sources/Text.Zip.cinch) · line 70

<a id="type-5a69704572726f72"></a>

## `ZipError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ZipError</span></code></pre>

A malformed archive, an unsupported entry or corrupted data.

[Source](/reference-sources/Text.Zip.cinch) · line 40

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZipNotFound</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZipBadEntry</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZipTruncated</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZipUnsupportedMethod</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZipChecksumMismatch</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZipSizeMismatch</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZipBadDeflate</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipError</span></code></pre>

<a id="type-5a697046696c65"></a>

## `ZipFile`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ZipFile</span></code></pre>

A file to write: name, comment, method, Unix mode (zero for none), modification time and data.

[Source](/reference-sources/Text.Zip.cinch) · line 320

# Values

<a id="value-7a69702d636f6d6d656e74"></a>

## `zip-comment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-comment</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipArchive</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Archive comment.

[Source](/reference-sources/Text.Zip.cinch) · line 136

<a id="value-7a69702d6465666c617465"></a>

## `zip-deflate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-deflate</span> :: <span class="tok-constructor">Nat</span></code></pre>

Method code for DEFLATE data.

[Source](/reference-sources/Text.Zip.cinch) · line 36

<a id="value-7a69702d656e7472696573"></a>

## `zip-entries`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-entries</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipArchive</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipEntry</span>))</code></pre>

Entries in central directory order.

[Source](/reference-sources/Text.Zip.cinch) · line 132

<a id="value-7a69702d656e7472792d636f6d6d656e74"></a>

## `zip-entry-comment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-entry-comment</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipEntry</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Entry comment.

[Source](/reference-sources/Text.Zip.cinch) · line 84

<a id="value-7a69702d656e7472792d636f6d707265737365642d73697a65"></a>

## `zip-entry-compressed-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-entry-compressed-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipEntry</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Compressed size in bytes.

[Source](/reference-sources/Text.Zip.cinch) · line 108

<a id="value-7a69702d656e7472792d6372633332"></a>

## `zip-entry-crc32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-entry-crc32</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipEntry</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Recorded CRC-32 of the uncompressed data.

[Source](/reference-sources/Text.Zip.cinch) · line 104

<a id="value-7a69702d656e7472792d63726561746f72"></a>

## `zip-entry-creator`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-entry-creator</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipEntry</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Version made by: the high byte names the creating system (3 is Unix).

[Source](/reference-sources/Text.Zip.cinch) · line 92

<a id="value-7a69702d656e7472792d65787465726e616c2d6174747273"></a>

## `zip-entry-external-attrs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-entry-external-attrs</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipEntry</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

External attributes; for Unix creators the high sixteen bits hold the mode.

[Source](/reference-sources/Text.Zip.cinch) · line 116

<a id="value-7a69702d656e7472792d6578747261"></a>

## `zip-entry-extra`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-entry-extra</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipEntry</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Raw central directory extra fields.

[Source](/reference-sources/Text.Zip.cinch) · line 88

<a id="value-7a69702d656e7472792d666c616773"></a>

## `zip-entry-flags`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-entry-flags</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipEntry</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

General-purpose flags (bit 3: data descriptor, bit 11: UTF-8 name).

[Source](/reference-sources/Text.Zip.cinch) · line 96

<a id="value-7a69702d656e7472792d69732d6469726563746f7279"></a>

## `zip-entry-is-directory`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-entry-is-directory</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipEntry</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether the entry names a directory (its name ends with a slash).

[Source](/reference-sources/Text.Zip.cinch) · line 128

<a id="value-7a69702d656e7472792d6d6574686f64"></a>

## `zip-entry-method`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-entry-method</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipEntry</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Compression method.

[Source](/reference-sources/Text.Zip.cinch) · line 100

<a id="value-7a69702d656e7472792d6d6f6465"></a>

## `zip-entry-mode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-entry-mode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipEntry</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Unix permission and type bits for Unix creators, otherwise zero.

[Source](/reference-sources/Text.Zip.cinch) · line 124

<a id="value-7a69702d656e7472792d6d6f646966696564"></a>

## `zip-entry-modified`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-entry-modified</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipEntry</span> -&gt; <span class="tok-constructor">Int</span>)</code></pre>

Modification time in Unix seconds: the extended timestamp when present, otherwise the MS-DOS fields as UTC.

[Source](/reference-sources/Text.Zip.cinch) · line 120

<a id="value-7a69702d656e7472792d6e616d65"></a>

## `zip-entry-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-entry-name</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipEntry</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Entry name.

[Source](/reference-sources/Text.Zip.cinch) · line 80

<a id="value-7a69702d656e7472792d73697a65"></a>

## `zip-entry-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-entry-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipEntry</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Uncompressed size in bytes.

[Source](/reference-sources/Text.Zip.cinch) · line 112

<a id="value-7a69702d6572726f722d6d657373616765"></a>

## `zip-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a zip error.

[Source](/reference-sources/Text.Zip.cinch) · line 57

<a id="value-7a69702d66696c65"></a>

## `zip-file`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-file</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipFile</span>)))</code></pre>

A file with the given name, method and data, no mode, no comment and no modification time.
Names ending in a slash are written as directories, always stored and without data.

[Source](/reference-sources/Text.Zip.cinch) · line 326

<a id="value-7a69702d66696c652d776974682d636f6d6d656e74"></a>

## `zip-file-with-comment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-file-with-comment</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipFile</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipFile</span>))</code></pre>

Attach a per-file comment.

[Source](/reference-sources/Text.Zip.cinch) · line 338

<a id="value-7a69702d66696c652d776974682d6d6f6465"></a>

## `zip-file-with-mode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-file-with-mode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipFile</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipFile</span>))</code></pre>

Record Unix permission and type bits (for example 0o100644 for a regular file or 0o40755 for a directory).

[Source](/reference-sources/Text.Zip.cinch) · line 330

<a id="value-7a69702d66696c652d776974682d6d6f646966696564"></a>

## `zip-file-with-modified`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-file-with-modified</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipFile</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipFile</span>))</code></pre>

Record a modification time in Unix seconds (UTC).

[Source](/reference-sources/Text.Zip.cinch) · line 334

<a id="value-7a69702d6f70656e"></a>

## `zip-open`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-open</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipEntry</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Read and verify an entry's data: stored bytes are copied, DEFLATE data is inflated, and the CRC-32
and uncompressed size must match the directory.

[Source](/reference-sources/Text.Zip.cinch) · line 290

<a id="value-7a69702d72656164"></a>

## `zip-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-read</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipArchive</span>))</code></pre>

Parse the central directory of a complete archive.

[Source](/reference-sources/Text.Zip.cinch) · line 178

<a id="value-7a69702d73746f7265"></a>

## `zip-store`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-store</span> :: <span class="tok-constructor">Nat</span></code></pre>

Method code for uncompressed data.

[Source](/reference-sources/Text.Zip.cinch) · line 32

<a id="value-7a69702d7772697465"></a>

## `zip-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-write</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipFile</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zip</span>.<span class="tok-constructor">ZipError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Write files and an archive comment as a complete archive.

[Source](/reference-sources/Text.Zip.cinch) · line 342

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Zip.cinch`. Source SHA-256: `ffbb8942b896c6de9979d22637871b4673d6c7dcae70bdaaf408e8b7713e6ddd`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
