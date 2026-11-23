# Data.Time.Zone.Tzif

Reading TZif time zone files (RFC 9636) into checked transition tables.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time.Zone.Tzif`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.Zone.Tzif.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md), [Data.Time.Zone.Types](Data.Time.Zone.Types.md), [Data.Time.Zone.Tzif](Data.Time.Zone.Tzif.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Time.Zone.Tzif: reading TZif files
(design/datetime.md, layer L2, lane C; RFC 9636).

TZif is the binary format of the IANA time zone database as
installed under /usr/share/zoneinfo. `parse-tzif name bytes` turns
one file's bytes into a `TimeZone` named `name`, or says by name
why it could not. It is pure: reading the file is
`Effect.Time.Zone`'s.

THE FORMAT, as read here (all integers big-endian):
  - a 44-byte header: the magic "TZif", a version byte (NUL for
    version 1, the digits '2', '3', '4' for the later ones), 15
    reserved bytes, and six 32-bit counts: isutcnt, isstdcnt,
    leapcnt, timecnt, typecnt, charcnt;
  - a data block: timecnt transition instants (Unix seconds, 32-bit
    signed), timecnt one-byte local time type indices, typecnt
    six-byte local time types (32-bit signed UTC offset, a daylight
    flag byte, a byte index into the abbreviations), charcnt bytes
    of NUL-terminated abbreviations, leapcnt leap second records,
    isstdcnt and isutcnt indicator bytes;
  - in version 2 and later, a second header and a second data
    block with 64-bit instants, then a footer: a newline, a POSIX
    TZ string (possibly empty) and a newline. The second block is
    the one used; the first is only skipped over.

Design law shape. The zone is the table the file states, under
`Data.Time.Zone`'s law: local time type 0 before the first
transition, each transition's type from it on, and the footer's
rule from the last transition on. Every field is read through one
bounds-checked reader after the block's whole extent has been
checked against the buffer's length, so no input, however
malformed, can make a read fall outside the buffer; a file is
either accepted whole or refused by name.

NAMED REFUSALS (each a `TzifError`):
  - fewer bytes than the header and counts require: `TzifTruncated`
    (also an empty input, and a version 2+ file cut anywhere in
    its second header, second block or footer).
  - a first or second header without the magic: `TzifBadMagic`.
  - a version byte other than NUL, '2', '3', '4':
    `TzifUnsupportedVersion`.
  - no local time type: `TzifNoLocalTimeTypes`.
  - a transition whose type index is not below typecnt:
    `TzifTypeIndexOutOfRange`.
  - transition instants not strictly ascending:
    `TzifUnsortedTransitions`.
  - a UTC offset outside `Data.Time.Offset`'s +-25:59:59:
    `TzifBadOffset`; a daylight flag other than 0 or 1:
    `TzifBadDstFlag`.
  - an abbreviation index outside the abbreviation bytes, or an
    abbreviation with no NUL before their end:
    `TzifAbbreviationOutOfRange`; abbreviation bytes that are not
    UTF-8: `TzifBadAbbreviation`.
  - leap second records (the "right/" files, whose instants are not
    Unix time): `TzifLeapSeconds`. Leap seconds are not modelled.
  - a footer that does not begin with a newline:
    `TzifMalformedFooter`; a footer string that is not a POSIX rule:
    `TzifBadFooter` with the parser's reason.
  The standard/wall and UT/local indicators are skipped: they only
  matter for interpreting a POSIX TZ string without rules against
  another file, which nothing here does.

</details>

# Types and constructors

<a id="type-547a69664572726f72"></a>

## `TzifError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TzifError</span></code></pre>

Why bytes are not a usable TZif file (the header's list).

[Source](/reference-sources/Data.Time.Zone.Tzif.cinch) · line 87

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TzifTruncated</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Tzif</span>.<span class="tok-constructor">TzifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TzifBadMagic</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Tzif</span>.<span class="tok-constructor">TzifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TzifUnsupportedVersion</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Tzif</span>.<span class="tok-constructor">TzifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TzifNoLocalTimeTypes</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Tzif</span>.<span class="tok-constructor">TzifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TzifTypeIndexOutOfRange</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Tzif</span>.<span class="tok-constructor">TzifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TzifUnsortedTransitions</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Tzif</span>.<span class="tok-constructor">TzifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TzifBadOffset</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Tzif</span>.<span class="tok-constructor">TzifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TzifBadDstFlag</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Tzif</span>.<span class="tok-constructor">TzifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TzifAbbreviationOutOfRange</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Tzif</span>.<span class="tok-constructor">TzifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TzifBadAbbreviation</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Tzif</span>.<span class="tok-constructor">TzifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TzifLeapSeconds</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Tzif</span>.<span class="tok-constructor">TzifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TzifMalformedFooter</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Tzif</span>.<span class="tok-constructor">TzifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TzifBadFooter</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixTzError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Tzif</span>.<span class="tok-constructor">TzifError</span>)</code></pre>

# Values

<a id="value-70617273652d747a6966"></a>

## `parse-tzif`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-tzif</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Tzif</span>.<span class="tok-constructor">TzifError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span>)))</code></pre>

A `TimeZone` named `name` from the bytes of a TZif file (the
header's format and refusals).

[Source](/reference-sources/Data.Time.Zone.Tzif.cinch) · line 318

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Time/Zone/Tzif.cinch`. Source SHA-256: `553280395cae4717281c2a60e19cd658e5589197362da4b23b9bc8f89f5b649c`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
