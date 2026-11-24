# Text.User

Pure Unix /etc/passwd and /etc/group parsers, user/group lookup tables, and supplementary group resolution.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.User`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.User.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md).

Non-embedded source closure: [Text.User](Text.User.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Pure parsers and lookup functions for Unix user and group account databases.
Supports colon-delimited passwd and group files as well as Linux process status
identity records (`/proc/self/status`). All parsers are total, skip blank lines
and comments, tolerate CRLF line endings, and return structured values or named
`UserError` diagnostics without performing I/O.

</details>

# Types and constructors

<a id="type-47726f7570496e666f"></a>

## `GroupInfo`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GroupInfo</span></code></pre>

Construct a group record with gid, group name, and member usernames.

[Source](/reference-sources/Text.User.cinch) · line 79

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GroupInfo</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">GroupInfo</span>)))</code></pre>

<a id="type-50726f634964656e74697479"></a>

## `ProcIdentity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ProcIdentity</span></code></pre>

Construct a process identity with real uid, effective uid, real gid, effective gid, and supplementary gids.

[Source](/reference-sources/Text.User.cinch) · line 159

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ProcIdentity</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">ProcIdentity</span>)))))</code></pre>

<a id="type-557365724572726f72"></a>

## `UserError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UserError</span></code></pre>

Structured errors returned by user and group lookup operations.

[Source](/reference-sources/Text.User.cinch) · line 115

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnknownUserError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnknownUserIdError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnknownGroupError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnknownGroupIdError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UserIoError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span>)</code></pre>

<a id="type-55736572496e666f"></a>

## `UserInfo`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UserInfo</span></code></pre>

Construct a user account record with uid, gid, username, primary display name, home directory, and login shell.

[Source](/reference-sources/Text.User.cinch) · line 33

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UserInfo</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span>))))))</code></pre>

# Values

<a id="value-67726f75702d676964"></a>

## `group-gid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">group-gid</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">GroupInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Numeric group identifier string.

[Source](/reference-sources/Text.User.cinch) · line 82

<a id="value-67726f75702d6d656d62657273"></a>

## `group-members`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">group-members</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">GroupInfo</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Explicit supplementary member usernames listed in the group entry.

[Source](/reference-sources/Text.User.cinch) · line 92

<a id="value-67726f75702d6e616d65"></a>

## `group-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">group-name</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">GroupInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Group name.

[Source](/reference-sources/Text.User.cinch) · line 87

<a id="value-6c6f6f6b75702d67726f75702d69642d696e"></a>

## `lookup-group-id-in`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lookup-group-id-in</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">GroupInfo</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">GroupInfo</span>)))</code></pre>

Look up a group by exact numeric GID string within a parsed list of `GroupInfo` records.

[Source](/reference-sources/Text.User.cinch) · line 518

<a id="value-6c6f6f6b75702d67726f75702d696e"></a>

## `lookup-group-in`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lookup-group-in</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">GroupInfo</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">GroupInfo</span>)))</code></pre>

Look up a group by exact group name within a parsed list of `GroupInfo` records.

[Source](/reference-sources/Text.User.cinch) · line 510

<a id="value-6c6f6f6b75702d757365722d69642d696e"></a>

## `lookup-user-id-in`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lookup-user-id-in</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span>)))</code></pre>

Look up a user by exact numeric UID string within a parsed list of `UserInfo` records.

[Source](/reference-sources/Text.User.cinch) · line 502

<a id="value-6c6f6f6b75702d757365722d696e"></a>

## `lookup-user-in`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lookup-user-in</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span>)))</code></pre>

Look up a user by exact username within a parsed list of `UserInfo` records.

[Source](/reference-sources/Text.User.cinch) · line 494

<a id="value-70617273652d67726f7570"></a>

## `parse-group`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-group</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">GroupInfo</span>))</code></pre>

Parse colon-delimited Unix `/etc/group` content into a list of `GroupInfo` records.
Skips blank lines, `#` comments, and malformed lines with fewer than 3 fields,
strips trailing `\r`, and splits the comma-delimited member list.

[Source](/reference-sources/Text.User.cinch) · line 390

<a id="value-70617273652d706173737764"></a>

## `parse-passwd`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-passwd</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span>))</code></pre>

Parse colon-delimited Unix `/etc/passwd` content into a list of `UserInfo` records.
Skips blank lines, `#` comments, and malformed lines with fewer than 6 fields,
strips trailing `\r`, and extracts the primary GECOS name before the first comma.

[Source](/reference-sources/Text.User.cinch) · line 339

<a id="value-70617273652d70726f632d737461747573"></a>

## `parse-proc-status`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-proc-status</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">ProcIdentity</span>))</code></pre>

Parse Linux `/proc/self/status` text for `Uid:`, `Gid:`, and `Groups:` lines.
Returns `None` if either `Uid:` or `Gid:` is absent or carries no numeric tokens.

[Source](/reference-sources/Text.User.cinch) · line 442

<a id="value-70726f632d6566666563746976652d676964"></a>

## `proc-effective-gid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">proc-effective-gid</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">ProcIdentity</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Effective group ID of the process.

[Source](/reference-sources/Text.User.cinch) · line 177

<a id="value-70726f632d6566666563746976652d756964"></a>

## `proc-effective-uid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">proc-effective-uid</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">ProcIdentity</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Effective user ID of the process.

[Source](/reference-sources/Text.User.cinch) · line 167

<a id="value-70726f632d7265616c2d676964"></a>

## `proc-real-gid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">proc-real-gid</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">ProcIdentity</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Real group ID of the process.

[Source](/reference-sources/Text.User.cinch) · line 172

<a id="value-70726f632d7265616c2d756964"></a>

## `proc-real-uid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">proc-real-uid</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">ProcIdentity</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Real user ID of the process.

[Source](/reference-sources/Text.User.cinch) · line 162

<a id="value-70726f632d737570706c656d656e746172792d67696473"></a>

## `proc-supplementary-gids`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">proc-supplementary-gids</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">ProcIdentity</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Supplementary group IDs reported for the process.

[Source](/reference-sources/Text.User.cinch) · line 182

<a id="value-757365722d6572726f722d6d657373616765"></a>

## `user-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Format a `UserError` as a human-readable diagnostic message.

[Source](/reference-sources/Text.User.cinch) · line 128

<a id="value-757365722d676964"></a>

## `user-gid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-gid</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Primary group identifier string for the user.

[Source](/reference-sources/Text.User.cinch) · line 41

<a id="value-757365722d67726f75702d6964732d696e"></a>

## `user-group-ids-in`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-group-ids-in</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">GroupInfo</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Compute the deduplicated list of group IDs for a user against a parsed group table.
The list begins with the user's primary GID (when non-empty), followed by each group
GID whose explicit member list includes the user's username.

[Source](/reference-sources/Text.User.cinch) · line 536

<a id="value-757365722d686f6d652d646972"></a>

## `user-home-dir`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-home-dir</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Absolute path to the user's home directory.

[Source](/reference-sources/Text.User.cinch) · line 56

<a id="value-757365722d6e616d65"></a>

## `user-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-name</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Primary display name extracted from the GECOS field before the first comma.

[Source](/reference-sources/Text.User.cinch) · line 51

<a id="value-757365722d7368656c6c"></a>

## `user-shell`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-shell</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Configured login shell path for the user, or empty when omitted.

[Source](/reference-sources/Text.User.cinch) · line 61

<a id="value-757365722d756964"></a>

## `user-uid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-uid</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Numeric user identifier string (for example, `"1000"`).

[Source](/reference-sources/Text.User.cinch) · line 36

<a id="value-757365722d757365726e616d65"></a>

## `user-username`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-username</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Login username for the user.

[Source](/reference-sources/Text.User.cinch) · line 46

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Text.User.Eq_GroupInfo` | `Relation.Eq.Eq Text.User.GroupInfo` | Implicit |
| `Text.User.Eq_ProcIdentity` | `Relation.Eq.Eq Text.User.ProcIdentity` | Implicit |
| `Text.User.Eq_UserError` | `Relation.Eq.Eq Text.User.UserError` | Implicit |
| `Text.User.Eq_UserInfo` | `Relation.Eq.Eq Text.User.UserInfo` | Implicit |

**`Text.User.Eq_GroupInfo`**: Structural equality for group account records.
**`Text.User.Eq_ProcIdentity`**: Structural equality for process identity records.
**`Text.User.Eq_UserError`**: Structural equality for user lookup errors.
**`Text.User.Eq_UserInfo`**: Structural equality for user account records.

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/User.cinch`. Source SHA-256: `55ddbdc83336cb6af2c90e48697c4d2633e2604c6ae4a087249e85c4623064ac`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
