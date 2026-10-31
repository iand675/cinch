# Data.Bytes

Mutable byte buffers with length, construction, copy, and bounds/range-aware access operations.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Bytes`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Bytes.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Hash](Data.Hash.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Bytes: boxed byte buffers.

`Bytes` is a core runtime type: indices and lengths are `Nat`, and a
byte is an `InRange 0 255 Nat`. Every operation is total by
construction (design/no-faults.md): a read or write takes evidence
that its index is below `bytes-len b`, a range copy evidence that both
ranges end inside their buffers, a read's result carries its range and
a write's value must prove it. A buffer's length is fixed when it is
made, so a fact over `bytes-len b` survives writes to `b`, and
`bytes-new n` gives `bytes-len out == n` to the name it is bound to.
A length no allocation can hold is the heap `Exhausted` exit.

Module scheme (design/base-library.md): the type's home carries its
instance family: `FromList` (`[1, 2, 3]` at `Bytes` fills a buffer)
over `U8` elements, so a literal's numerals are range-checked as
bytes at compile time, and the key pair `Eq`/`Hash` (a byte slice is
a plausible key: a compiler's names arrive as the lexer's bytes).

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-62797465732d636f6d70617265"></a>

## `bytes-compare`

```cinch
bytes-compare :: (Bytes -> (Bytes -> I64))
```

The lexicographic order of two buffers' contents as the raw -1/0/1
immediate (the `nat-cmp` convention), read in place: no copy and
no allocation.

[Source](/reference-sources/Data.Bytes.cinch) · line 53

<a id="value-62797465732d636f70792d72616e6765"></a>

## `bytes-copy-range`

```cinch
bytes-copy-range :: using ((arg2 + arg4) <= (Data.Bytes.bytes-len arg0), (arg3 + arg4) <= (Data.Bytes.bytes-len arg1)) => ((arg0 :: Bytes) -> ((arg1 :: Bytes) -> ((arg2 :: Nat) -> ((arg3 :: Nat) -> ((arg4 :: Nat) -> Unit)))))
```

`bytes-copy-range dst src di si n`: the `n` bytes of `src` from `si`
into `dst` at `di`, as one runtime `memmove` (the two may be the
same buffer and overlap). A copy loop written here would box a `Nat`
for every index step, so every bulk copy goes through this.

[Source](/reference-sources/Data.Bytes.cinch) · line 48

<a id="value-62797465732d66726f6d2d6c697374"></a>

## `bytes-from-list`

```cinch
bytes-from-list :: (List (InRange 0 255 Nat) -> Bytes)
```

The list's bytes, in order. A byte is an `InRange 0 255 Nat`, so a
list of numerals decides each element at compile time and a
computed list carries its elements' ranges.

[Source](/reference-sources/Data.Bytes.cinch) · line 64

<a id="value-62797465732d66726f6d2d753873"></a>

## `bytes-from-u8s`

```cinch
bytes-from-u8s :: (List U8 -> Bytes)
```

The `FromList` buffer: `bytes-from-list`'s shape over `U8`.

[Source](/reference-sources/Data.Bytes.cinch) · line 89

<a id="value-62797465732d676574"></a>

## `bytes-get`

```cinch
bytes-get :: using (arg1 < (Data.Bytes.bytes-len arg0)) => ((arg0 :: Bytes) -> ((arg1 :: Nat) -> InRange 0 255 Nat))
```

[Source](/reference-sources/Data.Bytes.cinch) · line 41

<a id="value-62797465732d6c656e"></a>

## `bytes-len`

```cinch
bytes-len :: (Bytes -> InRange 0 9223372036854775807 Nat)
```

`nat-cmp`/`int-cmp` return the raw -1/0/1 immediate at `I64`, so
equality is `eq-i64 (int-cmp a b) 0`.

`Bytes`: boxed byte buffers. `bytes-len` comes first: the other
signatures name it. A length is an `i64` in the runtime, so it is at
most `2^63 - 1` and every index below it converts to `I64`. The
index obligation is `nat-sub`'s shape: a loop's own guard (`if i <
bytes-len b`), a fresh buffer's length or an enclosing `using (i <=
bytes-len b)` supplies it. A string literal's read-only backing is
never one of these buffers: it lives inside a `String`, and every
conversion out of a `String` copies.

[Source](/reference-sources/Data.Bytes.cinch) · line 39

<a id="value-62797465732d6e6577"></a>

## `bytes-new`

```cinch
bytes-new :: ((a0 :: Nat) -> refined ((Data.Bytes.bytes-len out) == a0) Bytes)
```

[Source](/reference-sources/Data.Bytes.cinch) · line 40

<a id="value-62797465732d736574"></a>

## `bytes-set`

```cinch
bytes-set :: using (arg1 < (Data.Bytes.bytes-len arg0)) => ((arg0 :: Bytes) -> ((arg1 :: Nat) -> ((arg2 :: InRange 0 255 Nat) -> Unit)))
```

[Source](/reference-sources/Data.Bytes.cinch) · line 42

<a id="value-7368613235362d6279746573"></a>

## `sha256-bytes`

```cinch
sha256-bytes :: (Bytes -> Bytes)
```

SHA-256 (FIPS 180-4) at content-addressing strength over a
buffer (the P31 content identity; `compiler/runtime/src/text.rs`,
the `cinch.runtime.textSha256` intrinsic, nm-verified there): the
digest as a fresh 32-byte buffer.

[Source](/reference-sources/Data.Bytes.cinch) · line 59

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Bytes.Eq_Bytes` | `Eq Bytes` | Implicit |
| `Data.Bytes.FromList_Bytes` | `FromList Bytes U8` | Implicit |
| `Data.Bytes.Hash_Bytes` | `Hash Bytes` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Bytes.cinch`. Source SHA-256: `47f2c8ee650238fae49e2f10f60f84d4d7b1fce1ab2fc8b5588dcd85c50cfffe`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
