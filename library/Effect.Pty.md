# Effect.Pty

Pseudo-terminal-backed child processes, terminal I/O, and lifecycle operations.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Pty</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Pty/source/">View source</a><a href="/packages/search/?q=%2BEffect.Pty">Search module</a><a href="/reference-sources/Effect.Pty.cinch" download="Effect.Pty.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Path](Data.Path.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Effect.Pty`: [Effect.Pty](Effect.Pty.md).

```sh
cp seed/prelude/Effect/Pty.cinch ./Effect.Pty.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

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
(`seed/runtime/src/platform.rs`), which `pty-error-code` maps one
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

<details class="doc-synopsis" open><summary>Synopsis (13 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Pty">Pty</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Pty</strong></code></li><li><a class="syn-name" href="#t-PtyCommand">PtyCommand</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PtyCommand</strong></code></li><li><a class="syn-name" href="#t-PtyError">PtyError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PtyError</strong></code></li><li><a class="syn-name" href="#v-pty-close">pty-close</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pty-close</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a> <span class="tok-keyword">%1</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-pty-command">pty-command</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pty-command</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyCommand" title="Effect.Pty.PtyCommand">PtyCommand</a></code></li><li><a class="syn-name" href="#v-pty-command-with-env">pty-command-with-env</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pty-command-with-env</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyCommand" title="Effect.Pty.PtyCommand">PtyCommand</a></code></li><li><a class="syn-name" href="#v-pty-handle">pty-handle</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pty-handle</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-pty-open">pty-open</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pty-open</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a></code></li><li><a class="syn-name" href="#v-pty-read">pty-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pty-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a> <span class="tok-keyword">%1</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-pty-resize">pty-resize</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pty-resize</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a> <span class="tok-keyword">%1</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a>)</code></li><li><a class="syn-name" href="#v-pty-spawn">pty-spawn</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pty-spawn</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a> <span class="tok-keyword">%1</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyCommand" title="Effect.Pty.PtyCommand">PtyCommand</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a></code></li><li><a class="syn-name" href="#v-pty-wait">pty-wait</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pty-wait</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a> <span class="tok-keyword">%1</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-pty-write">pty-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pty-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a> <span class="tok-keyword">%1</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Pty">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Pty</strong></code><span class="doc-sig-links"><a href="#t-Pty" title="Permalink">#</a><a href="/packages/stdlib/Effect.Pty/source/#L143" title="View source at line 143">Source</a></span></div>
<div class="doc-body">

A live pseudo-terminal, as the runtime's handle word. The handle
owns the master end and the child; the slave end belongs to the
child once a spawn consumed it. The operations below take it at
`%1`, so a live terminal is a value threaded by hand rather than a
reference copied around. The handle stays the runtime's `I64` word:
it only ever goes back to the runtime.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Pty"><code><strong class="tok-constructor">Pty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PtyCommand">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PtyCommand</strong></code><span class="doc-sig-links"><a href="#t-PtyCommand" title="Permalink">#</a><a href="/packages/stdlib/Effect.Pty/source/#L159" title="View source at line 159">Source</a></span></div>
<div class="doc-body">

A program with its arguments and its environment additions
(`KEY=VALUE` pairs, added to this process's own environment).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PtyCommand"><code><strong class="tok-constructor">PtyCommand</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyCommand" title="Effect.Pty.PtyCommand">PtyCommand</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PtyError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PtyError</strong></code><span class="doc-sig-links"><a href="#t-PtyError" title="Permalink">#</a><a href="/packages/stdlib/Effect.Pty/source/#L131" title="View source at line 131">Source</a></span></div>
<div class="doc-body">

What went wrong before the child's own exit code could exist. The
constructors correspond one for one to the platform layer's shared
codes.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PtyNotFound"><code><strong class="tok-constructor">PtyNotFound</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a></code></li><li id="k-PtyPermissionDenied"><code><strong class="tok-constructor">PtyPermissionDenied</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a></code></li><li id="k-PtyInvalidArgument"><code><strong class="tok-constructor">PtyInvalidArgument</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a></code></li><li id="k-PtyIoError"><code><strong class="tok-constructor">PtyIoError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-pty-close">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pty-close</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a> <span class="tok-keyword">%1</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-pty-close" title="Permalink">#</a><a href="/packages/stdlib/Effect.Pty/source/#L272" title="View source at line 272">Source</a></span></div>
<div class="doc-body">

Close the terminal: a child no wait reaped is killed and reaped
first, and the handle is consumed. This is the one operation that
does not hand the handle back, which is the whole point: after it
there is no value to read from or to close a second time, so those
programs cannot be written. The named error is the boundary's
answer for a handle word that was never live.

</div>
</article>

<article class="doc-entry" id="v-pty-command">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pty-command</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyCommand" title="Effect.Pty.PtyCommand">PtyCommand</a></code><span class="doc-sig-links"><a href="#v-pty-command" title="Permalink">#</a><a href="/packages/stdlib/Effect.Pty/source/#L163" title="View source at line 163">Source</a></span></div>
<div class="doc-body">

A command without environment additions.

</div>
</article>

<article class="doc-entry" id="v-pty-command-with-env">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pty-command-with-env</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyCommand" title="Effect.Pty.PtyCommand">PtyCommand</a></code><span class="doc-sig-links"><a href="#v-pty-command-with-env" title="Permalink">#</a><a href="/packages/stdlib/Effect.Pty/source/#L167" title="View source at line 167">Source</a></span></div>
<div class="doc-body">

A command with environment additions.

</div>
</article>

<article class="doc-entry" id="v-pty-handle">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pty-handle</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-pty-handle" title="Permalink">#</a><a href="/packages/stdlib/Effect.Pty/source/#L153" title="View source at line 153">Source</a></span></div>
<div class="doc-body">

The handle word of a terminal, for the layers above this one. The
word is not a handle, and the projection is the one hole in the
discipline: a `Pty` minted from a word is a second value for the
same terminal, which the runtime answers with `PtyInvalidArgument`
once the real handle is closed (and a word that was never live
answers the same). The threading's guarantee covers the handles the
operations hand out, not the bare word.

</div>
</article>

<article class="doc-entry" id="v-pty-open">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pty-open</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a></code><span class="doc-sig-links"><a href="#v-pty-open" title="Permalink">#</a><a href="/packages/stdlib/Effect.Pty/source/#L196" title="View source at line 196">Source</a></span></div>
<div class="doc-body">

Open a pseudo-terminal pair: the master and slave ends in a handle,
or the named error. The slave belongs to the handle until a spawn
hands it to the child.

</div>
</article>

<article class="doc-entry" id="v-pty-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pty-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a> <span class="tok-keyword">%1</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-pty-read" title="Permalink">#</a><a href="/packages/stdlib/Effect.Pty/source/#L250" title="View source at line 250">Source</a></span></div>
<div class="doc-body">

Read up to `max` bytes of the terminal's master (what the child
wrote to its stdout or stderr, plus the terminal's echo of the
input): the handle and `Ok` of what one platform call produced, or
the handle and the named error. A failed read leaves the session
live, so the handle is handed back rather than dropped.

</div>
</article>

<article class="doc-entry" id="v-pty-resize">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pty-resize</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a> <span class="tok-keyword">%1</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a>)</code><span class="doc-sig-links"><a href="#v-pty-resize" title="Permalink">#</a><a href="/packages/stdlib/Effect.Pty/source/#L281" title="View source at line 281">Source</a></span></div>
<div class="doc-body">

Set the terminal's window size to `cols` columns by `rows` rows:
the handle and the unit, or the handle and the named error (the
handle comes back for the same reason as `pty-write`'s).

</div>
</article>

<article class="doc-entry" id="v-pty-spawn">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pty-spawn</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a> <span class="tok-keyword">%1</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyCommand" title="Effect.Pty.PtyCommand">PtyCommand</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a></code><span class="doc-sig-links"><a href="#v-pty-spawn" title="Permalink">#</a><a href="/packages/stdlib/Effect.Pty/source/#L212" title="View source at line 212">Source</a></span></div>
<div class="doc-body">

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

</div>
</article>

<article class="doc-entry" id="v-pty-wait">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pty-wait</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a> <span class="tok-keyword">%1</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-pty-wait" title="Permalink">#</a><a href="/packages/stdlib/Effect.Pty/source/#L260" title="View source at line 260">Source</a></span></div>
<div class="doc-body">

Wait for the child: the handle and its exit code (128 + the signal
number for a signal death, the shell convention), or the handle and
the named error. A second wait answers the same code, so waiting is
not what ends the handle's life; `pty-close` is.

</div>
</article>

<article class="doc-entry" id="v-pty-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pty-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a> <span class="tok-keyword">%1</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-Pty" title="Effect.Pty.Pty">Pty</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pty/#t-PtyError" title="Effect.Pty.PtyError">PtyError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-pty-write" title="Permalink">#</a><a href="/packages/stdlib/Effect.Pty/source/#L239" title="View source at line 239">Source</a></span></div>
<div class="doc-body">

Write `bs` to the terminal's master (the child reads it on its
stdin): the handle and the byte count (the whole buffer goes), or
the handle and the named error. The handle comes back because a
failed write is a failed write, not a dead terminal: the caller
keeps the session.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Pty.cinch`](/packages/stdlib/Effect.Pty/source/). Source SHA-256: `5c376b14e53fa6547230922be94c981f19ccd06b8ca360c6effdb38bec68aa48`.
