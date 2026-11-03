# Effect.Concurrent

Shipped task/worker vocabulary supplementing the compiler-installed structured scope and join operations.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Effect.Concurrent`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Concurrent.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Result](Data.Result.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Concurrent: the structured concurrency worked vocabulary.

The worked vocabulary of the concurrency specification's `loadPair`
sketch (section 2). A `Request` names bytes to read from the
runtime's input source; `fetch` performs the read under the `Io r`
capability and reports a short read as a `LoadError`. `withScope`,
`spawn`, `joinAllOrCancel` and the `Io r` operations (`ioWrite`,
`ioRead`, `ioSleep`, `pollCancel`) are builtin values with
compiler-known schemes (like `newIO`), never foreign imports.

</details>

# Types and constructors

<a id="type-4c6f61644572726f72"></a>

## `LoadError`

```cinch
data LoadError
```

[Source](/reference-sources/Effect.Concurrent.cinch) · line 27

```cinch
LoadError :: (Nat -> LoadError)
```

<a id="type-52657175657374"></a>

## `Request`

```cinch
data Request
```

Structured concurrency (concurrency specification, section 2): the
worked vocabulary of the spec's `loadPair` sketch. A `Request` names
bytes to read from the runtime's input source; `fetch` performs the
read under the `Io r` capability and reports a short read as a
`LoadError`. `withScope`, `spawn`, `joinAllOrCancel` and the `Io r`
operations (`ioWrite`, `ioRead`, `ioSleep`, `pollCancel`) are builtin
values with compiler-known schemes (like `newIO`), never foreign
imports.

[Source](/reference-sources/Effect.Concurrent.cinch) · line 24

```cinch
Request :: (Nat -> Request)
```

Structured concurrency (concurrency specification, section 2): the
worked vocabulary of the spec's `loadPair` sketch. A `Request` names
bytes to read from the runtime's input source; `fetch` performs the
read under the `Io r` capability and reports a short read as a
`LoadError`. `withScope`, `spawn`, `joinAllOrCancel` and the `Io r`
operations (`ioWrite`, `ioRead`, `ioSleep`, `pollCancel`) are builtin
values with compiler-known schemes (like `newIO`), never foreign
imports.

# Values

<a id="value-6665746368"></a>

## `fetch`

```cinch
fetch :: forall r . using (io :: Io r) => (Request ->{IO r, Diverge} Result LoadError Bytes)
```

[Source](/reference-sources/Effect.Concurrent.cinch) · line 30

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Concurrent.cinch`. Source SHA-256: `07b1741a8b937105d3fefff700f6cf6ba415e1f8781e2924c396532bccf5d62a`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
