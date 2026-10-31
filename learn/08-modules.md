# 8. Split a program into modules

A module gives declarations an identity and controls what other modules can use. Start with a small public API. Keeping implementation helpers private lets you change them without asking every caller to change.

# A two-module project

Save the following as `Pricing.cinch` beside your entry file:

<!-- cinch:project group=pricing file=Pricing.cinch -->
```cinch
module Pricing where
  export subtotal

  subtotal :: Nat -> Nat -> Nat
  subtotal price count = price * count

  internal-fee :: Nat
  internal-fee = 2
```

Then save `Main.cinch`:

<!-- cinch:project group=pricing file=Main.cinch -->
```cinch
module Main where
  import Pricing

  main :: Int
  main = if Pricing.subtotal 7 3 == 21 then 0 else 1
```

Run `cinch check Main.cinch`, then `cinch run Main.cinch`; success exits 0. The compiler loads the imported module. It does not require you to concatenate files or pass every file as a separate CLI argument. This exact multi-file project is included in the documentation example checks.

`export subtotal` is an explicit public surface. Without any export declaration, a module exports its declared names. Imported names remain private unless re-exported. With the explicit declaration above, `internal-fee` is private. A library facade can re-export selected names or use `export module Other` for an imported module's surface.

# Name lookup and collisions

`Pricing.subtotal` is qualified. `import Pricing as P` gives the alias `P.subtotal`. A selected import can rename an individual binding: `import Pricing (subtotal as cost)`. If two imported modules expose the same unqualified name, choose an alias or selection instead of relying on import order.

Imports resolve by the declared module name. The loader uses loaded modules, a matching sibling source file, and a scan of sibling files for a uniquely matching declaration. Use a predictable file layout even though the declaration has the final say. Cyclic runtime imports are rejected; separate shared data or signatures to break a cycle. See [modules and packages](../reference/07-modules-and-packages.md) for the precise rules.

# The implicit Prelude and explicit libraries

Every program sees a curated `Prelude`. An operation absent from that facade may still be available from its defining module. For example, import `Text.Show` for the `Show` class and `Data.HashMap` for hash maps.

Some library modules ship as source and are not embedded in the compiler. To use one, place its source and its non-embedded dependencies beside the entry file, using the copying instructions in the [library guide](../library/00-index.md). The module directory tells you which kind each module is. An `import` alone does not fetch a package from the internet.

# Exercise and answer

Expose a second function, `with-fee`, without exposing `internal-fee`.

<details><summary>Answer</summary>

Add `with-fee` to `export subtotal, with-fee`, declare `with-fee :: Nat -> Nat`, and define `with-fee amount = amount + internal-fee`. Clients may call the function but cannot refer to the private binding. A private helper may still be used in a public function's implementation.

</details>
