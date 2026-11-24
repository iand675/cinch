# Effect.Time.Zone

The system time zone database and the current system zone.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Time.Zone</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Time.Zone/source/">View source</a><a href="/packages/search/?q=%2BEffect.Time.Zone">Search module</a><a href="/reference-sources/Effect.Time.Zone.cinch" download="Effect.Time.Zone.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Path](Data.Path.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zone.Tzif](Data.Time.Zone.Tzif.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Effect.Fs](Effect.Fs.md), [Effect.Time](Effect.Time.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

Effect.Time.Zone: the time zone database and the system zone
(design/datetime.md, layer L5, lane C; T5, T12).

Getting a `TimeZone` by name reads a file, so it is an effect under
the `Io r` capability and it is kept apart from using a zone, which
is pure (T5). A `TzDb` is a value naming a zoneinfo directory (a
tree of TZif files, one per zone, at the path its IANA name
spells); `tzdb-lookup` reads one file through `Effect.Fs` and
parses it with `Data.Time.Zone.Tzif`. Nothing is cached: keep the
`TimeZone`.

Design law shape. `tzdb-lookup db name` reads exactly the file
`<directory>/<name>` and nothing else, and only for a name that
`tzdb-valid-name` accepts: non-empty, at most 255 bytes, made of
`/`-separated components that are each non-empty, not `.` or `..`,
and spelled with ASCII letters, digits, `.`, `_`, `+` and `-`. Such
a name has no leading `/` and no `..`, so the path cannot leave the
database directory by its spelling (a symbolic link inside the
directory is the directory owner's business). The zone's name is
the name it was looked up under. The error says which stage
refused: the name, the file system, or the file's contents.
`system-time-zone` never fails: it answers the zone it found and
`system-time-zone-source` says where from.

NAMED REFUSALS:
  - the `TZ` and `TZDIR` environment variables: the prelude has no
    reader for the process environment (`Effect.Process`'s header
    records the refusal), so `tzdb-system` is always
    /usr/share/zoneinfo and `system-time-zone` does not consult
    `TZ`. When an environment reader exists, `TZ` goes first in
    `system-time-zone` and `TZDIR` in `tzdb-system`; meanwhile a
    caller holding the variable's text can use `tzdb-lookup` or
    `Data.Time.Zone.Posix.parse-posix-tz` on it.
  - the target of the /etc/localtime symbolic link: `Effect.Fs` has
    no `readlink`, so the zone's NAME is taken from /etc/timezone
    (the Debian convention) when that file names a zone the
    database has; otherwise the bytes of /etc/localtime are parsed
    and the zone is named "Etc/Unknown" (Jiff's name for a system
    zone it cannot identify).
  - a bundled or build-time database (T13): not built.
  - caching and reloading.
  - names outside the grammar above, even if a file exists for
    them (`posixrules`, `right/...` parse or not on their merits;
    `../x` and `/x` are refused unread).

</details>

<details class="doc-synopsis" open><summary>Synopsis (15 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-SystemZone">SystemZone</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SystemZone</strong></code></li><li><a class="syn-name" href="#t-SystemZoneSource">SystemZoneSource</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SystemZoneSource</strong></code></li><li><a class="syn-name" href="#t-TzDb">TzDb</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TzDb</strong></code></li><li><a class="syn-name" href="#t-TzLookupError">TzLookupError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TzLookupError</strong></code></li><li><a class="syn-name" href="#v-system-time-zone">system-time-zone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">system-time-zone</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code></li><li><a class="syn-name" href="#v-system-time-zone-source">system-time-zone-source</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">system-time-zone-source</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-SystemZone" title="Effect.Time.Zone.SystemZone">SystemZone</a></code></li><li><a class="syn-name" href="#v-system-zone-source">system-zone-source</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">system-zone-source</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-SystemZone" title="Effect.Time.Zone.SystemZone">SystemZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-SystemZoneSource" title="Effect.Time.Zone.SystemZoneSource">SystemZoneSource</a></code></li><li><a class="syn-name" href="#v-system-zone-time-zone">system-zone-time-zone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">system-zone-time-zone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-SystemZone" title="Effect.Time.Zone.SystemZone">SystemZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code></li><li><a class="syn-name" href="#v-tzdb-at">tzdb-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tzdb-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a></code></li><li><a class="syn-name" href="#v-tzdb-directory">tzdb-directory</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tzdb-directory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-tzdb-lookup">tzdb-lookup</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tzdb-lookup</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzLookupError" title="Effect.Time.Zone.TzLookupError">TzLookupError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code></li><li><a class="syn-name" href="#v-tzdb-system">tzdb-system</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tzdb-system</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a></code></li><li><a class="syn-name" href="#v-tzdb-valid-name">tzdb-valid-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tzdb-valid-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-zoned-now">zoned-now</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-now</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-now-system">zoned-now-system</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-now-system</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-SystemZone">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SystemZone</strong></code><span class="doc-sig-links"><a href="#t-SystemZone" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time.Zone/source/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

The system zone and where it came from.

</div>
</article>

<article class="doc-entry" id="t-SystemZoneSource">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SystemZoneSource</strong></code><span class="doc-sig-links"><a href="#t-SystemZoneSource" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time.Zone/source/#L90" title="View source at line 90">Source</a></span></div>
<div class="doc-body">

Where `system-time-zone` found its answer.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SystemFromEtcTimezone"><code><strong class="tok-constructor">SystemFromEtcTimezone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-SystemZoneSource" title="Effect.Time.Zone.SystemZoneSource">SystemZoneSource</a></code></li><li id="k-SystemFromEtcLocaltime"><code><strong class="tok-constructor">SystemFromEtcLocaltime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-SystemZoneSource" title="Effect.Time.Zone.SystemZoneSource">SystemZoneSource</a></code></li><li id="k-SystemFallbackUtc"><code><strong class="tok-constructor">SystemFallbackUtc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-SystemZoneSource" title="Effect.Time.Zone.SystemZoneSource">SystemZoneSource</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-TzDb">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TzDb</strong></code><span class="doc-sig-links"><a href="#t-TzDb" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time.Zone/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

A zoneinfo directory. Abstract; `tzdb-at` makes one.

</div>
</article>

<article class="doc-entry" id="t-TzLookupError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TzLookupError</strong></code><span class="doc-sig-links"><a href="#t-TzLookupError" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time.Zone/source/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

Why a lookup failed.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-TzInvalidName"><code><strong class="tok-constructor">TzInvalidName</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzLookupError" title="Effect.Time.Zone.TzLookupError">TzLookupError</a></code></li><li id="k-TzNotFound"><code><strong class="tok-constructor">TzNotFound</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzLookupError" title="Effect.Time.Zone.TzLookupError">TzLookupError</a></code></li><li id="k-TzUnreadable"><code><strong class="tok-constructor">TzUnreadable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Fs/#t-FsError" title="Effect.Fs.FsError">FsError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzLookupError" title="Effect.Time.Zone.TzLookupError">TzLookupError</a></code></li><li id="k-TzMalformed"><code><strong class="tok-constructor">TzMalformed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone.Tzif/#t-TzifError" title="Data.Time.Zone.Tzif.TzifError">TzifError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzLookupError" title="Effect.Time.Zone.TzLookupError">TzLookupError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-system-time-zone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">system-time-zone</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code><span class="doc-sig-links"><a href="#v-system-time-zone" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time.Zone/source/#L259" title="View source at line 259">Source</a></span></div>
<div class="doc-body">

The system's zone.

</div>
</article>

<article class="doc-entry" id="v-system-time-zone-source">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">system-time-zone-source</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-SystemZone" title="Effect.Time.Zone.SystemZone">SystemZone</a></code><span class="doc-sig-links"><a href="#v-system-time-zone-source" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time.Zone/source/#L251" title="View source at line 251">Source</a></span></div>
<div class="doc-body">

The system's zone and where it was found: /etc/timezone's name
looked up in the system database, else the contents of
/etc/localtime, else UTC. (`TZ` is not consulted; see NAMED
REFUSALS.)

</div>
</article>

<article class="doc-entry" id="v-system-zone-source">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">system-zone-source</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-SystemZone" title="Effect.Time.Zone.SystemZone">SystemZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-SystemZoneSource" title="Effect.Time.Zone.SystemZoneSource">SystemZoneSource</a></code><span class="doc-sig-links"><a href="#v-system-zone-source" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time.Zone/source/#L106" title="View source at line 106">Source</a></span></div>
</article>

<article class="doc-entry" id="v-system-zone-time-zone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">system-zone-time-zone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-SystemZone" title="Effect.Time.Zone.SystemZone">SystemZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code><span class="doc-sig-links"><a href="#v-system-zone-time-zone" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time.Zone/source/#L102" title="View source at line 102">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tzdb-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tzdb-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a></code><span class="doc-sig-links"><a href="#v-tzdb-at" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time.Zone/source/#L121" title="View source at line 121">Source</a></span></div>
<div class="doc-body">

The database rooted at a directory (absolute, or relative to the
process's working directory).

</div>
</article>

<article class="doc-entry" id="v-tzdb-directory">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tzdb-directory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-tzdb-directory" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time.Zone/source/#L124" title="View source at line 124">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tzdb-lookup">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tzdb-lookup</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzLookupError" title="Effect.Time.Zone.TzLookupError">TzLookupError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code><span class="doc-sig-links"><a href="#v-tzdb-lookup" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time.Zone/source/#L180" title="View source at line 180">Source</a></span></div>
<div class="doc-body">

The zone called `name` in the database. The path's kind is asked
first and only a regular file is read: a zone name that is a
directory of the database (`America`) is `TzNotFound`, and
`fs-read` is never pointed at a directory (it sizes its buffer
from `lseek`, which on a directory can answer an enormous
length and exhaust the heap).

</div>
</article>

<article class="doc-entry" id="v-tzdb-system">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tzdb-system</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Time.Zone/#t-TzDb" title="Effect.Time.Zone.TzDb">TzDb</a></code><span class="doc-sig-links"><a href="#v-tzdb-system" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time.Zone/source/#L116" title="View source at line 116">Source</a></span></div>
<div class="doc-body">

The system database, /usr/share/zoneinfo (see NAMED REFUSALS for
`TZDIR`).

</div>
</article>

<article class="doc-entry" id="v-tzdb-valid-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tzdb-valid-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tzdb-valid-name" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time.Zone/source/#L167" title="View source at line 167">Source</a></span></div>
<div class="doc-body">

Whether a string is a zone name (the header's grammar). Pure.

</div>
</article>

<article class="doc-entry" id="v-zoned-now">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-now</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-now" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time.Zone/source/#L267" title="View source at line 267">Source</a></span></div>
<div class="doc-body">

The current instant read in a zone.

</div>
</article>

<article class="doc-entry" id="v-zoned-now-system">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-now-system</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-now-system" title="Permalink">#</a><a href="/packages/stdlib/Effect.Time.Zone/source/#L271" title="View source at line 271">Source</a></span></div>
<div class="doc-body">

The current instant read in the system's zone.

</div>
</article>
