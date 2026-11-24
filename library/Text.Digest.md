# Text.Digest

Digest and HMAC operations over bytes. Availability of a legacy digest is not a recommendation for a security protocol.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Digest</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Digest/source/">View source</a><a href="/packages/search/?q=%2BText.Digest">Search module</a><a href="/reference-sources/Text.Digest.cinch" download="Text.Digest.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

Text.Digest: content identity and message authentication over the
runtime's digest primitives.

Module scheme (design/base-library.md): `Text.*` is conversion to and
from String/Bytes; this module is the Digest half of the Ruby surface
curated in design/stdlib-parity-ruby.md (the crypto/digest bullet:
`Digest` first, then `HMAC`), the P34 platform layer's
content-integrity family.

THE BOUNDARY: this module BINDS the runtime's content-integrity
primitives, it does not implement them. The algorithms are the
runtime's responsibility, and their vectors are pinned in
`seed/runtime/src/text.rs`'s tests and in
`seed/tests/runtime_prims.rs`; nothing here re-derives a digest in
language. The four intrinsics below are the plain `foreign import
ccall "cinch.runtime.<unit>"` machinery around the `cinch.runtime.text*`
externs, exactly as `Effect.Fs` declares its own. `sha256-bytes` is
reached through the same selector as the rest (it is declared in
`Data.Bytes`, the type's home, for the P31 content-addressing use).

THE CARRIER SPLIT (decision D88): the input is a `Bytes` (or a
`String`, encoded to UTF-8 and then digested); the digest is a fresh
`Bytes` of the algorithm's own width (`digest-length` names it), and
`digest-hex` renders that buffer as lower-case hex text, a `String`,
two characters per byte. So the types carry the whole distinction and
no duplicate spellings are needed.

IDENTITY versus AUTHENTICATION (the module's central line): the plain
digests are CONTENT IDENTITY. A `digest-bytes` result says which bytes
these are and nothing else; anyone can produce the bytes that hash to
any digest, so it is not evidence that the content is authentic, nor
that a sender knows a secret. HMAC-SHA-256 is AUTHENTICATION: given a
shared key it attests that the message came from a holder of the key
(RFC 2104), and it is the only primitive here that involves a key
(`hmac-sha256`, `hmac-sha256-hex`, separate from the selector). SHA-1
and MD5 are LEGACY IDENTITY ONLY: they exist to recognize bytes that
some other system already names, never to authenticate (both have
practical collisions), so no HMAC is offered over them and a caller
reaching for one of them in a security position is at the wrong door.

LAW SHAPE (the selectors and the renderer; the algorithms themselves
are the runtime's, per the boundary above):
  digest-string d s == digest-bytes d (string-to-bytes s)
  string-byte-length (digest-hex d b) == 2 * digest-length d
  digest-hex d b == bytes-hex (digest-bytes d b)
  digest-bytes d1 b /= digest-bytes d2 b for d1 /= d2 on the same b
and every hex character is in `0`..`9`/`a`..`f` (lower case:
`digest-hex` is the canonical rendering, and two spellings of one
digest never arise here).

Named refusals (boundaries this module deliberately does not cross):
  implementing an algorithm: the module names the runtime's output,
    and a caller who needs a digest of a stream rather than a buffer
    sizes the buffer first (there is no streaming digest in the
    bootstrap slice);
  secret hygiene beyond the buffer: the key crosses as an ordinary
    `Bytes` value and nothing here zeroes it after use;
  authentication over SHA-1 or MD5: refused by omission (no
    `hmac-sha1`/`hmac-md5`), because identity-only algorithms are not
    authentication;
  uppercase or truncated hex: `digest-hex` is lower-case and the full
    width, always.

</details>

<details class="doc-synopsis" open><summary>Synopsis (8 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Digest">Digest</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Digest</strong></code></li><li><a class="syn-name" href="#v-digest-bytes">digest-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">digest-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Digest/#t-Digest" title="Text.Digest.Digest">Digest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-digest-hex">digest-hex</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">digest-hex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Digest/#t-Digest" title="Text.Digest.Digest">Digest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-digest-length">digest-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">digest-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Digest/#t-Digest" title="Text.Digest.Digest">Digest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-digest-name">digest-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">digest-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Digest/#t-Digest" title="Text.Digest.Digest">Digest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-digest-string">digest-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">digest-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Digest/#t-Digest" title="Text.Digest.Digest">Digest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-hmac-sha256">hmac-sha256</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hmac-sha256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-hmac-sha256-hex">hmac-sha256-hex</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hmac-sha256-hex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Digest">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Digest</strong></code><span class="doc-sig-links"><a href="#t-Digest" title="Permalink">#</a><a href="/packages/stdlib/Text.Digest/source/#L87" title="View source at line 87">Source</a></span></div>
<div class="doc-body">

Which algorithm computes the digest. The selector is live: two
different values disagree on the same input (digest-bytes is the
algorithm's own output, the module does not normalize it).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-DigestSha256"><code><strong class="tok-constructor">DigestSha256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Digest/#t-Digest" title="Text.Digest.Digest">Digest</a></code></li><li id="k-DigestSha512"><code><strong class="tok-constructor">DigestSha512</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Digest/#t-Digest" title="Text.Digest.Digest">Digest</a></code></li><li id="k-DigestSha1"><code><strong class="tok-constructor">DigestSha1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Digest/#t-Digest" title="Text.Digest.Digest">Digest</a></code></li><li id="k-DigestMd5"><code><strong class="tok-constructor">DigestMd5</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Digest/#t-Digest" title="Text.Digest.Digest">Digest</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-digest-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">digest-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Digest/#t-Digest" title="Text.Digest.Digest">Digest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-digest-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Digest/source/#L104" title="View source at line 104">Source</a></span></div>
<div class="doc-body">

The digest of `b` under the named algorithm: content identity, the
algorithm's own output width (see `digest-length`).

</div>
</article>

<article class="doc-entry" id="v-digest-hex">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">digest-hex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Digest/#t-Digest" title="Text.Digest.Digest">Digest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-digest-hex" title="Permalink">#</a><a href="/packages/stdlib/Text.Digest/source/#L117" title="View source at line 117">Source</a></span></div>
<div class="doc-body">

The digest of `b` as lower-case hex, two characters per byte (2N
characters for an N-byte digest).

</div>
</article>

<article class="doc-entry" id="v-digest-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">digest-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Digest/#t-Digest" title="Text.Digest.Digest">Digest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-digest-length" title="Permalink">#</a><a href="/packages/stdlib/Text.Digest/source/#L131" title="View source at line 131">Source</a></span></div>
<div class="doc-body">

The digest's width in bytes: the length of `digest-bytes`' result
(so `2 * digest-length d` is the length of `digest-hex`'s).

</div>
</article>

<article class="doc-entry" id="v-digest-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">digest-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Digest/#t-Digest" title="Text.Digest.Digest">Digest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-digest-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Digest/source/#L122" title="View source at line 122">Source</a></span></div>
<div class="doc-body">

The algorithm's conventional name (`"sha256"` ...), the accessor a
report or filename keys on.

</div>
</article>

<article class="doc-entry" id="v-digest-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">digest-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Digest/#t-Digest" title="Text.Digest.Digest">Digest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-digest-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Digest/source/#L112" title="View source at line 112">Source</a></span></div>
<div class="doc-body">

The digest of `s`' UTF-8 encoding (the encode-then-digest law).

</div>
</article>

<article class="doc-entry" id="v-hmac-sha256">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hmac-sha256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-hmac-sha256" title="Permalink">#</a><a href="/packages/stdlib/Text.Digest/source/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

HMAC-SHA-256 (RFC 2104) over `message` under `key`: AUTHENTICATION,
distinct from the plain digests above. The key's bytes are the
secret; the result is a 32-byte buffer.

</div>
</article>

<article class="doc-entry" id="v-hmac-sha256-hex">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hmac-sha256-hex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-hmac-sha256-hex" title="Permalink">#</a><a href="/packages/stdlib/Text.Digest/source/#L146" title="View source at line 146">Source</a></span></div>
<div class="doc-body">

The HMAC-SHA-256 tag as lower-case hex (the same rendering as
`digest-hex`, 64 characters).

</div>
</article>
