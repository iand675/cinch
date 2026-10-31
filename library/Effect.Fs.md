# Effect.Fs

Filesystem reads/writes, directory and metadata operations, with Path values and named Result errors.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Effect.Fs`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Fs.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Path](Data.Path.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Fs: the filesystem capability (P34, the platform layer).

The operations read, write, list, stat and atomically rename
filesystem bytes, under the `Io r` capability in the mold of the
runtime's `Io r` operations (decision D45: the filesystem is
consulted at runtime through the capability, never ambient at
compile time; a `stage` body cannot reach it, see Effect.Clock's
header). Paths are `Data.Path` values; content is `Bytes`; every
failure is a NAMED `FsError` value in a `Result` (the house style:
`Text.Parse`'s named vocabulary shape), never a fault.

The boundary words (`compiler/runtime/src/platform.rs`, the
`cinch_fs_*`/`cinch_buf_*` externs) are the plain `foreign import
ccall` machinery: machine scalars only, so paths and content cross
through runtime word buffers, filled and drained one byte at a time
below. The error codes those words return map one for one onto the
named constructors (`NotFound` ... `IoError`).

Named refusals (boundaries this module deliberately does not
cross):
  non-atomic write: `fs-write` creates or truncates and is NOT
    atomic; atomic replacement is write-then-`fs-rename` (the probe
    in compiler/tests/platform.rs exercises exactly that);
  partial reads: `fs-read` reads the whole file or fails (there is
    no streaming reader in the bootstrap slice);
  permissions beyond the OS's own: the error vocabulary reports
    what the OS said and nothing is widened;
  recursive listing or deletion: `fs-list` names one directory's
    entries (sorted ascending for determinism), and nothing removes
    files;
  symlinks: the OS resolves them at the call (`fs-metadata` reports
    the target's kind).

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary.

</details>

# Types and constructors

<a id="type-46696c654b696e64"></a>

## `FileKind`

```cinch
data FileKind
```

What a path names.

[Source](/reference-sources/Effect.Fs.cinch) · line 68

```cinch
FileIsFile :: FileKind
```

```cinch
FileIsDirectory :: FileKind
```

```cinch
FileIsOther :: FileKind
```

<a id="type-46696c654d657461"></a>

## `FileMeta`

```cinch
data FileMeta
```

A file's metadata: its kind and its size in bytes.

[Source](/reference-sources/Effect.Fs.cinch) · line 74

```cinch
FileMeta :: (FileKind -> (Nat -> FileMeta))
```

A file's metadata: its kind and its size in bytes.

<a id="type-46734572726f72"></a>

## `FsError`

```cinch
data FsError
```

What went wrong, named (the `std::io::ErrorKind` vocabulary the
runtime maps its codes from).

[Source](/reference-sources/Effect.Fs.cinch) · line 58

```cinch
NotFound :: FsError
```

```cinch
PermissionDenied :: FsError
```

```cinch
AlreadyExists :: FsError
```

```cinch
NotADirectory :: FsError
```

```cinch
IsADirectory :: FsError
```

```cinch
InvalidName :: FsError
```

```cinch
IoError :: FsError
```

# Values

<a id="value-66696c652d6d6574612d6b696e64"></a>

## `file-meta-kind`

```cinch
file-meta-kind :: (FileMeta -> FileKind)
```

[Source](/reference-sources/Effect.Fs.cinch) · line 77

<a id="value-66696c652d6d6574612d73697a65"></a>

## `file-meta-size`

```cinch
file-meta-size :: (FileMeta -> Nat)
```

[Source](/reference-sources/Effect.Fs.cinch) · line 81

<a id="value-66732d6c697374"></a>

## `fs-list`

```cinch
fs-list :: forall r . using (io :: Io r) => (Path ->{IO r} Result FsError (List String))
```

The names in the directory `p`, sorted ascending (the OS listing
order is not specified; the sorted listing is this home's
observable order).

[Source](/reference-sources/Effect.Fs.cinch) · line 225

<a id="value-66732d6d65746164617461"></a>

## `fs-metadata`

```cinch
fs-metadata :: forall r . using (io :: Io r) => (Path ->{IO r} Result FsError FileMeta)
```

The kind and size of what `p` names.

[Source](/reference-sources/Effect.Fs.cinch) · line 199

<a id="value-66732d72656164"></a>

## `fs-read`

```cinch
fs-read :: forall r . using (io :: Io r) => (Path ->{IO r} Result FsError Bytes)
```

The whole file at `p`, or the named error (missing paths report
`NotFound`).

[Source](/reference-sources/Effect.Fs.cinch) · line 155

<a id="value-66732d72656e616d65"></a>

## `fs-rename`

```cinch
fs-rename :: forall r . using (io :: Io r) => (Path -> (Path ->{IO r} Result FsError Unit))
```

Atomically rename `from` to `to` (POSIX `rename`: the target sees
the whole old or the whole new content, never a partial).

[Source](/reference-sources/Effect.Fs.cinch) · line 186

<a id="value-66732d7772697465"></a>

## `fs-write`

```cinch
fs-write :: forall r . using (io :: Io r) => (Path -> (Bytes ->{IO r} Result FsError Unit))
```

Write `content` to `p`, creating or truncating it (NOT atomic;
atomic replacement is `fs-rename`).

[Source](/reference-sources/Effect.Fs.cinch) · line 172

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Fs.cinch`. Source SHA-256: `5d3634d76d38f248b1d2e97c51ff8b2b3cc20585723608c14a073e1ff7d7ebae`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
