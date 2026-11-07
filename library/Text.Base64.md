# Text.Base64

Base64 encoding/decoding over bytes with explicit invalid-input results.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Base64`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Base64.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

Non-embedded source closure: [Text.Base64](Text.Base64.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Base64: Base64 text to and from `Bytes` (RFC 4648).

Module scheme (design/base-library.md): `Text.*` is conversion to and
from String/Bytes; this module is the Base64 half of the Ruby surface
curated in design/stdlib-parity-ruby.md (`Base64`, the collections/
text-utilities bullet). The carrier split is decision D88's: the
encoded form is ASCII text, a `String`; the decoded form is raw bytes,
a `Bytes`.

The Ruby methods behind each name (the port's provenance):
  `base64-encode`      = `Base64.strict_encode64` (the standard
                         alphabet, no line wrapping);
  `base64-decode`      = `Base64.strict_decode64` (named errors, not
                         exceptions);
  `base64-url-encode`  = `Base64.urlsafe_encode64` (default padding);
  `base64-url-decode`  = `Base64.urlsafe_decode64`;
  `base64-decode-lenient` = `Base64.decode64` (every non-alphabet
                         character, line breaks included, is ignored
                         and the result is the bytes; it never fails).
The `Bytes`/`String` distinction is carried entirely by the
signatures (encode takes `Bytes`, yields `String`; decode the
other way), so no duplicate spellings are needed.

DESIGN LAW SHAPE (RFC 4648 section 4; the round trip is the module's
contract):
  base64-decode (base64-encode b) == Ok b         for every Bytes
  base64-decode-lenient (base64-encode b) == b    for every Bytes
  string-byte-length (base64-encode b) == 4 * ceil (bytes-len b / 3)
and the padding law: the encoding of an n-byte buffer ends in
`=` * ((3 - n rem 3) rem 3) pad characters, so lengths 0/3 mod 3
carry no pad, 2 mod 3 one pad, 1 mod 3 two. The canonical alphabet is
used for encoding; decoding accepts exactly it (plus `=` in the
padding positions).

Named refusals (the strict decoder is total over its own alphabet and
refuses everything else with a named error, never a fault):
  a character outside the alphabet (`InvalidBase64Character`): the
    standard decoder refuses `-`/`_`, the URL-safe decoder refuses
    nothing the standard one accepts (Ruby's union behaviour);
  a length that is not a multiple of 4 (`InvalidBase64Length`): the
    input can never be a complete sequence of 4-character groups;
  a misplaced or excessive pad (`InvalidBase64Padding`): `=` is legal
    only in the last group's last one or two positions, never earlier
    and never as a third pad.
Not carried (unproductions this module deliberately does not build):
  line wrapping (Ruby's `encode64`, 60 characters plus newline) and
  the MIME mode; the decoder's whitespace tolerance is the only nod to
  wrapped input, through `base64-decode-lenient`.

</details>

# Types and constructors

<a id="type-4261736536344572726f72"></a>

## `Base64Error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Base64Error</span></code></pre>

The named decode failures (the house style: Text.Parse's named
vocabulary shape, never a fault).

[Source](/reference-sources/Text.Base64.cinch) · line 68

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidBase64Character</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Base64Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidBase64Length</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Base64Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidBase64Padding</span> :: <span class="tok-constructor">Base64Error</span></code></pre>

# Values

<a id="value-6261736536342d6465636f6465"></a>

## `base64-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-decode</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Result</span> <span class="tok-constructor">Base64Error</span> <span class="tok-constructor">Bytes</span>)</code></pre>

Strict decoding.

[Source](/reference-sources/Text.Base64.cinch) · line 191

<a id="value-6261736536342d6465636f64652d6c656e69656e74"></a>

## `base64-decode-lenient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-decode-lenient</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Lenient decoding (Ruby's `decode64`): drop every non-alphabet
scalar and pack what remains, never failing.

[Source](/reference-sources/Text.Base64.cinch) · line 310

<a id="value-6261736536342d656e636f6465"></a>

## `base64-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-encode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">String</span>)</code></pre>

Encoding.

[Source](/reference-sources/Text.Base64.cinch) · line 129

<a id="value-6261736536342d75726c2d6465636f6465"></a>

## `base64-url-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-url-decode</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Result</span> <span class="tok-constructor">Base64Error</span> <span class="tok-constructor">Bytes</span>)</code></pre>

[Source](/reference-sources/Text.Base64.cinch)

<a id="value-6261736536342d75726c2d656e636f6465"></a>

## `base64-url-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-url-encode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Base64.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Base64.cinch`. Source SHA-256: `54948138dd53d893cb82067bdaba883faa404e81e0013dd750c75f723c0b409d`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
