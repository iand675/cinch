# Effect.Test.Fs

Immutable in-memory MapFs with synthesized directories, path validation, globbing, subtrees and filesystem conformance checks.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Test.Fs`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Test.Fs.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Map](Data.Map.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Stream](Effect.Stream.md), [Text.Path](Text.Path.md).

Non-embedded source closure: [Effect.Stream](Effect.Stream.md), [Effect.Test.Fs](Effect.Test.Fs.md), [Text.Path](Text.Path.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Immutable in-memory test filesystem and conformance verification suite.
Models slash-separated relative paths, automatic parent directory synthesis,
lexical directory listings, shell-pattern globbing, subtree projection,
stream opening, and full structural conformance checks.

</details>

# Types and constructors

<a id="type-4d6170446972456e747279"></a>

## `MapDirEntry`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MapDirEntry</span></code></pre>

Entry base name, directory flag, type mode bits, and full metadata.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 95

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MapDirEntry</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFileInfo</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapDirEntry</span>))))</code></pre>

<a id="type-4d617046696c65"></a>

## `MapFile`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MapFile</span></code></pre>

File payload bytes, mode bits, modification timestamp in Unix seconds, and directory flag.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 40

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MapFile</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFile</span>))))</code></pre>

<a id="type-4d617046696c65496e666f"></a>

## `MapFileInfo`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MapFileInfo</span></code></pre>

Base name, byte size (zero for directories), mode bits, modification time, and directory flag.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 70

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MapFileInfo</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFileInfo</span>)))))</code></pre>

<a id="type-4d61704673"></a>

## `MapFs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MapFs</span></code></pre>

Ordered map from relative path strings to snapshotted MapFile values.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 138

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MapFs</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">Map</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFile</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFs</span>)</code></pre>

<a id="type-4d617046734572726f72"></a>

## `MapFsError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MapFsError</span></code></pre>

Named errors returned by MapFs operations.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 114

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FsInvalidPath</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFsError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FsNotFound</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFsError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FsIsDirectory</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFsError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FsNotDirectory</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFsError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FsBadPattern</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFsError</span>)</code></pre>

# Values

<a id="value-6469722d656e7472792d696e666f"></a>

## `dir-entry-info`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dir-entry-info</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapDirEntry</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFileInfo</span>)</code></pre>

Return the full MapFileInfo associated with a directory entry.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 110

<a id="value-6469722d656e7472792d69732d646972"></a>

## `dir-entry-is-dir`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dir-entry-is-dir</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapDirEntry</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether a directory entry refers to a subdirectory.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 102

<a id="value-6469722d656e7472792d6e616d65"></a>

## `dir-entry-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dir-entry-name</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapDirEntry</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the base name of a directory entry.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 98

<a id="value-6469722d656e7472792d74797065"></a>

## `dir-entry-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dir-entry-type</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapDirEntry</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Return the file type mode bits for a directory entry.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 106

<a id="value-66696c652d696e666f2d69732d646972"></a>

## `file-info-is-dir`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">file-info-is-dir</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFileInfo</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether the described entry is a directory.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 89

<a id="value-66696c652d696e666f2d6d6f642d74696d65"></a>

## `file-info-mod-time`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">file-info-mod-time</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFileInfo</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Return the modification timestamp recorded for the entry.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 85

<a id="value-66696c652d696e666f2d6d6f6465"></a>

## `file-info-mode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">file-info-mode</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFileInfo</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Return the mode bits recorded for the entry.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 81

<a id="value-66696c652d696e666f2d6e616d65"></a>

## `file-info-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">file-info-name</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFileInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the base name of the described entry, or "." for the root directory.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 73

<a id="value-66696c652d696e666f2d73697a65"></a>

## `file-info-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">file-info-size</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFileInfo</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Return the payload size in bytes (zero for directories).

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 77

<a id="value-66732d6d6f64652d646972"></a>

## `fs-mode-dir`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fs-mode-dir</span> :: <span class="tok-constructor">U32</span></code></pre>

Mode bit marking a directory entry (bit 31, matching POSIX/virtual filesystem directory flag).

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 30

<a id="value-66732d76616c69642d70617468"></a>

## `fs-valid-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fs-valid-path</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Validate a relative slash-separated filesystem path.
"." is valid for the root directory; empty paths, leading or trailing slashes,
empty segments ("//"), and "." or ".." components are rejected.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 226

<a id="value-6d61702d646972"></a>

## `map-dir`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-dir</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFile</span>))</code></pre>

Construct an explicit empty directory MapFile with the given permission bits and timestamp.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 47

<a id="value-6d61702d66696c65"></a>

## `map-file`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-file</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFile</span>)</code></pre>

Construct a regular MapFile with default mode and zero modification time.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 43

<a id="value-6d61702d66696c652d64617461"></a>

## `map-file-data`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-file-data</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFile</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Read a fresh copy of the payload bytes stored in a MapFile.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 51

<a id="value-6d61702d66696c652d69732d646972"></a>

## `map-file-is-dir`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-file-is-dir</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFile</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether a MapFile represents a directory via its flag or directory mode bit.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 63

<a id="value-6d61702d66696c652d6d6f642d74696d65"></a>

## `map-file-mod-time`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-file-mod-time</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFile</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Return the modification timestamp of a MapFile.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 59

<a id="value-6d61702d66696c652d6d6f6465"></a>

## `map-file-mode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-file-mode</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFile</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Return the mode bits of a MapFile.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 55

<a id="value-6d61702d66732d656d707479"></a>

## `map-fs-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-fs-empty</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFs</span></code></pre>

Create an empty in-memory filesystem containing only the synthesized root directory ".".

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 157

<a id="value-6d61702d66732d6572726f722d6d657373616765"></a>

## `map-fs-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-fs-error-message</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFsError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a human-readable diagnostic for a MapFsError.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 127

<a id="value-6d61702d66732d66726f6d2d6c697374"></a>

## `map-fs-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-fs-from-list</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFile</span>)) -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFs</span>)</code></pre>

Construct a MapFs from a list of (path, file) pairs, keeping later entries on duplicate keys.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 166

<a id="value-6d61702d66732d676c6f62"></a>

## `map-fs-glob`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-fs-glob</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFs</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFsError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))))</code></pre>

Find all paths in a MapFs matching a shell pattern in ascending lexical order.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 416

<a id="value-6d61702d66732d696e73657274"></a>

## `map-fs-insert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-fs-insert</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFs</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFile</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFs</span>)))</code></pre>

Insert or replace a single path entry in a MapFs, snapshotting its byte payload.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 161

<a id="value-6d61702d66732d6f70656e2d726561646572"></a>

## `map-fs-open-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-fs-open-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFs</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFsError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))))</code></pre>

Open a regular file in a MapFs as an Effect.Stream ByteReader.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 313

<a id="value-6d61702d66732d726561642d646972"></a>

## `map-fs-read-dir`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-fs-read-dir</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFs</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFsError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapDirEntry</span>))))</code></pre>

List direct children of a directory in ascending lexical order, deduplicating synthesized subdirectories.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 319

<a id="value-6d61702d66732d726561642d66696c65"></a>

## `map-fs-read-file`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-fs-read-file</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFs</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFsError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Read the complete byte contents of a regular file.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 297

<a id="value-6d61702d66732d73746174"></a>

## `map-fs-stat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-fs-stat</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFs</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFsError</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFileInfo</span>)))</code></pre>

Inspect metadata for a file or directory, synthesizing parent directories automatically.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 282

<a id="value-6d61702d66732d737562"></a>

## `map-fs-sub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-fs-sub</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFs</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFsError</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFs</span>)))</code></pre>

Project a subdirectory of a MapFs into a new MapFs rooted at that directory.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 360

<a id="value-746573742d6673"></a>

## `test-fs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">test-fs</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">MapFs</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) <span class="tok-constructor">Unit</span>)))</code></pre>

Run structural conformance checks on a MapFs and verify expected paths.
If expected is empty, the filesystem must contain no entries beneath root ".".
Otherwise, every listed path must exist and pass metadata, listing, read, glob,
subtree, and invalid-path rejection checks.

[Source](/reference-sources/Effect.Test.Fs.cinch) · line 469

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Test/Fs.cinch`. Source SHA-256: `93be07ac63625d40fdfb22be608d12888af50841bcbea5fef460a4016866a072`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
