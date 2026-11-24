# Text.Tar

Read V7, USTAR, PAX and GNU tape archives and write USTAR or PAX archives, with SIMD header checksums.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Tar`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Tar.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Tar](Text.Tar.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-546172456e747279"></a>

## `TarEntry`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TarEntry</span></code></pre>

One archive member: its header and its data (empty for links, directories and devices).

[Source](/reference-sources/Text.Tar.cinch) · line 92

<a id="type-5461724572726f72"></a>

## `TarError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TarError</span></code></pre>

A malformed archive or an entry that cannot be written.

[Source](/reference-sources/Text.Tar.cinch) · line 193

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TarBadHeader</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TarBadChecksum</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TarTruncated</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TarFieldTooLong</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TarBadPax</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarError</span>)</code></pre>

<a id="type-546172486561646572"></a>

## `TarHeader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TarHeader</span></code></pre>

Entry metadata: type byte, name, link target, mode, user and group ids, data size, modification
time in seconds since 1970, user and group names, device numbers, PAX records and the format read.

[Source](/reference-sources/Text.Tar.cinch) · line 87

# Values

<a id="value-7461722d626c6f636b"></a>

## `tar-block`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-block</span> :: <span class="tok-constructor">Nat</span></code></pre>

Block device, type byte '4'.

[Source](/reference-sources/Text.Tar.cinch) · line 50

<a id="value-7461722d63686172"></a>

## `tar-char`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-char</span> :: <span class="tok-constructor">Nat</span></code></pre>

Character device, type byte '3'.

[Source](/reference-sources/Text.Tar.cinch) · line 46

<a id="value-7461722d636f6e746967756f7573"></a>

## `tar-contiguous`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-contiguous</span> :: <span class="tok-constructor">Nat</span></code></pre>

Contiguous file, type byte '7', read as a regular file.

[Source](/reference-sources/Text.Tar.cinch) · line 62

<a id="value-7461722d6465766d616a6f72"></a>

## `tar-devmajor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-devmajor</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Device major number.

[Source](/reference-sources/Text.Tar.cinch) · line 153

<a id="value-7461722d6465766d696e6f72"></a>

## `tar-devminor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-devminor</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Device minor number.

[Source](/reference-sources/Text.Tar.cinch) · line 157

<a id="value-7461722d6469726563746f7279"></a>

## `tar-directory`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-directory</span> :: <span class="tok-constructor">Nat</span></code></pre>

Directory, type byte '5'.

[Source](/reference-sources/Text.Tar.cinch) · line 54

<a id="value-7461722d656e747279"></a>

## `tar-entry`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-entry</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarEntry</span>))</code></pre>

Pair a header with data; the written size is always the data length.

[Source](/reference-sources/Text.Tar.cinch) · line 101

<a id="value-7461722d656e7472792d64617461"></a>

## `tar-entry-data`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-entry-data</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarEntry</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

An entry's data.

[Source](/reference-sources/Text.Tar.cinch) · line 109

<a id="value-7461722d656e7472792d686561646572"></a>

## `tar-entry-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-entry-header</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarEntry</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span>)</code></pre>

An entry's header.

[Source](/reference-sources/Text.Tar.cinch) · line 105

<a id="value-7461722d6572726f722d6d657373616765"></a>

## `tar-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a tar error.

[Source](/reference-sources/Text.Tar.cinch) · line 206

<a id="value-7461722d6669666f"></a>

## `tar-fifo`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-fifo</span> :: <span class="tok-constructor">Nat</span></code></pre>

Named pipe, type byte '6'.

[Source](/reference-sources/Text.Tar.cinch) · line 58

<a id="value-7461722d666f726d6174"></a>

## `tar-format`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-format</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Format bits observed when reading; zero for headers built in memory.

[Source](/reference-sources/Text.Tar.cinch) · line 165

<a id="value-7461722d666f726d61742d676e75"></a>

## `tar-format-gnu`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-format-gnu</span> :: <span class="tok-constructor">Nat</span></code></pre>

Format bit: GNU extensions.

[Source](/reference-sources/Text.Tar.cinch) · line 82

<a id="value-7461722d666f726d61742d706178"></a>

## `tar-format-pax`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-format-pax</span> :: <span class="tok-constructor">Nat</span></code></pre>

Format bit: POSIX PAX extended headers.

[Source](/reference-sources/Text.Tar.cinch) · line 78

<a id="value-7461722d666f726d61742d7573746172"></a>

## `tar-format-ustar`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-format-ustar</span> :: <span class="tok-constructor">Nat</span></code></pre>

Format bit: POSIX USTAR.

[Source](/reference-sources/Text.Tar.cinch) · line 74

<a id="value-7461722d666f726d61742d7637"></a>

## `tar-format-v7`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-format-v7</span> :: <span class="tok-constructor">Nat</span></code></pre>

Format bit: original Unix V7 headers.

[Source](/reference-sources/Text.Tar.cinch) · line 70

<a id="value-7461722d676964"></a>

## `tar-gid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-gid</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Owner group id.

[Source](/reference-sources/Text.Tar.cinch) · line 133

<a id="value-7461722d676e616d65"></a>

## `tar-gname`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-gname</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Owner group name.

[Source](/reference-sources/Text.Tar.cinch) · line 149

<a id="value-7461722d686172646c696e6b"></a>

## `tar-hardlink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-hardlink</span> :: <span class="tok-constructor">Nat</span></code></pre>

Hard link to an earlier entry, type byte '1'.

[Source](/reference-sources/Text.Tar.cinch) · line 38

<a id="value-7461722d686561646572"></a>

## `tar-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-header</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span>))</code></pre>

A header of the given type and name with mode 0644 (0755 for directories) and every other field zero or empty.

[Source](/reference-sources/Text.Tar.cinch) · line 97

<a id="value-7461722d6b696e64"></a>

## `tar-kind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-kind</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Type byte.

[Source](/reference-sources/Text.Tar.cinch) · line 113

<a id="value-7461722d6c696e6b"></a>

## `tar-link`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-link</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Link target for hard and symbolic links.

[Source](/reference-sources/Text.Tar.cinch) · line 121

<a id="value-7461722d6d6f6465"></a>

## `tar-mode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-mode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Permission and mode bits.

[Source](/reference-sources/Text.Tar.cinch) · line 125

<a id="value-7461722d6d74696d65"></a>

## `tar-mtime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-mtime</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; <span class="tok-constructor">Int</span>)</code></pre>

Modification time in whole seconds since 1970-01-01 UTC.

[Source](/reference-sources/Text.Tar.cinch) · line 141

<a id="value-7461722d6e616d65"></a>

## `tar-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-name</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Entry name, after any USTAR prefix, GNU long name or PAX path.

[Source](/reference-sources/Text.Tar.cinch) · line 117

<a id="value-7461722d706178"></a>

## `tar-pax`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-pax</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>)))</code></pre>

PAX records that apply to this entry and are not represented by another field, in archive order.

[Source](/reference-sources/Text.Tar.cinch) · line 161

<a id="value-7461722d7061782d676c6f62616c"></a>

## `tar-pax-global`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-pax-global</span> :: <span class="tok-constructor">Nat</span></code></pre>

PAX global header, type byte 'g'. Read as an entry holding only its name and records, with no data.

[Source](/reference-sources/Text.Tar.cinch) · line 66

<a id="value-7461722d72656164"></a>

## `tar-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-read</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarEntry</span>)))</code></pre>

Read every entry of a complete archive, stopping at the first zero block (normally one of two).

[Source](/reference-sources/Text.Tar.cinch) · line 347

<a id="value-7461722d726567756c6172"></a>

## `tar-regular`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-regular</span> :: <span class="tok-constructor">Nat</span></code></pre>

Regular file, type byte '0'.

[Source](/reference-sources/Text.Tar.cinch) · line 34

<a id="value-7461722d73697a65"></a>

## `tar-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Data size in bytes as recorded in the archive.

[Source](/reference-sources/Text.Tar.cinch) · line 137

<a id="value-7461722d73796d6c696e6b"></a>

## `tar-symlink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-symlink</span> :: <span class="tok-constructor">Nat</span></code></pre>

Symbolic link, type byte '2'.

[Source](/reference-sources/Text.Tar.cinch) · line 42

<a id="value-7461722d756964"></a>

## `tar-uid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-uid</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Owner user id.

[Source](/reference-sources/Text.Tar.cinch) · line 129

<a id="value-7461722d756e616d65"></a>

## `tar-uname`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-uname</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Owner user name.

[Source](/reference-sources/Text.Tar.cinch) · line 145

<a id="value-7461722d776974682d646576696365"></a>

## `tar-with-device`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-with-device</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span>)))</code></pre>

Set device major and minor numbers.

[Source](/reference-sources/Text.Tar.cinch) · line 185

<a id="value-7461722d776974682d6c696e6b"></a>

## `tar-with-link`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-with-link</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span>))</code></pre>

Set the link target.

[Source](/reference-sources/Text.Tar.cinch) · line 169

<a id="value-7461722d776974682d6d6f6465"></a>

## `tar-with-mode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-with-mode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span>))</code></pre>

Set the mode bits.

[Source](/reference-sources/Text.Tar.cinch) · line 173

<a id="value-7461722d776974682d6d74696d65"></a>

## `tar-with-mtime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-with-mtime</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span>))</code></pre>

Set the modification time in seconds since 1970.

[Source](/reference-sources/Text.Tar.cinch) · line 181

<a id="value-7461722d776974682d6f776e6572"></a>

## `tar-with-owner`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-with-owner</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span>)))))</code></pre>

Set user and group ids and names.

[Source](/reference-sources/Text.Tar.cinch) · line 177

<a id="value-7461722d776974682d706178"></a>

## `tar-with-pax`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-with-pax</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>)) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarHeader</span>))</code></pre>

Set extra PAX records written with the entry (forcing a PAX header) and reported when read back.

[Source](/reference-sources/Text.Tar.cinch) · line 189

<a id="value-7461722d7772697465"></a>

## `tar-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tar-write</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarEntry</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Tar</span>.<span class="tok-constructor">TarError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Write entries as a complete archive ending in two zero blocks. Each entry is written as USTAR
when every field fits and otherwise with a preceding PAX extended header.

[Source](/reference-sources/Text.Tar.cinch) · line 567

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Tar.cinch`. Source SHA-256: `3c3fd9231f8b031fb2c9a56bf880aec842cd3f0ce3c59bcbc1ae1a30e10c6bdb`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
