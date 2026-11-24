# Text.Smtp

SMTP without a connection: reply state machine, command validation, extensions, dot-stuffing and PLAIN/CRAM-MD5 exchanges.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Smtp</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Smtp/">View source</a><a href="/library/search/?q=%2BText.Smtp">Search module</a><a href="/reference-sources/Text.Smtp.cinch" download="Text.Smtp.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Base64](Text.Base64.md), [Text.Hex](Text.Hex.md), [Text.Md5](Text.Md5.md), [Text.Show](Text.Show.md).

<details class="doc-closure"><summary>Ship-source closure (7 files)</summary>

Copy these modules beside your entry file before importing `Text.Smtp`: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Base64](Text.Base64.md), [Text.Binary](Text.Binary.md), [Text.Hex](Text.Hex.md), [Text.Md5](Text.Md5.md), [Text.Smtp](Text.Smtp.md).

```sh
cp seed/prelude/Data/Bytes/Build.cinch ./Data.Bytes.Build.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Text/Base64.cinch ./Text.Base64.cinch
cp seed/prelude/Text/Binary.cinch ./Text.Binary.cinch
cp seed/prelude/Text/Hex.cinch ./Text.Hex.cinch
cp seed/prelude/Text/Md5.cinch ./Text.Md5.cinch
cp seed/prelude/Text/Smtp.cinch ./Text.Smtp.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis"><summary>Synopsis (19 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ReplyReader">ReplyReader</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ReplyReader</strong></code></li><li><a class="syn-name" href="#t-ReplyStep">ReplyStep</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ReplyStep</strong></code></li><li><a class="syn-name" href="#t-SmtpAuth">SmtpAuth</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SmtpAuth</strong></code></li><li><a class="syn-name" href="#t-SmtpError">SmtpError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SmtpError</strong></code></li><li><a class="syn-name" href="#t-SmtpReply">SmtpReply</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SmtpReply</strong></code></li><li><a class="syn-name" href="#v-smtp-auth-mechanisms">smtp-auth-mechanisms</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">smtp-auth-mechanisms</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-smtp-auth-next">smtp-auth-next</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">smtp-auth-next</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpAuth" title="Text.Smtp.SmtpAuth">SmtpAuth</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-smtp-auth-start">smtp-auth-start</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">smtp-auth-start</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpAuth" title="Text.Smtp.SmtpAuth">SmtpAuth</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-smtp-dot-encode">smtp-dot-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">smtp-dot-encode</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-smtp-error-message">smtp-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">smtp-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-smtp-extension-lookup">smtp-extension-lookup</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">smtp-extension-lookup</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-smtp-extensions">smtp-extensions</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">smtp-extensions</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-smtp-is-localhost">smtp-is-localhost</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">smtp-is-localhost</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-smtp-reply-code">smtp-reply-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">smtp-reply-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpReply" title="Text.Smtp.SmtpReply">SmtpReply</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-smtp-reply-line">smtp-reply-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">smtp-reply-line</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-ReplyReader" title="Text.Smtp.ReplyReader">ReplyReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-ReplyStep" title="Text.Smtp.ReplyStep">ReplyStep</a></code></li><li><a class="syn-name" href="#v-smtp-reply-message">smtp-reply-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">smtp-reply-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpReply" title="Text.Smtp.SmtpReply">SmtpReply</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-smtp-reply-reader">smtp-reply-reader</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">smtp-reply-reader</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-ReplyReader" title="Text.Smtp.ReplyReader">ReplyReader</a></code></li><li><a class="syn-name" href="#v-smtp-strip-line">smtp-strip-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">smtp-strip-line</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-smtp-validate-line">smtp-validate-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">smtp-validate-line</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ReplyReader">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ReplyReader</strong></code><span class="doc-sig-links"><a href="#t-ReplyReader" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L105" title="View source at line 105">Source</a></span></div>
<div class="doc-body">

Reading a reply: before its first line (with the expected code: 0 accepts any, one or two digits match a
prefix), or within a multi-line reply (code, message so far, and whether the first code was unexpected).

</div>
</article>

<article class="doc-entry" id="t-ReplyStep">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ReplyStep</strong></code><span class="doc-sig-links"><a href="#t-ReplyStep" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L112" title="View source at line 112">Source</a></span></div>
<div class="doc-body">

What one line does to a reply in progress.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-NeedLine"><code><strong class="tok-constructor">NeedLine</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-ReplyReader" title="Text.Smtp.ReplyReader">ReplyReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-ReplyStep" title="Text.Smtp.ReplyStep">ReplyStep</a></code></li><li id="k-ReplyDone"><code><strong class="tok-constructor">ReplyDone</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpReply" title="Text.Smtp.SmtpReply">SmtpReply</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-ReplyStep" title="Text.Smtp.ReplyStep">ReplyStep</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-SmtpAuth">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SmtpAuth</strong></code><span class="doc-sig-links"><a href="#t-SmtpAuth" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L345" title="View source at line 345">Source</a></span></div>
<div class="doc-body">

A way to authenticate.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SmtpPlainAuth"><code><strong class="tok-constructor">SmtpPlainAuth</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpAuth" title="Text.Smtp.SmtpAuth">SmtpAuth</a></code></li><li id="k-SmtpCramMd5Auth"><code><strong class="tok-constructor">SmtpCramMd5Auth</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpAuth" title="Text.Smtp.SmtpAuth">SmtpAuth</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-SmtpError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SmtpError</strong></code><span class="doc-sig-links"><a href="#t-SmtpError" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

Why an SMTP exchange failed.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SmtpReplyError"><code><strong class="tok-constructor">SmtpReplyError</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a></code></li><li id="k-SmtpProtocolError"><code><strong class="tok-constructor">SmtpProtocolError</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a></code></li><li id="k-SmtpEof"><code><strong class="tok-constructor">SmtpEof</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a></code></li><li id="k-SmtpNetError"><code><strong class="tok-constructor">SmtpNetError</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a></code></li><li id="k-SmtpBadLine"><code><strong class="tok-constructor">SmtpBadLine</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a></code></li><li id="k-SmtpAuthRefused"><code><strong class="tok-constructor">SmtpAuthRefused</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a></code></li><li id="k-SmtpHelloTooLate"><code><strong class="tok-constructor">SmtpHelloTooLate</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a></code></li><li id="k-SmtpNoAuth"><code><strong class="tok-constructor">SmtpNoAuth</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a></code></li><li id="k-SmtpBadChallenge"><code><strong class="tok-constructor">SmtpBadChallenge</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a></code></li><li id="k-SmtpClosed"><code><strong class="tok-constructor">SmtpClosed</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-SmtpReply">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SmtpReply</strong></code><span class="doc-sig-links"><a href="#t-SmtpReply" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L93" title="View source at line 93">Source</a></span></div>
<div class="doc-body">

Code and message.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SmtpReply"><code><strong class="tok-constructor">SmtpReply</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpReply" title="Text.Smtp.SmtpReply">SmtpReply</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-smtp-auth-mechanisms">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">smtp-auth-mechanisms</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-smtp-auth-mechanisms" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L261" title="View source at line 261">Source</a></span></div>
<div class="doc-body">

The authentication mechanisms an extension list offers (the AUTH keyword's space-separated parameters).

</div>
</article>

<article class="doc-entry" id="v-smtp-auth-next">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">smtp-auth-next</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpAuth" title="Text.Smtp.SmtpAuth">SmtpAuth</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-smtp-auth-next" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L368" title="View source at line 368">Source</a></span></div>
<div class="doc-body">

Answer a server message (decoded) that is a challenge when more is set: the next response, or None when the
exchange is complete on this side.

</div>
</article>

<article class="doc-entry" id="v-smtp-auth-start">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">smtp-auth-start</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpAuth" title="Text.Smtp.SmtpAuth">SmtpAuth</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-smtp-auth-start" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L358" title="View source at line 358">Source</a></span></div>
<div class="doc-body">

Start an exchange with a server (its name and whether the connection is encrypted): the mechanism and the
initial response (empty when the mechanism sends none).

</div>
</article>

<article class="doc-entry" id="v-smtp-dot-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">smtp-dot-encode</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-smtp-dot-encode" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L286" title="View source at line 286">Source</a></span></div>
<div class="doc-body">

The transparency encoding of a message body: a lone LF becomes CR LF, a dot that starts a line is doubled,
and the end-of-data line follows (after a CR LF unless the body already ends with one).

</div>
</article>

<article class="doc-entry" id="v-smtp-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">smtp-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-smtp-error-message" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

The error's text: replies as "%03d message", the rest as the protocol's standard wording.

</div>
</article>

<article class="doc-entry" id="v-smtp-extension-lookup">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">smtp-extension-lookup</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-smtp-extension-lookup" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L235" title="View source at line 235">Source</a></span></div>
<div class="doc-body">

The parameters of an extension keyword (compared after upper-casing the query).

</div>
</article>

<article class="doc-entry" id="v-smtp-extensions">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">smtp-extensions</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-smtp-extensions" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L200" title="View source at line 200">Source</a></span></div>
<div class="doc-body">

The extensions of an EHLO reply: each line after the first as (keyword, parameters), latest first so a
repeated keyword's last parameters win.

</div>
</article>

<article class="doc-entry" id="v-smtp-is-localhost">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">smtp-is-localhost</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-smtp-is-localhost" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L353" title="View source at line 353">Source</a></span></div>
<div class="doc-body">

Whether a server name is the local machine.

</div>
</article>

<article class="doc-entry" id="v-smtp-reply-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">smtp-reply-code</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpReply" title="Text.Smtp.SmtpReply">SmtpReply</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-smtp-reply-code" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L96" title="View source at line 96">Source</a></span></div>
<div class="doc-body">

The reply code.

</div>
</article>

<article class="doc-entry" id="v-smtp-reply-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">smtp-reply-line</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-ReplyReader" title="Text.Smtp.ReplyReader">ReplyReader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-ReplyStep" title="Text.Smtp.ReplyStep">ReplyStep</a></code><span class="doc-sig-links"><a href="#v-smtp-reply-line" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L130" title="View source at line 130">Source</a></span></div>
<div class="doc-body">

Feed one line (without its line end) to a reply in progress.

</div>
</article>

<article class="doc-entry" id="v-smtp-reply-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">smtp-reply-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpReply" title="Text.Smtp.SmtpReply">SmtpReply</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-smtp-reply-message" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L100" title="View source at line 100">Source</a></span></div>
<div class="doc-body">

The reply message.

</div>
</article>

<article class="doc-entry" id="v-smtp-reply-reader">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">smtp-reply-reader</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-ReplyReader" title="Text.Smtp.ReplyReader">ReplyReader</a></code><span class="doc-sig-links"><a href="#v-smtp-reply-reader" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L119" title="View source at line 119">Source</a></span></div>
<div class="doc-body">

A reader for a reply with an expected code.

</div>
</article>

<article class="doc-entry" id="v-smtp-strip-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">smtp-strip-line</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-smtp-strip-line" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L180" title="View source at line 180">Source</a></span></div>
<div class="doc-body">

A received line without its LF and a CR before it.

</div>
</article>

<article class="doc-entry" id="v-smtp-validate-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">smtp-validate-line</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Smtp/#t-SmtpError" title="Text.Smtp.SmtpError">SmtpError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-smtp-validate-line" title="Permalink">#</a><a href="/library/source/Text.Smtp/#L191" title="View source at line 191">Source</a></span></div>
<div class="doc-body">

Refuse a command argument that contains CR or LF.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Smtp.cinch`](/library/source/Text.Smtp/). Source SHA-256: `ab46aeac80b9aa3bcb33399324012c1533fb999ecbf5c9012df64a01130da419`.
