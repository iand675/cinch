# Text.FilePath

POSIX lexical relative paths, locality tests, filesystem-name conversion and search-list splitting.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.FilePath`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.FilePath.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Semigroup](Algebra.Semigroup.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Text.Path](Text.Path.md).

Non-embedded source closure: [Text.FilePath](Text.FilePath.md), [Text.Path](Text.Path.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Lexical POSIX filesystem paths over UTF-8 text. Relative path calculations
and locality checks use component boundaries and never resolve symbolic links.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-66696c652d706174682d69732d6c6f63616c"></a>

## `file-path-is-local`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">file-path-is-local</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

A local path is nonempty, relative, and stays beneath its starting directory after lexical cleaning.

[Source](/reference-sources/Text.FilePath.cinch) · line 32

<a id="value-66696c652d706174682d6c6f63616c697a65"></a>

## `file-path-localize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">file-path-localize</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Convert a slash-separated filesystem name to POSIX text. The root name is dot.
Every other name requires nonempty elements, no dot or parent elements, and no NUL.

[Source](/reference-sources/Text.FilePath.cinch) · line 40

<a id="value-66696c652d706174682d72656c6174697665"></a>

## `file-path-relative`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">file-path-relative</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Compute a lexical route from base to destination. Different roots and unmatched
parent components in the base require knowledge of the working directory.

[Source](/reference-sources/Text.FilePath.cinch) · line 57

<a id="value-66696c652d706174682d73706c69742d6c697374"></a>

## `file-path-split-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">file-path-split-list</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

An empty POSIX search list has no entries. Colons in nonempty lists preserve empty entries.

[Source](/reference-sources/Text.FilePath.cinch) · line 22

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/FilePath.cinch`. Source SHA-256: `e465f4641efc3618645ed4ff1bfa12c6a1a91c65de4579a2cbc84462642c44a7`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
