# Effect.Signal

Operating-system signal vocabulary and runtime operations with explicit error/effect handling.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Signal</code></div><div class="doc-mod-bar-links"><a href="/library/source/Effect.Signal/">View source</a><a href="/library/search/?q=%2BEffect.Signal">Search module</a><a href="/reference-sources/Effect.Signal.cinch" download="Effect.Signal.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Result](Data.Result.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Effect.Signal`: [Effect.Signal](Effect.Signal.md).

```sh
cp seed/prelude/Effect/Signal.cinch ./Effect.Signal.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Effect.Signal: POSIX signal delivery, the safe subset of Ruby's
`Signal` (P34, the platform layer; design/stdlib-parity-ruby.md's
OS/process line: `` `Signal` handling reduced to the safe subset'').

A `Signal` names a signal this module will send; `signal-number`
reads its POSIX number and `signal-send` delivers it to a pid
through the `cinch.runtime.signalSend` intrinsic (POSIX `kill(2)`)
under the `Io r` capability in the mold of the runtime's `Io r`
operations (decision D45: signals are sent at runtime through the
capability, never ambient at compile time; a `stage` body cannot
reach the boundary, see Effect.Clock's header). Every failure is a
NAMED `SignalError` value in a `Result` (the house style,
Effect.Fs's shape), never a fault.

Law shape: `signal-number` is the total map from the named signals
to the host's POSIX numbers, and `signal-send pid s` delivers
exactly `signal-number s` to `pid`, reporting `Ok ()` exactly when
`kill(2)` returned `0`. The probe is the zero-signal case:
`signal-probe pid == signal-send pid SigNone` (a permission and
existence check that delivers nothing).

THE NUMBER TABLE IS THE HOST'S, NOT A PORTABLE ONE: the numbers
`signal-number` returns are the POSIX table of the platform the
runtime was built for (SIGHUP 1, SIGINT 2, SIGKILL 9, SIGTERM 15,
identical on Darwin and Linux). A portable mapping would need a
runtime intrinsic that reports the host's numbers; none exists in
this slice (see the refusals below).

The boundary word (`seed/runtime/src/platform.rs`,
`cinch_signal_send`) takes two machine words (pid then signal
number) and returns `kill(2)`'s raw result: `0` on success, `-1` on
failure, or `-6` (the runtime's `INVALID_ARGUMENT` sentinel) when
the pid or the signal number falls outside the platform's `i32`
range.

THE ERRNO LIMIT: `kill(2)` reports ESRCH (no such process) and EPERM
(not permitted) through `errno`, and the boundary does NOT carry
`errno` back (there is no errno intrinsic in the runtime). The two
cannot be separated from the raw `-1`, so both fold into the one
named variant `SignalNotDelivered`. The follow-up is exact: for
ESRCH and EPERM to be separate variants, the runtime would have to
carry `errno` across the extern (a `cinch_signal_send` that also
surfaces the failing call's errno). No variant this boundary cannot
produce is shipped.

Named refusals (boundaries this module deliberately does not
cross, each with its reason):
  install a trap or a handler: there is no runtime hook for signal
    delivery, and a handler is exactly the point where a safe
    language's safety story would have to reason about arbitrary
    interruption points (delivery can land between any two
    operations and observe half-updated state); the safe subset
    sends signals, it does not receive them;
  signal masks: block/unblock is process-wide receiver-side state
    with the same interruption-point reasoning, and no intrinsic
    exposes it;
  `Signal.trap` blocks: the Ruby block form implies a handler
    callback, refused for the reason above;
  raising signals into the current process as a control flow
    mechanism (Ruby's `Process.kill` to self / `Signal.trap("INT")`
    to unwind): control flow is the effect system's job, and a
    signal delivered into a Cinch process has no safe unwinding
    point (the runtime installs no handler);
  Windows emulation: the boundary is POSIX `kill(2)`; a Windows
    port would need a console-control path (GenerateConsoleCtrlEvent)
    that names a different signal set, and emulating POSIX numbers
    over it would be a lie about the host table;
  `SigUsr1`/`SigUsr2`: their numbers differ by platform (Darwin
    30/31, Linux 10/12) and no runtime intrinsic supplies a
    portable mapping, so a single source spelling would be wrong on
    one platform and silently send the wrong signal (on Darwin,
    signal 10 is SIGBUS); they wait for the host-number intrinsic
    the note above names.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary.

</details>

<details class="doc-synopsis" open><summary>Synopsis (5 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Signal">Signal</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Signal</strong></code></li><li><a class="syn-name" href="#t-SignalError">SignalError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SignalError</strong></code></li><li><a class="syn-name" href="#v-signal-number">signal-number</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">signal-number</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Signal/#t-Signal" title="Effect.Signal.Signal">Signal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-signal-probe">signal-probe</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">signal-probe</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Signal/#t-SignalError" title="Effect.Signal.SignalError">SignalError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-signal-send">signal-send</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">signal-send</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Signal/#t-Signal" title="Effect.Signal.Signal">Signal</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Signal/#t-SignalError" title="Effect.Signal.SignalError">SignalError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Signal">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Signal</strong></code><span class="doc-sig-links"><a href="#t-Signal" title="Permalink">#</a><a href="/library/source/Effect.Signal/#L90" title="View source at line 90">Source</a></span></div>
<div class="doc-body">

A signal this module will send. The set is the portable core: the
four signals whose numbers agree on Darwin and Linux plus
`SigNone`, the zero signal (the existence and permission probe the
kernel delivers to nothing).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SigNone"><code><strong class="tok-constructor">SigNone</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Signal/#t-Signal" title="Effect.Signal.Signal">Signal</a></code></li><li id="k-SigHup"><code><strong class="tok-constructor">SigHup</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Signal/#t-Signal" title="Effect.Signal.Signal">Signal</a></code></li><li id="k-SigInt"><code><strong class="tok-constructor">SigInt</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Signal/#t-Signal" title="Effect.Signal.Signal">Signal</a></code></li><li id="k-SigKill"><code><strong class="tok-constructor">SigKill</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Signal/#t-Signal" title="Effect.Signal.Signal">Signal</a></code></li><li id="k-SigTerm"><code><strong class="tok-constructor">SigTerm</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Signal/#t-Signal" title="Effect.Signal.Signal">Signal</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-SignalError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SignalError</strong></code><span class="doc-sig-links"><a href="#t-SignalError" title="Permalink">#</a><a href="/library/source/Effect.Signal/#L98" title="View source at line 98">Source</a></span></div>
<div class="doc-body">

Why a delivery did not happen.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SignalNotDelivered"><code><strong class="tok-constructor">SignalNotDelivered</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Signal/#t-SignalError" title="Effect.Signal.SignalError">SignalError</a></code></li><li id="k-SignalInvalidNumber"><code><strong class="tok-constructor">SignalInvalidNumber</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Signal/#t-SignalError" title="Effect.Signal.SignalError">SignalError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-signal-number">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">signal-number</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Signal/#t-Signal" title="Effect.Signal.Signal">Signal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-signal-number" title="Permalink">#</a><a href="/library/source/Effect.Signal/#L109" title="View source at line 109">Source</a></span></div>
<div class="doc-body">

The host's POSIX number for a signal (the table is the host's;
see the header).

</div>
</article>

<article class="doc-entry" id="v-signal-probe">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">signal-probe</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Signal/#t-SignalError" title="Effect.Signal.SignalError">SignalError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-signal-probe" title="Permalink">#</a><a href="/library/source/Effect.Signal/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

Probe `pid` without delivering anything (the zero signal): `Ok ()`
when the process exists and the caller may signal it, the named
error otherwise. This is the one form that earns its place: it is
how a caller checks the target before a real delivery.

</div>
</article>

<article class="doc-entry" id="v-signal-send">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">signal-send</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Signal/#t-Signal" title="Effect.Signal.Signal">Signal</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Signal/#t-SignalError" title="Effect.Signal.SignalError">SignalError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-signal-send" title="Permalink">#</a><a href="/library/source/Effect.Signal/#L132" title="View source at line 132">Source</a></span></div>
<div class="doc-body">

Deliver `sig` to `pid` (pid first, then the signal): the named
error on any failure, `Ok ()` when the kernel accepted it.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Signal.cinch`](/library/source/Effect.Signal/). Source SHA-256: `e64e11d90a176c3f9f3dfe07e218ac54b086a1beab27ae54009f98dde9fc2724`.
