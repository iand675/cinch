# Prim.Sys

The syscall layer for the compile target under one name: conditional imports and re-exports select Prim.Sys.Darwin on a Darwin target, Prim.Sys.Linux on a Linux target, and the portable floor everywhere else, so a caller imports one module and never names an operating system.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Prim.Sys`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Prim.Sys.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Prim.Sys.Darwin](Prim.Sys.Darwin.md), [Prim.Sys.Linux](Prim.Sys.Linux.md), [Prim.Sys.Portable](Prim.Sys.Portable.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Prim.Sys: the operating system's filesystem calls, whichever way this
target makes them.

The syscall layer has one module per implementation kind, not one per
concept: `Prim.Sys.Darwin` binds libSystem with `foreign import ccall`,
and `Prim.Sys.Linux` issues the `syscall` instruction from `prim`
bodies. Both export the same entry names and the same `SysError`
constructors, so a caller needs one name and not two.

This module is that one name. Its imports are conditional on the
compile target's operating system, and its re-exports are conditional
with them, so the surface here is exactly the selected backend's
surface: on a Darwin target `Prim.Sys` is `Prim.Sys.Darwin` name for
name, on a Linux target it is `Prim.Sys.Linux` name for name, and a
module importing `Prim.Sys` never has to know which one it got.

The selection is fixed at compile time against the artifact's target
and never against the build host (primitive operations and inline
target IR specification section 13; modules specification section 11),
so one compiler binary cross compiling for `x86_64-unknown-linux-gnu`
selects the Linux bodies while running on a Darwin machine.

A target with neither of those (wasm32, or a Windows profile) gets
`Prim.Sys.Portable`: the same names, the same types and the same
`SysError` constructors, with every entry point answering
`Err SysIoError`. A real backend for such a target joins the other two
and the facade selects it; the portable module stays the floor, so the
facade's surface is never empty and the prelude family checks for every
profile.

</details>

# Types and constructors

<a id="type-5379734572726f72"></a>

## `SysError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SysError</span></code></pre>

What the kernel said, named. The same constructors as the Darwin
module's, so a caller's mapping is written once.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 79

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysNotFound</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysPermissionDenied</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysAlreadyExists</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysNotADirectory</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysIsADirectory</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysInvalidName</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysIoError</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span></code></pre>

# Values

<a id="value-7379732d636c6f7365"></a>

## `sys-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Close `fd`.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 353

<a id="value-7379732d6370617468"></a>

## `sys-cpath`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-cpath</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

`s` as a C string, the same shape as the Darwin module's.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 311

<a id="value-7379732d637265617465"></a>

## `sys-create`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-create</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>))</code></pre>

Create or truncate `path` for writing: the descriptor.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 334

<a id="value-7379732d6572726f722d6f66"></a>

## `sys-error-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-error-of</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span>)</code></pre>

The kernel's negative return as the named vocabulary. The six numbers
are fixed by POSIX; anything else is `SysIoError`.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 288

<a id="value-7379732d66737461742d696e746f"></a>

## `sys-fstat-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-fstat-into</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>))))</code></pre>

Fill `buf` with the raw `struct stat` of `path`. The layout belongs
to the caller that reads the fields out of it.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 398

<a id="value-7379732d6673796e63"></a>

## `sys-fsync`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-fsync</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Flush `fd` to the storage device.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 391

<a id="value-7379732d676574706964"></a>

## `sys-getpid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-getpid</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">I64</span>)</code></pre>

This process's own id.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 407

<a id="value-7379732d6c7365656b"></a>

## `sys-lseek`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-lseek</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>))))</code></pre>

Move `fd`'s offset.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 359

<a id="value-7379732d6d6b646972"></a>

## `sys-mkdir`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-mkdir</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Create the directory `path`, one level.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 373

<a id="value-7379732d6f2d72646f6e6c79"></a>

## `sys-o-rdonly`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-o-rdonly</span> :: <span class="tok-constructor">I64</span></code></pre>

`O_RDONLY`.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 94

<a id="value-7379732d6f70656e2d72656164"></a>

## `sys-open-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-open-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>))</code></pre>

Open `path` for reading: the descriptor.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 328

<a id="value-7379732d72656164"></a>

## `sys-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>)))))</code></pre>

Read up to `count` bytes of `fd` into `buf` at `at`: the count read.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 340

<a id="value-7379732d72656e616d65"></a>

## `sys-rename`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-rename</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Rename `from` onto `dst`, atomically.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 363

<a id="value-7379732d7365656b2d656e64"></a>

## `sys-seek-end`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-seek-end</span> :: <span class="tok-constructor">I64</span></code></pre>

`SEEK_END`.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 111

<a id="value-7379732d7365656b2d736574"></a>

## `sys-seek-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-seek-set</span> :: <span class="tok-constructor">I64</span></code></pre>

`SEEK_SET`.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 107

<a id="value-7379732d756e6c696e6b"></a>

## `sys-unlink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-unlink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Remove the name `path` with `unlinkat`'s flag word: 0 removes a file
or a link, `sys-at-removedir` (0x200) an empty directory. The flag is
spelled out so the caller owns the choice.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 383

<a id="value-7379732d7772697465"></a>

## `sys-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>)))))</code></pre>

Write up to `count` bytes of `buf` from `at` to `fd`: the count
written.

[Source](/reference-sources/Prim.Sys.Linux.cinch) · line 347

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Prim/Sys.cinch`. Source SHA-256: `00b3342acf6fe183229cf1c2135fcaa3d16df0136a6ac827b925e2ca4a4aa79d`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
