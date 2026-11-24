# Effect.User

Operating-system user and group lookups (current user, by name/UID/GID, supplementary group IDs) over Prim.Sys and Effect.Process.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.User`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.User.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Path](Data.Path.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Fs](Effect.Fs.md), [Effect.Process](Effect.Process.md), [Prim.Sys](Prim.Sys.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.User](Text.User.md).

Non-embedded source closure: [Effect.User](Effect.User.md), [Text.User](Text.User.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effectful Unix user and group account lookups backed by system account files
and process identity records.
On Linux, current-user resolution reads `/proc/self/status` via streaming `Prim.Sys`
reads (avoiding `lseek` on procfs pseudo-files) and resolves the process UID against
`/etc/passwd`, falling back to `/proc/self/environ` synthesis when the UID is absent.
On platforms without `/proc/self/status`, current-user resolution falls back to
`Effect.Process` (`id` and `printenv`) paired with `/etc/passwd`.
Explicit-path variants allow hermetic testing against custom account files.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-757365722d63757272656e74"></a>

## `user-current`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-current</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span>))</code></pre>

Return the current process's user account information.
On Linux, reads the process UID from `/proc/self/status` and looks it up in `/etc/passwd`,
falling back to `/proc/self/environ` synthesis when the UID is absent from `/etc/passwd`.
On platforms without `/proc/self/status`, falls back to `id` / `printenv` and `/etc/passwd`.

[Source](/reference-sources/Effect.User.cinch) · line 457

<a id="value-757365722d67726f75702d696473"></a>

## `user-group-ids`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-group-ids</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Return all deduplicated group IDs to which `info` belongs.
Reads `/etc/group` and, when `info` matches the current process's UID, merges
active supplementary group IDs reported by `/proc/self/status`.

[Source](/reference-sources/Effect.User.cinch) · line 493

<a id="value-757365722d67726f75702d6964732d696e2d66696c65"></a>

## `user-group-ids-in-file`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-group-ids-in-file</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Path</span>.<span class="tok-constructor">Path</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))))</code></pre>

Return deduplicated group IDs for `info` computed strictly from an explicit group-format file path.

[Source](/reference-sources/Effect.User.cinch) · line 541

<a id="value-757365722d6c6f6f6b7570"></a>

## `user-lookup`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-lookup</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span>))</code></pre>

Look up a user account by username in `/etc/passwd`.

[Source](/reference-sources/Effect.User.cinch) · line 475

<a id="value-757365722d6c6f6f6b75702d67726f7570"></a>

## `user-lookup-group`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-lookup-group</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">GroupInfo</span>))</code></pre>

Look up a group by group name in `/etc/group`.

[Source](/reference-sources/Effect.User.cinch) · line 483

<a id="value-757365722d6c6f6f6b75702d67726f75702d6964"></a>

## `user-lookup-group-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-lookup-group-id</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">GroupInfo</span>))</code></pre>

Look up a group by numeric GID string in `/etc/group`.

[Source](/reference-sources/Effect.User.cinch) · line 487

<a id="value-757365722d6c6f6f6b75702d67726f75702d69642d696e2d66696c65"></a>

## `user-lookup-group-id-in-file`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-lookup-group-id-in-file</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Path</span>.<span class="tok-constructor">Path</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">GroupInfo</span>)))</code></pre>

Look up a group by numeric GID string in an explicit group-format file path.

[Source](/reference-sources/Effect.User.cinch) · line 533

<a id="value-757365722d6c6f6f6b75702d67726f75702d696e2d66696c65"></a>

## `user-lookup-group-in-file`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-lookup-group-in-file</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Path</span>.<span class="tok-constructor">Path</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">GroupInfo</span>)))</code></pre>

Look up a group by group name in an explicit group-format file path.

[Source](/reference-sources/Effect.User.cinch) · line 525

<a id="value-757365722d6c6f6f6b75702d6964"></a>

## `user-lookup-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-lookup-id</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span>))</code></pre>

Look up a user account by numeric UID string in `/etc/passwd`.

[Source](/reference-sources/Effect.User.cinch) · line 479

<a id="value-757365722d6c6f6f6b75702d69642d696e2d66696c65"></a>

## `user-lookup-id-in-file`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-lookup-id-in-file</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Path</span>.<span class="tok-constructor">Path</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span>)))</code></pre>

Look up a user by numeric UID string in an explicit passwd-format file path.

[Source](/reference-sources/Effect.User.cinch) · line 517

<a id="value-757365722d6c6f6f6b75702d696e2d66696c65"></a>

## `user-lookup-in-file`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">user-lookup-in-file</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Path</span>.<span class="tok-constructor">Path</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">User</span>.<span class="tok-constructor">UserInfo</span>)))</code></pre>

Look up a user by username in an explicit passwd-format file path.

[Source](/reference-sources/Effect.User.cinch) · line 509

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/User.cinch`. Source SHA-256: `0bb8f25ae6d322c61eabe1fc404ddf8bbc52940b6f38bfd1d961aa78a8472243`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
