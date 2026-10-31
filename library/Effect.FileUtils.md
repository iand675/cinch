# Effect.FileUtils

Convenience filesystem operations built on the lower-level filesystem API.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.FileUtils`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.FileUtils.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Option](Data.Option.md), [Data.Path](Data.Path.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Fs](Effect.Fs.md), [Effect.Random](Effect.Random.md).

Non-embedded source closure: [Effect.FileUtils](Effect.FileUtils.md), [Effect.Random](Effect.Random.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.FileUtils: the mutating filesystem operations (P40, the Ruby
`FileUtils` port named in design/stdlib-parity-ruby.md's OS-utilities
sweep).

Where `Data.Path` carries the pure Pathname operations (basename,
dirname, normalize: text in, text out, no I/O), this home carries the
operations that WRITE to the disk: `mkdir-p`, `rm`/`rm-r`, `cp`,
`mv`, `touch`, and the two temp-name constructors `temp-file` and
`temp-dir`. Every one takes the `Io r` capability and performs
`IO r` (decision D45: the filesystem is consulted at runtime through
the capability, never ambient at compile time; a `stage` body cannot
reach it), and every failure is a NAMED `FileUtilsError` value in a
`Result` (the house style: `Effect.Fs`'s named vocabulary shape),
never a fault.

The operations ride three landed surfaces: `Effect.Fs` (`fs-read`,
`fs-write`, `fs-rename`, `fs-metadata`) for the read, write and
atomic-move path; the `cinch.runtime.fsMkdir`/`fsRemove`/`fsTempDir`
intrinsics (`compiler/runtime/src/platform.rs`) for creation and
removal, which `Effect.Fs` does not expose; and `Effect.Random`
(`random-bytes` for the entropy draw and the pure `hex-of-bytes` for
the temp-name nonce). The draw is BOUND to a `let` before rendering,
because the nested `hex-of-bytes (random-bytes n)` shape inside a
capability-constrained function miscompiles to a wrong-word fault
today (a compiler-side bug, reported; the bound shape is verified
good).

Named refusals (boundaries this module deliberately does not cross):
  permissions and ownership: nothing here reads or writes a mode or
    an owner; there is no `chmod`/`chown` (the OS's own permission
    failures surface as `FileUtilsPermissionDenied`, never a change);
  symlink creation: no `symlink` operation exists in the slice (the
    temp-file test constructs its link from the Rust side);
  recursive copy of a symlinked tree: `cp` copies a REGULAR FILE's
    bytes (read plus write) and refuses a directory source
    (`FileUtilsIsADirectory`), so it never walks a tree at all and
    therefore never follows or copies a link;
  timestamp and metadata preservation: `cp` and `touch` do NOT
    preserve or set mtime/atime/mode (`shutil.copy2`'s behavior is
    out of scope; `touch` creates a missing file and leaves an
    existing one's contents unchanged, it does not bump a clock);
  exclusive creation: the slice has no `O_EXCL` create, so
    `temp-file`/`temp-dir` compose a 128-bit random name and then
    CHECK for the name before creating; a name already taken is
    reported as `FileUtilsAlreadyExists`, never silently truncated
    over (the uniqueness argument is the entropy, see `temp-file`).

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary.

</details>

# Types and constructors

<a id="type-46696c655574696c734572726f72"></a>

## `FileUtilsError`

```cinch
data FileUtilsError
```

What went wrong, named (the shared platform codes `Effect.Fs` maps
its `FsError` from, in this home's vocabulary).

[Source](/reference-sources/Effect.FileUtils.cinch) · line 66

```cinch
FileUtilsNotFound :: FileUtilsError
```

```cinch
FileUtilsAlreadyExists :: FileUtilsError
```

```cinch
FileUtilsIsADirectory :: FileUtilsError
```

```cinch
FileUtilsNotADirectory :: FileUtilsError
```

```cinch
FileUtilsPermissionDenied :: FileUtilsError
```

```cinch
FileUtilsIoError :: FileUtilsError
```

# Values

<a id="value-6370"></a>

## `cp`

```cinch
cp :: forall r . using (io :: Io r) => (Path -> (Path ->{IO r} Result FileUtilsError Unit))
```

`cp src dst`: copy a regular file's bytes from `src` to `dst`
(read plus write). A directory `src` is `FileUtilsIsADirectory`,
a missing `src` is `FileUtilsNotFound`, and a `dst` whose parent
is not a directory is `FileUtilsNotADirectory`. The write is
`fs-write`: it creates or truncates `dst` and is NOT atomic (the
atomic replacement is write-then-`fs-rename`, which `mv` uses).

[Source](/reference-sources/Effect.FileUtils.cinch) · line 142

<a id="value-6d6b6469722d70"></a>

## `mkdir-p`

```cinch
mkdir-p :: forall r . using (io :: Io r) => (Path ->{IO r} Result FileUtilsError Unit)
```

`mkdir-p p`: create `p` and every missing parent, like `mkdir -p`.
An existing directory is success (idempotent); a non-directory at
`p` is `FileUtilsAlreadyExists`.

[Source](/reference-sources/Effect.FileUtils.cinch) · line 111

<a id="value-6d76"></a>

## `mv`

```cinch
mv :: forall r . using (io :: Io r) => (Path -> (Path ->{IO r} Result FileUtilsError Unit))
```

`mv src dst`: move `src` to `dst` with `fs-rename`, the POSIX
atomic rename. It is used rather than copy-then-remove because it
is atomic (a concurrent reader sees the whole old or the whole new
state, never a partial) and it preserves the object's identity; the
price is the OS's own rule that a rename cannot cross a filesystem
(that failure surfaces as `FileUtilsIoError`).

[Source](/reference-sources/Effect.FileUtils.cinch) · line 155

<a id="value-726d"></a>

## `rm`

```cinch
rm :: forall r . using (io :: Io r) => (Path ->{IO r} Result FileUtilsError Unit)
```

`rm p`: remove what `p` names WITHOUT recursion: a regular file,
a symlink (the link itself, never its target: the intrinsic
resolves the kind through `symlink_metadata`), or an empty
directory. A non-empty directory is the OS's refusal
(`FileUtilsIoError`; `rm-r` is the recursive form) and a missing
path is `FileUtilsNotFound`.

[Source](/reference-sources/Effect.FileUtils.cinch) · line 122

<a id="value-726d2d72"></a>

## `rm-r`

```cinch
rm-r :: forall r . using (io :: Io r) => (Path ->{IO r} Result FileUtilsError Unit)
```

`rm-r p`: remove what `p` names recursively: a file, a symlink, or
a whole directory subtree. A symlink is removed as the LINK (the
runtime resolves it through `symlink_metadata` and its recursive
walk does not follow links), so the target is never touched.

[Source](/reference-sources/Effect.FileUtils.cinch) · line 131

<a id="value-74656d702d646972"></a>

## `temp-dir`

```cinch
temp-dir :: forall r . using (io :: Io r) => (Unit ->{IO r} Result FileUtilsError Path)
```

`temp-dir u`: create (and name) a fresh directory under the
platform temp directory. The name carries the 128-bit nonce and the
directory is created with a bare `mkdir` (no parents), so a name
the OS already knows is reported as `FileUtilsAlreadyExists`
rather than reused.

[Source](/reference-sources/Effect.FileUtils.cinch) · line 199

<a id="value-74656d702d66696c65"></a>

## `temp-file`

```cinch
temp-file :: forall r . using (io :: Io r) => (Unit ->{IO r} Result FileUtilsError Path)
```

`temp-file u`: create (and name) a fresh EMPTY file under the
platform temp directory. UNIQUENESS: the same 128-bit nonce; the
name is CHECKED with `fs-metadata` before the create, so a name
already taken (a collision the entropy makes astronomically
unlikely, roughly 2^-64 over any realistic number of calls) is
`FileUtilsAlreadyExists`, never a silent truncation of another
file. There is no exclusive-create primitive in the slice, so the
check and the create are not one atomic step; the entropy, not the
check, is what makes a collision negligible, and the refusal is
named rather than guessed away by a retry loop.

[Source](/reference-sources/Effect.FileUtils.cinch) · line 221

<a id="value-746f756368"></a>

## `touch`

```cinch
touch :: forall r . using (io :: Io r) => (Path ->{IO r} Result FileUtilsError Unit)
```

`touch p`: create `p` as an empty file when it does not exist and
leave an existing file's contents unchanged (the cheap touch; it
does NOT bump a timestamp, see the refusals). The create is
`fs-write` of no bytes, so a missing parent is reported.

[Source](/reference-sources/Effect.FileUtils.cinch) · line 164

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/FileUtils.cinch`. Source SHA-256: `17e8649cadb5ba3ee76cccdc2f18b9306f7749857f55addaa9f517ffc4e71777`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
