# Effect.Find

Filesystem walking/search utilities with the module's traversal and error policy.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Find`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Find.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.List](Data.List.md), [Data.Path](Data.Path.md), [Data.Result](Data.Result.md), [Effect.Fs](Effect.Fs.md).

Non-embedded source closure: [Effect.Find](Effect.Find.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Find: the recursive directory walk (P40, the Ruby `Find`
port named in design/stdlib-parity-ruby.md's OS-utilities sweep).

`find` enumerates a subtree depth-first in PRE-ORDER: the root is
reported first, then each directory's entries in `fs-list`'s order
(names ascending, the listing order decision D335 records, recorded
in compiler/CONFORMANCE.md), a directory entered before its next
sibling. That is Ruby's `Find.find` order, and the entry order is
the contract: after a reported directory `d` and before the next
sibling of `d`, every reported path lies beneath `d`.

The visitor is asked once per entry, in that same order, and answers
with a `Step`:
  `Continue` descends into the entry when it is a directory;
  `Prune` keeps the entry and skips its contents (Ruby's
    `Find.prune`; on a non-directory it is a no-op);
  `Stop` ends the walk at that entry, which is the last one
    reported.

Laws:
  `find` reports every visited path exactly once, and reports the
    root first (a root that does not exist is `Err` with nothing
    reported);
  `Prune` at `p` removes exactly the descendants of `p` (when `p`
    names a directory) and moves no other entry;
  `Stop` at `p` makes the result the reported prefix ending at `p`;
  `find-files r` is the `FileIsFile` sublist of the walk that
    descends through everything and `find-dirs r` the
    `FileIsDirectory` one, so a filter changes what is reported,
    never what is visited.

Named refusals (boundaries this module deliberately does not
cross):
  partial results: the first named `FsError` ends the walk and `Err`
    carries nothing collected (a truncated list would be
    indistinguishable from a complete one); a caller who wants the
    frontier reads `fs-list`/`fs-metadata` directly;
  error swallowing: there is no `ignore_error` flag (Ruby's default
    skips a directory it cannot read); a skip would make the result
    a claim about a tree the walk did not read;
  symlink cycles: nothing here detects them, and `fs-metadata`
    reports the target's kind (Effect.Fs's rule), so a directory
    symlink cycle walks until the OS refuses a path; the caller
    breaks it with `Prune`;
  effectful visitors: the visitor is a plain value
    (`Path -> Step`), so the walk's row is the filesystem's `{IO r}`
    alone; a caller whose visit acts collects the paths first and
    acts afterwards;
  a depth bound: the recursion follows the tree's shape, so its
    depth is the tree's.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary, and `Effect.Fs` is the capability walked here.

</details>

# Types and constructors

<a id="type-53746570"></a>

## `Step`

```cinch
data Step
```

What the visitor decides about the entry it is shown.

[Source](/reference-sources/Effect.Find.cinch) · line 66

```cinch
Continue :: Step
```

```cinch
Prune :: Step
```

```cinch
Stop :: Step
```

# Values

<a id="value-66696e64"></a>

## `find`

```cinch
find :: forall r . using (io :: Io r) => (Path -> ((Path -> Step) ->{IO r} Result FsError (List Path)))
```

The subtree beneath `root`, pre-order, reporting the paths the
visitor's decisions leave in the walk (see the header for the
order guarantee and the decisions).

[Source](/reference-sources/Effect.Find.cinch) · line 148

<a id="value-66696e642d64697273"></a>

## `find-dirs`

```cinch
find-dirs :: forall r . using (io :: Io r) => (Path ->{IO r} Result FsError (List Path))
```

Every directory beneath `root` (the root included when it is one),
pre-order.

[Source](/reference-sources/Effect.Find.cinch) · line 158

<a id="value-66696e642d66696c6573"></a>

## `find-files`

```cinch
find-files :: forall r . using (io :: Io r) => (Path ->{IO r} Result FsError (List Path))
```

Every file beneath `root` (the root itself only when it is one),
pre-order, descending through every directory.

[Source](/reference-sources/Effect.Find.cinch) · line 153

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Find.cinch`. Source SHA-256: `8897fed1fe39856ce9857902d0a5f5fe258a1064bf3be7cf1ba35c033af98080`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
