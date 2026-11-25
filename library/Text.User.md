# Text.User

Pure Unix /etc/passwd and /etc/group parsers, user/group lookup tables, and supplementary group resolution.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.User</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.User/source/">View source</a><a href="/packages/search/?q=%2BText.User">Search module</a><a href="/reference-sources/Text.User.cinch" download="Text.User.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md).

<details class="doc-notes"><summary>Module notes</summary>

Pure parsers and lookup functions for Unix user and group account databases.
Supports colon-delimited passwd and group files as well as Linux process status
identity records (`/proc/self/status`). All parsers are total, skip blank lines
and comments, tolerate CRLF line endings, and return structured values or named
`UserError` diagnostics without performing I/O.

</details>

<details class="doc-synopsis"><summary>Synopsis (27 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-GroupInfo">GroupInfo</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GroupInfo</strong></code></li><li><a class="syn-name" href="#t-ProcIdentity">ProcIdentity</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ProcIdentity</strong></code></li><li><a class="syn-name" href="#t-UserError">UserError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UserError</strong></code></li><li><a class="syn-name" href="#t-UserInfo">UserInfo</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UserInfo</strong></code></li><li><a class="syn-name" href="#v-group-gid">group-gid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">group-gid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-group-members">group-members</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">group-members</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-group-name">group-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">group-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-lookup-group-id-in">lookup-group-id-in</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lookup-group-id-in</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserError" title="Text.User.UserError">UserError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a></code></li><li><a class="syn-name" href="#v-lookup-group-in">lookup-group-in</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lookup-group-in</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserError" title="Text.User.UserError">UserError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a></code></li><li><a class="syn-name" href="#v-lookup-user-id-in">lookup-user-id-in</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lookup-user-id-in</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserError" title="Text.User.UserError">UserError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a></code></li><li><a class="syn-name" href="#v-lookup-user-in">lookup-user-in</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lookup-user-in</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserError" title="Text.User.UserError">UserError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a></code></li><li><a class="syn-name" href="#v-parse-group">parse-group</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-group</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a></code></li><li><a class="syn-name" href="#v-parse-passwd">parse-passwd</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-passwd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a></code></li><li><a class="syn-name" href="#v-parse-proc-status">parse-proc-status</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-proc-status</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-ProcIdentity" title="Text.User.ProcIdentity">ProcIdentity</a></code></li><li><a class="syn-name" href="#v-proc-effective-gid">proc-effective-gid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">proc-effective-gid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-ProcIdentity" title="Text.User.ProcIdentity">ProcIdentity</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-proc-effective-uid">proc-effective-uid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">proc-effective-uid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-ProcIdentity" title="Text.User.ProcIdentity">ProcIdentity</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-proc-real-gid">proc-real-gid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">proc-real-gid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-ProcIdentity" title="Text.User.ProcIdentity">ProcIdentity</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-proc-real-uid">proc-real-uid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">proc-real-uid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-ProcIdentity" title="Text.User.ProcIdentity">ProcIdentity</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-proc-supplementary-gids">proc-supplementary-gids</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">proc-supplementary-gids</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-ProcIdentity" title="Text.User.ProcIdentity">ProcIdentity</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-user-error-message">user-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">user-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserError" title="Text.User.UserError">UserError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-user-gid">user-gid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">user-gid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-user-group-ids-in">user-group-ids-in</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">user-group-ids-in</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-user-home-dir">user-home-dir</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">user-home-dir</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-user-name">user-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">user-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-user-shell">user-shell</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">user-shell</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-user-uid">user-uid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">user-uid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-user-username">user-username</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">user-username</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-GroupInfo">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GroupInfo</strong></code><span class="doc-sig-links"><a href="#t-GroupInfo" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L79" title="View source at line 79">Source</a></span></div>
<div class="doc-body">

Construct a group record with gid, group name, and member usernames.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-GroupInfo"><code><strong class="tok-constructor">GroupInfo</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Text.User/#instances">Text.User</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-ProcIdentity">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ProcIdentity</strong></code><span class="doc-sig-links"><a href="#t-ProcIdentity" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L159" title="View source at line 159">Source</a></span></div>
<div class="doc-body">

Construct a process identity with real uid, effective uid, real gid, effective gid, and supplementary gids.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ProcIdentity"><code><strong class="tok-constructor">ProcIdentity</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-ProcIdentity" title="Text.User.ProcIdentity">ProcIdentity</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-ProcIdentity" title="Text.User.ProcIdentity">ProcIdentity</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Text.User/#instances">Text.User</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-UserError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UserError</strong></code><span class="doc-sig-links"><a href="#t-UserError" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L115" title="View source at line 115">Source</a></span></div>
<div class="doc-body">

Structured errors returned by user and group lookup operations.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UnknownUserError"><code><strong class="tok-constructor">UnknownUserError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserError" title="Text.User.UserError">UserError</a></code></li><li id="k-UnknownUserIdError"><code><strong class="tok-constructor">UnknownUserIdError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserError" title="Text.User.UserError">UserError</a></code></li><li id="k-UnknownGroupError"><code><strong class="tok-constructor">UnknownGroupError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserError" title="Text.User.UserError">UserError</a></code></li><li id="k-UnknownGroupIdError"><code><strong class="tok-constructor">UnknownGroupIdError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserError" title="Text.User.UserError">UserError</a></code></li><li id="k-UserIoError"><code><strong class="tok-constructor">UserIoError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserError" title="Text.User.UserError">UserError</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserError" title="Text.User.UserError">UserError</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Text.User/#instances">Text.User</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-UserInfo">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UserInfo</strong></code><span class="doc-sig-links"><a href="#t-UserInfo" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L33" title="View source at line 33">Source</a></span></div>
<div class="doc-body">

Construct a user account record with uid, gid, username, primary display name, home directory, and login shell.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UserInfo"><code><strong class="tok-constructor">UserInfo</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Text.User/#instances">Text.User</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-group-gid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">group-gid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-group-gid" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

Numeric group identifier string.

</div>
</article>

<article class="doc-entry" id="v-group-members">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">group-members</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-group-members" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L92" title="View source at line 92">Source</a></span></div>
<div class="doc-body">

Explicit supplementary member usernames listed in the group entry.

</div>
</article>

<article class="doc-entry" id="v-group-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">group-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-group-name" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L87" title="View source at line 87">Source</a></span></div>
<div class="doc-body">

Group name.

</div>
</article>

<article class="doc-entry" id="v-lookup-group-id-in">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lookup-group-id-in</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserError" title="Text.User.UserError">UserError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a></code><span class="doc-sig-links"><a href="#v-lookup-group-id-in" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L518" title="View source at line 518">Source</a></span></div>
<div class="doc-body">

Look up a group by exact numeric GID string within a parsed list of `GroupInfo` records.

</div>
</article>

<article class="doc-entry" id="v-lookup-group-in">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lookup-group-in</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserError" title="Text.User.UserError">UserError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a></code><span class="doc-sig-links"><a href="#v-lookup-group-in" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L510" title="View source at line 510">Source</a></span></div>
<div class="doc-body">

Look up a group by exact group name within a parsed list of `GroupInfo` records.

</div>
</article>

<article class="doc-entry" id="v-lookup-user-id-in">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lookup-user-id-in</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserError" title="Text.User.UserError">UserError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a></code><span class="doc-sig-links"><a href="#v-lookup-user-id-in" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L502" title="View source at line 502">Source</a></span></div>
<div class="doc-body">

Look up a user by exact numeric UID string within a parsed list of `UserInfo` records.

</div>
</article>

<article class="doc-entry" id="v-lookup-user-in">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lookup-user-in</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserError" title="Text.User.UserError">UserError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a></code><span class="doc-sig-links"><a href="#v-lookup-user-in" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L494" title="View source at line 494">Source</a></span></div>
<div class="doc-body">

Look up a user by exact username within a parsed list of `UserInfo` records.

</div>
</article>

<article class="doc-entry" id="v-parse-group">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-group</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a></code><span class="doc-sig-links"><a href="#v-parse-group" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L390" title="View source at line 390">Source</a></span></div>
<div class="doc-body">

Parse colon-delimited Unix `/etc/group` content into a list of `GroupInfo` records.
Skips blank lines, `#` comments, and malformed lines with fewer than 3 fields,
strips trailing `\r`, and splits the comma-delimited member list.

</div>
</article>

<article class="doc-entry" id="v-parse-passwd">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-passwd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a></code><span class="doc-sig-links"><a href="#v-parse-passwd" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L339" title="View source at line 339">Source</a></span></div>
<div class="doc-body">

Parse colon-delimited Unix `/etc/passwd` content into a list of `UserInfo` records.
Skips blank lines, `#` comments, and malformed lines with fewer than 6 fields,
strips trailing `\r`, and extracts the primary GECOS name before the first comma.

</div>
</article>

<article class="doc-entry" id="v-parse-proc-status">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-proc-status</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-ProcIdentity" title="Text.User.ProcIdentity">ProcIdentity</a></code><span class="doc-sig-links"><a href="#v-parse-proc-status" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L442" title="View source at line 442">Source</a></span></div>
<div class="doc-body">

Parse Linux `/proc/self/status` text for `Uid:`, `Gid:`, and `Groups:` lines.
Returns `None` if either `Uid:` or `Gid:` is absent or carries no numeric tokens.

</div>
</article>

<article class="doc-entry" id="v-proc-effective-gid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">proc-effective-gid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-ProcIdentity" title="Text.User.ProcIdentity">ProcIdentity</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-proc-effective-gid" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L177" title="View source at line 177">Source</a></span></div>
<div class="doc-body">

Effective group ID of the process.

</div>
</article>

<article class="doc-entry" id="v-proc-effective-uid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">proc-effective-uid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-ProcIdentity" title="Text.User.ProcIdentity">ProcIdentity</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-proc-effective-uid" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L167" title="View source at line 167">Source</a></span></div>
<div class="doc-body">

Effective user ID of the process.

</div>
</article>

<article class="doc-entry" id="v-proc-real-gid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">proc-real-gid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-ProcIdentity" title="Text.User.ProcIdentity">ProcIdentity</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-proc-real-gid" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L172" title="View source at line 172">Source</a></span></div>
<div class="doc-body">

Real group ID of the process.

</div>
</article>

<article class="doc-entry" id="v-proc-real-uid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">proc-real-uid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-ProcIdentity" title="Text.User.ProcIdentity">ProcIdentity</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-proc-real-uid" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L162" title="View source at line 162">Source</a></span></div>
<div class="doc-body">

Real user ID of the process.

</div>
</article>

<article class="doc-entry" id="v-proc-supplementary-gids">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">proc-supplementary-gids</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-ProcIdentity" title="Text.User.ProcIdentity">ProcIdentity</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-proc-supplementary-gids" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L182" title="View source at line 182">Source</a></span></div>
<div class="doc-body">

Supplementary group IDs reported for the process.

</div>
</article>

<article class="doc-entry" id="v-user-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">user-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserError" title="Text.User.UserError">UserError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-user-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L128" title="View source at line 128">Source</a></span></div>
<div class="doc-body">

Format a `UserError` as a human-readable diagnostic message.

</div>
</article>

<article class="doc-entry" id="v-user-gid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">user-gid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-user-gid" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

Primary group identifier string for the user.

</div>
</article>

<article class="doc-entry" id="v-user-group-ids-in">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">user-group-ids-in</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-user-group-ids-in" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L536" title="View source at line 536">Source</a></span></div>
<div class="doc-body">

Compute the deduplicated list of group IDs for a user against a parsed group table.
The list begins with the user's primary GID (when non-empty), followed by each group
GID whose explicit member list includes the user's username.

</div>
</article>

<article class="doc-entry" id="v-user-home-dir">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">user-home-dir</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-user-home-dir" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L56" title="View source at line 56">Source</a></span></div>
<div class="doc-body">

Absolute path to the user's home directory.

</div>
</article>

<article class="doc-entry" id="v-user-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">user-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-user-name" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L51" title="View source at line 51">Source</a></span></div>
<div class="doc-body">

Primary display name extracted from the GECOS field before the first comma.

</div>
</article>

<article class="doc-entry" id="v-user-shell">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">user-shell</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-user-shell" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

Configured login shell path for the user, or empty when omitted.

</div>
</article>

<article class="doc-entry" id="v-user-uid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">user-uid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-user-uid" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L36" title="View source at line 36">Source</a></span></div>
<div class="doc-body">

Numeric user identifier string (for example, `"1000"`).

</div>
</article>

<article class="doc-entry" id="v-user-username">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">user-username</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-user-username" title="Permalink">#</a><a href="/packages/stdlib/Text.User/source/#L46" title="View source at line 46">Source</a></span></div>
<div class="doc-body">

Login username for the user.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_GroupInfo"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-GroupInfo" title="Text.User.GroupInfo">GroupInfo</a></code> <span class="doc-inst-meta">Implicit · <code>Text.User.Eq_GroupInfo</code></span><div class="doc-child-prose">Structural equality for group account records.</div></li>
<li id="i-Eq_ProcIdentity"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-ProcIdentity" title="Text.User.ProcIdentity">ProcIdentity</a></code> <span class="doc-inst-meta">Implicit · <code>Text.User.Eq_ProcIdentity</code></span><div class="doc-child-prose">Structural equality for process identity records.</div></li>
<li id="i-Eq_UserError"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserError" title="Text.User.UserError">UserError</a></code> <span class="doc-inst-meta">Implicit · <code>Text.User.Eq_UserError</code></span><div class="doc-child-prose">Structural equality for user lookup errors.</div></li>
<li id="i-Eq_UserInfo"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.User/#t-UserInfo" title="Text.User.UserInfo">UserInfo</a></code> <span class="doc-inst-meta">Implicit · <code>Text.User.Eq_UserInfo</code></span><div class="doc-child-prose">Structural equality for user account records.</div></li>
</ul>
