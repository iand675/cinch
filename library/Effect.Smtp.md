# Effect.Smtp

An SMTP client over sockets with explicit client state: greeting with HELO fallback, authentication, envelope, data, quit and send-mail.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Smtp`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Smtp.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Net](Effect.Net.md), [Text.Base64](Text.Base64.md), [Text.Smtp](Text.Smtp.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Net](Effect.Net.md), [Effect.Smtp](Effect.Smtp.md), [Text.Base64](Text.Base64.md), [Text.Binary](Text.Binary.md), [Text.Hex](Text.Hex.md), [Text.Md5](Text.Md5.md), [Text.Smtp](Text.Smtp.md).

<details><summary>Behaviour and boundary notes from the source</summary>

An SMTP client over Effect.Net (RFC 5321): greeting, EHLO with a HELO fallback, extensions,
PLAIN and CRAM-MD5 authentication, envelope commands, message data, verification, reset and quit,
plus a one-call send-mail.

A client is an explicit value: the socket, the bytes read ahead of the current reply, the server's
name, the name to greet with, the greeting state and whether the client is still open. Every operation
returns its result together with the next client, so the greeting state machine (not greeted, greeted
with or without extensions, or greeting failed for good) is visible in the types. The first command
that needs a greeting sends EHLO, falling back to HELO when EHLO is refused; a failed greeting is
remembered and returned by every later command. TLS is not available, so PLAIN authentication is only
offered to the local machine.

</details>

# Types and constructors

<a id="type-536d7470436c69656e74"></a>

## `SmtpClient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SmtpClient</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Effect.Smtp.cinch) · line 39

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SmtpClient</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">Greeting</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>))))))</code></pre>

# Values

<a id="value-736d74702d61757468"></a>

## `smtp-auth`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-auth</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpAuth</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>)))</code></pre>

Authenticate. A refusal on this side, or any failure during the exchange, cancels it with "*" and quits.

[Source](/reference-sources/Effect.Smtp.cinch) · line 158

<a id="value-736d74702d617574682d6f666665726564"></a>

## `smtp-auth-offered`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-auth-offered</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

The authentication mechanisms the server offers.

[Source](/reference-sources/Effect.Smtp.cinch) · line 152

<a id="value-736d74702d636c6f7365"></a>

## `smtp-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>))</code></pre>

Close the connection without quitting.

[Source](/reference-sources/Effect.Smtp.cinch) · line 269

<a id="value-736d74702d64617461"></a>

## `smtp-data`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-data</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>)))</code></pre>

Send the message: DATA, the transparency-encoded body and its end, then the server's 250.

[Source](/reference-sources/Effect.Smtp.cinch) · line 233

<a id="value-736d74702d6469616c"></a>

## `smtp-dial`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-dial</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>)))</code></pre>

Connect to a server and read its 220 greeting. The host is also the name authentication checks against.

[Source](/reference-sources/Effect.Smtp.cinch) · line 50

<a id="value-736d74702d657874656e73696f6e"></a>

## `smtp-extension`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-extension</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>)))</code></pre>

Whether the server offers an extension, and its parameters (greeting first if needed).

[Source](/reference-sources/Effect.Smtp.cinch) · line 144

<a id="value-736d74702d68656c6c6f"></a>

## `smtp-hello`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-hello</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>)))</code></pre>

Choose the name to greet with; refused once the server has been greeted.

[Source](/reference-sources/Effect.Smtp.cinch) · line 135

<a id="value-736d74702d6d61696c"></a>

## `smtp-mail`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-mail</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>)))</code></pre>

Start a mail transaction from a sender (greeting first if needed); BODY=8BITMIME and SMTPUTF8 are
requested when the server offers them.

[Source](/reference-sources/Effect.Smtp.cinch) · line 207

<a id="value-736d74702d6e6f6f70"></a>

## `smtp-noop`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-noop</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>))</code></pre>

Check the connection.

[Source](/reference-sources/Effect.Smtp.cinch) · line 252

<a id="value-736d74702d71756974"></a>

## `smtp-quit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-quit</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>))</code></pre>

Quit: QUIT and the server's 221, then close the connection (it stays open when the reply is wrong).

[Source](/reference-sources/Effect.Smtp.cinch) · line 262

<a id="value-736d74702d72637074"></a>

## `smtp-rcpt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-rcpt</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>)))</code></pre>

Add a recipient (any 25x reply accepts it).

[Source](/reference-sources/Effect.Smtp.cinch) · line 227

<a id="value-736d74702d7265736574"></a>

## `smtp-reset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-reset</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>))</code></pre>

Abort the current mail transaction.

[Source](/reference-sources/Effect.Smtp.cinch) · line 248

<a id="value-736d74702d73656e642d6d61696c"></a>

## `smtp-send-mail`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-send-mail</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpAuth</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>)))))))</code></pre>

Send one message: connect, greet, authenticate when asked (the server must offer AUTH), give the sender
and recipients, send the data and quit.

[Source](/reference-sources/Effect.Smtp.cinch) · line 279

<a id="value-736d74702d766572696679"></a>

## `smtp-verify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-verify</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>)))</code></pre>

Ask the server to verify an address.

[Source](/reference-sources/Effect.Smtp.cinch) · line 242

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Smtp.cinch`. Source SHA-256: `03aabb42280978376631405c057b4e0dc6bc306e3d1beda841644e14034c42cf`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
