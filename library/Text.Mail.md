# Text.Mail

Parse and format RFC 5322 mail addresses, read message header blocks and parse header dates, with SIMD atom scanning.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Mail`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Mail.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.IpAddr](Data.IpAddr.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Text.Mime](Text.Mime.md), [Text.Show](Text.Show.md), [Text.TextProto](Text.TextProto.md), [Text.Time.Rfc2822](Text.Time.Rfc2822.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.IpAddr](Data.IpAddr.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md), [Data.Time.Zone.Types](Data.Time.Zone.Types.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Mail](Text.Mail.md), [Text.Mime](Text.Mime.md), [Text.TextProto](Text.TextProto.md), [Text.Time.Rfc2822](Text.Time.Rfc2822.md), [Text.Time.Scan](Text.Time.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Internet mail messages (RFC 5322): header blocks, mailbox addresses and dates.
Addresses parse as RFC 5322 address lists, including groups, comments,
quoted strings, domain literals holding IP addresses, UTF-8 text (RFC 6532)
and RFC 2047 encoded words in display names (UTF-8, ISO-8859-1 and US-ASCII
charsets). An address is tried first as a bare addr-spec, optionally followed
by a comment that becomes its display name, then as a phrase and an angle
address. Empty list entries are skipped, as the obsolete syntax permits.
Formatting quotes local parts and display names only where needed and encodes
non-ASCII names as encoded words. Runs of atom and quoted-string text are
scanned sixteen bytes at a time; non-ASCII bytes fall to a scalar path that
validates UTF-8. Header blocks are read without the stricter field-name rules
of HTTP, and dates accept the obsolete forms of RFC 5322.

</details>

# Types and constructors

<a id="type-4d61696c41646472657373"></a>

## `MailAddress`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MailAddress</span></code></pre>

The display name, then the addr-spec.

[Source](/reference-sources/Text.Mail.cinch) · line 153

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailAddress</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailAddress</span>))</code></pre>

<a id="type-4d61696c4572726f72"></a>

## `MailError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MailError</span></code></pre>

Why an address, header block or date was rejected.

[Source](/reference-sources/Text.Mail.cinch) · line 47

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailNoAddress</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailExpectedComma</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailExpectedSingleAddress</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailEmptyGroup</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailGroupWithMultipleAddresses</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailMissingAtOrAngleAddress</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailNoAngleAddress</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailUnclosedAngleAddress</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailBadComment</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailNoAddrSpec</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailEmptyQuotedLocalPart</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailMissingAtInAddrSpec</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailNoDomain</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailInvalidAtom</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailLeadingDot</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailDoubleDot</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailTrailingDot</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailInvalidUtf8</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailUnclosedQuotedString</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailBadQuotedCharacter</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailUnclosedDomainLiteral</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailBadDomainLiteralCharacter</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailInvalidDomainLiteral</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailMissingWordInPhrase</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailUnknownCharset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailHeaderNotPresent</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailMalformedInitialLine</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailMalformedHeaderLine</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailEmptyMessage</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailBareCarriageReturn</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailDateSyntax</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Rfc2822</span>.<span class="tok-constructor">Rfc2822Error</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span>)</code></pre>

<a id="type-4d61696c4d657373616765"></a>

## `MailMessage`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MailMessage</span></code></pre>

The header, then the body.

[Source](/reference-sources/Text.Mail.cinch) · line 602

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MailMessage</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailMessage</span>))</code></pre>

# Values

<a id="value-6d61696c2d61646472657373"></a>

## `mail-address`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mail-address</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailAddress</span>))</code></pre>

Make an address from a display name and an addr-spec.

[Source](/reference-sources/Text.Mail.cinch) · line 156

<a id="value-6d61696c2d616464726573732d6e616d65"></a>

## `mail-address-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mail-address-name</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailAddress</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The display name, empty when there is none.

[Source](/reference-sources/Text.Mail.cinch) · line 160

<a id="value-6d61696c2d616464726573732d73706563"></a>

## `mail-address-spec`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mail-address-spec</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailAddress</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The addr-spec.

[Source](/reference-sources/Text.Mail.cinch) · line 164

<a id="value-6d61696c2d616464726573732d746f2d737472696e67"></a>

## `mail-address-to-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mail-address-to-string</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailAddress</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Format an address as "<local@domain>" or "name <local@domain>". The local part is quoted unless it is
a valid dot-atom; a printable ASCII name is quoted, and any other name becomes UTF-8 encoded words
(Base64 when the name holds characters an encoded word in a phrase may not, Q encoding otherwise).

[Source](/reference-sources/Text.Mail.cinch) · line 509

<a id="value-6d61696c2d6572726f722d6d657373616765"></a>

## `mail-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mail-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an error.

[Source](/reference-sources/Text.Mail.cinch) · line 112

<a id="value-6d61696c2d6865616465722d616464726573732d6c697374"></a>

## `mail-header-address-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mail-header-address-list</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailMessage</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailAddress</span>))))</code></pre>

A field's value parsed as an address list.

[Source](/reference-sources/Text.Mail.cinch) · line 691

<a id="value-6d61696c2d6865616465722d64617465"></a>

## `mail-header-date`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mail-header-date</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailMessage</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>)))</code></pre>

The message's Date field, parsed.

[Source](/reference-sources/Text.Mail.cinch) · line 685

<a id="value-6d61696c2d6865616465722d676574"></a>

## `mail-header-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mail-header-get</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailMessage</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

The first value of a header field, empty when absent.

[Source](/reference-sources/Text.Mail.cinch) · line 679

<a id="value-6d61696c2d6d6573736167652d626f6479"></a>

## `mail-message-body`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mail-message-body</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailMessage</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The body bytes.

[Source](/reference-sources/Text.Mail.cinch) · line 609

<a id="value-6d61696c2d6d6573736167652d686561646572"></a>

## `mail-message-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mail-message-header</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailMessage</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span>)</code></pre>

The header fields.

[Source](/reference-sources/Text.Mail.cinch) · line 605

<a id="value-6d61696c2d70617273652d61646472657373"></a>

## `mail-parse-address`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mail-parse-address</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailAddress</span>))</code></pre>

Parse exactly one address (a group is accepted when it holds exactly one).

[Source](/reference-sources/Text.Mail.cinch) · line 168

<a id="value-6d61696c2d70617273652d616464726573732d6279746573"></a>

## `mail-parse-address-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mail-parse-address-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailAddress</span>))</code></pre>

Parse exactly one address from raw header bytes, which need not be valid UTF-8 (invalid sequences are errors).

[Source](/reference-sources/Text.Mail.cinch) · line 172

<a id="value-6d61696c2d70617273652d616464726573732d6c697374"></a>

## `mail-parse-address-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mail-parse-address-list</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailAddress</span>)))</code></pre>

Parse a comma-separated address list; groups contribute their members.

[Source](/reference-sources/Text.Mail.cinch) · line 184

<a id="value-6d61696c2d70617273652d616464726573732d6c6973742d6279746573"></a>

## `mail-parse-address-list-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mail-parse-address-list-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailAddress</span>)))</code></pre>

Parse an address list from raw header bytes, which need not be valid UTF-8 (invalid sequences are errors).

[Source](/reference-sources/Text.Mail.cinch) · line 188

<a id="value-6d61696c2d70617273652d64617465"></a>

## `mail-parse-date`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mail-parse-date</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>)))</code></pre>

Parse a header date: line breaks (CR LF) are removed first, a lone CR is rejected, and the date may use
the obsolete forms (comments, two- and three-digit years, North American zone names).

[Source](/reference-sources/Text.Mail.cinch) · line 579

<a id="value-6d61696c2d726561642d6d657373616765"></a>

## `mail-read-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mail-read-message</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Mail</span>.<span class="tok-constructor">MailMessage</span>))</code></pre>

Read a message: header lines (folded continuation lines joined by one space) up to a blank line or the
end of the input, then the body. Input that ends before any field or blank line is an empty message. Field names are canonicalized but not otherwise restricted; a field with
an empty name is skipped, and leading blanks are removed from values.

[Source](/reference-sources/Text.Mail.cinch) · line 615

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Mail.cinch`. Source SHA-256: `1885b646b2bcd7e39ad20326334b337c32e134496b8b0dc0cac4e14bea486090`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
