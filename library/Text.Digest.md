# Text.Digest

Digest and HMAC operations over bytes. Availability of a legacy digest is not a recommendation for a security protocol.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Digest`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Digest.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Text.Digest](Text.Digest.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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
`compiler/runtime/src/text.rs`'s tests and in
`compiler/tests/runtime_prims.rs`; nothing here re-derives a digest in
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

# Types and constructors

<a id="type-446967657374"></a>

## `Digest`

```cinch
data Digest
```

Which algorithm computes the digest. The selector is live: two
different values disagree on the same input (digest-bytes is the
algorithm's own output, the module does not normalize it).

[Source](/reference-sources/Text.Digest.cinch) · line 87

```cinch
DigestSha256 :: Digest
```

```cinch
DigestSha512 :: Digest
```

```cinch
DigestSha1 :: Digest
```

```cinch
DigestMd5 :: Digest
```

# Values

<a id="value-6469676573742d6279746573"></a>

## `digest-bytes`

```cinch
digest-bytes :: (Digest -> (Bytes -> Bytes))
```

The digest of `b` under the named algorithm: content identity, the
algorithm's own output width (see `digest-length`).

[Source](/reference-sources/Text.Digest.cinch) · line 104

<a id="value-6469676573742d686578"></a>

## `digest-hex`

```cinch
digest-hex :: (Digest -> (Bytes -> String))
```

The digest of `b` as lower-case hex, two characters per byte (2N
characters for an N-byte digest).

[Source](/reference-sources/Text.Digest.cinch) · line 117

<a id="value-6469676573742d6c656e677468"></a>

## `digest-length`

```cinch
digest-length :: (Digest -> Nat)
```

The digest's width in bytes: the length of `digest-bytes`' result
(so `2 * digest-length d` is the length of `digest-hex`'s).

[Source](/reference-sources/Text.Digest.cinch) · line 131

<a id="value-6469676573742d6e616d65"></a>

## `digest-name`

```cinch
digest-name :: (Digest -> String)
```

The algorithm's conventional name (`"sha256"` ...), the accessor a
report or filename keys on.

[Source](/reference-sources/Text.Digest.cinch) · line 122

<a id="value-6469676573742d737472696e67"></a>

## `digest-string`

```cinch
digest-string :: (Digest -> (String -> Bytes))
```

The digest of `s`' UTF-8 encoding (the encode-then-digest law).

[Source](/reference-sources/Text.Digest.cinch) · line 112

<a id="value-686d61632d736861323536"></a>

## `hmac-sha256`

```cinch
hmac-sha256 :: (Bytes -> (Bytes -> Bytes))
```

HMAC-SHA-256 (RFC 2104) over `message` under `key`: AUTHENTICATION,
distinct from the plain digests above. The key's bytes are the
secret; the result is a 32-byte buffer.

[Source](/reference-sources/Text.Digest.cinch) · line 141

<a id="value-686d61632d7368613235362d686578"></a>

## `hmac-sha256-hex`

```cinch
hmac-sha256-hex :: (Bytes -> (Bytes -> String))
```

The HMAC-SHA-256 tag as lower-case hex (the same rendering as
`digest-hex`, 64 characters).

[Source](/reference-sources/Text.Digest.cinch) · line 146

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Digest.cinch`. Source SHA-256: `91f1b7630e30fefea87a0e50b97bc3d42d58e69c73b7d60c4c175d07513588d2`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
