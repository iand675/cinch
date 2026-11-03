# Data.Hash

Hashing vocabulary and admitted hash implementations for keys.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Hash`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Hash.cinch) · [How library modules are loaded](00-index.md)

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Hash: the hashing class.

`hash` maps a key to a `Nat` bucket digest. The class law is
consistency with `Eq` (asserted over sample values in
compiler/tests/prelude_split.rs, and over every shipped key type
in compiler/tests/lib_hash_keys.rs):
  x == y implies hash x == hash y
Equality without a common hash is a broken key, so every keyed
structure states both constraints (Data.HashMap does).

`Sym` hashing (design/base-library.md) is identity-based and
trivial: a `Sym` is a word handle to a process-wide interned name
(primitive types specification section 1), so its identity IS its
hash. Data.Sym is now that home; because nothing exposes a cell
handle as data, the instance there hashes the interned name's bytes,
which is the reachable stand-in for the identity and the only digest
that is reproducible across runs.

Instance homes. The ownership rule is "instances live with their
type": the instances live in the carriers' vocabulary modules
(Data.Nat, Data.Int, Data.Bool, Data.Char, Data.String, Data.Bytes,
Data.Sym, Data.Rational). The class needs only builtin types
(`hash :: a -> Nat`), so the class home sits at the bottom of the
module order.

</details>

# Types and constructors

# Signatures and methods

<a id="class-48617368"></a>

## `Hash`

```cinch
signature Hash a
```

[Source](/reference-sources/Data.Hash.cinch) · line 29

<a id="method-486173682e68617368"></a>

## `Hash.hash`

```cinch
hash :: forall a . using (dict :: Hash a) => (a -> Nat)
```

[Source](/reference-sources/Data.Hash.cinch) · line 30

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Hash.cinch`. Source SHA-256: `3e7b1186345ffec5b63ee1bfaf8abe885afd60e693b2c2e984ff71b4c530ec00`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
