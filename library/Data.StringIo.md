# Data.StringIo

In-memory string-oriented I/O utilities with explicit state and error results.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.StringIo`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.StringIo.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Result](Data.Result.md).

Non-embedded source closure: [Data.StringIo](Data.StringIo.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.StringIo: Ruby's StringIO as an in-memory byte buffer with a
read/write cursor.

Module scheme (design/base-library.md): `Data.*` is the home of a
concrete type, and this is a concrete in-memory buffer over
`Data.Bytes`; the ported Ruby item is `StringIO`, the
collections/text-utilities bullet of design/stdlib-parity-ruby.md.
The shape is exactly the byte-level one Ruby's `StringIO` has: one
buffer plus a cursor, where `write` overwrites at the cursor and
extends the buffer, `read` consumes up to n bytes and advances,
`read-line` stops after the next LF, and the buffer is observable
(`len`, `pos`, `eof`) and its position movable (`rewind`, `seek`).
The carrier split is decision D88's: `StringIo` is bytes, and
`Data.String` owns the UTF-8 interpretation.

DESIGN LAW SHAPE (the value contract, and the one deviation from
Ruby): a `StringIo` is a VALUE, not Ruby's mutable object. Every
operation returns a handle and leaves the one it was given
unchanged, so a retained handle keeps reading what it read before:
  string-io-to-bytes (string-io-from-bytes b) == b   for every Bytes
  string-io-read (string-io-write io bs) (bytes-len bs)
    == (bs, cursor just past bs)
  string-io-rewind io reads again what io read
and the cursor invariant `0 <= pos <= length` holds by construction,
which is what every guarded subtraction below relies on (`Nat`
subtraction below zero is a fault, exit 71). The buffer is never
written after it is built, so two handles that share one (from
`rewind`, `seek`, `read` or a no-op `truncate`) are
indistinguishable.
A write never mutates in place: a `Bytes` handle stays reachable
through every binding that names it (binders default to `%many`), so
an in-place write would show through a retained handle. The value
contract reallocates the grown buffer instead, which costs
O(length + written) per write; a build-once/read-many use pays it
once.

Named refusals (boundaries this module deliberately does not cross):
  the `IO` inheritance: no file descriptor, no `open`/`close`, no
    blocking read, no `StringIO.for_fd`; this is a buffer, not a
    handle to a stream;
  encoding handling: the buffer is bytes. Ruby's `StringIO` carries
    an external/internal encoding pair and transcodes as it reads
    and writes; here the bytes are opaque, and the `String`
    conversion belongs to `Data.String`;
  `each_line` with a custom separator: `string-io-read-line` splits
    on LF only (Ruby's default record separator); the separator
    argument is not here, and a caller wanting another one reads and
    scans for it;
  seeking past the end: `string-io-seek` refuses a target past the
    buffer length with `SeekOutOfRange` rather than padding (Ruby
    allows the seek and the read then returns nothing); a negative
    target cannot be written down at all, because the cursor is a
    `Nat`;
  the `?` suffix: Ruby's `eof?` predicate is `string-io-eof` here.
    An identifier's characters are letters, digits, `_` and `'` (with
    a dash gluing two words), and `?` is an operator character, so no
    name in the language carries a question mark; Data.List's `null`
    is the standing precedent for a Bool-valued name without one.

</details>

# Types and constructors

<a id="type-537472696e67496f"></a>

## `StringIo`

```cinch
data StringIo
```

The buffer and the cursor. The representation is module-private: the
constructor is deliberately NOT the type's name (`IoBuf`, not
`StringIo`, the shape of Data.String's `StrBuf`), because a name
that is both a type and its constructor is exported as one item and
the export would hand out the representation. Every handle comes
from `string-io-new` or `string-io-from-bytes`, so the cursor
invariant (0 <= pos <= bytes-len buffer) holds by construction and
a caller cannot put the cursor outside it.

[Source](/reference-sources/Data.StringIo.cinch) · line 79

<a id="type-537472696e67496f4572726f72"></a>

## `StringIoError`

```cinch
data StringIoError
```

The named seek refusal: the requested target and the length it
exceeds.

[Source](/reference-sources/Data.StringIo.cinch) · line 84

```cinch
SeekOutOfRange :: (Nat -> (Nat -> StringIoError))
```

# Values

<a id="value-737472696e672d696f2d656f66"></a>

## `string-io-eof`

```cinch
string-io-eof :: (StringIo -> Bool)
```

At or past the last byte: a read returns nothing and the cursor
does not move.

[Source](/reference-sources/Data.StringIo.cinch) · line 132

<a id="value-737472696e672d696f2d66726f6d2d6279746573"></a>

## `string-io-from-bytes`

```cinch
string-io-from-bytes :: (Bytes -> StringIo)
```

The conversion in copies (the wrap/copy discipline Data.String
states for its backing): the caller keeps its mutable buffer and a
later `bytes-set` on it never shows through the handle. The cursor
starts at the beginning.

[Source](/reference-sources/Data.StringIo.cinch) · line 110

<a id="value-737472696e672d696f2d6c656e"></a>

## `string-io-len`

```cinch
string-io-len :: (StringIo -> Nat)
```

[Source](/reference-sources/Data.StringIo.cinch) · line 124

<a id="value-737472696e672d696f2d6e6577"></a>

## `string-io-new`

```cinch
string-io-new :: StringIo
```

[Source](/reference-sources/Data.StringIo.cinch) · line 103

<a id="value-737472696e672d696f2d706f73"></a>

## `string-io-pos`

```cinch
string-io-pos :: (StringIo -> Nat)
```

[Source](/reference-sources/Data.StringIo.cinch) · line 127

<a id="value-737472696e672d696f2d72656164"></a>

## `string-io-read`

```cinch
string-io-read :: (StringIo -> (Nat -> (Bytes, StringIo)))
```

Read up to n bytes from the cursor, advancing by the number
consumed. At or past the end the result is the empty buffer and the
cursor does not move (`string-io-eof` is how a caller sees it;
Ruby's nil is not spelled here).

[Source](/reference-sources/Data.StringIo.cinch) · line 162

<a id="value-737472696e672d696f2d726561642d6c696e65"></a>

## `string-io-read-line`

```cinch
string-io-read-line :: (StringIo -> (Bytes, StringIo))
```

Read through the next LF (0x0A), inclusive; the last line without a
terminator yields the rest of the buffer.

[Source](/reference-sources/Data.StringIo.cinch) · line 167

<a id="value-737472696e672d696f2d726577696e64"></a>

## `string-io-rewind`

```cinch
string-io-rewind :: (StringIo -> StringIo)
```

The cursor to the start: what was read is read again.

[Source](/reference-sources/Data.StringIo.cinch) · line 198

<a id="value-737472696e672d696f2d7365656b"></a>

## `string-io-seek`

```cinch
string-io-seek :: (StringIo -> (Nat -> Result StringIoError StringIo))
```

The cursor to an absolute byte offset. A target past the buffer
length is the named refusal: this module never pads, so the cursor
would leave the invariant 0 <= pos <= length that the rest of the
module relies on. A negative target cannot be written down (the
cursor is a `Nat`).

[Source](/reference-sources/Data.StringIo.cinch) · line 206

<a id="value-737472696e672d696f2d746f2d6279746573"></a>

## `string-io-to-bytes`

```cinch
string-io-to-bytes :: (StringIo -> Bytes)
```

The conversion out copies too, for the same reason.

[Source](/reference-sources/Data.StringIo.cinch) · line 117

<a id="value-737472696e672d696f2d7472756e63617465"></a>

## `string-io-truncate`

```cinch
string-io-truncate :: (StringIo -> (Nat -> StringIo))
```

Shrink the buffer to at most n bytes (a larger n is a no-op: this
never extends, matching Ruby's `truncate`). The cursor is clamped
into the new length, which keeps 0 <= pos <= length.

[Source](/reference-sources/Data.StringIo.cinch) · line 215

<a id="value-737472696e672d696f2d7772697465"></a>

## `string-io-write`

```cinch
string-io-write :: (StringIo -> (Bytes -> StringIo))
```

Write at the cursor: `bs` overwrites the bytes already there and
extends the buffer when it runs past the end; the bytes after the
write stay where they were (Ruby's overwrite, never an insert).
The cursor lands just past the last byte written, so a write of no
bytes leaves buffer and cursor unchanged.

[Source](/reference-sources/Data.StringIo.cinch) · line 140

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/StringIo.cinch`. Source SHA-256: `9eaf153aa82781838f853ea18293df64e0bda38a2055952c74da9dd84e94c4e1`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
