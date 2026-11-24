# Text.Codec

Encoding/decoding signatures and codec helpers with explicit failure results.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Codec</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Codec/source/">View source</a><a href="/packages/search/?q=%2BText.Codec">Search module</a><a href="/reference-sources/Text.Codec.cinch" download="Text.Codec.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Json](Data.Json.md), [Data.Result](Data.Result.md).

<details class="doc-notes"><summary>Module notes</summary>

Text.Codec: conversion to and from `Bytes` and `Json`.

Module scheme (design/base-library.md): `Text.Codec` aligns with the
A6 `CodecGenerator.derive` vocabulary (modules specification
section 11): the generated codec pair is `encodeT :: T -> Bytes`
and `decodeT :: Bytes -> Option T`, in a fixed little-endian column
format. The class states that pair once so the generator's output
and hand-written codecs share one vocabulary.

Derived codec instances where expressible: exactly the A6 column
encoders' covered set (decision D65), `I64` as 8 bytes
little-endian (Data.Machine) and `Bool` as one byte (Data.Bool).
REAL needs float bit access and TEXT a length-prefixed copy (D65);
those instances land with their generators. The class law shape is
the round trip: `decode (encode x) == Some x` (asserted over sample
values in seed/tests/prelude_split.rs).

`JsonCodec` is the JSON-value column beside it (P31): the same law
shape over the `Json` family, with a named decode error instead of
`Option` (a JSON column carries a shape to report). Its instances
live with their types (Data.Machine's `JsonCodec I64`, Data.Bool's
`JsonCodec Bool`, and `Text.Json`'s identity column), and the
error vocabulary sits with the class, like `Text.Parse`'s
`ParseError` sits with its parsers.

</details>

<details class="doc-synopsis" open><summary>Synopsis (8 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-JsonDecodeError">JsonDecodeError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonDecodeError</strong></code></li><li><a class="syn-name" href="#c-Codec">Codec</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Codec</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Codec-encode">encode</a><code class="syn-sig"><strong class="tok-variable">encode</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-Codec" title="Text.Codec.Codec">Codec</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li class="syn-sub"><a class="syn-name" href="#m-Codec-decode">decode</a><code class="syn-sig"><strong class="tok-variable">decode</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-Codec" title="Text.Codec.Codec">Codec</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#c-JsonCodec">JsonCodec</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">JsonCodec</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-JsonCodec-encode-json">encode-json</a><code class="syn-sig"><strong class="tok-variable">encode-json</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-JsonCodec" title="Text.Codec.JsonCodec">JsonCodec</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li class="syn-sub"><a class="syn-name" href="#m-JsonCodec-decode-json">decode-json</a><code class="syn-sig"><strong class="tok-variable">decode-json</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-JsonCodec" title="Text.Codec.JsonCodec">JsonCodec</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#t-JsonDecodeError" title="Text.Codec.JsonDecodeError">JsonDecodeError</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-error-message">json-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#t-JsonDecodeError" title="Text.Codec.JsonDecodeError">JsonDecodeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-JsonDecodeError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonDecodeError</strong></code><span class="doc-sig-links"><a href="#t-JsonDecodeError" title="Permalink">#</a><a href="/packages/stdlib/Text.Codec/source/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">

The named decode errors: a wrong shape (the expected column kind)
and a value outside the column's range.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-TypeMismatch"><code><strong class="tok-constructor">TypeMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#t-JsonDecodeError" title="Text.Codec.JsonDecodeError">JsonDecodeError</a></code></li><li id="k-OutOfRange"><code><strong class="tok-constructor">OutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#t-JsonDecodeError" title="Text.Codec.JsonDecodeError">JsonDecodeError</a></code></li></ul></div>
</div>
</article>

# Signatures

<article class="doc-entry" id="c-Codec">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Codec</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Codec" title="Permalink">#</a><a href="/packages/stdlib/Text.Codec/source/#L34" title="View source at line 34">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Codec-encode"><div class="doc-method-row"><code><strong class="tok-variable">encode</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-Codec" title="Text.Codec.Codec">Codec</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#m-Codec-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.Codec/source/#L35">Source</a></span></div></li><li id="m-Codec-decode"><div class="doc-method-row"><code><strong class="tok-variable">decode</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-Codec" title="Text.Codec.Codec">Codec</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Codec-decode" title="Permalink">#</a><a href="/packages/stdlib/Text.Codec/source/#L36">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-Codec" title="Text.Codec.Codec">Codec</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Bool/#instances">Data.Bool</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-Codec" title="Text.Codec.Codec">Codec</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Machine/#instances">Data.Machine</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-JsonCodec">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">JsonCodec</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-JsonCodec" title="Permalink">#</a><a href="/packages/stdlib/Text.Codec/source/#L49" title="View source at line 49">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-JsonCodec-encode-json"><div class="doc-method-row"><code><strong class="tok-variable">encode-json</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-JsonCodec" title="Text.Codec.JsonCodec">JsonCodec</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#m-JsonCodec-encode-json" title="Permalink">#</a><a href="/packages/stdlib/Text.Codec/source/#L50">Source</a></span></div></li><li id="m-JsonCodec-decode-json"><div class="doc-method-row"><code><strong class="tok-variable">decode-json</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-JsonCodec" title="Text.Codec.JsonCodec">JsonCodec</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#t-JsonDecodeError" title="Text.Codec.JsonDecodeError">JsonDecodeError</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-JsonCodec-decode-json" title="Permalink">#</a><a href="/packages/stdlib/Text.Codec/source/#L51">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (3)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-JsonCodec" title="Text.Codec.JsonCodec">JsonCodec</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Bool/#instances">Data.Bool</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-JsonCodec" title="Text.Codec.JsonCodec">JsonCodec</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#c-JsonCodec" title="Text.Codec.JsonCodec">JsonCodec</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Text.Json/#instances">Text.Json</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-json-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Codec/#t-JsonDecodeError" title="Text.Codec.JsonDecodeError">JsonDecodeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Codec/source/#L44" title="View source at line 44">Source</a></span></div>
</article>
