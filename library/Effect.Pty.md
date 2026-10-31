# Effect.Pty

Pseudo-terminal-backed child processes, terminal I/O, and lifecycle operations.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Pty`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Pty.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Path](Data.Path.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md).

Non-embedded source closure: [Effect.Pty](Effect.Pty.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Pty: pseudo-terminal children (the `Open3`/`PTY` shape).

A pseudo-terminal is a terminal device pair: the parent reads and
writes the MASTER end and the child's stdin, stdout and stderr are the
SLAVE end. `pty-open` allocates the pair, `pty-spawn` starts a program
with the slave as all three of its standard streams, and the caller
drives the master with `pty-read` and `pty-write`, waits with
`pty-wait`, sizes the window with `pty-resize`, and releases the
handle with `pty-close`. The handle is LINEAR: every operation takes
it at `%1` and, except `pty-close`, hands it back, so a terminal is
threaded through a program rather than aliased. This is the
`Open3`/`PTY` shape of design/stdlib-parity-ruby.md under P34's
Process capability, the refusal D367 that the piped-child ABI
(`Effect.Process`) recorded because the runtime had no
pseudo-terminal support.

The operations run under the `Io r` capability (decision D45's
determinism rule: a terminal is consulted when a program with the
capability calls it, never ambient at compile time). Every failure is
a NAMED `PtyError` value in a `Result` (the house style), never a
fault: the boundary's raw word carries the shared platform error codes
(`compiler/runtime/src/platform.rs`), which `pty-error-code` maps one
for one.

The boundary words themselves are the `cinch.runtime.pty*` units
(machine scalars and `Bytes`; see the runtime module's header for the
return-code convention). Those units are pure at the boundary, as
every `cinch.runtime.*` intrinsic is (a primitive's declared type is
its `sig()`, which carries no effect row); this module's wrappers are
where the `IO r` discipline lives, exactly as the primitive-type home
wraps `bytesNew`.

THE LAW SHAPE (what the slice's tests pin):
  1. round trip: bytes written to the master reach the child's stdin,
     and what the child writes to its stdout or stderr is what
     `pty-read` answers. The terminal's line discipline echoes the
     input and may interleave it with the child's own output, so a
     caller matches on the substring's presence, never an exact
     buffer;
  2. the handle is threaded: every operation takes it at `%1` and,
     except `pty-close`, hands it back. A `%1` parameter must be used
     exactly once on every path, so the handle a closed terminal was
     is gone: a program has no variable left to read from after a
     close, none to close twice, and none to use on one branch only.
     Read-after-close and double-close are therefore not writable
     programs, and the compiler says so (a second use of the parameter
     is `multiplicity %1: p must be used exactly once on every path,
     but is used 1 to 2 time(s)`; using it on one branch only answers
     the same with `(unused on some path)` appended). The boundary
     still refuses a handle word the caller fabricated (`Pty 4096`
     answers `PtyInvalidArgument`): the discipline removes the
     accidental misuses, the runtime check remains for invented
     words;
  3. the slave is consumed once: a handle whose child already holds
     the slave refuses a second spawn (`PtyInvalidArgument`). This
     stays a runtime refusal, since a threaded handle can legally be
     spawned twice; what the discipline removes is the spawn of a
     handle that is already gone;
  4. refusal is a value: a program that does not exist answers `Err`
     (the documented code), never a fault;
  5. the child's exit code is its own, or 128 + the signal number for
     a signal death (the shell convention).

WHY THE RETURN SHAPE KEEPS THE HANDLE ON THE ERROR PATH: an operation
answers `(Pty, Result PtyError a)` rather than `Result PtyError a`
because the failure does not free the handle. A bare `Result` would
drop the handle on the error path, and dropping it is exactly the leak
this shape exists to prevent. `pty-spawn` is the one operation whose
error path really does free the handle (the slave is consumed by the
attempt, so the handle is closed rather than handed back), which is
why it answers a bare `Result PtyError Pty`: there is nothing left to
return.

WHAT THE THREADING DOES NOT GIVE YOU (stated so the shape is not
overread):
  a terminal opened and never closed stays legal. A `let` or `case`
    binder defaults to `%many` and cannot carry a multiplicity
    annotation, so a `Pty` bound from `pty-open` and never handed to
    `pty-close` is accepted, and so is a handle dropped on one branch
    of a `case` on some other value. The discipline is enforced where
    a `%1` arrow exists: it forbids misusing a handle you hold, not
    holding one. Closing the last handle is a caller obligation, not a
    type error;
  no unrestricted wrapper. linear-base's `Ur a` (a regular value
    inside a constructor, extracted without a linear use) needs `Copy`
    evidence the language does not give (`Copy` is compiler-known and
    cannot be declared or given instances), so there is no `Ur Pty`
    spelling and no way to say "unrestricted once inside";
  no multiplicity-polymorphic binder and no user-declared
    `Consumable`/`Dupable`/`Movable` classes: a function cannot let its
    caller pick the discipline per use site, and the classes that would
    name that choice cannot be declared;
  no linear closure: a lambda body's uses are not counted per
    parameter, so a `%1 ->` continuation has to be a named function.
    The threading holds at the module's function boundaries.

Named refusals (boundaries this module deliberately does not cross):
  no terminal emulation beyond the pseudo-terminal itself: no
    escape-sequence interpretation and no screen buffer;
  no job control: the terminal's line discipline and process groups
    are the platform's;
  no session or detach manipulation: no `setsid`, no `TIOCNOTTY`;
  no Windows: `openpty` is POSIX;
  one read, not a filled length: `pty-read` returns what one platform
    call produced, never a loop to fill the requested size.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary.

</details>

# Types and constructors

<a id="type-507479"></a>

## `Pty`

```cinch
data Pty
```

A live pseudo-terminal, as the runtime's handle word. The handle
owns the master end and the child; the slave end belongs to the
child once a spawn consumed it. The operations below take it at
`%1`, so a live terminal is a value threaded by hand rather than a
reference copied around. The handle stays the runtime's `I64` word:
it only ever goes back to the runtime.

[Source](/reference-sources/Effect.Pty.cinch) · line 143

```cinch
Pty :: (I64 -> Pty)
```

A live pseudo-terminal, as the runtime's handle word. The handle
owns the master end and the child; the slave end belongs to the
child once a spawn consumed it. The operations below take it at
`%1`, so a live terminal is a value threaded by hand rather than a
reference copied around. The handle stays the runtime's `I64` word:
it only ever goes back to the runtime.

<a id="type-507479436f6d6d616e64"></a>

## `PtyCommand`

```cinch
data PtyCommand
```

A program with its arguments and its environment additions
(`KEY=VALUE` pairs, added to this process's own environment).

[Source](/reference-sources/Effect.Pty.cinch) · line 159

```cinch
PtyCommand :: (Path -> (List String -> (List (String, String) -> PtyCommand)))
```

A program with its arguments and its environment additions
(`KEY=VALUE` pairs, added to this process's own environment).

<a id="type-5074794572726f72"></a>

## `PtyError`

```cinch
data PtyError
```

What went wrong before the child's own exit code could exist. The
constructors correspond one for one to the platform layer's shared
codes.

[Source](/reference-sources/Effect.Pty.cinch) · line 131

```cinch
PtyNotFound :: PtyError
```

```cinch
PtyPermissionDenied :: PtyError
```

```cinch
PtyInvalidArgument :: PtyError
```

```cinch
PtyIoError :: PtyError
```

# Values

<a id="value-7074792d636c6f7365"></a>

## `pty-close`

```cinch
pty-close :: forall r . using (io :: Io r) => (Pty %1 ->{IO r} Result PtyError Unit)
```

Close the terminal: a child no wait reaped is killed and reaped
first, and the handle is consumed. This is the one operation that
does not hand the handle back, which is the whole point: after it
there is no value to read from or to close a second time, so those
programs cannot be written. The named error is the boundary's
answer for a handle word that was never live.

[Source](/reference-sources/Effect.Pty.cinch) · line 272

<a id="value-7074792d636f6d6d616e64"></a>

## `pty-command`

```cinch
pty-command :: (Path -> (List String -> PtyCommand))
```

A command without environment additions.

[Source](/reference-sources/Effect.Pty.cinch) · line 163

<a id="value-7074792d636f6d6d616e642d776974682d656e76"></a>

## `pty-command-with-env`

```cinch
pty-command-with-env :: (Path -> (List String -> (List (String, String) -> PtyCommand)))
```

A command with environment additions.

[Source](/reference-sources/Effect.Pty.cinch) · line 167

<a id="value-7074792d68616e646c65"></a>

## `pty-handle`

```cinch
pty-handle :: (Pty -> I64)
```

The handle word of a terminal, for the layers above this one. The
word is not a handle, and the projection is the one hole in the
discipline: a `Pty` minted from a word is a second value for the
same terminal, which the runtime answers with `PtyInvalidArgument`
once the real handle is closed (and a word that was never live
answers the same). The threading's guarantee covers the handles the
operations hand out, not the bare word.

[Source](/reference-sources/Effect.Pty.cinch) · line 153

<a id="value-7074792d6f70656e"></a>

## `pty-open`

```cinch
pty-open :: forall r . using (io :: Io r) => (Unit ->{IO r} Result PtyError Pty)
```

Open a pseudo-terminal pair: the master and slave ends in a handle,
or the named error. The slave belongs to the handle until a spawn
hands it to the child.

[Source](/reference-sources/Effect.Pty.cinch) · line 196

<a id="value-7074792d72656164"></a>

## `pty-read`

```cinch
pty-read :: forall r . using (io :: Io r) => (Pty %1 -> (Nat ->{IO r} (Pty, Result PtyError Bytes)))
```

Read up to `max` bytes of the terminal's master (what the child
wrote to its stdout or stderr, plus the terminal's echo of the
input): the handle and `Ok` of what one platform call produced, or
the handle and the named error. A failed read leaves the session
live, so the handle is handed back rather than dropped.

[Source](/reference-sources/Effect.Pty.cinch) · line 250

<a id="value-7074792d726573697a65"></a>

## `pty-resize`

```cinch
pty-resize :: forall r . using (io :: Io r) => (Pty %1 -> (Nat -> (Nat ->{IO r} (Pty, Result PtyError Unit))))
```

Set the terminal's window size to `cols` columns by `rows` rows:
the handle and the unit, or the handle and the named error (the
handle comes back for the same reason as `pty-write`'s).

[Source](/reference-sources/Effect.Pty.cinch) · line 281

<a id="value-7074792d737061776e"></a>

## `pty-spawn`

```cinch
pty-spawn :: forall r . using (io :: Io r) => (Pty %1 -> (Option Path -> (PtyCommand ->{IO r} Result PtyError Pty)))
```

Start `cmd` on the terminal `p`, running in `dir` (`None` inherits
this process's own directory): the same handle with the child beside
its master, or the named error. The child's stdin, stdout and stderr
are all the slave end, so it is a terminal session. The handle is
consumed at `%1`: the slave leaves it for the child, so the handle
the caller held is the one the attempt used, and it is a fresh
result (never the same variable). A handle whose child already holds
the slave refuses a second spawn; on a spawn failure the handle is
closed rather than left live with a consumed slave, which is why the
error path carries no handle: it is already freed, and handing back
a closed one would be a trap for the caller.

[Source](/reference-sources/Effect.Pty.cinch) · line 212

<a id="value-7074792d77616974"></a>

## `pty-wait`

```cinch
pty-wait :: forall r . using (io :: Io r) => (Pty %1 ->{IO r} (Pty, Result PtyError Nat))
```

Wait for the child: the handle and its exit code (128 + the signal
number for a signal death, the shell convention), or the handle and
the named error. A second wait answers the same code, so waiting is
not what ends the handle's life; `pty-close` is.

[Source](/reference-sources/Effect.Pty.cinch) · line 260

<a id="value-7074792d7772697465"></a>

## `pty-write`

```cinch
pty-write :: forall r . using (io :: Io r) => (Pty %1 -> (Bytes ->{IO r} (Pty, Result PtyError Nat)))
```

Write `bs` to the terminal's master (the child reads it on its
stdin): the handle and the byte count (the whole buffer goes), or
the handle and the named error. The handle comes back because a
failed write is a failed write, not a dead terminal: the caller
keeps the session.

[Source](/reference-sources/Effect.Pty.cinch) · line 239

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Pty.cinch`. Source SHA-256: `04dfbae47c6b1b0db95b5393eabbe0a509d14e3962e2597f2e24e205f5a9c1be`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
