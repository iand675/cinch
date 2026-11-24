# Text.Mail

Parse and format RFC 5322 mail addresses, read message header blocks and parse header dates, with SIMD atom scanning.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Mail</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Mail/source/">View source</a><a href="/packages/search/?q=%2BText.Mail">Search module</a><a href="/reference-sources/Text.Mail.cinch" download="Text.Mail.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.IpAddr](Data.IpAddr.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Text.Mime](Text.Mime.md), [Text.Show](Text.Show.md), [Text.TextProto](Text.TextProto.md), [Text.Time.Rfc2822](Text.Time.Rfc2822.md).

<details class="doc-closure"><summary>Ship-source closure (23 files)</summary>

Copy these modules beside your entry file before importing `Text.Mail`: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.IpAddr](Data.IpAddr.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md), [Data.Time.Zone.Types](Data.Time.Zone.Types.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Mail](Text.Mail.md), [Text.Mime](Text.Mime.md), [Text.TextProto](Text.TextProto.md), [Text.Time.Rfc2822](Text.Time.Rfc2822.md), [Text.Time.Scan](Text.Time.Scan.md).

```sh
cp seed/prelude/Data/Bytes/Build.cinch ./Data.Bytes.Build.cinch
cp seed/prelude/Data/Bytes/Utf8.cinch ./Data.Bytes.Utf8.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Data/IpAddr.cinch ./Data.IpAddr.cinch
cp seed/prelude/Data/Time/Date.cinch ./Data.Time.Date.cinch
cp seed/prelude/Data/Time/DateTime.cinch ./Data.Time.DateTime.cinch
cp seed/prelude/Data/Time/Duration.cinch ./Data.Time.Duration.cinch
cp seed/prelude/Data/Time/Offset.cinch ./Data.Time.Offset.cinch
cp seed/prelude/Data/Time/TimeOfDay.cinch ./Data.Time.TimeOfDay.cinch
cp seed/prelude/Data/Time/Timestamp.cinch ./Data.Time.Timestamp.cinch
cp seed/prelude/Data/Time/Weekday.cinch ./Data.Time.Weekday.cinch
cp seed/prelude/Data/Time/Zone.cinch ./Data.Time.Zone.cinch
cp seed/prelude/Data/Time/Zone/Posix.cinch ./Data.Time.Zone.Posix.cinch
cp seed/prelude/Data/Time/Zone/Types.cinch ./Data.Time.Zone.Types.cinch
cp seed/prelude/Data/Time/Zoned.cinch ./Data.Time.Zoned.cinch
cp seed/prelude/Data/Unicode/Kernel.cinch ./Data.Unicode.Kernel.cinch
cp seed/prelude/Data/Unicode/Tables.cinch ./Data.Unicode.Tables.cinch
cp seed/prelude/Text/Base64/Encoding.cinch ./Text.Base64.Encoding.cinch
cp seed/prelude/Text/Mail.cinch ./Text.Mail.cinch
cp seed/prelude/Text/Mime.cinch ./Text.Mime.cinch
cp seed/prelude/Text/TextProto.cinch ./Text.TextProto.cinch
cp seed/prelude/Text/Time/Rfc2822.cinch ./Text.Time.Rfc2822.cinch
cp seed/prelude/Text/Time/Scan.cinch ./Text.Time.Scan.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis"><summary>Synopsis (19 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-MailAddress">MailAddress</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MailAddress</strong></code></li><li><a class="syn-name" href="#t-MailError">MailError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MailError</strong></code></li><li><a class="syn-name" href="#t-MailMessage">MailMessage</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MailMessage</strong></code></li><li><a class="syn-name" href="#v-mail-address">mail-address</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mail-address</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a></code></li><li><a class="syn-name" href="#v-mail-address-name">mail-address-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mail-address-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-mail-address-spec">mail-address-spec</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mail-address-spec</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-mail-address-to-string">mail-address-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mail-address-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-mail-error-message">mail-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mail-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-mail-header-address-list">mail-header-address-list</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mail-header-address-list</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailMessage" title="Text.Mail.MailMessage">MailMessage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a>)</code></li><li><a class="syn-name" href="#v-mail-header-date">mail-header-date</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mail-header-date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailMessage" title="Text.Mail.MailMessage">MailMessage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a>)</code></li><li><a class="syn-name" href="#v-mail-header-get">mail-header-get</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mail-header-get</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailMessage" title="Text.Mail.MailMessage">MailMessage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-mail-message-body">mail-message-body</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mail-message-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailMessage" title="Text.Mail.MailMessage">MailMessage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-mail-message-header">mail-message-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mail-message-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailMessage" title="Text.Mail.MailMessage">MailMessage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a></code></li><li><a class="syn-name" href="#v-mail-parse-address">mail-parse-address</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mail-parse-address</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a></code></li><li><a class="syn-name" href="#v-mail-parse-address-bytes">mail-parse-address-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mail-parse-address-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a></code></li><li><a class="syn-name" href="#v-mail-parse-address-list">mail-parse-address-list</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mail-parse-address-list</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a>)</code></li><li><a class="syn-name" href="#v-mail-parse-address-list-bytes">mail-parse-address-list-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mail-parse-address-list-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a>)</code></li><li><a class="syn-name" href="#v-mail-parse-date">mail-parse-date</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mail-parse-date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a>)</code></li><li><a class="syn-name" href="#v-mail-read-message">mail-read-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mail-read-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailMessage" title="Text.Mail.MailMessage">MailMessage</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-MailAddress">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MailAddress</strong></code><span class="doc-sig-links"><a href="#t-MailAddress" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L153" title="View source at line 153">Source</a></span></div>
<div class="doc-body">

The display name, then the addr-spec.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MailAddress"><code><strong class="tok-constructor">MailAddress</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MailError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MailError</strong></code><span class="doc-sig-links"><a href="#t-MailError" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L47" title="View source at line 47">Source</a></span></div>
<div class="doc-body">

Why an address, header block or date was rejected.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MailNoAddress"><code><strong class="tok-constructor">MailNoAddress</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailExpectedComma"><code><strong class="tok-constructor">MailExpectedComma</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailExpectedSingleAddress"><code><strong class="tok-constructor">MailExpectedSingleAddress</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailEmptyGroup"><code><strong class="tok-constructor">MailEmptyGroup</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailGroupWithMultipleAddresses"><code><strong class="tok-constructor">MailGroupWithMultipleAddresses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailMissingAtOrAngleAddress"><code><strong class="tok-constructor">MailMissingAtOrAngleAddress</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailNoAngleAddress"><code><strong class="tok-constructor">MailNoAngleAddress</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailUnclosedAngleAddress"><code><strong class="tok-constructor">MailUnclosedAngleAddress</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailBadComment"><code><strong class="tok-constructor">MailBadComment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailNoAddrSpec"><code><strong class="tok-constructor">MailNoAddrSpec</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailEmptyQuotedLocalPart"><code><strong class="tok-constructor">MailEmptyQuotedLocalPart</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailMissingAtInAddrSpec"><code><strong class="tok-constructor">MailMissingAtInAddrSpec</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailNoDomain"><code><strong class="tok-constructor">MailNoDomain</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailInvalidAtom"><code><strong class="tok-constructor">MailInvalidAtom</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailLeadingDot"><code><strong class="tok-constructor">MailLeadingDot</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailDoubleDot"><code><strong class="tok-constructor">MailDoubleDot</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailTrailingDot"><code><strong class="tok-constructor">MailTrailingDot</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailInvalidUtf8"><code><strong class="tok-constructor">MailInvalidUtf8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailUnclosedQuotedString"><code><strong class="tok-constructor">MailUnclosedQuotedString</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailBadQuotedCharacter"><code><strong class="tok-constructor">MailBadQuotedCharacter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailUnclosedDomainLiteral"><code><strong class="tok-constructor">MailUnclosedDomainLiteral</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailBadDomainLiteralCharacter"><code><strong class="tok-constructor">MailBadDomainLiteralCharacter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailInvalidDomainLiteral"><code><strong class="tok-constructor">MailInvalidDomainLiteral</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailMissingWordInPhrase"><code><strong class="tok-constructor">MailMissingWordInPhrase</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailUnknownCharset"><code><strong class="tok-constructor">MailUnknownCharset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailHeaderNotPresent"><code><strong class="tok-constructor">MailHeaderNotPresent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailMalformedInitialLine"><code><strong class="tok-constructor">MailMalformedInitialLine</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailMalformedHeaderLine"><code><strong class="tok-constructor">MailMalformedHeaderLine</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailEmptyMessage"><code><strong class="tok-constructor">MailEmptyMessage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailBareCarriageReturn"><code><strong class="tok-constructor">MailBareCarriageReturn</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li><li id="k-MailDateSyntax"><code><strong class="tok-constructor">MailDateSyntax</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Time.Rfc2822/#t-Rfc2822Error" title="Text.Time.Rfc2822.Rfc2822Error">Rfc2822Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MailMessage">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MailMessage</strong></code><span class="doc-sig-links"><a href="#t-MailMessage" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L602" title="View source at line 602">Source</a></span></div>
<div class="doc-body">

The header, then the body.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MailMessage"><code><strong class="tok-constructor">MailMessage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailMessage" title="Text.Mail.MailMessage">MailMessage</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-mail-address">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mail-address</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a></code><span class="doc-sig-links"><a href="#v-mail-address" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L156" title="View source at line 156">Source</a></span></div>
<div class="doc-body">

Make an address from a display name and an addr-spec.

</div>
</article>

<article class="doc-entry" id="v-mail-address-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mail-address-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-mail-address-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L160" title="View source at line 160">Source</a></span></div>
<div class="doc-body">

The display name, empty when there is none.

</div>
</article>

<article class="doc-entry" id="v-mail-address-spec">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mail-address-spec</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-mail-address-spec" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L164" title="View source at line 164">Source</a></span></div>
<div class="doc-body">

The addr-spec.

</div>
</article>

<article class="doc-entry" id="v-mail-address-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mail-address-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-mail-address-to-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L509" title="View source at line 509">Source</a></span></div>
<div class="doc-body">

Format an address as "<local@domain>" or "name <local@domain>". The local part is quoted unless it is
a valid dot-atom; a printable ASCII name is quoted, and any other name becomes UTF-8 encoded words
(Base64 when the name holds characters an encoded word in a phrase may not, Q encoding otherwise).

</div>
</article>

<article class="doc-entry" id="v-mail-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mail-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-mail-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L112" title="View source at line 112">Source</a></span></div>
<div class="doc-body">

Render an error.

</div>
</article>

<article class="doc-entry" id="v-mail-header-address-list">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mail-header-address-list</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailMessage" title="Text.Mail.MailMessage">MailMessage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a>)</code><span class="doc-sig-links"><a href="#v-mail-header-address-list" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L691" title="View source at line 691">Source</a></span></div>
<div class="doc-body">

A field's value parsed as an address list.

</div>
</article>

<article class="doc-entry" id="v-mail-header-date">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mail-header-date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailMessage" title="Text.Mail.MailMessage">MailMessage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a>)</code><span class="doc-sig-links"><a href="#v-mail-header-date" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L685" title="View source at line 685">Source</a></span></div>
<div class="doc-body">

The message's Date field, parsed.

</div>
</article>

<article class="doc-entry" id="v-mail-header-get">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mail-header-get</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailMessage" title="Text.Mail.MailMessage">MailMessage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-mail-header-get" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L679" title="View source at line 679">Source</a></span></div>
<div class="doc-body">

The first value of a header field, empty when absent.

</div>
</article>

<article class="doc-entry" id="v-mail-message-body">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mail-message-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailMessage" title="Text.Mail.MailMessage">MailMessage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-mail-message-body" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L609" title="View source at line 609">Source</a></span></div>
<div class="doc-body">

The body bytes.

</div>
</article>

<article class="doc-entry" id="v-mail-message-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mail-message-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailMessage" title="Text.Mail.MailMessage">MailMessage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a></code><span class="doc-sig-links"><a href="#v-mail-message-header" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L605" title="View source at line 605">Source</a></span></div>
<div class="doc-body">

The header fields.

</div>
</article>

<article class="doc-entry" id="v-mail-parse-address">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mail-parse-address</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a></code><span class="doc-sig-links"><a href="#v-mail-parse-address" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L168" title="View source at line 168">Source</a></span></div>
<div class="doc-body">

Parse exactly one address (a group is accepted when it holds exactly one).

</div>
</article>

<article class="doc-entry" id="v-mail-parse-address-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mail-parse-address-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a></code><span class="doc-sig-links"><a href="#v-mail-parse-address-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L172" title="View source at line 172">Source</a></span></div>
<div class="doc-body">

Parse exactly one address from raw header bytes, which need not be valid UTF-8 (invalid sequences are errors).

</div>
</article>

<article class="doc-entry" id="v-mail-parse-address-list">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mail-parse-address-list</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a>)</code><span class="doc-sig-links"><a href="#v-mail-parse-address-list" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L184" title="View source at line 184">Source</a></span></div>
<div class="doc-body">

Parse a comma-separated address list; groups contribute their members.

</div>
</article>

<article class="doc-entry" id="v-mail-parse-address-list-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mail-parse-address-list-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailAddress" title="Text.Mail.MailAddress">MailAddress</a>)</code><span class="doc-sig-links"><a href="#v-mail-parse-address-list-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L188" title="View source at line 188">Source</a></span></div>
<div class="doc-body">

Parse an address list from raw header bytes, which need not be valid UTF-8 (invalid sequences are errors).

</div>
</article>

<article class="doc-entry" id="v-mail-parse-date">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mail-parse-date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a>)</code><span class="doc-sig-links"><a href="#v-mail-parse-date" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L579" title="View source at line 579">Source</a></span></div>
<div class="doc-body">

Parse a header date: line breaks (CR LF) are removed first, a lone CR is rejected, and the date may use
the obsolete forms (comments, two- and three-digit years, North American zone names).

</div>
</article>

<article class="doc-entry" id="v-mail-read-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mail-read-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailError" title="Text.Mail.MailError">MailError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mail/#t-MailMessage" title="Text.Mail.MailMessage">MailMessage</a></code><span class="doc-sig-links"><a href="#v-mail-read-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Mail/source/#L615" title="View source at line 615">Source</a></span></div>
<div class="doc-body">

Read a message: header lines (folded continuation lines joined by one space) up to a blank line or the
end of the input, then the body. Input that ends before any field or blank line is an empty message. Field names are canonicalized but not otherwise restricted; a field with
an empty name is skipped, and leading blanks are removed from values.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Mail.cinch`](/packages/stdlib/Text.Mail/source/). Source SHA-256: `1885b646b2bcd7e39ad20326334b337c32e134496b8b0dc0cac4e14bea486090`.
