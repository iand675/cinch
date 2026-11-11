# Effect.Fs

Filesystem reads/writes, directory and metadata operations, with Path values and named Result errors.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Effect.Fs`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Fs.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Path](Data.Path.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Prim.Sys](Prim.Sys.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Fs: the filesystem capability (P34, the platform layer).

The operations read, write, list, stat and atomically rename
filesystem bytes, under the `Io r` capability in the mold of the
runtime's `Io r` operations (decision D45: the filesystem is
consulted at runtime through the capability, never ambient at
compile time; a `stage` body cannot reach it, see Effect.Clock's
header). Paths are `Data.Path` values; content is `Bytes`; every
failure is a NAMED `FsError` value in a `Result` (the house style:
`Text.Parse`'s named vocabulary shape), never a fault.

Read and write issue their calls in Cinch through `Prim.Sys`, the
facade over the platform layer: on a Darwin target it re-exports
`Prim.Sys.Darwin`, which binds `open`/`creat`/`read`/`write`/`close`/
`lseek` with `foreign import ccall`, and on a Linux target it
re-exports `Prim.Sys.Linux`, which carries the same entry points as
`prim` bodies that issue `syscall`. Both answer the same `SysError`
constructors, which map one for one onto the named constructors below
(`NotFound` ... `IoError`). The import line here is the same for every
target: the selection is the compile target's, made in the facade.

The remaining operations (atomic rename, metadata, the directory
listing) still cross through the runtime's `cinch_fs_*` word buffers
(`compiler/runtime/src/platform.rs`, the plain `foreign import ccall`
machinery): machine scalars only, so paths and content cross through
word buffers filled and drained one byte at a time below. The error
codes those words return map onto the same named constructors, so the
two routes present one vocabulary.

Named refusals (boundaries this module deliberately does not
cross):
  non-atomic write: `fs-write` creates or truncates and is NOT
    atomic; atomic replacement is write-then-`fs-rename` (the probe
    in compiler/tests/platform.rs exercises exactly that);
  partial reads: `fs-read` reads the whole file or fails (there is
    no streaming reader in the bootstrap slice);
  permissions beyond the OS's own: the error vocabulary reports
    what the OS said and nothing is widened;
  recursive listing or deletion: `fs-list` names one directory's
    entries (sorted ascending for determinism), and nothing removes
    files;
  symlinks: the OS resolves them at the call (`fs-metadata` reports
    the target's kind).

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary.

</details>

# Types and constructors

<a id="type-46696c654b696e64"></a>

## `FileKind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">FileKind</span></code></pre>

What a path names.

[Source](/reference-sources/Effect.Fs.cinch) · line 80

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FileIsFile</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FileKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FileIsDirectory</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FileKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FileIsOther</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FileKind</span></code></pre>

<a id="type-46696c654d657461"></a>

## `FileMeta`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">FileMeta</span></code></pre>

A file's metadata: its kind and its size in bytes.

[Source](/reference-sources/Effect.Fs.cinch) · line 86

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FileMeta</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FileKind</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FileMeta</span>))</code></pre>

<a id="type-46734572726f72"></a>

## `FsError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">FsError</span></code></pre>

What went wrong, named (the `std::io::ErrorKind` vocabulary the
runtime maps its codes from).

[Source](/reference-sources/Effect.Fs.cinch) · line 70

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">NotFound</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FsError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PermissionDenied</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FsError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlreadyExists</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FsError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">NotADirectory</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FsError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsADirectory</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FsError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidName</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FsError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IoError</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FsError</span></code></pre>

# Values

<a id="value-66696c652d6d6574612d6b696e64"></a>

## `file-meta-kind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">file-meta-kind</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FileMeta</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FileKind</span>)</code></pre>

[Source](/reference-sources/Effect.Fs.cinch)

<a id="value-66696c652d6d6574612d73697a65"></a>

## `file-meta-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">file-meta-size</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FileMeta</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Effect.Fs.cinch)

<a id="value-66732d6c697374"></a>

## `fs-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fs-list</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Path</span>.<span class="tok-constructor">Path</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FsError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

The names in the directory `p`, sorted ascending (the OS listing
order is not specified; the sorted listing is this home's
observable order).

[Source](/reference-sources/Effect.Fs.cinch) · line 301

<a id="value-66732d6d65746164617461"></a>

## `fs-metadata`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fs-metadata</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Path</span>.<span class="tok-constructor">Path</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FsError</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FileMeta</span>))</code></pre>

The kind and size of what `p` names.

[Source](/reference-sources/Effect.Fs.cinch) · line 275

<a id="value-66732d72656164"></a>

## `fs-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fs-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Path</span>.<span class="tok-constructor">Path</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FsError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

The whole file at `p`, or the named error (missing paths report
`NotFound`).

The calls are `Prim.Sys`'s: open, the size from `lseek`, one buffer
that large, then a read loop to the buffer's end, then close. The
size is the file's length when the read starts, so a file that grows
while it is being read is read to that length; a file that shrank is
read a second time at its new length, so the answer is exactly what
the kernel produced either way.

[Source](/reference-sources/Effect.Fs.cinch) · line 187

<a id="value-66732d72656e616d65"></a>

## `fs-rename`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fs-rename</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Path</span>.<span class="tok-constructor">Path</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Path</span>.<span class="tok-constructor">Path</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FsError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Atomically rename `from` to `to` (POSIX `rename`: the target sees
the whole old or the whole new content, never a partial).

[Source](/reference-sources/Effect.Fs.cinch) · line 262

<a id="value-66732d7772697465"></a>

## `fs-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fs-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Path</span>.<span class="tok-constructor">Path</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FsError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Write `content` to `p`, creating or truncating it (NOT atomic;
atomic replacement is `fs-rename`). The calls are `Prim.Sys`'s:
`creat` (the fixed argument form of the creating open), a write loop
(a write may be short), then close.

[Source](/reference-sources/Effect.Fs.cinch) · line 242

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Fs.cinch`. Source SHA-256: `ecc727aab81580727c460bd4188cac015bd581bc500dec690ca1eef9cd3d543f`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
