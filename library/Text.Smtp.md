# Text.Smtp

SMTP without a connection: reply state machine, command validation, extensions, dot-stuffing and PLAIN/CRAM-MD5 exchanges.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Smtp`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Smtp.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Base64](Text.Base64.md), [Text.Hex](Text.Hex.md), [Text.Md5](Text.Md5.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Base64](Text.Base64.md), [Text.Binary](Text.Binary.md), [Text.Hex](Text.Hex.md), [Text.Md5](Text.Md5.md), [Text.Smtp](Text.Smtp.md).

<details><summary>Behaviour and boundary notes from the source</summary>

The SMTP protocol without a connection (RFC 5321, with the AUTH extension's PLAIN and CRAM-MD5
mechanisms): reply reading, command lines, extension lists, the message body's transparency
encoding and authentication exchanges. Effect.Smtp runs these over a socket.

A reply is read one line at a time through an explicit state machine: a three-digit code followed by a
space ends a reply and a hyphen continues it; continuation lines join the message with newlines, a line
with another code (or none) joins it whole, and a reply whose first code is not the expected one becomes
an error carrying the complete message. The body encoding is a four-state machine: line ends become CR LF,
a dot that starts a line is doubled, and the body is closed with CR LF "." CR LF. Commands refuse
arguments that contain CR or LF, so no argument can inject a second command.

</details>

# Types and constructors

<a id="type-5265706c79526561646572"></a>

## `ReplyReader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ReplyReader</span></code></pre>

Reading a reply: before its first line (with the expected code: 0 accepts any, one or two digits match a
prefix), or within a multi-line reply (code, message so far, and whether the first code was unexpected).

[Source](/reference-sources/Text.Smtp.cinch) · line 105

<a id="type-5265706c7953746570"></a>

## `ReplyStep`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ReplyStep</span></code></pre>

What one line does to a reply in progress.

[Source](/reference-sources/Text.Smtp.cinch) · line 112

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">NeedLine</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">ReplyReader</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">ReplyStep</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ReplyDone</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpReply</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">ReplyStep</span>)</code></pre>

<a id="type-536d747041757468"></a>

## `SmtpAuth`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SmtpAuth</span></code></pre>

A way to authenticate.

[Source](/reference-sources/Text.Smtp.cinch) · line 345

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SmtpPlainAuth</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpAuth</span>))))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SmtpCramMd5Auth</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpAuth</span>))</code></pre>

<a id="type-536d74704572726f72"></a>

## `SmtpError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SmtpError</span></code></pre>

Why an SMTP exchange failed.

[Source](/reference-sources/Text.Smtp.cinch) · line 37

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SmtpReplyError</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SmtpProtocolError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SmtpEof</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SmtpNetError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SmtpBadLine</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SmtpAuthRefused</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SmtpHelloTooLate</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SmtpNoAuth</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SmtpBadChallenge</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SmtpClosed</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span></code></pre>

<a id="type-536d74705265706c79"></a>

## `SmtpReply`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SmtpReply</span></code></pre>

Code and message.

[Source](/reference-sources/Text.Smtp.cinch) · line 93

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SmtpReply</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpReply</span>))</code></pre>

# Values

<a id="value-736d74702d617574682d6d656368616e69736d73"></a>

## `smtp-auth-mechanisms`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-auth-mechanisms</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

The authentication mechanisms an extension list offers (the AUTH keyword's space-separated parameters).

[Source](/reference-sources/Text.Smtp.cinch) · line 261

<a id="value-736d74702d617574682d6e657874"></a>

## `smtp-auth-next`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-auth-next</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpAuth</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Answer a server message (decoded) that is a challenge when more is set: the next response, or None when the
exchange is complete on this side.

[Source](/reference-sources/Text.Smtp.cinch) · line 368

<a id="value-736d74702d617574682d7374617274"></a>

## `smtp-auth-start`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-auth-start</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpAuth</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Bytes</span>)))))</code></pre>

Start an exchange with a server (its name and whether the connection is encrypted): the mechanism and the
initial response (empty when the mechanism sends none).

[Source](/reference-sources/Text.Smtp.cinch) · line 358

<a id="value-736d74702d646f742d656e636f6465"></a>

## `smtp-dot-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-dot-encode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The transparency encoding of a message body: a lone LF becomes CR LF, a dot that starts a line is doubled,
and the end-of-data line follows (after a CR LF unless the body already ends with one).

[Source](/reference-sources/Text.Smtp.cinch) · line 286

<a id="value-736d74702d6572726f722d6d657373616765"></a>

## `smtp-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The error's text: replies as "%03d message", the rest as the protocol's standard wording.

[Source](/reference-sources/Text.Smtp.cinch) · line 61

<a id="value-736d74702d657874656e73696f6e2d6c6f6f6b7570"></a>

## `smtp-extension-lookup`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-extension-lookup</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

The parameters of an extension keyword (compared after upper-casing the query).

[Source](/reference-sources/Text.Smtp.cinch) · line 235

<a id="value-736d74702d657874656e73696f6e73"></a>

## `smtp-extensions`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-extensions</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

The extensions of an EHLO reply: each line after the first as (keyword, parameters), latest first so a
repeated keyword's last parameters win.

[Source](/reference-sources/Text.Smtp.cinch) · line 200

<a id="value-736d74702d69732d6c6f63616c686f7374"></a>

## `smtp-is-localhost`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-is-localhost</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a server name is the local machine.

[Source](/reference-sources/Text.Smtp.cinch) · line 353

<a id="value-736d74702d7265706c792d636f6465"></a>

## `smtp-reply-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-reply-code</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpReply</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The reply code.

[Source](/reference-sources/Text.Smtp.cinch) · line 96

<a id="value-736d74702d7265706c792d6c696e65"></a>

## `smtp-reply-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-reply-line</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">ReplyReader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">ReplyStep</span>))</code></pre>

Feed one line (without its line end) to a reply in progress.

[Source](/reference-sources/Text.Smtp.cinch) · line 130

<a id="value-736d74702d7265706c792d6d657373616765"></a>

## `smtp-reply-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-reply-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpReply</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The reply message.

[Source](/reference-sources/Text.Smtp.cinch) · line 100

<a id="value-736d74702d7265706c792d726561646572"></a>

## `smtp-reply-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-reply-reader</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">ReplyReader</span>)</code></pre>

A reader for a reply with an expected code.

[Source](/reference-sources/Text.Smtp.cinch) · line 119

<a id="value-736d74702d73747269702d6c696e65"></a>

## `smtp-strip-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-strip-line</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

A received line without its LF and a CR before it.

[Source](/reference-sources/Text.Smtp.cinch) · line 180

<a id="value-736d74702d76616c69646174652d6c696e65"></a>

## `smtp-validate-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-validate-line</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Refuse a command argument that contains CR or LF.

[Source](/reference-sources/Text.Smtp.cinch) · line 191

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Smtp.cinch`. Source SHA-256: `ab46aeac80b9aa3bcb33399324012c1533fb999ecbf5c9012df64a01130da419`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
