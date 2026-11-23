# Text.Time.Scan

Shared byte cursor, digit readers and digit writers for the time text modules.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Time.Scan`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Time.Scan.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Text.Time.Scan](Text.Time.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-7363616e2d627974652d6174"></a>

## `scan-byte-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-byte-at</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

Reading.

[Source](/reference-sources/Text.Time.Scan.cinch) · line 88

<a id="value-7363616e2d62797465732d6f662d737472696e67"></a>

## `scan-bytes-of-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-bytes-of-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The UTF-8 bytes of a string: one copy (see the header).

[Source](/reference-sources/Text.Time.Scan.cinch) · line 71

<a id="value-7363616e2d636f756e742d646967697473"></a>

## `scan-count-digits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-count-digits</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)))</code></pre>

The length of the run of digits at `i`, at most `limit`.

[Source](/reference-sources/Text.Time.Scan.cinch) · line 126

<a id="value-7363616e2d64696769742d6174"></a>

## `scan-digit-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-digit-at</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Text.Time.Scan.cinch)

<a id="value-7363616e2d646967697473"></a>

## `scan-digits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-digits</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)))</code></pre>

Exactly `n` digits at `i` as a number, -1 if one is missing. The
accumulator is below 10^n <= 10^18, so the wrapping operations never
wrap for n <= 18; a larger `n` is refused with -1.

[Source](/reference-sources/Text.Time.Scan.cinch) · line 114

<a id="value-7363616e2d6672616374696f6e2d646967697473"></a>

## `scan-fraction-digits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-fraction-digits</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">9</span> <span class="tok-constructor">I64</span>))</code></pre>

The number of fraction digits the shortest exact rendering of a
subsecond nanosecond count needs: 0 for zero, otherwise 9 less the
trailing zeros.

[Source](/reference-sources/Text.Time.Scan.cinch) · line 240

<a id="value-7363616e2d6672616374696f6e2d68656164"></a>

## `scan-fraction-head`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-fraction-head</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

The first `digits` digits of a nine-digit subsecond (truncation);
all nine for `digits` of 9 or more, the first for 1 or less.

[Source](/reference-sources/Text.Time.Scan.cinch) · line 255

<a id="value-7363616e2d69732d7370616365"></a>

## `scan-is-space`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-is-space</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

ASCII white space: space, tab, LF, VT, FF, CR.

[Source](/reference-sources/Text.Time.Scan.cinch) · line 108

<a id="value-7363616e2d6c656e677468"></a>

## `scan-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-length</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Text.Time.Scan.cinch)

<a id="value-7363616e2d6c6f7765722d6174"></a>

## `scan-lower-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-lower-at</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

The byte at `i` with ASCII upper case letters folded to lower case;
other bytes (and -1) unchanged.

[Source](/reference-sources/Text.Time.Scan.cinch) · line 102

<a id="value-7363616e2d6e6577"></a>

## `scan-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-new</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

A zeroed buffer of `n` bytes (none for a negative `n`).

[Source](/reference-sources/Text.Time.Scan.cinch) · line 140

<a id="value-7363616e2d706f773130"></a>

## `scan-pow10`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-pow10</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

10^k for k in 0..=9; 1 below 0 and 10^9 above 9.

[Source](/reference-sources/Text.Time.Scan.cinch) · line 224

<a id="value-7363616e2d7075742d62797465"></a>

## `scan-put-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-put-byte</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)))</code></pre>

Writes the low eight bits of `v` at `i`; answers `i + 1`. Positions
are far below 2^62, so the increment does not wrap.

[Source](/reference-sources/Text.Time.Scan.cinch) · line 145

<a id="value-7363616e2d7075742d6279746573"></a>

## `scan-put-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-put-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)))))</code></pre>

Copies `n` bytes of `src` from `si` to `dst` at `di`; answers
`di + n`. A range that does not fit either buffer copies nothing.

[Source](/reference-sources/Text.Time.Scan.cinch) · line 176

<a id="value-7363616e2d7075742d646967697473"></a>

## `scan-put-digits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-put-digits</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))))</code></pre>

`v` as exactly `width` decimal digits at `i`; answers `i + width`.
Written from the least significant digit backward. A negative `v`
is written as zeros.

[Source](/reference-sources/Text.Time.Scan.cinch) · line 159

<a id="value-7363616e2d71756f74"></a>

## `scan-quot`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-quot</span> :: (<span class="tok-constructor">I64</span> -&gt; ((<span class="tok-variable">d</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">2</span> <span class="tok-number">9223372036854775807</span> <span class="tok-constructor">I64</span>)) -&gt; <span class="tok-constructor">I64</span>))</code></pre>

Arithmetic.

[Source](/reference-sources/Text.Time.Scan.cinch) · line 213

<a id="value-7363616e2d72656d"></a>

## `scan-rem`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-rem</span> :: (<span class="tok-constructor">I64</span> -&gt; ((<span class="tok-variable">d</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">2</span> <span class="tok-number">9223372036854775807</span> <span class="tok-constructor">I64</span>)) -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Text.Time.Scan.cinch)

<a id="value-7363616e2d736c696365"></a>

## `scan-slice`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-slice</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bytes</span>)))</code></pre>

The bytes `start .. end` of `b` as a fresh buffer; the empty buffer
when the range is empty or does not lie inside `b`.

[Source](/reference-sources/Text.Time.Scan.cinch) · line 200

<a id="value-7363616e2d737472696e672d6f662d6279746573"></a>

## `scan-string-of-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-string-of-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The string a printer's buffer holds. The printers write ASCII only,
so the validation never fails; the fallback is the empty string.

[Source](/reference-sources/Text.Time.Scan.cinch) · line 76

<a id="value-7363616e2d74616b65"></a>

## `scan-take`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-take</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

The first `n` bytes of `b` as a fresh buffer (all of it when `n` is
its length or more).

[Source](/reference-sources/Text.Time.Scan.cinch) · line 190

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Time/Scan.cinch`. Source SHA-256: `47ad87f939deb5508bdab9756d9ecfc9358a47c3a11b12719065dab101aaaf2c`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
