# 8. Split a program into modules

A module gives a group of declarations a name and decides which of them other modules may use. Splitting a program is how you keep an implementation detail changeable, and the compiler's job is to enforce the boundary you wrote. This chapter builds a three-file project, names the export and import forms, states where the compiler looks for an imported module, and quotes the diagnostics for a missing module, a private name, and a name imported from two places. Work in one directory with `Main.cinch` as the entry file:

```sh
cinch check Main.cinch
cinch run Main.cinch
```

Build the compiler and put it on `PATH` with [using the compiler](../reference/01-using-the-compiler.md). `check` loads every imported module and typechecks; `run` builds and executes.

**Prerequisites.** [Chapter 3](03-functions.md) for signatures and application, and [chapter 5](05-recursion-and-collections.md) for `List` and the exit-status convention that `main :: Int` uses.

# A module is a name and a surface

A file starts with `module Name where`, and the declarations below it belong to that module. `Name` is capitalized, and the file may also carry an export declaration that lists the names other modules are allowed to use.

The project has three files in one directory. `Money.cinch` holds a small arithmetic surface:

<!-- cinch:project group=pricing file=Money.cinch -->
```cinch
module Money where
  export zero, add-cents

  zero :: Nat
  zero = 0

  add-cents :: Nat -> Nat -> Nat
  add-cents a b = a + b

  round-up :: Nat -> Nat
  round-up cents = add-cents cents 99
```

`Orders.cinch` imports it and exports one function while keeping a helper private:

<!-- cinch:project group=pricing file=Orders.cinch -->
```cinch
module Orders where
  import Money

  export order-total

  line-total :: Nat -> Nat -> Nat
  line-total price count = add-cents zero (price * count)

  order-total :: List (Nat, Nat) -> Nat
  order-total lines = case lines of
    Nil -> zero
    Cons pair rest -> case pair of
      (price, count) -> add-cents (line-total price count) (order-total rest)
```

`Main.cinch` is the entry file and imports both modules:

<!-- cinch:project group=pricing file=Main.cinch exit=0 -->
```cinch
module Main where
  import Orders
  import Money as M

  main :: Int
  main = if Orders.order-total [(7, 2), (8, 3)] == 38 && M.zero == 0 then 0 else 1
```

`cinch run Main.cinch` exits 0. The compiler loads `Orders.cinch` because `import Orders` resolves to that sibling file, then loads `Money.cinch` for `Orders`'s own import. You do not concatenate the files and you do not list them on the command line: the entry file's imports are the whole project description.

# Export lists

`export zero, add-cents` in `Money` is the public surface. `round-up` is declared in the same module and is not in that list, so no other module can refer to it. `Orders` re-exports only `order-total`, so `line-total` stays private even though `Orders` itself is imported.

Four forms cover the cases:

| Form | Effect |
| --- | --- |
| No `export` declaration | every name the module declares is public |
| `export name, other` | exactly those names are public |
| `export module Other` | the imported module `Other`'s surface is re-exported |
| `export name` plus a private helper | the helper is callable from public definitions and invisible outside |

A private name is still usable inside its own module. `line-total` calls `add-cents` and `zero` from `Money`, and `order-total` calls `line-total`, so privacy controls the surface and not the implementation. That is the point: you can rename `round-up` or delete it without any other module noticing.

# Imports

An import brings a module's surface into scope. Four spellings, all verified against this project:

| Spelling | Names in scope |
| --- | --- |
| `import Money` | `zero`, `add-cents`, and `Money.zero`, `Money.add-cents` |
| `import Money as M` | `M.zero`, `M.add-cents` only |
| `import Money (zero)` | `zero` and `Money.zero`, and nothing else |
| `import Money (zero as nothing)` | the selected name, under a new local spelling |

An unqualified import brings both the bare name and the qualified name, which is why `Main` can call `Orders.order-total` while `Orders` calls `add-cents` bare. An alias replaces the bare names with the aliased qualifier, so `import Money as M` makes only `M.zero` available and a bare `zero` becomes an unknown name.

Two modules that export the same bare name are refused at the import, not at the use:

```text
cinch: the name `value` is imported from both Alpha and Beta: rename one (a selection like `import Beta (value as ...)`) (in Main.cinch)
```

The fix is the one the message names: select the name you want under a different local spelling, as in `import Beta (value as beta-value)`. Import order does not decide the winner, which keeps the meaning of a name readable from the file's imports alone.

# Where the compiler looks for a module

`import Name` resolves in this order:

1. a module declared in a file that is already loaded, including the entry file itself;
2. a file named `Name.cinch` beside the entry file;
3. a unique `.cinch` file beside the entry file, or in a subdirectory of it, that declares `module Name`.

The second rule is the one to design for: name the file after the module, with the module name's spelling and capitalization. It is also the only rule that applies when the entry is given as a bare file name in its own directory, which is how `cinch run Main.cinch` is normally spelled. The third rule is a fallback for a source tree whose file names do not match, and it refuses an ambiguity rather than picking one:

```text
cinch: module Shared is imported, but several source files declare it (Two.cinch, One.cinch); imports resolve to a unique module
```

Two different files may not declare the same module name. The message lists the files it found, with the paths as you wrote them:

```text
cinch: module Shared is imported, but several source files declare it (m8d/Two.cinch, m8d/One.cinch); imports resolve to a unique module
```

The scan skips `.git`, `target`, `build` and `node_modules`.

An import cycle is refused before any typechecking happens, because a module's checked surface has to exist before its importer can be checked:

```text
cinch: interface coherence error: import cycle through A
```

Break a cycle by moving the shared declarations into a third module that both sides import.

# One file, several modules

A file may declare more than one module, which is useful for a quick experiment before the code is worth a file of its own. The second module imports the first by name:

<!-- cinch:run exit=0 -->
```cinch
module Helper where
  export doubled

  doubled :: Nat -> Nat
  doubled n = n * 2

module Main where
  import Helper

  main :: Int
  main = if Helper.doubled 21 == 42 then 0 else 1
```

The rules that apply are the same: `doubled` is exported, `import Helper` makes both `doubled` and `Helper.doubled` available, and the second module sees nothing else from the first. A named module in a separate file remains the unit you should use for anything that outlives the experiment, because the file name is what makes an import findable.

# The implicit Prelude and explicit libraries

Every program sees the curated `Prelude` without importing anything, which is why `Nat`, `List`, `main` and `string-to-bytes` work in a file whose only import is another local module. That facade deliberately does not name everything: `Category.Foldable`'s class methods, `Data.NonZero`'s constructors, `Data.NonEmpty`'s constructor and `Text.Show`'s class all need an explicit import of their home, as chapters 5, 6 and 7 showed. A name that is absent from the facade is not absent from the library.

Some library modules ship as source rather than being embedded in the compiler. For those, the module's source and its non-embedded dependencies have to sit beside the entry file, using the copying instructions in the [library guide](../library/00-index.md). An `import` never reaches the network: it resolves through the three rules above and then, for an embedded name, through the compiler's own copies.

# What the compiler enforces

- **The export list is the surface.** A name that no import can reach is an unknown name at the use, exactly as a misspelling is.
- **A module is declared in one file.** Two files declaring the same module is refused, and the message names both paths.
- **An import resolves to a unique module.** An ambiguous sibling is an error, never a first-match.
- **Cycles are refused.** A module's checked surface exists before its importers are checked, so an import cycle has no order the compiler could use.
- **A bare name has one origin.** Two imports exporting the same bare name conflict until one is selected or aliased.
- **The entry file's type is fixed.** `main :: Int` returns the process exit status, and `main :: Unit ->{IO r} Int` is the effectful form from [chapter 10](10-effects-and-io.md).

# Diagnostics you will meet

The missing-module message names the file it looked for and the declaration it scanned for. It has no line and column, because the import could point at any line of any module:

<!-- cinch:reject contains="module Missing is imported, but Missing.cinch does not exist" -->
```cinch
module Main where
  import Missing

  main :: Int
  main = 0
```

```text
cinch: module Missing is imported, but Missing.cinch does not exist and no other source file beside the entry declares `module Missing`
```

A private name is reported as an unknown name, because from the importing module it does not exist. This file declares both modules so that the example is one file:

<!-- cinch:reject contains="unknown name Pricing.internal-fee" -->
```cinch
module Pricing where
  export subtotal

  subtotal :: Nat -> Nat -> Nat
  subtotal price count = price * count

  internal-fee :: Nat
  internal-fee = 2

module Main where
  import Pricing

  main :: Int
  main = if Pricing.internal-fee == 2 then 0 else 1
```

```text
cinch: 14:13: unknown name Pricing.internal-fee (in Main.cinch)
```

The same position rule applies to a name that was never declared: `cinch: 3:13: unknown name total (in Main.cinch)` from chapter 5 is the same diagnostic, and the fix is an import, an export entry, or a corrected spelling.

# Exercises

### Exercise 1

`Money` exports `zero` and `add-cents`. Add `with-fee`, which adds a two-cent fee, without exposing a new constant.

<details><summary>Answer</summary>

Add the name to the export list and keep the fee private. The file keeps its `module Money where` header; the declarations become:

```cinch
  export zero, add-cents, with-fee

  zero :: Nat
  zero = 0

  add-cents :: Nat -> Nat -> Nat
  add-cents a b = a + b

  fee :: Nat
  fee = 2

  with-fee :: Nat -> Nat
  with-fee amount = add-cents amount fee
```

`fee` is absent from the export list, so `M.fee` is an unknown name outside the module while `with-fee` uses it freely. Clients cannot depend on the size of the fee, so changing it stays local.

</details>

### Exercise 2

`Orders` exports only `order-total`, and a second caller needs the per-line total. Decide between exporting `line-total` and adding a named function, and write the version you chose.

<details><summary>Answer</summary>

Exporting `line-total` publishes a helper whose signature may change once discounts or tax exist. A named operation states the caller's need instead. Keep the `module Orders where` header and the `import Money` line, then use this surface:

```cinch
  export order-total, line-summary

  line-summary :: (Nat, Nat) -> Nat
  line-summary pair = case pair of
    (price, count) -> line-total price count
```

`line-total` stays private, so it can be refactored or removed later without breaking a caller. The same reasoning applies to a facade module that re-exports a curated selection with `export module Other`.

</details>

[Chapter 9](09-signatures-and-instances.md) takes the same boundary one level up: a signature states an interface that several modules can implement, and a module parameter chooses the implementation at the call site.
