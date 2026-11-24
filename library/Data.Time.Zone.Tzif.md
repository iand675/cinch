# Data.Time.Zone.Tzif

Reading TZif time zone files (RFC 9636) into checked transition tables.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Time.Zone.Tzif</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Time.Zone.Tzif/source/">View source</a><a href="/packages/search/?q=%2BData.Time.Zone.Tzif">Search module</a><a href="/reference-sources/Data.Time.Zone.Tzif.cinch" download="Data.Time.Zone.Tzif.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis" open><summary>Synopsis (2 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-TzifError">TzifError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TzifError</strong></code></li><li><a class="syn-name" href="#v-parse-tzif">parse-tzif</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-tzif</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Tzif/#t-TzifError" title="Data.Time.Zone.Tzif.TzifError">TzifError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-TzifError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TzifError</strong></code><span class="doc-sig-links"><a href="#t-TzifError" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Tzif/source/#L87" title="View source at line 87">Source</a></span></div>
<div class="doc-body">

Why bytes are not a usable TZif file (the header's list).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-TzifTruncated"><code><strong class="tok-constructor">TzifTruncated</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Tzif/#t-TzifError" title="Data.Time.Zone.Tzif.TzifError">TzifError</a></code></li><li id="k-TzifBadMagic"><code><strong class="tok-constructor">TzifBadMagic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Tzif/#t-TzifError" title="Data.Time.Zone.Tzif.TzifError">TzifError</a></code></li><li id="k-TzifUnsupportedVersion"><code><strong class="tok-constructor">TzifUnsupportedVersion</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Tzif/#t-TzifError" title="Data.Time.Zone.Tzif.TzifError">TzifError</a></code></li><li id="k-TzifNoLocalTimeTypes"><code><strong class="tok-constructor">TzifNoLocalTimeTypes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Tzif/#t-TzifError" title="Data.Time.Zone.Tzif.TzifError">TzifError</a></code></li><li id="k-TzifTypeIndexOutOfRange"><code><strong class="tok-constructor">TzifTypeIndexOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Tzif/#t-TzifError" title="Data.Time.Zone.Tzif.TzifError">TzifError</a></code></li><li id="k-TzifUnsortedTransitions"><code><strong class="tok-constructor">TzifUnsortedTransitions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Tzif/#t-TzifError" title="Data.Time.Zone.Tzif.TzifError">TzifError</a></code></li><li id="k-TzifBadOffset"><code><strong class="tok-constructor">TzifBadOffset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Tzif/#t-TzifError" title="Data.Time.Zone.Tzif.TzifError">TzifError</a></code></li><li id="k-TzifBadDstFlag"><code><strong class="tok-constructor">TzifBadDstFlag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Tzif/#t-TzifError" title="Data.Time.Zone.Tzif.TzifError">TzifError</a></code></li><li id="k-TzifAbbreviationOutOfRange"><code><strong class="tok-constructor">TzifAbbreviationOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Tzif/#t-TzifError" title="Data.Time.Zone.Tzif.TzifError">TzifError</a></code></li><li id="k-TzifBadAbbreviation"><code><strong class="tok-constructor">TzifBadAbbreviation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Tzif/#t-TzifError" title="Data.Time.Zone.Tzif.TzifError">TzifError</a></code></li><li id="k-TzifLeapSeconds"><code><strong class="tok-constructor">TzifLeapSeconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Tzif/#t-TzifError" title="Data.Time.Zone.Tzif.TzifError">TzifError</a></code></li><li id="k-TzifMalformedFooter"><code><strong class="tok-constructor">TzifMalformedFooter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Tzif/#t-TzifError" title="Data.Time.Zone.Tzif.TzifError">TzifError</a></code></li><li id="k-TzifBadFooter"><code><strong class="tok-constructor">TzifBadFooter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Posix/#t-PosixTzError" title="Data.Time.Zone.Posix.PosixTzError">PosixTzError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Tzif/#t-TzifError" title="Data.Time.Zone.Tzif.TzifError">TzifError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-parse-tzif">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-tzif</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Tzif/#t-TzifError" title="Data.Time.Zone.Tzif.TzifError">TzifError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code><span class="doc-sig-links"><a href="#v-parse-tzif" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zone.Tzif/source/#L318" title="View source at line 318">Source</a></span></div>
<div class="doc-body">

A `TimeZone` named `name` from the bytes of a TZif file (the
header's format and refusals).

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Time/Zone/Tzif.cinch`](/packages/stdlib/Data.Time.Zone.Tzif/source/). Source SHA-256: `553280395cae4717281c2a60e19cd658e5589197362da4b23b9bc8f89f5b649c`.
