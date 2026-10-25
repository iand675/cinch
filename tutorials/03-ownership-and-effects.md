# Tutorial 3 Ownership and effects

Source: <https://chatgpt.com/space/page_564c957a4e5c8191b7166e65d755285c>  
Exported: 2026-10-22T18:59:36Z

---

[Documentation overview](../overview.md)

Learn how ownership protects both memory and facts about mutable state. Prerequisite: Tutorials 1 and 2. Borrow and resource syntax is provisional.

# Borrow instead of duplicate

```haskell
swap :: forall a n region. Mut region (Vec n a)
     -> Fin n -> Fin n ->{State region} ()
swap xs i j = Vec.swap xs i j
```

`Vec.swap` exchanges elements without copying or discarding them. The exclusive borrow prevents incompatible aliases during mutation. The vector length stays `n`, so the index evidence remains valid. The library operation must justify the equal-index case; a caller cannot create two mutable references to the same element by pretending the indices differ.

# Mutable refinements have a lifetime

Consider the following intended rejection:

```haskell
-- Rejected: evidence describes the old buffer view.
do
  let evidence = decide (i < length buffer)
  truncate buffer 0
  getUsingOldProof buffer i evidence
```

The evidence refers to the earlier state. Either the borrow rules prevent truncation while a dependent view is live, or the state transition invalidates the fact. A proof about yesterday's length is not a present bounds guarantee.

# Effects follow higher-order code

```haskell
map :: forall a b e. (a ->{|e} b) -> List a ->{|e} List b
```

Mapping a pure callback stays pure. Mapping a callback that reads files carries I/O. The operation preserves its specified callback order; effect polymorphism does not imply that callbacks commute.

A capability such as `FileReadCap` determines which reads are permitted. The `IO` effect only says that I/O may happen. Pure code cannot access an ambient network client without an effectful interface and the necessary authority.

# Acquire a resource in a scope

```haskell
loadText :: forall region. Ref region FileReadCap -> Path
         ->{IO, Diverge} Result ReadError String
loadText cap path = withFile cap path (\file -> File.readText file)
```

The proposed `withFile` combinator owns the handle and lends it to the callback. Its library contract defines cleanup on return, read failure, and supported cancellation paths. A process crash or a failed operating-system close needs the separately stated runtime policy.

Expected rejection: return a closure that captures `file` and can be called after `withFile` exits. The closure's borrow would outlive the handle.

# Exercise and answer

Can a multi-shot effect handler resume a continuation twice if it captured an owned transaction? Not without a valid duplication protocol. The ordinary continuation must be one-shot. Copyable captures permit more freedom; linear captures do not.

Next: Tutorial 4 packages operations and their laws into reusable modules.

[Previous tutorial](02-proofs-and-runtime-checks.md) · [Next tutorial](04-modules-and-laws.md) · [Documentation overview](../overview.md)