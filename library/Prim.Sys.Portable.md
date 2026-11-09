# Prim.Sys.Portable

The floor under Prim.Sys: the same entry names and the same named errors as the real backends, with every entry point answering Err SysIoError. It is what a target with no backend yet gets, and it keeps the prelude surface from being empty there.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Prim.Sys.Portable`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Prim.Sys.Portable.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Prim.Sys.Portable: the filesystem entry points on a target that has no
syscall backend of its own.

`Prim.Sys` selects `Prim.Sys.Darwin` on a Darwin target and
`Prim.Sys.Linux` on a Linux target. Every other target reaches this
module instead, and it answers the same names with the same types and
the same `SysError` constructors, so the facade's surface is the same
shape everywhere and `Effect.Fs` checks for every target.

Nothing here issues a call. Every entry point answers `Err SysIoError`,
which is the honest report for "this target has no filesystem layer in
this build": a program that asks for a file gets a named error, never a
wrong answer and never a link failure. The bodies are pure Cinch with
no `prim` and no `foreign import`, so this module needs no target
support at all, and it stays checkable on every profile.

A target that grows a real backend gets its own module beside Darwin's
and Linux's, and the facade selects it; this module stays the floor.

</details>

# Types and constructors

<a id="type-5379734572726f72"></a>

## `SysError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SysError</span></code></pre>

What the operating system said, named. The same constructors as the
Darwin and the Linux modules, so a caller's mapping is written once.

[Source](/reference-sources/Prim.Sys.Portable.cinch) · line 36

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysNotFound</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Portable</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysPermissionDenied</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Portable</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysAlreadyExists</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Portable</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysNotADirectory</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Portable</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysIsADirectory</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Portable</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysInvalidName</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Portable</span>.<span class="tok-constructor">SysError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SysIoError</span> :: <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Portable</span>.<span class="tok-constructor">SysError</span></code></pre>

# Values

<a id="value-7379732d636c6f7365"></a>

## `sys-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Portable</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">Unit</span>))</code></pre>

[Source](/reference-sources/Prim.Sys.Portable.cinch)

<a id="value-7379732d637265617465"></a>

## `sys-create`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-create</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Portable</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Sys.Portable.cinch)

<a id="value-7379732d6572726f722d6f66"></a>

## `sys-error-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-error-of</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Portable</span>.<span class="tok-constructor">SysError</span>)</code></pre>

There is no error slot to read and no number to map: every call this
module answers already failed at the boundary.

[Source](/reference-sources/Prim.Sys.Portable.cinch) · line 47

<a id="value-7379732d6c7365656b"></a>

## `sys-lseek`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-lseek</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Portable</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>))))</code></pre>

[Source](/reference-sources/Prim.Sys.Portable.cinch)

<a id="value-7379732d6f70656e2d72656164"></a>

## `sys-open-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-open-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Portable</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Sys.Portable.cinch)

<a id="value-7379732d72656164"></a>

## `sys-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Portable</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>)))))</code></pre>

[Source](/reference-sources/Prim.Sys.Portable.cinch)

<a id="value-7379732d7365656b2d656e64"></a>

## `sys-seek-end`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-seek-end</span> :: <span class="tok-constructor">I64</span></code></pre>

[Source](/reference-sources/Prim.Sys.Portable.cinch)

<a id="value-7379732d7365656b2d736574"></a>

## `sys-seek-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-seek-set</span> :: <span class="tok-constructor">I64</span></code></pre>

[Source](/reference-sources/Prim.Sys.Portable.cinch)

<a id="value-7379732d7772697465"></a>

## `sys-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sys-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Portable</span>.<span class="tok-constructor">SysError</span> <span class="tok-constructor">I64</span>)))))</code></pre>

[Source](/reference-sources/Prim.Sys.Portable.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Prim/Sys/Portable.cinch`. Source SHA-256: `2a7ccce7586ac23a9c7b6c10e64a5164032c75691c4fb446ff314894a8b73d7c`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
