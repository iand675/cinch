# Data.StringIo

In-memory string-oriented I/O utilities with explicit state and error results.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Data.StringIo</code></div><div class="doc-mod-bar-links"><a href="/library/source/Data.StringIo/">View source</a><a href="/library/search/?q=%2BData.StringIo">Search module</a><a href="/reference-sources/Data.StringIo.cinch" download="Data.StringIo.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Result](Data.Result.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Data.StringIo`: [Data.StringIo](Data.StringIo.md).

```sh
cp seed/prelude/Data/StringIo.cinch ./Data.StringIo.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Data.StringIo: Ruby's StringIO as an in-memory byte buffer with a
read/write cursor.

Module scheme (design/base-library.md): `Data.*` is the home of a
concrete type, and this is a concrete in-memory buffer over
`Data.Bytes`; the ported Ruby item is `StringIO`, the
collections/text-utilities bullet of design/stdlib-parity-ruby.md.
The shape is exactly the byte-level one Ruby's `StringIO` has: one
buffer plus a cursor, where `write` overwrites at the cursor and
extends the buffer, `read` consumes up to n bytes and advances,
`read-line` stops after the next LF, and the buffer is observable
(`len`, `pos`, `eof`) and its position movable (`rewind`, `seek`).
The carrier split is decision D88's: `StringIo` is bytes, and
`Data.String` owns the UTF-8 interpretation.

DESIGN LAW SHAPE (the value contract, and the one deviation from
Ruby): a `StringIo` is a VALUE, not Ruby's mutable object. Every
operation returns a handle and leaves the one it was given
unchanged, so a retained handle keeps reading what it read before:
  string-io-to-bytes (string-io-from-bytes b) == b   for every Bytes
  string-io-read (string-io-write io bs) (bytes-len bs)
    == (bs, cursor just past bs)
  string-io-rewind io reads again what io read
and the cursor invariant `0 <= pos <= length` holds by construction,
which is what every guarded subtraction below relies on (`Nat`
subtraction below zero is a fault, exit 71). The buffer is never
written after it is built, so two handles that share one (from
`rewind`, `seek`, `read` or a no-op `truncate`) are
indistinguishable.
A write never mutates in place: a `Bytes` handle stays reachable
through every binding that names it (binders default to `%many`), so
an in-place write would show through a retained handle. The value
contract reallocates the grown buffer instead, which costs
O(length + written) per write; a build-once/read-many use pays it
once.

Named refusals (boundaries this module deliberately does not cross):
  the `IO` inheritance: no file descriptor, no `open`/`close`, no
    blocking read, no `StringIO.for_fd`; this is a buffer, not a
    handle to a stream;
  encoding handling: the buffer is bytes. Ruby's `StringIO` carries
    an external/internal encoding pair and transcodes as it reads
    and writes; here the bytes are opaque, and the `String`
    conversion belongs to `Data.String`;
  `each_line` with a custom separator: `string-io-read-line` splits
    on LF only (Ruby's default record separator); the separator
    argument is not here, and a caller wanting another one reads and
    scans for it;
  seeking past the end: `string-io-seek` refuses a target past the
    buffer length with `SeekOutOfRange` rather than padding (Ruby
    allows the seek and the read then returns nothing); a negative
    target cannot be written down at all, because the cursor is a
    `Nat`;
  the `?` suffix: Ruby's `eof?` predicate is `string-io-eof` here.
    An identifier's characters are letters, digits, `_` and `'` (with
    a dash gluing two words), and `?` is an operator character, so no
    name in the language carries a question mark; Data.List's `null`
    is the standing precedent for a Bool-valued name without one.

</details>

<details class="doc-synopsis" open><summary>Synopsis (14 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-StringIo">StringIo</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">StringIo</strong></code></li><li><a class="syn-name" href="#t-StringIoError">StringIoError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">StringIoError</strong></code></li><li><a class="syn-name" href="#v-string-io-eof">string-io-eof</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-io-eof</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-string-io-from-bytes">string-io-from-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-io-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a></code></li><li><a class="syn-name" href="#v-string-io-len">string-io-len</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-io-len</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-string-io-new">string-io-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-io-new</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a></code></li><li><a class="syn-name" href="#v-string-io-pos">string-io-pos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-io-pos</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-string-io-read">string-io-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-io-read</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a>)</code></li><li><a class="syn-name" href="#v-string-io-read-line">string-io-read-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-io-read-line</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a>)</code></li><li><a class="syn-name" href="#v-string-io-rewind">string-io-rewind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-io-rewind</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a></code></li><li><a class="syn-name" href="#v-string-io-seek">string-io-seek</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-io-seek</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIoError" title="Data.StringIo.StringIoError">StringIoError</a> <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a></code></li><li><a class="syn-name" href="#v-string-io-to-bytes">string-io-to-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-io-to-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-string-io-truncate">string-io-truncate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-io-truncate</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a></code></li><li><a class="syn-name" href="#v-string-io-write">string-io-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-io-write</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-StringIo">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">StringIo</strong></code><span class="doc-sig-links"><a href="#t-StringIo" title="Permalink">#</a><a href="/library/source/Data.StringIo/#L79" title="View source at line 79">Source</a></span></div>
<div class="doc-body">

The buffer and the cursor. The representation is module-private: the
constructor is deliberately NOT the type's name (`IoBuf`, not
`StringIo`, the shape of Data.String's `StrBuf`), because a name
that is both a type and its constructor is exported as one item and
the export would hand out the representation. Every handle comes
from `string-io-new` or `string-io-from-bytes`, so the cursor
invariant (0 <= pos <= bytes-len buffer) holds by construction and
a caller cannot put the cursor outside it.

</div>
</article>

<article class="doc-entry" id="t-StringIoError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">StringIoError</strong></code><span class="doc-sig-links"><a href="#t-StringIoError" title="Permalink">#</a><a href="/library/source/Data.StringIo/#L84" title="View source at line 84">Source</a></span></div>
<div class="doc-body">

The named seek refusal: the requested target and the length it
exceeds.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SeekOutOfRange"><code><strong class="tok-constructor">SeekOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIoError" title="Data.StringIo.StringIoError">StringIoError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-string-io-eof">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-io-eof</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-string-io-eof" title="Permalink">#</a><a href="/library/source/Data.StringIo/#L132" title="View source at line 132">Source</a></span></div>
<div class="doc-body">

At or past the last byte: a read returns nothing and the cursor
does not move.

</div>
</article>

<article class="doc-entry" id="v-string-io-from-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-io-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a></code><span class="doc-sig-links"><a href="#v-string-io-from-bytes" title="Permalink">#</a><a href="/library/source/Data.StringIo/#L110" title="View source at line 110">Source</a></span></div>
<div class="doc-body">

The conversion in copies (the wrap/copy discipline Data.String
states for its backing): the caller keeps its mutable buffer and a
later `bytes-set` on it never shows through the handle. The cursor
starts at the beginning.

</div>
</article>

<article class="doc-entry" id="v-string-io-len">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-io-len</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-string-io-len" title="Permalink">#</a><a href="/library/source/Data.StringIo/#L124" title="View source at line 124">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-io-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-io-new</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a></code><span class="doc-sig-links"><a href="#v-string-io-new" title="Permalink">#</a><a href="/library/source/Data.StringIo/#L103" title="View source at line 103">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-io-pos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-io-pos</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-string-io-pos" title="Permalink">#</a><a href="/library/source/Data.StringIo/#L127" title="View source at line 127">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-io-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-io-read</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a>)</code><span class="doc-sig-links"><a href="#v-string-io-read" title="Permalink">#</a><a href="/library/source/Data.StringIo/#L162" title="View source at line 162">Source</a></span></div>
<div class="doc-body">

Read up to n bytes from the cursor, advancing by the number
consumed. At or past the end the result is the empty buffer and the
cursor does not move (`string-io-eof` is how a caller sees it;
Ruby's nil is not spelled here).

</div>
</article>

<article class="doc-entry" id="v-string-io-read-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-io-read-line</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a>)</code><span class="doc-sig-links"><a href="#v-string-io-read-line" title="Permalink">#</a><a href="/library/source/Data.StringIo/#L167" title="View source at line 167">Source</a></span></div>
<div class="doc-body">

Read through the next LF (0x0A), inclusive; the last line without a
terminator yields the rest of the buffer.

</div>
</article>

<article class="doc-entry" id="v-string-io-rewind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-io-rewind</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a></code><span class="doc-sig-links"><a href="#v-string-io-rewind" title="Permalink">#</a><a href="/library/source/Data.StringIo/#L198" title="View source at line 198">Source</a></span></div>
<div class="doc-body">

The cursor to the start: what was read is read again.

</div>
</article>

<article class="doc-entry" id="v-string-io-seek">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-io-seek</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIoError" title="Data.StringIo.StringIoError">StringIoError</a> <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a></code><span class="doc-sig-links"><a href="#v-string-io-seek" title="Permalink">#</a><a href="/library/source/Data.StringIo/#L206" title="View source at line 206">Source</a></span></div>
<div class="doc-body">

The cursor to an absolute byte offset. A target past the buffer
length is the named refusal: this module never pads, so the cursor
would leave the invariant 0 <= pos <= length that the rest of the
module relies on. A negative target cannot be written down (the
cursor is a `Nat`).

</div>
</article>

<article class="doc-entry" id="v-string-io-to-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-io-to-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-string-io-to-bytes" title="Permalink">#</a><a href="/library/source/Data.StringIo/#L117" title="View source at line 117">Source</a></span></div>
<div class="doc-body">

The conversion out copies too, for the same reason.

</div>
</article>

<article class="doc-entry" id="v-string-io-truncate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-io-truncate</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a></code><span class="doc-sig-links"><a href="#v-string-io-truncate" title="Permalink">#</a><a href="/library/source/Data.StringIo/#L215" title="View source at line 215">Source</a></span></div>
<div class="doc-body">

Shrink the buffer to at most n bytes (a larger n is a no-op: this
never extends, matching Ruby's `truncate`). The cursor is clamped
into the new length, which keeps 0 <= pos <= length.

</div>
</article>

<article class="doc-entry" id="v-string-io-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-io-write</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.StringIo/#t-StringIo" title="Data.StringIo.StringIo">StringIo</a></code><span class="doc-sig-links"><a href="#v-string-io-write" title="Permalink">#</a><a href="/library/source/Data.StringIo/#L140" title="View source at line 140">Source</a></span></div>
<div class="doc-body">

Write at the cursor: `bs` overwrites the bytes already there and
extends the buffer when it runs past the end; the bytes after the
write stay where they were (Ruby's overwrite, never an insert).
The cursor lands just past the last byte written, so a write of no
bytes leaves buffer and cursor unchanged.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/StringIo.cinch`](/library/source/Data.StringIo/). Source SHA-256: `9eaf153aa82781838f853ea18293df64e0bda38a2055952c74da9dd84e94c4e1`.
