# Effect.Smtp

An SMTP client over sockets and TLS 1.3 (implicit SMTPS and RFC 3207 STARTTLS) with explicit client state, HELO fallback, PLAIN/CRAM-MD5 authentication, envelope, data, quit and send-mail.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Smtp`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Smtp.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Crypto.Tls](Crypto.Tls.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Net](Effect.Net.md), [Effect.Tls](Effect.Tls.md), [Text.Base64](Text.Base64.md), [Text.Smtp](Text.Smtp.md).

Non-embedded source closure: [Crypto.Aes](Crypto.Aes.md), [Crypto.ChaCha20Poly1305](Crypto.ChaCha20Poly1305.md), [Crypto.Cipher](Crypto.Cipher.md), [Crypto.ConstantTime](Crypto.ConstantTime.md), [Crypto.Ecdsa](Crypto.Ecdsa.md), [Crypto.Ed25519](Crypto.Ed25519.md), [Crypto.Field25519](Crypto.Field25519.md), [Crypto.Hmac](Crypto.Hmac.md), [Crypto.Kdf](Crypto.Kdf.md), [Crypto.P256](Crypto.P256.md), [Crypto.Pkix](Crypto.Pkix.md), [Crypto.Rsa](Crypto.Rsa.md), [Crypto.Tls](Crypto.Tls.md), [Crypto.X25519](Crypto.X25519.md), [Crypto.X509](Crypto.X509.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.IpAddr](Data.IpAddr.md), [Data.NetIp](Data.NetIp.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Effect.Net](Effect.Net.md), [Effect.Random](Effect.Random.md), [Effect.Smtp](Effect.Smtp.md), [Effect.Stream](Effect.Stream.md), [Effect.Tls](Effect.Tls.md), [Text.Asn1](Text.Asn1.md), [Text.Base64](Text.Base64.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Binary](Text.Binary.md), [Text.Digest](Text.Digest.md), [Text.Hex](Text.Hex.md), [Text.Md5](Text.Md5.md), [Text.Pem](Text.Pem.md), [Text.Sha1](Text.Sha1.md), [Text.Sha256](Text.Sha256.md), [Text.Sha512](Text.Sha512.md), [Text.Smtp](Text.Smtp.md).

<details><summary>Behaviour and boundary notes from the source</summary>

An SMTP client over Effect.Net and Effect.Tls (RFC 5321, RFC 3207): greeting, EHLO with a HELO fallback,
extensions, implicit TLS 1.3 and STARTTLS upgrades, PLAIN and CRAM-MD5 authentication, envelope commands,
message data, verification, reset and quit, plus one-call send-mail helpers.

A client is an explicit value: the socket or TLS connection, the bytes read ahead of the current reply, the
server's name, the name to greet with, the greeting state and whether the client is still open. Every operation
returns its result together with the next client, so the greeting state machine (not greeted, greeted
with or without extensions, or greeting failed for good) is visible in the types. The first command
that needs a greeting sends EHLO, falling back to HELO when EHLO is refused; a failed greeting is
remembered and returned by every later command. Over plaintext sockets PLAIN authentication is only
offered to the local machine; over `SmtpTlsClient` (via `smtp-dial-tls` or `smtp-starttls`) PLAIN
authentication is allowed to remote hosts over the encrypted TLS 1.3 link.

</details>

# Types and constructors

<a id="type-536d7470436c69656e74"></a>

## `SmtpClient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SmtpClient</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Effect.Smtp.cinch) · line 46

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SmtpClient</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">Greeting</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>))))))</code></pre>

<a id="type-536d7470546c73436c69656e74"></a>

## `SmtpTlsClient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SmtpTlsClient</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>)</code></pre>

Fields in the order listed above.

[Source](/reference-sources/Effect.Smtp.cinch) · line 342

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SmtpTlsClient</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConn</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">Greeting</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>))))))</code></pre>

# Values

<a id="value-736d74702d61757468"></a>

## `smtp-auth`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-auth</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpAuth</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>)))</code></pre>

Authenticate. A refusal on this side, or any failure during the exchange, cancels it with "*" and quits.

[Source](/reference-sources/Effect.Smtp.cinch) · line 165

<a id="value-736d74702d617574682d6f666665726564"></a>

## `smtp-auth-offered`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-auth-offered</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

The authentication mechanisms the server offers.

[Source](/reference-sources/Effect.Smtp.cinch) · line 159

<a id="value-736d74702d636c6f7365"></a>

## `smtp-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>))</code></pre>

Close the connection without quitting.

[Source](/reference-sources/Effect.Smtp.cinch) · line 276

<a id="value-736d74702d64617461"></a>

## `smtp-data`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-data</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>)))</code></pre>

Send the message: DATA, the transparency-encoded body and its end, then the server's 250.

[Source](/reference-sources/Effect.Smtp.cinch) · line 240

<a id="value-736d74702d6469616c"></a>

## `smtp-dial`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-dial</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>)))</code></pre>

Connect to a server and read its 220 greeting. The host is also the name authentication checks against.

[Source](/reference-sources/Effect.Smtp.cinch) · line 57

<a id="value-736d74702d6469616c2d746c73"></a>

## `smtp-dial-tls`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-dial-tls</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>)))))</code></pre>

Connect to an SMTP server using implicit TLS 1.3 (SMTPS) and read its 220 greeting.

[Source](/reference-sources/Effect.Smtp.cinch) · line 360

<a id="value-736d74702d657874656e73696f6e"></a>

## `smtp-extension`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-extension</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>)))</code></pre>

Whether the server offers an extension, and its parameters (greeting first if needed).

[Source](/reference-sources/Effect.Smtp.cinch) · line 151

<a id="value-736d74702d68656c6c6f"></a>

## `smtp-hello`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-hello</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>)))</code></pre>

Choose the name to greet with; refused once the server has been greeted.

[Source](/reference-sources/Effect.Smtp.cinch) · line 142

<a id="value-736d74702d6d61696c"></a>

## `smtp-mail`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-mail</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>)))</code></pre>

Start a mail transaction from a sender (greeting first if needed); BODY=8BITMIME and SMTPUTF8 are
requested when the server offers them.

[Source](/reference-sources/Effect.Smtp.cinch) · line 214

<a id="value-736d74702d6e6f6f70"></a>

## `smtp-noop`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-noop</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>))</code></pre>

Check the connection.

[Source](/reference-sources/Effect.Smtp.cinch) · line 259

<a id="value-736d74702d71756974"></a>

## `smtp-quit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-quit</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>))</code></pre>

Quit: QUIT and the server's 221, then close the connection (it stays open when the reply is wrong).

[Source](/reference-sources/Effect.Smtp.cinch) · line 269

<a id="value-736d74702d72637074"></a>

## `smtp-rcpt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-rcpt</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>)))</code></pre>

Add a recipient (any 25x reply accepts it).

[Source](/reference-sources/Effect.Smtp.cinch) · line 234

<a id="value-736d74702d7265736574"></a>

## `smtp-reset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-reset</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>))</code></pre>

Abort the current mail transaction.

[Source](/reference-sources/Effect.Smtp.cinch) · line 255

<a id="value-736d74702d73656e642d6d61696c"></a>

## `smtp-send-mail`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-send-mail</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpAuth</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>)))))))</code></pre>

Send one message: connect, greet, authenticate when asked (the server must offer AUTH), give the sender
and recipients, send the data and quit.

[Source](/reference-sources/Effect.Smtp.cinch) · line 286

<a id="value-736d74702d73656e642d6d61696c2d7374617274746c73"></a>

## `smtp-send-mail-starttls`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-send-mail-starttls</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpAuth</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>))))))))</code></pre>

Send one message using `STARTTLS` (RFC 3207): dial plaintext, upgrade to TLS 1.3 via `STARTTLS`,
greet over the encrypted channel, authenticate when requested, deliver the envelope and body, and quit.

[Source](/reference-sources/Effect.Smtp.cinch) · line 620

<a id="value-736d74702d73656e642d6d61696c2d746c73"></a>

## `smtp-send-mail-tls`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-send-mail-tls</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpAuth</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>))))))))</code></pre>

Send one message over implicit TLS 1.3 (SMTPS): dial TLS, greet, authenticate when requested,
deliver the envelope and body, and quit.

[Source](/reference-sources/Effect.Smtp.cinch) · line 603

<a id="value-736d74702d7374617274746c73"></a>

## `smtp-starttls`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-starttls</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>))))</code></pre>

Upgrade a plaintext `SmtpClient` to TLS 1.3 via `STARTTLS` (RFC 3207).
Greets if needed, verifies the server advertises `STARTTLS`, issues `STARTTLS` expecting `220`,
completes the TLS 1.3 client handshake, and resets the greeting state to `NotGreeted` so a fresh
encrypted `EHLO` is sent per RFC 3207 section 4.2.

[Source](/reference-sources/Effect.Smtp.cinch) · line 375

<a id="value-736d74702d746c732d61757468"></a>

## `smtp-tls-auth`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-tls-auth</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpAuth</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>))))</code></pre>

Authenticate over an encrypted TLS 1.3 SMTP connection. Passes `True` for TLS to
`smtp-auth-start` so `PLAIN` authentication is permitted to non-localhost hosts.

[Source](/reference-sources/Effect.Smtp.cinch) · line 466

<a id="value-736d74702d746c732d617574682d6f666665726564"></a>

## `smtp-tls-auth-offered`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-tls-auth-offered</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return the authentication mechanisms advertised by the server over TLS.

[Source](/reference-sources/Effect.Smtp.cinch) · line 459

<a id="value-736d74702d746c732d636c6f7365"></a>

## `smtp-tls-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-tls-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>)))</code></pre>

Close the TLS SMTP connection (sending `close_notify` and closing the socket) without `QUIT`.

[Source](/reference-sources/Effect.Smtp.cinch) · line 558

<a id="value-736d74702d746c732d64617461"></a>

## `smtp-tls-data`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-tls-data</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>))))</code></pre>

Send the transparency-encoded message body over TLS after `DATA` (354) and expect 250.

[Source](/reference-sources/Effect.Smtp.cinch) · line 522

<a id="value-736d74702d746c732d657874656e73696f6e"></a>

## `smtp-tls-extension`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-tls-extension</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>))))</code></pre>

Query whether the server offers an extension over TLS, greeting first if needed.

[Source](/reference-sources/Effect.Smtp.cinch) · line 451

<a id="value-736d74702d746c732d68656c6c6f"></a>

## `smtp-tls-hello`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-tls-hello</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>))))</code></pre>

Choose the local name to greet with over TLS; refused once the server has been greeted.

[Source](/reference-sources/Effect.Smtp.cinch) · line 442

<a id="value-736d74702d746c732d6d61696c"></a>

## `smtp-tls-mail`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-tls-mail</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>))))</code></pre>

Start a mail transaction over TLS (greeting first if needed), requesting `BODY=8BITMIME`
and `SMTPUTF8` when advertised.

[Source](/reference-sources/Effect.Smtp.cinch) · line 508

<a id="value-736d74702d746c732d6e6f6f70"></a>

## `smtp-tls-noop`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-tls-noop</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>)))</code></pre>

Check the TLS SMTP connection (`NOOP`).

[Source](/reference-sources/Effect.Smtp.cinch) · line 541

<a id="value-736d74702d746c732d71756974"></a>

## `smtp-tls-quit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-tls-quit</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>)))</code></pre>

Send `QUIT` over TLS expecting 221, then send `close_notify` and close the underlying socket.

[Source](/reference-sources/Effect.Smtp.cinch) · line 551

<a id="value-736d74702d746c732d72637074"></a>

## `smtp-tls-rcpt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-tls-rcpt</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>))))</code></pre>

Add a recipient over TLS (accepting any 25x reply).

[Source](/reference-sources/Effect.Smtp.cinch) · line 516

<a id="value-736d74702d746c732d7265736574"></a>

## `smtp-tls-reset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-tls-reset</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>)))</code></pre>

Abort the current mail transaction over TLS (`RSET`).

[Source](/reference-sources/Effect.Smtp.cinch) · line 537

<a id="value-736d74702d746c732d766572696679"></a>

## `smtp-tls-verify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-tls-verify</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpTlsClient</span> <span class="tok-variable">r</span>))))</code></pre>

Ask the server to verify an address over TLS (`VRFY`).

[Source](/reference-sources/Effect.Smtp.cinch) · line 531

<a id="value-736d74702d766572696679"></a>

## `smtp-verify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">smtp-verify</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Smtp</span>.<span class="tok-constructor">SmtpClient</span>)))</code></pre>

Ask the server to verify an address.

[Source](/reference-sources/Effect.Smtp.cinch) · line 249

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Smtp.cinch`. Source SHA-256: `16d83b27183ae475078bad236cbab185a08f43eb061a6b5e287e3e79cb4c43fa`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
