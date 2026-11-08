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

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">intern</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Sym</span>)</code></pre>

The prelude surface of section 9.

[Source](/reference-sources/Data.Sym.cinch) · line 39

<a id="value-73796d2d6e616d65"></a>

## `sym-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sym-name</span> :: (<span class="tok-constructor">Sym</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Data.Sym.cinch)

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Sym.Eq_Sym` | `Relation.Eq.Eq Sym` | Implicit |
| `Data.Sym.Hash_Sym` | `Data.Hash.Hash Sym` | Implicit |
| `Data.Sym.Ord_Sym` | `Relation.Ord.Ord Sym` | Implicit |
| `Data.Sym.Show_Sym` | `Text.Show.Show Sym` | Implicit |

**`Data.Sym.Eq_Sym`**: `Eq` is identity: the same interned cell (section 9).
**`Data.Sym.Hash_Sym`**: The digest is the interned name's bytes through the `Bytes`
instance (Data.Bytes owns the fold), read straight off the
intrinsic layer with no `String` in between. Interning keys on the
name, so two names equal under `Eq` are the same cell and hash
alike (the `Hash` law), and the digest never reads a cell address,
so it is deterministic across runs exactly as `Ord` is.

The cell's own word would be the stronger answer (identity is
already the `Eq`), but nothing here exposes a handle as data: the
type has no C counterpart and no `Sym`-to-word conversion exists,
and inventing one to hash by address would buy only the weaker
property (addresses are not reproducible across runs).
`symName` hands out a fresh buffer per call, so a hash allocates
once.
**`Data.Sym.Ord_Sym`**: `Ord` is lexicographic by name: `symCmp`'s immediate against 0
(the `natCmp`/`intCmp` reading), never the cell addresses.
**`Data.Sym.Show_Sym`**: `Show` prints `'foo` (section 9): the tick is `char-string '\''`
(raw ASCII 39), the name renders as itself.

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Sym.cinch`. Source SHA-256: `60ba339671cd9fdad4d749571825ac19400f401c1a63e4ef9b0672f2cbd2ba73`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
