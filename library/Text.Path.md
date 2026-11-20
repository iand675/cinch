# Text.Path

Portable slash-path lexical operations and checked shell patterns with reusable compiled matchers and exact SIMD scans.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Path`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Path.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

Non-embedded source closure: [Text.Path](Text.Path.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Portable lexical operations and shell patterns for slash-separated UTF-8 paths.
Only / separates components; backslashes, drive letters and NUL are ordinary
path text. Cleaning resolves dots lexically without touching the filesystem.
Matching supports *, ?, escaped literals and nonempty Unicode range classes.
Stars never consume /; classes can explicitly match it. Patterns are checked
completely before matching, including patterns whose first term fails.
Complete scans use inline LLVM SIMD lanes with retained owners and exact tails.

</details>

# Types and constructors

<a id="type-506174685061747465726e"></a>

## `PathPattern`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PathPattern</span></code></pre>

An immutable, fully checked shell pattern reusable across independent path names.

[Source](/reference-sources/Text.Path.cinch) · line 354

<a id="type-506174685061747465726e4572726f72"></a>

## `PathPatternError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PathPatternError</span></code></pre>

A shell pattern contains an incomplete escape or malformed Unicode range class.

[Source](/reference-sources/Text.Path.cinch) · line 334

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidPathPattern</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Path</span>.<span class="tok-constructor">PathPatternError</span></code></pre>

# Values

<a id="value-706174682d62617365"></a>

## `path-base`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">path-base</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the final nonempty component, / for slash-only paths, and . for empty input.

[Source](/reference-sources/Text.Path.cinch) · line 226

<a id="value-706174682d636c65616e"></a>

## `path-clean`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">path-clean</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Lexically collapse separators and dot components; an empty result becomes .
Rooted parent components stop at /; leading relative parent components are retained.

[Source](/reference-sources/Text.Path.cinch) · line 255

<a id="value-706174682d646972"></a>

## `path-dir`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">path-dir</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the cleaned directory portion, using . when no directory is present.

[Source](/reference-sources/Text.Path.cinch) · line 239

<a id="value-706174682d657874"></a>

## `path-ext`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">path-ext</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the final dot and following bytes of the last component, or empty text.
Leading dots count as extensions; a path ending in / has no extension.

[Source](/reference-sources/Text.Path.cinch) · line 250

<a id="value-706174682d69732d6162736f6c757465"></a>

## `path-is-absolute`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">path-is-absolute</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a path starts with /; no normalization or filesystem lookup is performed.

[Source](/reference-sources/Text.Path.cinch) · line 222

<a id="value-706174682d6a6f696e"></a>

## `path-join`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">path-join</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Join nonempty path elements with / and clean the result; all-empty input returns empty text.
Later absolute-looking elements do not discard preceding elements.

[Source](/reference-sources/Text.Path.cinch) · line 327

<a id="value-706174682d6d61746368"></a>

## `path-match`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">path-match</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Path</span>.<span class="tok-constructor">PathPatternError</span> <span class="tok-constructor">Bool</span>)))</code></pre>

Check and match a shell pattern against the entire UTF-8 path name.

[Source](/reference-sources/Text.Path.cinch) · line 514

<a id="value-706174682d7061747465726e"></a>

## `path-pattern`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">path-pattern</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Path</span>.<span class="tok-constructor">PathPatternError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Path</span>.<span class="tok-constructor">PathPattern</span>))</code></pre>

Parse a complete pattern, rejecting malformed suffixes even when an earlier term cannot match.

[Source](/reference-sources/Text.Path.cinch) · line 366

<a id="value-706174682d7061747465726e2d6572726f722d6d657373616765"></a>

## `path-pattern-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">path-pattern-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Path</span>.<span class="tok-constructor">PathPatternError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render the stable malformed-pattern diagnostic.

[Source](/reference-sources/Text.Path.cinch) · line 339

<a id="value-706174682d7061747465726e2d6d61746368"></a>

## `path-pattern-match`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">path-pattern-match</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Path</span>.<span class="tok-constructor">PathPattern</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Match a reusable checked pattern against an entire path name.
Pattern state and path text remain immutable across repeated calls.

[Source](/reference-sources/Text.Path.cinch) · line 510

<a id="value-706174682d73706c6974"></a>

## `path-split`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">path-split</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Split immediately after the final slash, preserving repeated and trailing separators.

[Source](/reference-sources/Text.Path.cinch) · line 233

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Path.cinch`. Source SHA-256: `8a9f7c29198f038a541bd510a68aee1502f1d8c3d3693500406fe24aba86cbad`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
