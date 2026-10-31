# Effect.Signal

Operating-system signal vocabulary and runtime operations with explicit error/effect handling.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Signal`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Signal.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Result](Data.Result.md).

Non-embedded source closure: [Effect.Signal](Effect.Signal.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

The boundary word (`compiler/runtime/src/platform.rs`,
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

# Types and constructors

<a id="type-5369676e616c"></a>

## `Signal`

```cinch
data Signal
```

A signal this module will send. The set is the portable core: the
four signals whose numbers agree on Darwin and Linux plus
`SigNone`, the zero signal (the existence and permission probe the
kernel delivers to nothing).

[Source](/reference-sources/Effect.Signal.cinch) · line 90

```cinch
SigNone :: Signal
```

```cinch
SigHup :: Signal
```

```cinch
SigInt :: Signal
```

```cinch
SigKill :: Signal
```

```cinch
SigTerm :: Signal
```

<a id="type-5369676e616c4572726f72"></a>

## `SignalError`

```cinch
data SignalError
```

Why a delivery did not happen.

[Source](/reference-sources/Effect.Signal.cinch) · line 98

```cinch
SignalNotDelivered :: SignalError
```

`kill(2)` failed: the target is gone or the caller is not
permitted to signal it (the two are one here, see the errno
note above), or another OS failure.

```cinch
SignalInvalidNumber :: SignalError
```

The pid or the signal number fell outside the host's `i32`
range (the runtime's `INVALID_ARGUMENT` sentinel, `-6`).

# Values

<a id="value-7369676e616c2d6e756d626572"></a>

## `signal-number`

```cinch
signal-number :: (Signal -> I64)
```

The host's POSIX number for a signal (the table is the host's;
see the header).

[Source](/reference-sources/Effect.Signal.cinch) · line 109

<a id="value-7369676e616c2d70726f6265"></a>

## `signal-probe`

```cinch
signal-probe :: forall r . using (io :: Io r) => (I64 ->{IO r} Result SignalError Unit)
```

Probe `pid` without delivering anything (the zero signal): `Ok ()`
when the process exists and the caller may signal it, the named
error otherwise. This is the one form that earns its place: it is
how a caller checks the target before a real delivery.

[Source](/reference-sources/Effect.Signal.cinch) · line 141

<a id="value-7369676e616c2d73656e64"></a>

## `signal-send`

```cinch
signal-send :: forall r . using (io :: Io r) => (I64 -> (Signal ->{IO r} Result SignalError Unit))
```

Deliver `sig` to `pid` (pid first, then the signal): the named
error on any failure, `Ok ()` when the kernel accepted it.

[Source](/reference-sources/Effect.Signal.cinch) · line 132

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Signal.cinch`. Source SHA-256: `93778e910173dd3b7d4cf143c34a1611734bb03524199b2bd125d79ac854a598`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
