# Effect.Io

A worked configuration-reading API over real filesystem bytes; parse-config treats a nonempty payload as configuration.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Effect.Io`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Io.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Path](Data.Path.md), [Data.Result](Data.Result.md), [Effect.Fs](Effect.Fs.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Io: the `Io r` worked vocabulary.

The worked vocabulary of the sequencing chapter's `loadConfig`
sketch (runtime capabilities, effects and sequencing
specification), provided over the filesystem like Effect.Fs's
operations. A `Path` (Data.Path, the platform layer's real path
type: this module's placeholder `Path :: Nat -> Path` over the
runtime's input source is replaced by it) names bytes to read;
`read-config-bytes` performs the read under the `Io r` capability
and reports a missing or unreadable file as a `ConfigError`.
`parse-config` is pure over the read bytes: the bootstrap's config
document is the payload, and an empty document is not a config.

</details>

# Types and constructors

<a id="type-436f6e666967"></a>

## `Config`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Config</span></code></pre>

[Source](/reference-sources/Effect.Io.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Config</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Config</span>)</code></pre>

<a id="type-436f6e6669674572726f72"></a>

## `ConfigError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ConfigError</span></code></pre>

The worked vocabulary of the sequencing chapter's `loadConfig`
sketch (runtime capabilities, effects and sequencing
specification), provided over the filesystem like Effect.Fs's
operations. A `Path` (Data.Path) names bytes to read;
`read-config-bytes` performs the read under the `Io r`
capability and reports a missing or unreadable file as a
`ConfigError`. `parse-config` is pure over the read bytes: the
bootstrap's config document is the payload, and an empty
document is not a config.

[Source](/reference-sources/Effect.Io.cinch) · line 30

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BadConfig</span> :: <span class="tok-constructor">ConfigError</span></code></pre>

<a id="type-50617468"></a>

## `Path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Path</span></code></pre>

The path text (POSIX; UTF-8 at the boundary).

[Source](/reference-sources/Data.Path.cinch) · line 48

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Path</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Path</span>)</code></pre>

# Values

<a id="value-70617273652d636f6e666967"></a>

## `parse-config`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-config</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Result</span> <span class="tok-constructor">ConfigError</span> <span class="tok-constructor">Config</span>)</code></pre>

[Source](/reference-sources/Effect.Io.cinch)

<a id="value-726561642d636f6e6669672d6279746573"></a>

## `read-config-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">read-config-bytes</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Path</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Result</span> <span class="tok-constructor">ConfigError</span> <span class="tok-constructor">Bytes</span>)</code></pre>

[Source](/reference-sources/Effect.Io.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Io.cinch`. Source SHA-256: `82b9a0b0d648194592d1aba2d9f200eba228b97b0b310e4646843499fd755804`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
