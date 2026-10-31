# Effect.Process

One-shot and live subprocesses, command arguments/environment additions, pipes, exit codes, and this process's arguments.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Effect.Process`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Process.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Path](Data.Path.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Process: subprocesses over real pipes (P34, the platform
layer: the shape that drives `clang` and `lld`), one shot and live.

A `Command` names a program, its arguments and its environment
additions; `process-run` runs it with the given input on its stdin,
captures its stdout and reports its exit code, and `process-spawn`
starts a LIVE `Child` whose three stdio pipes the caller drives
(`child-write-stdin`, `child-close-stdin`, `child-read`,
`child-wait`, `child-close`), all under the `Io r` capability in the
mold of the runtime's `Io r` operations (decision D45: subprocesses
run at runtime through the capability, never ambient at compile
time; a `stage` body reaches a subprocess only through staging's own
recorded `StageProcess` capability, and this home has no staging
path). Failures are NAMED `ProcessError` values in a `Result` (the
house style), never faults.

Two boundary conventions live here, both in
`compiler/runtime/src/platform.rs`: the one-shot `cinch_proc_*`/
`cinch_buf_*` externs are plain `foreign import ccall` declarations
whose paths, arguments and output cross through runtime word buffers
one byte at a time (see Effect.Fs's bridge), while the live-child
`cinch.runtime.process*` units take `Bytes` words for the program,
the NUL-separated argument and environment words and every payload
(a long-lived child is a value stream, not a byte-at-a-time bridge).
Those units are pure at the boundary, as every `cinch.runtime.*`
intrinsic is; the `IO r` discipline lives in this module's wrappers,
exactly as the primitive-type home wraps `bytesNew`.

This module also closes the argv gap that made a real bootstrap tool
impossible: `cinch_argv_*` (`cinch_argv_init`, then
`cinch_argv_count`/`cinch_argv_get`) is THIS process's own command
line, captured by the generated entry glue BEFORE any program code
runs (the native `@main` of `compiler/src/codegen.rs`), and
`process-args` is the binding a program reads it with. The capture
mirrors the operating system exactly: word 0 is the program name and
the count includes it, so `process-args` reports the words AFTER the
program name, in the order they were given. An index outside the
capture is the boundary's own defined out-of-bounds fault (the
`bytesGet` convention), which only a caller walking past the count
can reach, not `process-args`.

Named refusals (boundaries this module deliberately does not
cross):
  the process's own environment: the command line is captured, the
    environment is not. It is the SAME SIZE to add (one sibling
    `cinch_env_count`/`cinch_env_get` pair over `environ`, or the
    entry's third parameter `envp`, plus one `Effect.Env`-style
    binding), and it is refused here only because environment access
    is its own home's vocabulary, not the process home's; a caller
    that needs it gets it from the same entry glue;
  a non-UTF-8 word: a command line is OS bytes, so `process-args`
    reports `ProcessInvalidArgument` instead of substituting or
    dropping text (the `fs-list` refusal, one vocabulary up);
  the live child's pipes are separate: a caller reads stdout and
    stderr on its own schedule, so a child that fills one pipe while
    the caller reads the other blocks until it is drained (there is
    no select or multiplexing here), and `child-wait` blocks, so the
    caller drains what it needs before it waits;
  stderr capture of the one-shot shape: `process-run`'s child
    inherits this process's stderr (only stdout is captured), while a
    spawned child's stderr is its own pipe;
  killing by signal: an exit code is the child's own (a signal death
    reports 128 + the signal number, the shell convention), and the
    only signal this module sends is the kill a `child-close` of an
    unreaped child does;
  a stdin payload larger than the pipe buffer while the child does
    not read: the write blocks (the worked drivers consume theirs);
  no pseudo-terminal: a child's stdio here are pipes, never a
    terminal; the terminal-backed child is its own home, `Effect.Pty`
    (`pty-spawn` and its siblings);
  an empty argument is not expressible: the argument list crosses
    NUL-separated, so an empty word between separators is refused
    (`ProcessInvalidArgument`) rather than dropped.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary.

</details>

# Types and constructors

<a id="type-4368696c64"></a>

## `Child`

```cinch
data Child
```

The live child (the piped ABI)

A live child with three stdio pipes of its own.

[Source](/reference-sources/Effect.Process.cinch) · line 310

```cinch
Child :: (I64 -> Child)
```

The live child (the piped ABI)

A live child with three stdio pipes of its own.

<a id="type-436f6d6d616e64"></a>

## `Command`

```cinch
data Command
```

A program with its arguments and its environment additions
(`KEY=VALUE` pairs, added to this process's own environment).

[Source](/reference-sources/Effect.Process.cinch) · line 107

```cinch
Command :: (Path -> (List String -> (List (String, String) -> Command)))
```

A program with its arguments and its environment additions
(`KEY=VALUE` pairs, added to this process's own environment).

<a id="type-50726f636573734572726f72"></a>

## `ProcessError`

```cinch
data ProcessError
```

What went wrong before the child's own exit code could exist.

[Source](/reference-sources/Effect.Process.cinch) · line 99

```cinch
ProcessNotFound :: ProcessError
```

```cinch
ProcessPermissionDenied :: ProcessError
```

```cinch
ProcessInvalidArgument :: ProcessError
```

```cinch
ProcessIoError :: ProcessError
```

<a id="type-53747265616d"></a>

## `Stream`

```cinch
data Stream
```

Which of the child's two output pipes a read takes.

[Source](/reference-sources/Effect.Process.cinch) · line 314

```cinch
Stdout :: Stream
```

```cinch
Stderr :: Stream
```

# Values

<a id="value-6368696c642d636c6f7365"></a>

## `child-close`

```cinch
child-close :: forall r . using (io :: Io r) => (Child ->{IO r} Result ProcessError Unit)
```

Close the child: a child no wait reaped is killed and reaped
first, and the handle is gone afterwards (a second close is the
named error).

[Source](/reference-sources/Effect.Process.cinch) · line 438

<a id="value-6368696c642d636c6f73652d737464696e"></a>

## `child-close-stdin`

```cinch
child-close-stdin :: forall r . using (io :: Io r) => (Child ->{IO r} Result ProcessError Unit)
```

Close the child's stdin, which is how it sees the end of its
input: the unit, or the named error.

[Source](/reference-sources/Effect.Process.cinch) · line 406

<a id="value-6368696c642d68616e646c65"></a>

## `child-handle`

```cinch
child-handle :: (Child -> I64)
```

The child's handle word, for the layers above this one.

[Source](/reference-sources/Effect.Process.cinch) · line 319

<a id="value-6368696c642d72656164"></a>

## `child-read`

```cinch
child-read :: forall r . using (io :: Io r) => (Child -> (Stream -> (Nat ->{IO r} Result ProcessError Bytes)))
```

Read up to `max` bytes of one of the child's output pipes: `Ok` of
what one platform call produced (empty at that pipe's end), or the
named error.

[Source](/reference-sources/Effect.Process.cinch) · line 415

<a id="value-6368696c642d77616974"></a>

## `child-wait`

```cinch
child-wait :: forall r . using (io :: Io r) => (Child ->{IO r} Result ProcessError Nat)
```

Wait for the child: its exit code (128 + the signal number for a
signal death, the shell convention), or the named error.

[Source](/reference-sources/Effect.Process.cinch) · line 429

<a id="value-6368696c642d77726974652d737464696e"></a>

## `child-write-stdin`

```cinch
child-write-stdin :: forall r . using (io :: Io r) => (Child -> (Bytes ->{IO r} Result ProcessError Nat))
```

Write `bs` to the child's stdin: the byte count, or the named
error.

[Source](/reference-sources/Effect.Process.cinch) · line 398

<a id="value-636f6d6d616e64"></a>

## `command`

```cinch
command :: (Path -> (List String -> Command))
```

A command without environment additions.

[Source](/reference-sources/Effect.Process.cinch) · line 111

<a id="value-636f6d6d616e642d776974682d656e76"></a>

## `command-with-env`

```cinch
command-with-env :: (Path -> (List String -> (List (String, String) -> Command)))
```

A command with environment additions.

[Source](/reference-sources/Effect.Process.cinch) · line 115

<a id="value-70726f636573732d61726773"></a>

## `process-args`

```cinch
process-args :: forall r . using (io :: Io r) => (Unit ->{IO r} Result ProcessError (List String))
```

The arguments this process was invoked with, the program name
(word 0) excluded, decoded as text. An empty command line is
`Ok Nil`, and a word that is not valid UTF-8 is `Err
ProcessInvalidArgument` (a command line is OS bytes: the `fs-list`
refusal, one vocabulary up). Words that carry a space, a `-` or
nothing at all arrive intact: this reads the operating system's
own `argv` words, so there is no splitting, quoting or trimming
between the shell's exec and this list.
`process-args` is an effectful computation, so it takes the `Unit`
an effectful value always takes in this tree (`main :: Unit ->{IO
r} Int`'s shape): `process-args ()`.

[Source](/reference-sources/Effect.Process.cinch) · line 290

<a id="value-70726f636573732d72756e"></a>

## `process-run`

```cinch
process-run :: forall r . using (io :: Io r) => (Command -> (String ->{IO r, Diverge} Result ProcessError (Nat, Bytes)))
```

Run `cmd` with `input` on its stdin: the child's exit code and
its captured stdout (raw bytes), or the named error.

[Source](/reference-sources/Effect.Process.cinch) · line 180

<a id="value-70726f636573732d737061776e"></a>

## `process-spawn`

```cinch
process-spawn :: forall r . using (io :: Io r) => (Option Path -> (Command ->{IO r} Result ProcessError Child))
```

Start `cmd` with its three stdio pipes, running in `dir` (`None`
inherits this process's own directory): the live child, or the
named error. The child inherits this process's environment plus
`cmd`'s additions.

[Source](/reference-sources/Effect.Process.cinch) · line 350

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Process.cinch`. Source SHA-256: `08f3ecd91ba9170eec876e5a2df0662eb0bc4289aa4708533a36d4a45bfffa60`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
