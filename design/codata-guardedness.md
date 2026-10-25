# Codata and guardedness design

Design note for the SafeLang toolchain, roadmap slice P8 in
`compiler/CONFORMANCE.md`. Companion to the core language specification
(section 9) and the laziness and reclamation specification.

Status: design accepted for implementation.

# 1 Two deferral mechanisms, kept apart

The laziness specification already separates them and this slice keeps the
separation:

| Mechanism | Meaning | Reclamation |
| --- | --- | --- |
| `Later a` | guarded observation: a value available at the next tick | none (logical modality, erased) |
| `Thunk {e} a` | deferred computation with a runtime representation | consumed on force |

`Later` is the productivity guard for `codata`; `Thunk` is the implementation
of `delay`'s runtime behavior. `Stream`'s `tail :: Later (Stream a)` is
guarded, not a memo cell: an infinite stream is not a memo of its elements
(laziness specification, section 7).

# 2 `codata` declarations

```haskell
codata Stream (a :: Type) where
  head :: a
  tail :: Later (Stream a)
```

- A codata type declares observations (fields). Construction is by the
  observed record form `Stream { head = e, tail = delay (\() -> ...) }`
  (the example's spelling), which elaborates to a closure per observation:
  a codata value is a record of delayed observations, boxed like ADTs.
- Observation `xs.head` runs the corresponding closure; `xs.tail` yields the
  `Later` value, which `force` turns into the next `Stream`. The runtime
  representation is the existing closure machinery; no new runtime support.

# 3 `corec` and productivity

```haskell
corec from :: Nat -> Stream Nat
corec from n = Stream { head = n, tail = delay (\() -> from (n + 1)) }
```

`corec` marks a definition whose recursion must be productive: every
recursive call must sit under a `delay` that is placed in a `Later` position
of the result (guarded corecursion). The checker verifies guardness
syntactically in this slice: a `corec` body's recursive calls are allowed
only inside `delay` thunks that flow to a `Later`-typed observation. The
core specification leaves open whether the final discipline is guarded
clocks, sized types, or another sound choice; the bootstrap implements the
syntactic guard check and records the open choice (dependent core kernel
design, section 9).

Unproductive definitions (`corec loop = loop`) are rejected: the guard check
requires a constructor/observation layer per recursion level. This is the
same discipline as the memo black-hole rule (demand cannot re-enter) but
applied to productivity: a stream must produce every finite observation in
finite time.

# 4 Bisimulation

Stream bisimulation is a relation, not definitional equality (core
specification, section 9). This slice provides `bisim` as a `proof`-level
relation over observations (matching heads and recursively related tails)
and does NOT add it to conversion. Rewrites over streams must cite
bisimilarity evidence explicitly.

# 5 Implementation mapping

- `parser.rs`: `codata` currently rejects; parse `record_decl` with the
  `codata` keyword into a distinct declaration kind (P1 left the rejection
  in place).
- `check.rs`: codata types as observation records; `corec` definitions with
  the syntactic guard check; `Later` as a builtin type constructor with
  `delay`/`force` discipline (the laziness forms already exist; `Later`
  elaborates to them at runtime while staying erasable in types).
- `codegen.rs`: codata values as records of closures (the ADT block with
  closure-word fields); observation as a call through the record.
- Streams themselves live in the prelude (`Stream`, `from`, `map`,
  `fold`-style observations), matching example 04.

# 6 Open decisions

- Guarded clocks vs sized types for the final productivity discipline.
- Whether `Later` should be erased entirely at runtime or keep a thunk
  representation (the bootstrap keeps the thunk; erasure is a later pass).
- Bisimulation in rewrite rules and its congruence evidence.
