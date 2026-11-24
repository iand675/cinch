# Effect.Process

One-shot and live subprocesses, command arguments/environment additions, pipes, exit codes, and this process's arguments.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Process</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Process/source/">View source</a><a href="/packages/search/?q=%2BEffect.Process">Search module</a><a href="/reference-sources/Effect.Process.cinch" download="Effect.Process.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Path](Data.Path.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md).

<details class="doc-notes"><summary>Module notes</summary>

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
`seed/runtime/src/platform.rs`: the one-shot `cinch_proc_*`/
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
runs (the native `@main` of `seed/src/codegen.rs`), and
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

<details class="doc-synopsis" open><summary>Synopsis (15 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Child">Child</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Child</strong></code></li><li><a class="syn-name" href="#t-Command">Command</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Command</strong></code></li><li><a class="syn-name" href="#t-ProcessError">ProcessError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ProcessError</strong></code></li><li><a class="syn-name" href="#t-Stream">Stream</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Stream</strong></code></li><li><a class="syn-name" href="#v-child-close">child-close</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">child-close</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Child" title="Effect.Process.Child">Child</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-child-close-stdin">child-close-stdin</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">child-close-stdin</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Child" title="Effect.Process.Child">Child</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-child-handle">child-handle</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">child-handle</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Child" title="Effect.Process.Child">Child</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-child-read">child-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">child-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Child" title="Effect.Process.Child">Child</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Stream" title="Effect.Process.Stream">Stream</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-child-wait">child-wait</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">child-wait</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Child" title="Effect.Process.Child">Child</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-child-write-stdin">child-write-stdin</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">child-write-stdin</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Child" title="Effect.Process.Child">Child</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-command">command</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">command</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Command" title="Effect.Process.Command">Command</a></code></li><li><a class="syn-name" href="#v-command-with-env">command-with-env</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">command-with-env</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Command" title="Effect.Process.Command">Command</a></code></li><li><a class="syn-name" href="#v-process-args">process-args</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">process-args</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-process-run">process-run</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">process-run</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Command" title="Effect.Process.Command">Command</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-process-spawn">process-spawn</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">process-spawn</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Command" title="Effect.Process.Command">Command</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Child" title="Effect.Process.Child">Child</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Child">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Child</strong></code><span class="doc-sig-links"><a href="#t-Child" title="Permalink">#</a><a href="/packages/stdlib/Effect.Process/source/#L310" title="View source at line 310">Source</a></span></div>
<div class="doc-body">

A live child with three stdio pipes of its own.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Child"><code><strong class="tok-constructor">Child</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Child" title="Effect.Process.Child">Child</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Command">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Command</strong></code><span class="doc-sig-links"><a href="#t-Command" title="Permalink">#</a><a href="/packages/stdlib/Effect.Process/source/#L107" title="View source at line 107">Source</a></span></div>
<div class="doc-body">

A program with its arguments and its environment additions
(`KEY=VALUE` pairs, added to this process's own environment).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Command"><code><strong class="tok-constructor">Command</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Command" title="Effect.Process.Command">Command</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ProcessError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ProcessError</strong></code><span class="doc-sig-links"><a href="#t-ProcessError" title="Permalink">#</a><a href="/packages/stdlib/Effect.Process/source/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

What went wrong before the child's own exit code could exist.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ProcessNotFound"><code><strong class="tok-constructor">ProcessNotFound</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a></code></li><li id="k-ProcessPermissionDenied"><code><strong class="tok-constructor">ProcessPermissionDenied</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a></code></li><li id="k-ProcessInvalidArgument"><code><strong class="tok-constructor">ProcessInvalidArgument</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a></code></li><li id="k-ProcessIoError"><code><strong class="tok-constructor">ProcessIoError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Stream">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Stream</strong></code><span class="doc-sig-links"><a href="#t-Stream" title="Permalink">#</a><a href="/packages/stdlib/Effect.Process/source/#L314" title="View source at line 314">Source</a></span></div>
<div class="doc-body">

Which of the child's two output pipes a read takes.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Stdout"><code><strong class="tok-constructor">Stdout</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Stream" title="Effect.Process.Stream">Stream</a></code></li><li id="k-Stderr"><code><strong class="tok-constructor">Stderr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Stream" title="Effect.Process.Stream">Stream</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-child-close">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">child-close</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Child" title="Effect.Process.Child">Child</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-child-close" title="Permalink">#</a><a href="/packages/stdlib/Effect.Process/source/#L438" title="View source at line 438">Source</a></span></div>
<div class="doc-body">

Close the child: a child no wait reaped is killed and reaped
first, and the handle is gone afterwards (a second close is the
named error).

</div>
</article>

<article class="doc-entry" id="v-child-close-stdin">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">child-close-stdin</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Child" title="Effect.Process.Child">Child</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-child-close-stdin" title="Permalink">#</a><a href="/packages/stdlib/Effect.Process/source/#L406" title="View source at line 406">Source</a></span></div>
<div class="doc-body">

Close the child's stdin, which is how it sees the end of its
input: the unit, or the named error.

</div>
</article>

<article class="doc-entry" id="v-child-handle">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">child-handle</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Child" title="Effect.Process.Child">Child</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-child-handle" title="Permalink">#</a><a href="/packages/stdlib/Effect.Process/source/#L319" title="View source at line 319">Source</a></span></div>
<div class="doc-body">

The child's handle word, for the layers above this one.

</div>
</article>

<article class="doc-entry" id="v-child-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">child-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Child" title="Effect.Process.Child">Child</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Stream" title="Effect.Process.Stream">Stream</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-child-read" title="Permalink">#</a><a href="/packages/stdlib/Effect.Process/source/#L415" title="View source at line 415">Source</a></span></div>
<div class="doc-body">

Read up to `max` bytes of one of the child's output pipes: `Ok` of
what one platform call produced (empty at that pipe's end), or the
named error.

</div>
</article>

<article class="doc-entry" id="v-child-wait">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">child-wait</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Child" title="Effect.Process.Child">Child</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-child-wait" title="Permalink">#</a><a href="/packages/stdlib/Effect.Process/source/#L429" title="View source at line 429">Source</a></span></div>
<div class="doc-body">

Wait for the child: its exit code (128 + the signal number for a
signal death, the shell convention), or the named error.

</div>
</article>

<article class="doc-entry" id="v-child-write-stdin">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">child-write-stdin</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Child" title="Effect.Process.Child">Child</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-child-write-stdin" title="Permalink">#</a><a href="/packages/stdlib/Effect.Process/source/#L398" title="View source at line 398">Source</a></span></div>
<div class="doc-body">

Write `bs` to the child's stdin: the byte count, or the named
error.

</div>
</article>

<article class="doc-entry" id="v-command">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">command</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Command" title="Effect.Process.Command">Command</a></code><span class="doc-sig-links"><a href="#v-command" title="Permalink">#</a><a href="/packages/stdlib/Effect.Process/source/#L111" title="View source at line 111">Source</a></span></div>
<div class="doc-body">

A command without environment additions.

</div>
</article>

<article class="doc-entry" id="v-command-with-env">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">command-with-env</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Command" title="Effect.Process.Command">Command</a></code><span class="doc-sig-links"><a href="#v-command-with-env" title="Permalink">#</a><a href="/packages/stdlib/Effect.Process/source/#L115" title="View source at line 115">Source</a></span></div>
<div class="doc-body">

A command with environment additions.

</div>
</article>

<article class="doc-entry" id="v-process-args">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">process-args</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-process-args" title="Permalink">#</a><a href="/packages/stdlib/Effect.Process/source/#L290" title="View source at line 290">Source</a></span></div>
<div class="doc-body">

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

</div>
</article>

<article class="doc-entry" id="v-process-run">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">process-run</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Command" title="Effect.Process.Command">Command</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-process-run" title="Permalink">#</a><a href="/packages/stdlib/Effect.Process/source/#L180" title="View source at line 180">Source</a></span></div>
<div class="doc-body">

Run `cmd` with `input` on its stdin: the child's exit code and
its captured stdout (raw bytes), or the named error.

</div>
</article>

<article class="doc-entry" id="v-process-spawn">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">process-spawn</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Path/#t-Path" title="Data.Path.Path">Path</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Command" title="Effect.Process.Command">Command</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-ProcessError" title="Effect.Process.ProcessError">ProcessError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Process/#t-Child" title="Effect.Process.Child">Child</a></code><span class="doc-sig-links"><a href="#v-process-spawn" title="Permalink">#</a><a href="/packages/stdlib/Effect.Process/source/#L350" title="View source at line 350">Source</a></span></div>
<div class="doc-body">

Start `cmd` with its three stdio pipes, running in `dir` (`None`
inherits this process's own directory): the live child, or the
named error. The child inherits this process's environment plus
`cmd`'s additions.

</div>
</article>
