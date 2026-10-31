# Data.Path

Platform path values and composition/conversion operations used by filesystem and process APIs.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Path`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Path.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Semigroup](Algebra.Semigroup.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Path: filesystem paths (P34, the platform layer).

A `Path` names filesystem bytes: the text of a POSIX path, carried
whole (no canonicalization, no separator rewriting). This replaces
the placeholder `Path :: Nat -> Path` that Effect.Io carried (an
index into the runtime's input source): a real path names BYTES ON
DISK, and the platform homes (Effect.Fs, Effect.Process) hand its
text to the OS as UTF-8 bytes across the foreign boundary.

Named refusals (boundaries this module deliberately does not
cross):
  path canonicalization: `path-join` concatenates with one `/` and
    never resolves `.`/`..`; `path-normalize` resolves `.`/`..`
    LEXICALLY (text in, text out, never a disk call and never a
    symlink resolution, which is the OS's job at the call: the
    error vocabulary reports what it found);
  Windows separators: the bootstrap's platform layer is POSIX (`/`
    is the only separator, and `\` is an ordinary character);
  non-UTF-8 path text: the carrier is `String` (UTF-8), so the
    platform layer's byte crossing is UTF-8 bytes;
  separator collapsing outside `path-normalize`: `path-join` puts
    exactly one `/` between two texts and the component readers
    (`path-basename`/`path-dirname`) drop trailing separators, but
    only `path-normalize` drops EMPTY components, so `a//b` stays
    `a//b` until it is normalized.

Module scheme (design/base-library.md): `Data.*` is data types; the
path's home carries its conversions, its join, and the pure
Pathname operations (the I/O half is `Effect.FileUtils`). The
Pathname parity target is `design/stdlib-parity-ruby.md`'s P40 OS
sweep; these are the operations that cost no I/O.

</details>

# Types and constructors

<a id="type-50617468"></a>

## `Path`

```cinch
data Path
```

The path text (POSIX; UTF-8 at the boundary).

[Source](/reference-sources/Data.Path.cinch) · line 48

```cinch
Path :: (String -> Path)
```

The path text (POSIX; UTF-8 at the boundary).

# Values

<a id="value-706174682d626173656e616d65"></a>

## `path-basename`

```cinch
path-basename :: (Path -> String)
```

[Source](/reference-sources/Data.Path.cinch) · line 161

<a id="value-706174682d6469726e616d65"></a>

## `path-dirname`

```cinch
path-dirname :: (Path -> String)
```

[Source](/reference-sources/Data.Path.cinch) · line 164

<a id="value-706174682d6578746e616d65"></a>

## `path-extname`

```cinch
path-extname :: (Path -> String)
```

[Source](/reference-sources/Data.Path.cinch) · line 185

<a id="value-706174682d66726f6d2d737472696e67"></a>

## `path-from-string`

```cinch
path-from-string :: (String -> Path)
```

A path from its text.

[Source](/reference-sources/Data.Path.cinch) · line 52

<a id="value-706174682d69732d6162736f6c757465"></a>

## `path-is-absolute`

```cinch
path-is-absolute :: (Path -> Bool)
```

`path-is-absolute`: the path begins with `/` (the empty path is
not absolute).

[Source](/reference-sources/Data.Path.cinch) · line 203

<a id="value-706174682d6a6f696e"></a>

## `path-join`

```cinch
path-join :: (Path -> (Path -> Path))
```

The path `a/b`: exactly one `/` between the two texts (no
separator collapsing; see the refusals above).

[Source](/reference-sources/Data.Path.cinch) · line 62

<a id="value-706174682d6e6f726d616c697a65"></a>

## `path-normalize`

```cinch
path-normalize :: (Path -> Path)
```

`path-normalize`: LEXICAL cleanup (text in, text out; no disk call
and no symlink resolution). Split on `/`, drop empty and `.`
segments, resolve each `..` as above; join with `/` and prefix
`/` when the input began with `/`.

[Source](/reference-sources/Data.Path.cinch) · line 277

<a id="value-706174682d72656c61746976652d746f"></a>

## `path-relative-to`

```cinch
path-relative-to :: (Path -> (Path -> Path))
```

`path-relative-to target base`: the relative path from `base` to
`target` under the same lexical form. Both are normalized first.
When their absoluteness differs no common prefix exists and the
normalized target is returned; otherwise the shared leading
components are dropped, one `..` is emitted per remaining base
component, and the remaining target components follow (nothing
remaining is `.`).

[Source](/reference-sources/Data.Path.cinch) · line 309

<a id="value-706174682d7374656d"></a>

## `path-stem`

```cinch
path-stem :: (Path -> String)
```

[Source](/reference-sources/Data.Path.cinch) · line 198

<a id="value-706174682d746f2d737472696e67"></a>

## `path-to-string`

```cinch
path-to-string :: (Path -> String)
```

The path's text.

[Source](/reference-sources/Data.Path.cinch) · line 56

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Path.cinch`. Source SHA-256: `1ce780bc27ffa8bb5951ac340ecf80e5ec0ebd44d0da1ce2f042175966b34772`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
