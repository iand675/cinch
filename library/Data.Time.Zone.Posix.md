# Data.Time.Zone.Posix

POSIX TZ rule strings: parsing and evaluating standard/daylight transition rules.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time.Zone.Posix`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.Zone.Posix.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Zone.Types](Data.Time.Zone.Types.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md), [Data.Time.Zone.Types](Data.Time.Zone.Types.md).

<details><summary>Behaviour and boundary notes from the source</summary>

"<" 3 or more of (ASCII letter, digit, "+", "-") ">"
  offset = [ "+" | "-" ] hh [ ":" mm [ ":" ss ] ]     hh 0..24
  date   = "J" n            n 1..365, February 29 is never counted

</details>

# Types and constructors

<a id="type-506f73697852756c65"></a>

## `PosixRule`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PosixRule</span></code></pre>

A parsed rule: standard time and, optionally, daylight time. Not
constructible outside this module: every field passed the parser.

[Source](/reference-sources/Data.Time.Zone.Posix.cinch) · line 108

<a id="type-506f736978547a4572726f72"></a>

## `PosixTzError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PosixTzError</span></code></pre>

Why a string is not a rule (the header's grammar).

[Source](/reference-sources/Data.Time.Zone.Posix.cinch) · line 112

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PosixEmpty</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixTzError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PosixBadName</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixTzError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PosixBadOffset</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixTzError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PosixMissingRule</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixTzError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PosixBadDate</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixTzError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PosixBadTime</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixTzError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PosixTrailing</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixTzError</span></code></pre>

# Values

<a id="value-70617273652d706f7369782d747a"></a>

## `parse-posix-tz`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-posix-tz</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixTzError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixRule</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zone.Posix.cinch)

<a id="value-70617273652d706f7369782d747a2d6279746573"></a>

## `parse-posix-tz-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-posix-tz-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixTzError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixRule</span>))</code></pre>

Parse a rule from its bytes (the header's grammar).

[Source](/reference-sources/Data.Time.Zone.Posix.cinch) · line 325

<a id="value-706f7369782d6473742d616262726576696174696f6e"></a>

## `posix-dst-abbreviation`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">posix-dst-abbreviation</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixRule</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zone.Posix.cinch)

<a id="value-706f7369782d6473742d6f6666736574"></a>

## `posix-dst-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">posix-dst-offset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixRule</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zone.Posix.cinch)

<a id="value-706f7369782d6861732d647374"></a>

## `posix-has-dst`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">posix-has-dst</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixRule</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Time.Zone.Posix.cinch)

<a id="value-706f7369782d696e666f2d6174"></a>

## `posix-info-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">posix-info-at</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixRule</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">OffsetInfo</span>))</code></pre>

Offset, daylight flag and abbreviation at an instant.

[Source](/reference-sources/Data.Time.Zone.Posix.cinch) · line 549

<a id="value-706f7369782d696e666f2d61742d756e69782d7365636f6e6473"></a>

## `posix-info-at-unix-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">posix-info-at-unix-seconds</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixRule</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">OffsetInfo</span>))</code></pre>

What the rule says at Unix second `t`.

[Source](/reference-sources/Data.Time.Zone.Posix.cinch) · line 515

<a id="value-706f7369782d6e6578742d7472616e736974696f6e"></a>

## `posix-next-transition`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">posix-next-transition</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixRule</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)))</code></pre>

The first transition strictly after `t`, and the last strictly
before it; `None` for a rule without daylight time or when the
instant is outside the timestamp range. A transition whose second
is `t`'s own second counts as before `t` only if `t` has a
fractional part.

[Source](/reference-sources/Data.Time.Zone.Posix.cinch) · line 569

<a id="value-706f7369782d6e6578742d7472616e736974696f6e2d756e69782d7365636f6e6473"></a>

## `posix-next-transition-unix-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">posix-next-transition-unix-seconds</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixRule</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)))</code></pre>

The first transition strictly after Unix second `t`.

[Source](/reference-sources/Data.Time.Zone.Posix.cinch) · line 525

<a id="value-706f7369782d6f66667365742d6174"></a>

## `posix-offset-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">posix-offset-at</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixRule</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zone.Posix.cinch)

<a id="value-706f7369782d70726576696f75732d7472616e736974696f6e"></a>

## `posix-previous-transition`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">posix-previous-transition</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixRule</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Zone.Posix.cinch)

<a id="value-706f7369782d70726576696f75732d7472616e736974696f6e2d756e69782d7365636f6e6473"></a>

## `posix-previous-transition-unix-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">posix-previous-transition-unix-seconds</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixRule</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)))</code></pre>

The last transition strictly before Unix second `t`.

[Source](/reference-sources/Data.Time.Zone.Posix.cinch) · line 535

<a id="value-706f7369782d7374642d616262726576696174696f6e"></a>

## `posix-std-abbreviation`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">posix-std-abbreviation</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixRule</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Data.Time.Zone.Posix.cinch)

<a id="value-706f7369782d7374642d6f6666736574"></a>

## `posix-std-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">posix-std-offset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixRule</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>)</code></pre>

[Source](/reference-sources/Data.Time.Zone.Posix.cinch)

<a id="value-706f7369782d746f2d616d626967756f7573"></a>

## `posix-to-ambiguous`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">posix-to-ambiguous</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Posix</span>.<span class="tok-constructor">PosixRule</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Ambiguous</span>))</code></pre>

The offsets a civil reading can have under the rule.

[Source](/reference-sources/Data.Time.Zone.Posix.cinch) · line 556

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Time/Zone/Posix.cinch`. Source SHA-256: `ab9f6f5fe7365fcf24af9c6d156b80f1de73e01b0edf070d1daf0b430d521ce9`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
