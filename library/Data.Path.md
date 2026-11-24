# Data.Path

Platform path values and composition/conversion operations used by filesystem and process APIs.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Path</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Path/source/">View source</a><a href="/packages/search/?q=%2BData.Path">Search module</a><a href="/reference-sources/Data.Path.cinch" download="Data.Path.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Semigroup](Algebra.Semigroup.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Path: filesystem paths (P34, the platform layer).

A `Path` names filesystem bytes: the text of a POSIX path, carried
whole (no canonicalization, no separator rewriting). This replaces
the placeholder `Path :: Nat -> Path` that Effect.Io carried (an
index into the runtime's input source): a real path names BYTES ON
DISK, and the platform homes (Effect.Fs, Effect.Process) hand its
text to the OS as UTF-8 bytes across the foreign boundary.

Named refusals (boundaries this module deliberately does not
cross):
  path canonicalization: `path-join` concatenates with one `/` and
    never resolves `.`/`..`; `path-normalize` resolves `.`/`..`
    LEXICALLY (text in, text out, never a disk call and never a
    symlink resolution, which is the OS's job at the call: the
    error vocabulary reports what it found);
  Windows separators: the bootstrap's platform layer is POSIX (`/`
    is the only separator, and `\` is an ordinary character);
  non-UTF-8 path text: the carrier is `String` (UTF-8), so the
    platform layer's byte crossing is UTF-8 bytes;
  separator collapsing outside `path-normalize`: `path-join` puts
    exactly one `/` between two texts and the component readers
    (`path-basename`/`path-dirname`) drop trailing separators, but
    only `path-normalize` drops EMPTY components, so `a//b` stays
    `a//b` until it is normalized.

Module scheme (design/base-library.md): `Data.*` is data types; the
path's home carries its conversions, its join, and the pure
Pathname operations (the I/O half is `Effect.FileUtils`). The
Pathname parity target is `design/stdlib-parity-ruby.md`'s P40 OS
sweep; these are the operations that cost no I/O.

</details>

<details class="doc-synopsis" open><summary>Synopsis (11 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Path">Path</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Path</strong></code></li><li><a class="syn-name" href="#v-path-basename">path-basename</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-basename</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-path-dirname">path-dirname</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-dirname</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-path-extname">path-extname</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-extname</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-path-from-string">path-from-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a></code></li><li><a class="syn-name" href="#v-path-is-absolute">path-is-absolute</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-is-absolute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-path-join">path-join</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-join</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a></code></li><li><a class="syn-name" href="#v-path-normalize">path-normalize</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-normalize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a></code></li><li><a class="syn-name" href="#v-path-relative-to">path-relative-to</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-relative-to</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a></code></li><li><a class="syn-name" href="#v-path-stem">path-stem</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-stem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-path-to-string">path-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">path-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Path">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Path</strong></code><span class="doc-sig-links"><a href="#t-Path" title="Permalink">#</a><a href="/packages/stdlib/Data.Path/source/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

The path text (POSIX; UTF-8 at the boundary).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Path"><code><strong class="tok-constructor">Path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-path-basename">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-basename</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-path-basename" title="Permalink">#</a><a href="/packages/stdlib/Data.Path/source/#L161" title="View source at line 161">Source</a></span></div>
</article>

<article class="doc-entry" id="v-path-dirname">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-dirname</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-path-dirname" title="Permalink">#</a><a href="/packages/stdlib/Data.Path/source/#L164" title="View source at line 164">Source</a></span></div>
</article>

<article class="doc-entry" id="v-path-extname">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-extname</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-path-extname" title="Permalink">#</a><a href="/packages/stdlib/Data.Path/source/#L185" title="View source at line 185">Source</a></span></div>
</article>

<article class="doc-entry" id="v-path-from-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a></code><span class="doc-sig-links"><a href="#v-path-from-string" title="Permalink">#</a><a href="/packages/stdlib/Data.Path/source/#L52" title="View source at line 52">Source</a></span></div>
<div class="doc-body">

A path from its text.

</div>
</article>

<article class="doc-entry" id="v-path-is-absolute">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-is-absolute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-path-is-absolute" title="Permalink">#</a><a href="/packages/stdlib/Data.Path/source/#L203" title="View source at line 203">Source</a></span></div>
<div class="doc-body">

`path-is-absolute`: the path begins with `/` (the empty path is
not absolute).

</div>
</article>

<article class="doc-entry" id="v-path-join">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-join</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a></code><span class="doc-sig-links"><a href="#v-path-join" title="Permalink">#</a><a href="/packages/stdlib/Data.Path/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

The path `a/b`: exactly one `/` between the two texts (no
separator collapsing; see the refusals above).

</div>
</article>

<article class="doc-entry" id="v-path-normalize">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-normalize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a></code><span class="doc-sig-links"><a href="#v-path-normalize" title="Permalink">#</a><a href="/packages/stdlib/Data.Path/source/#L277" title="View source at line 277">Source</a></span></div>
<div class="doc-body">

`path-normalize`: LEXICAL cleanup (text in, text out; no disk call
and no symlink resolution). Split on `/`, drop empty and `.`
segments, resolve each `..` as above; join with `/` and prefix
`/` when the input began with `/`.

</div>
</article>

<article class="doc-entry" id="v-path-relative-to">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-relative-to</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a></code><span class="doc-sig-links"><a href="#v-path-relative-to" title="Permalink">#</a><a href="/packages/stdlib/Data.Path/source/#L309" title="View source at line 309">Source</a></span></div>
<div class="doc-body">

`path-relative-to target base`: the relative path from `base` to
`target` under the same lexical form. Both are normalized first.
When their absoluteness differs no common prefix exists and the
normalized target is returned; otherwise the shared leading
components are dropped, one `..` is emitted per remaining base
component, and the remaining target components follow (nothing
remaining is `.`).

</div>
</article>

<article class="doc-entry" id="v-path-stem">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-stem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-path-stem" title="Permalink">#</a><a href="/packages/stdlib/Data.Path/source/#L198" title="View source at line 198">Source</a></span></div>
</article>

<article class="doc-entry" id="v-path-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">path-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-path-to-string" title="Permalink">#</a><a href="/packages/stdlib/Data.Path/source/#L56" title="View source at line 56">Source</a></span></div>
<div class="doc-body">

The path's text.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Path.cinch`](/packages/stdlib/Data.Path/source/). Source SHA-256: `1ce780bc27ffa8bb5951ac340ecf80e5ec0ebd44d0da1ce2f042175966b34772`.
