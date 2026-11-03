# Effect.Clock

Runtime monotonic and wall-clock readings under an I/O capability. Clocks are not ambient compile-time inputs.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Effect.Clock`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Clock.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Clock: the runtime clocks, durations and timeouts (P34).

The clock is a runtime capability in the `Io r` mold (decision D45's
determinism rule): every reading requires the `Io r` capability and
performs `IO r`, so nothing ambient exists and a program's build
output cannot depend on when it compiles. Staging use is refused
outright rather than recorded: a `stage` body's row is pure and the
compile-time interpreter refuses foreign calls ("a foreign import
cannot run at compile time"), so no clock reading can enter a build
product (the env/capability precedent of example 05 records
declared inputs; the clock records nothing because it cannot
participate).

Durations are milliseconds (`Duration`), the unit the `Io r`
suspension (`ioSleep`) takes. `timeout` runs a delayed computation
under the P12 structured machinery and cancels it at the deadline:
the computation is watched by a sibling task whose failure at the
deadline cancels it (the `joinAllOrCancel` failure policy), and its
result is discarded if cancellation was requested during the run.
A value is returned only when the computation ran without observing
a cancellation request; at the deadline either outcome is allowed.
Cancellation is cooperative (concurrency specification section 4):
the computation is not killed, it observes the request at its next
suspension point.

Named refusals (boundaries this module deliberately does not
cross):
  compile-time readings: see above (D45; there is no staging path);
  pre-epoch or far-future wall times: `cinch_clock_wall_ns` clamps
    to `0..=i64::MAX` nanoseconds (the Unix epoch to the year 2262);
  killing a timed-out computation: P12 cancellation is cooperative;
  a work row wider than `{IO r, Cancel r}`: the worked shape is the
    delayed computation (sleeps and observes cancellation); wider
    rows need the checker's row-variable unification to carry
    labels through a rigid tail, which it currently drops (see the
    report), so `timeout` states its worked row exactly.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary.

</details>

# Types and constructors

<a id="type-4475726174696f6e"></a>

## `Duration`

```cinch
data Duration
```

A length of time in milliseconds.

[Source](/reference-sources/Effect.Clock.cinch) · line 52

```cinch
Duration :: (Nat -> Duration)
```

A length of time in milliseconds.

<a id="type-54696d656f7574"></a>

## `Timeout`

```cinch
data Timeout (a :: Type)
```

The outcome of a computation raced against its deadline: it
finished (`Finished x`), or the deadline fired first and cancelled
it (`TimedOut`).

[Source](/reference-sources/Effect.Clock.cinch) · line 88

```cinch
TimedOut :: forall a . Timeout a
```

```cinch
Finished :: forall a . (a -> Timeout a)
```

# Values

<a id="value-636c6f636b2d6d6f6e6f746f6e69632d6e73"></a>

## `clock-monotonic-ns`

```cinch
clock-monotonic-ns :: forall r . using (io :: Io r) => (Unit ->{IO r} Nat)
```

Nanoseconds since an arbitrary origin: non-decreasing across
calls (two readings never go backwards).

[Source](/reference-sources/Effect.Clock.cinch) · line 78

<a id="value-636c6f636b2d77616c6c2d6e73"></a>

## `clock-wall-ns`

```cinch
clock-wall-ns :: forall r . using (io :: Io r) => (Unit ->{IO r} Nat)
```

Nanoseconds since the Unix epoch (the clamped range above).

[Source](/reference-sources/Effect.Clock.cinch) · line 82

<a id="value-6475726174696f6e2d66726f6d2d6d696c6c6973"></a>

## `duration-from-millis`

```cinch
duration-from-millis :: (Nat -> Duration)
```

[Source](/reference-sources/Effect.Clock.cinch) · line 55

<a id="value-6475726174696f6e2d6d696c6c6973"></a>

## `duration-millis`

```cinch
duration-millis :: (Duration -> Nat)
```

[Source](/reference-sources/Effect.Clock.cinch) · line 58

<a id="value-6d6f6e6f746f6e69632d656c6170736564"></a>

## `monotonic-elapsed`

```cinch
monotonic-elapsed :: (Nat -> (Nat -> Duration))
```

The time between two monotonic readings (the first earlier):
milliseconds, truncated. A duration is never negative, so readings
passed in the other order measure zero.

[Source](/reference-sources/Effect.Clock.cinch) · line 65

<a id="value-74696d656f7574"></a>

## `timeout`

```cinch
timeout :: forall a r . using (io :: Io r) => (Duration -> ((Unit ->{IO r, Cancel r} a) %1 ->{Async r, IO r, Cancel r, Diverge} Timeout a))
```

Run `work` under the deadline `d`: `Finished` its value when it
ran first, `TimedOut` when the deadline fired first and cancelled
it (the worked shape and its boundary above).

[Source](/reference-sources/Effect.Clock.cinch) · line 124

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Clock.cinch`. Source SHA-256: `1e4049e0bbd8d7f17d3955669904abb1916ab8f93a3cee2dcb3fa26826357b62`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
