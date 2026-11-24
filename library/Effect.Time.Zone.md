# Effect.Time.Zone

The system time zone database and the current system zone.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Time.Zone`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Time.Zone.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Path](Data.Path.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zone.Tzif](Data.Time.Zone.Tzif.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Effect.Fs](Effect.Fs.md), [Effect.Time](Effect.Time.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Monotonic](Data.Time.Monotonic.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md), [Data.Time.Zone.Types](Data.Time.Zone.Types.md), [Data.Time.Zone.Tzif](Data.Time.Zone.Tzif.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Effect.Time](Effect.Time.md), [Effect.Time.Zone](Effect.Time.Zone.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-53797374656d5a6f6e65"></a>

## `SystemZone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SystemZone</span></code></pre>

The system zone and where it came from.

[Source](/reference-sources/Effect.Time.Zone.cinch) · line 99

<a id="type-53797374656d5a6f6e65536f75726365"></a>

## `SystemZoneSource`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SystemZoneSource</span></code></pre>

Where `system-time-zone` found its answer.

[Source](/reference-sources/Effect.Time.Zone.cinch) · line 90

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SystemFromEtcTimezone</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">SystemZoneSource</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SystemFromEtcLocaltime</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">SystemZoneSource</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SystemFallbackUtc</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">SystemZoneSource</span></code></pre>

<a id="type-547a4462"></a>

## `TzDb`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TzDb</span></code></pre>

A zoneinfo directory. Abstract; `tzdb-at` makes one.

[Source](/reference-sources/Effect.Time.Zone.cinch) · line 74

<a id="type-547a4c6f6f6b75704572726f72"></a>

## `TzLookupError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TzLookupError</span></code></pre>

Why a lookup failed.

[Source](/reference-sources/Effect.Time.Zone.cinch) · line 78

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TzInvalidName</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TzLookupError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TzNotFound</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TzLookupError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TzUnreadable</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fs</span>.<span class="tok-constructor">FsError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TzLookupError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TzMalformed</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Tzif</span>.<span class="tok-constructor">TzifError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TzLookupError</span>)</code></pre>

# Values

<a id="value-73797374656d2d74696d652d7a6f6e65"></a>

## `system-time-zone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">system-time-zone</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span>)</code></pre>

The system's zone.

[Source](/reference-sources/Effect.Time.Zone.cinch) · line 259

<a id="value-73797374656d2d74696d652d7a6f6e652d736f75726365"></a>

## `system-time-zone-source`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">system-time-zone-source</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">SystemZone</span>)</code></pre>

The system's zone and where it was found: /etc/timezone's name
looked up in the system database, else the contents of
/etc/localtime, else UTC. (`TZ` is not consulted; see NAMED
REFUSALS.)

[Source](/reference-sources/Effect.Time.Zone.cinch) · line 251

<a id="value-73797374656d2d7a6f6e652d736f75726365"></a>

## `system-zone-source`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">system-zone-source</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">SystemZone</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">SystemZoneSource</span>)</code></pre>

[Source](/reference-sources/Effect.Time.Zone.cinch)

<a id="value-73797374656d2d7a6f6e652d74696d652d7a6f6e65"></a>

## `system-zone-time-zone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">system-zone-time-zone</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">SystemZone</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span>)</code></pre>

[Source](/reference-sources/Effect.Time.Zone.cinch)

<a id="value-747a64622d6174"></a>

## `tzdb-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tzdb-at</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TzDb</span>)</code></pre>

The database rooted at a directory (absolute, or relative to the
process's working directory).

[Source](/reference-sources/Effect.Time.Zone.cinch) · line 121

<a id="value-747a64622d6469726563746f7279"></a>

## `tzdb-directory`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tzdb-directory</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TzDb</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Effect.Time.Zone.cinch)

<a id="value-747a64622d6c6f6f6b7570"></a>

## `tzdb-lookup`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tzdb-lookup</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TzDb</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TzLookupError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span>)))</code></pre>

The zone called `name` in the database. The path's kind is asked
first and only a regular file is read: a zone name that is a
directory of the database (`America`) is `TzNotFound`, and
`fs-read` is never pointed at a directory (it sizes its buffer
from `lseek`, which on a directory can answer an enormous
length and exhaust the heap).

[Source](/reference-sources/Effect.Time.Zone.cinch) · line 180

<a id="value-747a64622d73797374656d"></a>

## `tzdb-system`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tzdb-system</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TzDb</span></code></pre>

The system database, /usr/share/zoneinfo (see NAMED REFUSALS for
`TZDIR`).

[Source](/reference-sources/Effect.Time.Zone.cinch) · line 116

<a id="value-747a64622d76616c69642d6e616d65"></a>

## `tzdb-valid-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tzdb-valid-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a string is a zone name (the header's grammar). Pure.

[Source](/reference-sources/Effect.Time.Zone.cinch) · line 167

<a id="value-7a6f6e65642d6e6f77"></a>

## `zoned-now`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-now</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)</code></pre>

The current instant read in a zone.

[Source](/reference-sources/Effect.Time.Zone.cinch) · line 267

<a id="value-7a6f6e65642d6e6f772d73797374656d"></a>

## `zoned-now-system`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-now-system</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)</code></pre>

The current instant read in the system's zone.

[Source](/reference-sources/Effect.Time.Zone.cinch) · line 271

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Time/Zone.cinch`. Source SHA-256: `00e4316ae15fdfd60d1b89f64409d536b0476812447d505a4a557e05b7139751`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
