# Effect.Syslog

RFC 3164 and RFC 5424 syslog message formatting, facility/severity priorities, Effect.Log sink bridge, and UDP/TCP network writers.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Syslog`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Syslog.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Clock](Effect.Clock.md), [Effect.Log](Effect.Log.md), [Effect.Net](Effect.Net.md), [Relation.Eq](Relation.Eq.md), [Text.Builder](Text.Builder.md), [Text.Json](Text.Json.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Log](Effect.Log.md), [Effect.Net](Effect.Net.md), [Effect.Syslog](Effect.Syslog.md), [Text.Builder](Text.Builder.md), [Text.Json](Text.Json.md), [Text.Lex](Text.Lex.md), [Text.Parse](Text.Parse.md), [Text.Scan](Text.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

RFC 3164 and RFC 5424 syslog formatting, UDP and TCP network writers, custom sink capture,
and a structured bridge from Effect.Log records.

Facilities, severities, and priority values follow RFC 5424 section 6.2.1. Pure formatters sanitize
embedded carriage returns, line feeds, and NUL bytes to prevent framing corruption or log injection.
Network transports operate explicitly under the Io r capability without ambient global writers.

</details>

# Types and constructors

<a id="type-5379736c6f674572726f72"></a>

## `SyslogError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SyslogError</span></code></pre>

Named errors reported by syslog transport and validation operations.

[Source](/reference-sources/Effect.Syslog.cinch) · line 232

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SyslogNetError</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SyslogClosed</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SyslogInvalidPriority</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span></code></pre>

<a id="type-5379736c6f67466163696c697479"></a>

## `SyslogFacility`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SyslogFacility</span></code></pre>

Syslog message facility values shifted by 3 bits (multiples of 8) per RFC 5424.

[Source](/reference-sources/Effect.Syslog.cinch) · line 84

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogKern</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogUser</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogMail</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogDaemon</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogAuth</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogSyslog</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogLpr</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogNews</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogUucp</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogCron</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogAuthpriv</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogFtp</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogLocal0</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogLocal1</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogLocal2</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogLocal3</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogLocal4</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogLocal5</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogLocal6</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogLocal7</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span></code></pre>

<a id="type-5379736c6f67466f726d6174"></a>

## `SyslogFormat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SyslogFormat</span></code></pre>

Syslog wire framing dialect selected for a writer.

[Source](/reference-sources/Effect.Syslog.cinch) · line 225

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SyslogRfc3164</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFormat</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SyslogRfc5424</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFormat</span></code></pre>

<a id="type-5379736c6f675072696f72697479"></a>

## `SyslogPriority`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SyslogPriority</span></code></pre>

Pair a facility and a severity into one priority value.

[Source](/reference-sources/Effect.Syslog.cinch) · line 158

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SyslogPriority</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogSeverity</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogPriority</span>))</code></pre>

<a id="type-5379736c6f675365766572697479"></a>

## `SyslogSeverity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SyslogSeverity</span></code></pre>

Syslog message severity levels ordered from emergency (0) through debug (7).

[Source](/reference-sources/Effect.Syslog.cinch) · line 48

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogEmerg</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogSeverity</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogAlert</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogSeverity</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogCrit</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogSeverity</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogErr</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogSeverity</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogWarning</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogSeverity</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogNotice</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogSeverity</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogInfo</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogSeverity</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LogDebug</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogSeverity</span></code></pre>

<a id="type-5379736c6f67577269746572"></a>

## `SyslogWriter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SyslogWriter</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>)</code></pre>

A configured syslog writer carrying facility, dialect, header metadata, and transport.

[Source](/reference-sources/Effect.Syslog.cinch) · line 450

# Values

<a id="value-7379736c6f672d616c657274"></a>

## `syslog-alert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-alert</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Emit an alert-severity (LogAlert) syslog message.

[Source](/reference-sources/Effect.Syslog.cinch) · line 546

<a id="value-7379736c6f672d636c6f7365"></a>

## `syslog-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Close the underlying transport of a syslog writer (closing the TCP socket when applicable).

[Source](/reference-sources/Effect.Syslog.cinch) · line 526

<a id="value-7379736c6f672d63726974"></a>

## `syslog-crit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-crit</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Emit a critical-severity (LogCrit) syslog message.

[Source](/reference-sources/Effect.Syslog.cinch) · line 550

<a id="value-7379736c6f672d6465627567"></a>

## `syslog-debug`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-debug</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Emit a debug-severity (LogDebug) syslog message.

[Source](/reference-sources/Effect.Syslog.cinch) · line 570

<a id="value-7379736c6f672d6469616c2d746370"></a>

## `syslog-dial-tcp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-dial-tcp</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>))))))</code></pre>

Open a TCP connection to `(host, port)` and return a newline-framed syslog writer.

[Source](/reference-sources/Effect.Syslog.cinch) · line 463

<a id="value-7379736c6f672d6469616c2d756470"></a>

## `syslog-dial-udp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-dial-udp</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>))))))</code></pre>

Create a UDP syslog writer sending one datagram per message to `(host, port)`.

[Source](/reference-sources/Effect.Syslog.cinch) · line 455

<a id="value-7379736c6f672d656d657267"></a>

## `syslog-emerg`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-emerg</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Emit an emergency-severity (LogEmerg) syslog message.

[Source](/reference-sources/Effect.Syslog.cinch) · line 542

<a id="value-7379736c6f672d656d69742d6c6f672d7265636f7264"></a>

## `syslog-emit-log-record`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-emit-log-record</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">LogLevel</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span> <span class="tok-constructor">Unit</span>)))))</code></pre>

Emit a structured Effect.Log record as canonical JSON over a syslog writer at the mapped severity.

[Source](/reference-sources/Effect.Syslog.cinch) · line 586

<a id="value-7379736c6f672d657272"></a>

## `syslog-err`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-err</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Emit an error-severity (LogErr) syslog message.

[Source](/reference-sources/Effect.Syslog.cinch) · line 554

<a id="value-7379736c6f672d6572726f722d6d657373616765"></a>

## `syslog-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-error-message</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a readable diagnostic message for a syslog error.

[Source](/reference-sources/Effect.Syslog.cinch) · line 241

<a id="value-7379736c6f672d666163696c6974792d636f6465"></a>

## `syslog-facility-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-facility-code</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Return the shifted RFC 5424 facility code (multiple of 8 in 0..184).

[Source](/reference-sources/Effect.Syslog.cinch) · line 127

<a id="value-7379736c6f672d666f726d61742d72666333313634"></a>

## `syslog-format-rfc3164`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-format-rfc3164</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogSeverity</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))))))</code></pre>

Format a syslog line according to RFC 3164 (`<PRI>TIMESTAMP HOST TAG[PID]: MSG`),
omitting `[PID]` when pid is 0 and sanitizing embedded CR/LF bytes.

[Source](/reference-sources/Effect.Syslog.cinch) · line 408

<a id="value-7379736c6f672d666f726d61742d72666335343234"></a>

## `syslog-format-rfc5424`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-format-rfc5424</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogSeverity</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))))))))</code></pre>

Format a syslog line according to RFC 5424 (`<PRI>1 TIMESTAMP HOST APP PROCID MSGID - MSG`),
substituting `-` for empty header tokens and sanitizing embedded CR/LF bytes.

[Source](/reference-sources/Effect.Syslog.cinch) · line 420

<a id="value-7379736c6f672d696e666f"></a>

## `syslog-info`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-info</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Emit an informational-severity (LogInfo) syslog message.

[Source](/reference-sources/Effect.Syslog.cinch) · line 566

<a id="value-7379736c6f672d6c6576656c2d66726f6d2d6c6f67"></a>

## `syslog-level-from-log`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-level-from-log</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">LogLevel</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogSeverity</span>)</code></pre>

Map an Effect.Log LogLevel to its corresponding RFC 5424 SyslogSeverity.

[Source](/reference-sources/Effect.Syslog.cinch) · line 578

<a id="value-7379736c6f672d6e6f74696365"></a>

## `syslog-notice`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-notice</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Emit a notice-severity (LogNotice) syslog message.

[Source](/reference-sources/Effect.Syslog.cinch) · line 562

<a id="value-7379736c6f672d70617273652d7072696f72697479"></a>

## `syslog-parse-priority`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-parse-priority</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span>, <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogSeverity</span>)))</code></pre>

Decode a numeric RFC 5424 priority value (0..191) into its facility and severity.

[Source](/reference-sources/Effect.Syslog.cinch) · line 208

<a id="value-7379736c6f672d7072696f72697479"></a>

## `syslog-priority`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-priority</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogSeverity</span> -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

Compute the RFC 5424 numeric priority value (facility code + severity code, in 0..191).

[Source](/reference-sources/Effect.Syslog.cinch) · line 161

<a id="value-7379736c6f672d7072696f726974792d636f6465"></a>

## `syslog-priority-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-priority-code</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogPriority</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Compute the numeric priority code of a SyslogPriority value.

[Source](/reference-sources/Effect.Syslog.cinch) · line 165

<a id="value-7379736c6f672d73616e6974697a65"></a>

## `syslog-sanitize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-sanitize</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Sanitize a syslog message by stripping trailing CR/LF and replacing embedded CR, LF, and NUL bytes with spaces.

[Source](/reference-sources/Effect.Syslog.cinch) · line 304

<a id="value-7379736c6f672d73657665726974792d636f6465"></a>

## `syslog-severity-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-severity-code</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogSeverity</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Return the RFC 5424 numeric code (0..7) for a syslog severity.

[Source](/reference-sources/Effect.Syslog.cinch) · line 67

<a id="value-7379736c6f672d74696d657374616d702d72666333313634"></a>

## `syslog-timestamp-rfc3164`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-timestamp-rfc3164</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Format Unix epoch nanoseconds as an RFC 3164 UTC timestamp (`Mmm dd hh:mm:ss`).

[Source](/reference-sources/Effect.Syslog.cinch) · line 391

<a id="value-7379736c6f672d74696d657374616d702d72666335343234"></a>

## `syslog-timestamp-rfc5424`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-timestamp-rfc5424</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Format Unix epoch nanoseconds as an RFC 5424 UTC timestamp (`YYYY-MM-DDTHH:MM:SSZ`).

[Source](/reference-sources/Effect.Syslog.cinch) · line 397

<a id="value-7379736c6f672d7761726e696e67"></a>

## `syslog-warning`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-warning</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Emit a warning-severity (LogWarning) syslog message.

[Source](/reference-sources/Effect.Syslog.cinch) · line 558

<a id="value-7379736c6f672d776974682d666f726d6174"></a>

## `syslog-with-format`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-with-format</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFormat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Select the wire format (RFC 3164 or RFC 5424) used by a syslog writer.

[Source](/reference-sources/Effect.Syslog.cinch) · line 477

<a id="value-7379736c6f672d776974682d686f73746e616d65"></a>

## `syslog-with-hostname`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-with-hostname</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Override the hostname emitted in syslog headers by a writer.

[Source](/reference-sources/Effect.Syslog.cinch) · line 482

<a id="value-7379736c6f672d776974682d6d73676964"></a>

## `syslog-with-msgid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-with-msgid</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Override the RFC 5424 MSGID field emitted by a writer.

[Source](/reference-sources/Effect.Syslog.cinch) · line 492

<a id="value-7379736c6f672d776974682d706964"></a>

## `syslog-with-pid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-with-pid</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Override the process ID emitted in syslog headers by a writer (0 omits PID).

[Source](/reference-sources/Effect.Syslog.cinch) · line 487

<a id="value-7379736c6f672d776974682d73696e6b"></a>

## `syslog-with-sink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-with-sink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFacility</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogFormat</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span> <span class="tok-constructor">Unit</span>)) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>)))))</code></pre>

Create a syslog writer backed by a custom sink callback for capture or testing.

[Source](/reference-sources/Effect.Syslog.cinch) · line 472

<a id="value-7379736c6f672d7772697465"></a>

## `syslog-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">syslog-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogSeverity</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Syslog</span>.<span class="tok-constructor">SyslogError</span> <span class="tok-constructor">Unit</span>))))</code></pre>

Format and deliver a message at the given severity through a syslog writer.

[Source](/reference-sources/Effect.Syslog.cinch) · line 507

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Effect.Syslog.Eq_SyslogFacility` | `Relation.Eq.Eq Effect.Syslog.SyslogFacility` | Implicit |
| `Effect.Syslog.Eq_SyslogSeverity` | `Relation.Eq.Eq Effect.Syslog.SyslogSeverity` | Implicit |

**`Effect.Syslog.Eq_SyslogFacility`**: Structural equality for syslog facilities.
**`Effect.Syslog.Eq_SyslogSeverity`**: Structural equality for syslog severities.

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Syslog.cinch`. Source SHA-256: `9572e489ac229451283ff67c780314b9e64856a47bcd935c4dcb0f186f2468a9`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
