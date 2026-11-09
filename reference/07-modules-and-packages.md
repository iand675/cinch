# Modules, signatures, instances, and packages

A module is a unit of naming, not a file. One file may declare several modules, a module may be declared in a file whose name differs from it, and a project may put several files beside its entry. What the compiler guarantees is that every imported name resolves to exactly one declaration, and that the record of that decision is a checked interface.

This page covers the declaration and resolution rules you type. The artifact the compiler writes for each module is on [artifacts and incremental compilation](18-artifacts-and-incremental.md); installing packages is on [packages, manifests, and the registry](29-package-management.md).

# Module headers

A module starts with `module NAME where` and its declarations form the indented block that follows. Names may contain dots:

```text
module Data.Core where
  export List, Nil, Cons
```

Every module declared in the file you compile belongs to that compile, and the program entry is the module that declares `main`. A file with no `module` declaration is refused before anything else:

```text
Main.cinch: no `module` declaration
```

Several modules in one file are legal and are how a generator is kept beside its consumer:

<!-- cinch:run exit=0 -->
```cinch
module Macros where
  macro double-it :: Syntax -> Declarations
  macro double-it arg = quoteDecls where
    doubled :: Int
    doubled = 2 * $(arg)

module Main where
  import stage Macros

  generate do
    emit (double-it!(21))

  main :: Int
  main = if doubled == 42 then 0 else 1
```

## Module parameters

A module may take parameters in its header. A signature-typed parameter is a dictionary the instantiating import supplies; a value-typed parameter is an ordinary binding:

```text
module Tally (S :: Stepper) where
  import Api

  bump :: Int -> Int
  bump x = step x
```

Instantiation happens at the import, and the argument is written in the same position as an imported member:

<!-- cinch:project group=modules-params file=Main.cinch exit=0 stdout="20" -->
```cinch
module Main where
  import Api
  import Tally (StepUp) as Up
  import Tally (StepDown) as Down

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let written = ioWrite (string-to-bytes (string-concat (int-to-string (Up.bump 1)) (int-to-string (Down.bump 1))))
    in 0
```

<!-- cinch:project group=modules-params file=Api.cinch -->
```cinch
module Api where
  signature Stepper where
    step :: Int -> Int

  instance StepUp :: Stepper where
    step x = x + 1

  instance StepDown :: Stepper where
    step x = x - 1
```

<!-- cinch:project group=modules-params file=Tally.cinch -->
```cinch
module Tally (S :: Stepper) where
  import Api

  bump :: Int -> Int
  bump x = step x
```

`Up.bump 1` and `Down.bump 1` are two copies of one module, each closed over a different dictionary. A value parameter works the same way: `module AddK (k :: Int) where` instantiates with `import AddK (1) as ByLit` or with any name in scope, `import AddK (base) as ByName`. The parameter and its argument are recorded in the interface: `Tally.cinchif` carries `"params": ["S :: Api.Stepper"]`.

# Import forms

| Form | Effect |
| --- | --- |
| `import Mod` | Brings `Mod` into the project and makes its exported names available unqualified |
| `import Mod (a, b)` | Brings `Mod` in and makes only `a` and `b` available unqualified |
| `import Mod (a as b)` | Same, renamed at the use site |
| `import Mod as M` | Available qualified only, as `M.a` |
| `import stage Mod` | Compile-time dependency; see [staging](14-staging-and-macros.md) |
| `import data "path" as name` | Binds a converted non-source input |

An import statement always loads the module. What a selection list changes is which of the module's names become unqualified in the importer, and whether the module's implicit instances are reachable (see below).

<!-- cinch:project group=modules-two file=Main.cinch exit=0 stdout="hello, world" -->
```cinch
module Main where
  import Greeting (greet)

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let written = ioWrite (string-to-bytes (greet "world"))
    in 0
```

<!-- cinch:project group=modules-two file=Greeting.cinch -->
```cinch
module Greeting where
  export greet

  greet :: String -> String
  greet name = string-concat "hello, " name
```

The qualified and renaming forms are ordinary:

<!-- cinch:project group=modules-qualified file=Main.cinch exit=0 -->
```cinch
module Main where
  import Greeting as G

  main :: Int
  main = if G.greet "x" == "hello, x" then 0 else 1
```

<!-- cinch:project group=modules-qualified file=Greeting.cinch -->
```cinch
module Greeting where
  export greet

  greet :: String -> String
  greet name = string-concat "hello, " name
```

<!-- cinch:project group=modules-rename file=Main.cinch exit=0 -->
```cinch
module Main where
  import Greeting (greet as hail)

  main :: Int
  main = if hail "x" == "hello, x" then 0 else 1
```

<!-- cinch:project group=modules-rename file=Greeting.cinch -->
```cinch
module Greeting where
  export greet

  greet :: String -> String
  greet name = string-concat "hello, " name
```

## How `import Mod` finds a module

The loader tries three sources in order, and the first hit wins:

1. A module already loaded, including one declared earlier in the same file.
2. A file named `Mod.cinch` beside the entry.
3. A unique `module Mod` declaration in any `.cinch` file under the entry's directory, searched recursively. `.git`, `target`, `build`, and `node_modules` are skipped.

Step 3 is what makes a `vendor/` tree importable, and it reads the entry path's directory. Compile `Main.cinch` and the directory to search is empty, so a module declared in a differently named file is invisible; compile `./Main.cinch` or an absolute path and the search runs. This is the decision D42 rule in `driver.rs` and the one resolution behaviour worth remembering, because the failure looks like a missing file.

Missing and ambiguous modules are separate diagnostics:

```text
cinch: module Thing is imported, but Thing.cinch does not exist and no other source file beside the entry declares `module Thing`
cinch: module Thing is imported, but several source files declare it (./B.cinch, ./A.cinch); imports resolve to a unique module
```

The paths in the second diagnostic are the loader's own joined paths, which is why they carry the entry's directory prefix.

# Export forms

A module without an `export` declaration exports the declarations it defines. An `export` declaration replaces that with exactly the listed surface.

| Item | Meaning |
| --- | --- |
| `export name` | Export a declaration of this module or an imported name |
| `export A.name` | Export one member of an imported module (exactly two segments) |
| `export module A` | Re-export an imported module's surface |

An import is private unless re-exported. Both re-export forms can appear together:

<!-- cinch:project group=modules-reexport file=Main.cinch exit=0 -->
```cinch
module Main where
  import Facade (value, other)

  main :: Int
  main = if value + other == 16 then 0 else 1
```

<!-- cinch:project group=modules-reexport file=Facade.cinch -->
```cinch
module Facade where
  import Inner

  export module Inner
  export Inner.other
```

<!-- cinch:project group=modules-reexport file=Inner.cinch -->
```cinch
module Inner where
  export value, other

  value :: Int
  value = 7

  other :: Int
  other = 9
```

There is no renaming form in an `export` item. `export Inner.other as renamed` is rejected by the parser with `expected `;` or `}` in the declaration block, found As`.

An export list naming something that is not in scope is a diagnostic at parse of the block:

```text
cinch: 2:10: `nothing-here` is not a name in scope to export (in Bad.cinch)
```

# Diagnostics for broken surfaces

Four failures account for most module-level mistakes. Each is real compiler output.

A name that exists but is not exported is reported by the importer, not the definer. Given `Lib` exporting only `public-one` and a `Main` that imports `helper`:

```text
cinch: `Lib` has no member `helper` to import (as `helper`) (in Main.cinch)
```

Requesting a member that does not exist produces the same shape:

<!-- cinch:reject contains="has no member `farewell` to import" -->
```cinch
module Greeting where
  export greet

  greet :: String -> String
  greet name = string-concat "hello, " name

module Main where
  import Greeting (greet, farewell)

  main :: Int
  main = if greet "x" == farewell "x" then 0 else 1
```

Two modules exporting one name, both imported unqualified, is not resolved by import order:

<!-- cinch:reject contains="is imported from both A and B" -->
```cinch
module A where
  export shared
  shared :: Int
  shared = 1

module B where
  export shared
  shared :: Int
  shared = 2

module Main where
  import A (shared)
  import B (shared)
  main :: Int
  main = shared
```

```text
cinch: the name `shared` is imported from both A and B: rename one (a selection like `import B (shared as ...)`) (in R3.cinch)
```

An import cycle is reported against the modules, after the per-module checks:

<!-- cinch:reject contains="interface coherence error: import cycle through A" -->
```cinch
module A where
  import B (b-value)
  export a-value
  a-value :: Int
  a-value = b-value

module B where
  import A (a-value)
  export b-value
  b-value :: Int
  b-value = a-value

module Main where
  import A (a-value)
  main :: Int
  main = a-value
```

```text
cinch: interface coherence error: import cycle through A
```

A repeated signature in one module is reported with its span:

```text
cinch: 4:3: duplicate signature for twice (in Main.cinch)
```

Two files each declaring the same module name reach the ambiguous-module diagnostic above. A module declared twice inside one file is not rejected today; keep one declaration per module name per file.

# Signatures and instances

A `signature` declares a class: the methods, their types, and optionally a default body for a method. An `instance` supplies the dictionary. `implicit instance` makes it usable by default resolution.

```cinch
  signature Label a where
    label :: a -> Nat
    level :: a -> Nat
    level x = label x + label x
```

That block sits inside `module Api where` (shown in full in the verified project below), with `import Data.Nat` and `export Label, label, level` at the top of the module.

An equation beneath a method's signature is that method's default. An instance that omits the method takes the default, checked at the instance's types; an instance that defines it overrides it:

<!-- cinch:project group=modules-default file=Main.cinch exit=0 stdout="10" -->
```cinch
module Main where
  import Api (Label, label, level)
  import Inst (Label_Box, Box)
  import Data.Nat

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let written = ioWrite (string-to-bytes (nat-to-string (level (Box 5))))
    in 0
```

<!-- cinch:project group=modules-default file=Api.cinch -->
```cinch
module Api where
  import Data.Nat

  export Label, label, level

  signature Label a where
    label :: a -> Nat
    level :: a -> Nat
    level x = label x + label x
```

<!-- cinch:project group=modules-default file=Inst.cinch -->
```cinch
module Inst where
  import Api (Label, label, level)
  import Data.Nat

  export Label_Box, Box

  data Box where
    Box :: Nat -> Box

  implicit instance Label_Box :: Label Box where
    label b = case b of
      Box n -> n
```

`level` is never defined at `Box`; the instance supplies `label` and the class default supplies `level`, so `level (Box 5)` is 10.

## Resolution rules

A default body is checked once at the class, before any instance takes it, so a broken default is reported against the class. A default must also be expressible in the closed first-order body tier the interface can carry; a default that calls an allocating function is refused:

```text
cinch: 9:5: class default `shout3` uses unsupported interface syntax (in Api3.cinch)
```

Omitting a method that has no default is an error at the instance:

```text
cinch: 10:3: instance Twice_Box2 does not define method twice2 (in Inst2.cinch)
```

Implicit instances are collected over the loaded import closure, not only over the importer's own selections. Importing a module with any selection makes its exported implicit instances reachable, and so does importing a module that imports it. Using a method with no reachable instance names the class and the target:

```text
cinch: 5:13: no instance for Tone.Tone Tone.Shade (in Main.cinch)
```

Two reachable implicit instances for the same class and target are ambiguous, and the diagnostic is at the use, not at either declaration:

```text
cinch: 7:13: ambiguous instances for Tone.Tone Tone.Shade (in Main.cinch)
```

That is why several implementations of one class are written as plain `instance`, which resolution never selects by accident, and chosen explicitly by name at a module argument (`import Tally (StepUp) as Up`).

There are no superclasses. A body that needs two classes states both constraints, and a `using` binder makes the requirement explicit:

```cinch
explicit :: forall a. using (d :: Label a) => a -> Nat
explicit x = level x
```

The binder is discharged automatically when an instance is reachable; `explicit (Box 5)` typechecks and evaluates through `Label_Box`. It is not a way to pass a dictionary at a call site: `explicit @{d = Label_Box}` is refused with `explicit has no capability parameter d`, because `@{...}` supplies capabilities, not dictionaries.

# Data and stage imports

`import stage Other` records a compile-time dependency on `Other`. The imported module is compiled before the importer, and its macros and generators run during that stage. `CINCH_STAGE_MODE` selects how external reads are observed: `refresh` (default) executes them and records the snapshots, `replay` uses the recorded ones, `live` performs current reads. A replay with no captured input is a build diagnostic.

`import data "path" as name` converts a non-source input at check time. Three converters are built in and are selected by extension: text, bytes, and JSON.

```cinch
  import data "config.json" as config
  import data "note.txt" as note
  import data "blob.bytes" as blob
```

A JSON input may decode directly into a declared `record` type. The converter, the file, and the content hash are recorded in the build manifest, never the file's contents:

```text
"data_imports": {"bindings": [{"converter": "txt", "file": "note.txt", "hash": "11831194018420276491", "module": "Main", "name": "note"}]}
```

The hash is a 64-bit FNV-1a digest of the input bytes, rendered as a decimal string. Changing the file changes the recorded hash and the artifact identity; the file itself stays out of the manifest.

# What is not implemented

<span class="support-tag support-tag--planned">Coming soon</span> **general parameterized-module composition.** The admitted forms are the ones exercised above: a signature-typed parameter and a value-typed parameter. Parameterized instances as module arguments, nested unit instantiation, and index references through parameterized functions are named refusals rather than inference features. `Io` capabilities stay lexical and `Copy` is structural; neither is an arbitrary module parameter.

<span class="support-tag support-tag--planned">Coming soon</span> **richer dependent module interfaces.** The [modules specification](../specifications/modules-and-compilation-specification.md) section 1 requires modules that package abstract and concrete types, associated type families, semantic models, laws, and staged generators, with later fields depending on earlier ones. What runs today is signatures, dictionaries, and the two parameter kinds above.

This schematic fragment from the specification is not accepted by the compiler; it is a design target, not syntax you can write:

```text
signature Filtering (c :: Type -> Type) where
  type Output :: forall a. (a -> Bool) -> c a -> Type
  filter :: forall a. (p :: a -> Bool) -> (xs :: c a) -> Output p xs
```

Signature blocks do admit `opaque type` and `record` members today, which the parser splices ahead of the signature so method types can name them. Exact applicative and generative functor identity rules remain open design decisions, so no syntax or type-equality rule for them is presented here as settled.

# Packages

A package is a manifest plus module sources. The embedded base library ships as `compiler/prelude/cinch-base.cinchpkg`, a manifest naming its modules and their check order; a project package is a `.cinchpkg` beside its sources, installed into `vendor/` by `cinch pkg`. Manifests, the `pkg` subcommands, the local store, and the registry protocol are on [packages, manifests, and the registry](29-package-management.md). The implicit `Prelude` is a facade that re-exports curated surfaces from the defining homes, not every module in the repository; a source module beside the entry shadows the embedded copy of the same name.

Installation is a separate step from compilation. `cinch pkg install` writes `vendor/` and a lock; `cinch check`, `build`, and `run` then resolve imports against the resulting source tree through rule 3 above.

Implementation: `driver.rs` (loader and `Package::parse`), `check_decls.rs` (imports, exports, instance resolution), `interface.rs` (coherence and the import-cycle check); suites: `modules.rs`, `params.rs`, `surface.rs`, `class_defaults.rs`, `fixity_interfaces.rs`, `package_manifest.rs`.
