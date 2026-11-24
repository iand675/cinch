# Text.Time.Scan

Shared byte cursor, digit readers and digit writers for the time text modules.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Time.Scan</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Time.Scan/source/">View source</a><a href="/packages/search/?q=%2BText.Time.Scan">Search module</a><a href="/reference-sources/Text.Time.Scan.cinch" download="Text.Time.Scan.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Text.Time.Scan`: [Text.Time.Scan](Text.Time.Scan.md).

```sh
cp seed/prelude/Text/Time/Scan.cinch ./Text.Time.Scan.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Text.Time.Scan: the byte cursor, digit readers and digit writers the
datetime text modules share (Text.Time.Iso, Text.Time.Strtime, and
the later RFC 9557 and duration modules).

Design law shape. An input is a `Bytes` and a cursor is a plain `I64`
byte position; nothing here allocates per byte read, and nothing
decodes to `Char`.
  - `scan-byte-at b i` is the byte at `i` as 0..=255, and -1 for every
    position outside the buffer (negative or past the end). The end
    of input is therefore one more "byte value" a state machine
    branches on, and no read can fault.
  - `scan-digit-at b i` is 0..=9 for an ASCII digit and -1 otherwise
    (including the end).
  - `scan-digits b i n` reads exactly `n` digits as a number, or -1 if
    any of the `n` bytes is not a digit. `n` is at most 18 so the
    value fits an `I64` (the callers pass at most 9).
  - `scan-count-digits b i limit` counts the run of digits at `i`,
    stopping at `limit`.
  - `scan-fraction-digits`/`scan-fraction-head` are the two halves of
    printing a subsecond: how many digits are exact, and the leading
    digits of the nine.
  - `scan-put-byte b i v` writes one byte and answers `i + 1`;
    `scan-put-digits b i width v` writes `v` (non-negative) as exactly
    `width` decimal digits, most significant first, keeping the low
    `width` digits. A write outside the buffer is skipped, never a
    fault; the callers size their buffers exactly, so no write is
    skipped in practice.

String views. `scan-bytes-of-string` is `Data.String.string-to-bytes`:
ONE `memmove` copy of the backing. `Data.String` keeps the backing
private on purpose (its immutability contract: "every conversion out
of a String copies"), so no no-copy byte view of a `String` exists
for a module outside `Data.String`. `scan-string-of-bytes` is
`string-from-bytes`: one UTF-8 validation pass and one copy.

NAMED REFUSALS:
  - a no-copy view of a `String` as bytes: not available (above). A
    parse costs one copy of the input; a print costs one copy of the
    output buffer.
  - machine-indexed byte access: `bytes-get`/`bytes-set` take a `Nat`
    index, so every byte read converts the `I64` cursor with
    `nat-from-i64` inside `scan-byte-at`. The cursor itself, and all
    arithmetic on it, stays an `I64`.
  - a total machine division: `scan-quot`/`scan-rem` take the divisor
    at `InRange 2 9223372036854775807 I64` (the `Data.Time.Date`
    convention), which excludes both `None` cases of the checked
    operations.

</details>

<details class="doc-synopsis"><summary>Synopsis (20 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-scan-byte-at">scan-byte-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-byte-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-scan-bytes-of-string">scan-bytes-of-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-bytes-of-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-scan-count-digits">scan-count-digits</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-count-digits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-scan-digit-at">scan-digit-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-digit-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-scan-digits">scan-digits</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-digits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-scan-fraction-digits">scan-fraction-digits</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-fraction-digits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-scan-fraction-head">scan-fraction-head</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-fraction-head</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-scan-is-space">scan-is-space</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-is-space</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-scan-length">scan-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-scan-lower-at">scan-lower-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-lower-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-scan-new">scan-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-scan-pow10">scan-pow10</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-pow10</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-scan-put-byte">scan-put-byte</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-put-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-scan-put-bytes">scan-put-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-put-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-scan-put-digits">scan-put-digits</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-put-digits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-scan-quot">scan-quot</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-quot</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">d</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 2 9223372036854775807 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-scan-rem">scan-rem</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-rem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">d</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 2 9223372036854775807 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-scan-slice">scan-slice</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-slice</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-scan-string-of-bytes">scan-string-of-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-string-of-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-scan-take">scan-take</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-take</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-scan-byte-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-byte-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-byte-at" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L88" title="View source at line 88">Source</a></span></div>
<div class="doc-body">

Reading.

</div>
</article>

<article class="doc-entry" id="v-scan-bytes-of-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-bytes-of-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-scan-bytes-of-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L71" title="View source at line 71">Source</a></span></div>
<div class="doc-body">

The UTF-8 bytes of a string: one copy (see the header).

</div>
</article>

<article class="doc-entry" id="v-scan-count-digits">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-count-digits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-count-digits" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L126" title="View source at line 126">Source</a></span></div>
<div class="doc-body">

The length of the run of digits at `i`, at most `limit`.

</div>
</article>

<article class="doc-entry" id="v-scan-digit-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-digit-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-digit-at" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L95" title="View source at line 95">Source</a></span></div>
</article>

<article class="doc-entry" id="v-scan-digits">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-digits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-digits" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L114" title="View source at line 114">Source</a></span></div>
<div class="doc-body">

Exactly `n` digits at `i` as a number, -1 if one is missing. The
accumulator is below 10^n <= 10^18, so the wrapping operations never
wrap for n <= 18; a larger `n` is refused with -1.

</div>
</article>

<article class="doc-entry" id="v-scan-fraction-digits">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-fraction-digits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-fraction-digits" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L240" title="View source at line 240">Source</a></span></div>
<div class="doc-body">

The number of fraction digits the shortest exact rendering of a
subsecond nanosecond count needs: 0 for zero, otherwise 9 less the
trailing zeros.

</div>
</article>

<article class="doc-entry" id="v-scan-fraction-head">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-fraction-head</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-fraction-head" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L255" title="View source at line 255">Source</a></span></div>
<div class="doc-body">

The first `digits` digits of a nine-digit subsecond (truncation);
all nine for `digits` of 9 or more, the first for 1 or less.

</div>
</article>

<article class="doc-entry" id="v-scan-is-space">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-is-space</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-scan-is-space" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L108" title="View source at line 108">Source</a></span></div>
<div class="doc-body">

ASCII white space: space, tab, LF, VT, FF, CR.

</div>
</article>

<article class="doc-entry" id="v-scan-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-length" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L81" title="View source at line 81">Source</a></span></div>
</article>

<article class="doc-entry" id="v-scan-lower-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-lower-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-lower-at" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

The byte at `i` with ASCII upper case letters folded to lower case;
other bytes (and -1) unchanged.

</div>
</article>

<article class="doc-entry" id="v-scan-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-scan-new" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L140" title="View source at line 140">Source</a></span></div>
<div class="doc-body">

A zeroed buffer of `n` bytes (none for a negative `n`).

</div>
</article>

<article class="doc-entry" id="v-scan-pow10">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-pow10</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-pow10" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L224" title="View source at line 224">Source</a></span></div>
<div class="doc-body">

10^k for k in 0..=9; 1 below 0 and 10^9 above 9.

</div>
</article>

<article class="doc-entry" id="v-scan-put-byte">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-put-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-put-byte" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L145" title="View source at line 145">Source</a></span></div>
<div class="doc-body">

Writes the low eight bits of `v` at `i`; answers `i + 1`. Positions
are far below 2^62, so the increment does not wrap.

</div>
</article>

<article class="doc-entry" id="v-scan-put-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-put-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-put-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L176" title="View source at line 176">Source</a></span></div>
<div class="doc-body">

Copies `n` bytes of `src` from `si` to `dst` at `di`; answers
`di + n`. A range that does not fit either buffer copies nothing.

</div>
</article>

<article class="doc-entry" id="v-scan-put-digits">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-put-digits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-put-digits" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L159" title="View source at line 159">Source</a></span></div>
<div class="doc-body">

`v` as exactly `width` decimal digits at `i`; answers `i + width`.
Written from the least significant digit backward. A negative `v`
is written as zeros.

</div>
</article>

<article class="doc-entry" id="v-scan-quot">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-quot</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">d</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 2 9223372036854775807 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-quot" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L213" title="View source at line 213">Source</a></span></div>
<div class="doc-body">

Arithmetic.

</div>
</article>

<article class="doc-entry" id="v-scan-rem">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-rem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">d</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 2 9223372036854775807 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-rem" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L218" title="View source at line 218">Source</a></span></div>
</article>

<article class="doc-entry" id="v-scan-slice">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-slice</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-scan-slice" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L200" title="View source at line 200">Source</a></span></div>
<div class="doc-body">

The bytes `start .. end` of `b` as a fresh buffer; the empty buffer
when the range is empty or does not lie inside `b`.

</div>
</article>

<article class="doc-entry" id="v-scan-string-of-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-string-of-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-scan-string-of-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L76" title="View source at line 76">Source</a></span></div>
<div class="doc-body">

The string a printer's buffer holds. The printers write ASCII only,
so the validation never fails; the fallback is the empty string.

</div>
</article>

<article class="doc-entry" id="v-scan-take">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-take</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-scan-take" title="Permalink">#</a><a href="/packages/stdlib/Text.Time.Scan/source/#L190" title="View source at line 190">Source</a></span></div>
<div class="doc-body">

The first `n` bytes of `b` as a fresh buffer (all of it when `n` is
its length or more).

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Time/Scan.cinch`](/packages/stdlib/Text.Time.Scan/source/). Source SHA-256: `47ad87f939deb5508bdab9756d9ecfc9358a47c3a11b12719065dab101aaaf2c`.
