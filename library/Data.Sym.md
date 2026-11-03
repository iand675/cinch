# Data.Sym

Interned symbols with identity equality, deterministic spelling order, and String conversions.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Sym`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Sym.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Hash](Data.Hash.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Sym: interned symbols.

`Sym` is a core runtime type (primitive types specification section
9): one word, a handle to a process-wide interned name cell. Two
equal names are the same cell, so `Eq` is identity (an O(1) pointer
test) and `Ord` is lexicographic by name (deterministic across runs;
never insertion order). `Show` prints `'foo`. Nothing here exposes a
cell address as data: the foreign boundary takes `sym-name` (or
`Bytes`), and the type has no C counterpart.

Literals are constants first: `'foo` interns at compile time and is
emitted as a constant (the P14 constant-emission path), and `intern`
of an equal name at runtime resolves to the same cell (the runtime
table is keyed by name). Symbol literal patterns test identity.

Module scheme (design/base-library.md): the type's home carries its
instance family (the `Eq`, `Ord` and `Show` instances over the
runtime cells).

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-696e7465726e"></a>

## `intern`

```cinch
intern :: (String -> Sym)
```

The prelude surface of section 9.

[Source](/reference-sources/Data.Sym.cinch) · line 39

<a id="value-73796d2d6e616d65"></a>

## `sym-name`

```cinch
sym-name :: (Sym -> String)
```

[Source](/reference-sources/Data.Sym.cinch) · line 42

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Sym.Eq_Sym` | `Eq Sym` | Implicit |
| `Data.Sym.Hash_Sym` | `Hash Sym` | Implicit |
| `Data.Sym.Ord_Sym` | `Ord Sym` | Implicit |
| `Data.Sym.Show_Sym` | `Show Sym` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Sym.cinch`. Source SHA-256: `60ba339671cd9fdad4d749571825ac19400f401c1a63e4ef9b0672f2cbd2ba73`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
