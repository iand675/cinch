# Effect.Io

A worked configuration-reading API over real filesystem bytes; parse-config treats a nonempty payload as configuration.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Io</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Io/source/">View source</a><a href="/packages/search/?q=%2BEffect.Io">Search module</a><a href="/reference-sources/Effect.Io.cinch" download="Effect.Io.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Path](Data.Path.md), [Data.Result](Data.Result.md), [Effect.Fs](Effect.Fs.md).

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis" open><summary>Synopsis (5 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Config">Config</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Config</strong></code></li><li><a class="syn-name" href="#t-ConfigError">ConfigError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ConfigError</strong></code></li><li><a class="syn-name" href="#t-Path">Path</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Path</strong></code></li><li><a class="syn-name" href="#v-parse-config">parse-config</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-config</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Io/#t-ConfigError" title="Effect.Io.ConfigError">ConfigError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Io/#t-Config" title="Effect.Io.Config">Config</a></code></li><li><a class="syn-name" href="#v-read-config-bytes">read-config-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">read-config-bytes</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Io/#t-ConfigError" title="Effect.Io.ConfigError">ConfigError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Config">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Config</strong></code><span class="doc-sig-links"><a href="#t-Config" title="Permalink">#</a><a href="/packages/stdlib/Effect.Io/source/#L34" title="View source at line 34">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Config"><code><strong class="tok-constructor">Config</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Io/#t-Config" title="Effect.Io.Config">Config</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ConfigError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ConfigError</strong></code><span class="doc-sig-links"><a href="#t-ConfigError" title="Permalink">#</a><a href="/packages/stdlib/Effect.Io/source/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

The worked vocabulary of the sequencing chapter's `loadConfig`
sketch (runtime capabilities, effects and sequencing
specification), provided over the filesystem like Effect.Fs's
operations. A `Path` (Data.Path) names bytes to read;
`read-config-bytes` performs the read under the `Io r`
capability and reports a missing or unreadable file as a
`ConfigError`. `parse-config` is pure over the read bytes: the
bootstrap's config document is the payload, and an empty
document is not a config.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-BadConfig"><code><strong class="tok-constructor">BadConfig</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Io/#t-ConfigError" title="Effect.Io.ConfigError">ConfigError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Path">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Path</strong></code><span class="doc-sig-links"><a href="#t-Path" title="Permalink">#</a><a href="/packages/stdlib/Data.Path/source/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

The path text (POSIX; UTF-8 at the boundary).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Path"><code><strong class="tok-constructor">Path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-parse-config">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-config</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Io/#t-ConfigError" title="Effect.Io.ConfigError">ConfigError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Io/#t-Config" title="Effect.Io.Config">Config</a></code><span class="doc-sig-links"><a href="#v-parse-config" title="Permalink">#</a><a href="/packages/stdlib/Effect.Io/source/#L41" title="View source at line 41">Source</a></span></div>
</article>

<article class="doc-entry" id="v-read-config-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">read-config-bytes</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Io/#t-ConfigError" title="Effect.Io.ConfigError">ConfigError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-read-config-bytes" title="Permalink">#</a><a href="/packages/stdlib/Effect.Io/source/#L36" title="View source at line 36">Source</a></span></div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Io.cinch`](/packages/stdlib/Effect.Io/source/). Source SHA-256: `82b9a0b0d648194592d1aba2d9f200eba228b97b0b310e4646843499fd755804`.
