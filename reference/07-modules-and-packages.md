# Modules, signatures, instances, and packages

A module is a namespace with a checked interface, not a file. One source file may declare multiple modules, a module's name need not match its filename, and a project may spread modules across any directory layout beside its entry. The compiler enforces two invariants across the module graph: every imported name resolves to a single declaration, and every module boundary is recorded in a verified interface.

Build artifacts and incremental rechecking are covered in [artifacts and incremental compilation](18-artifacts-and-incremental.md); manifests, lockfiles, and registry workflows are in [packages, manifests, and the registry](29-package-management.md).

# Module headers

A module begins with `module NAME where`, followed by an indented block of declarations. Module names may be dot-separated:

```text
module Data.Core where
  export List, Nil, Cons
```

Every module declared in the entry file participates in the compilation, and the executable entry point is the module declaring `main`. A source file without a `module` header is rejected immediately (`Main.cinch: no \`module\` declaration`).

Declaring multiple modules in one file is standard when keeping compile-time macros beside the module that invokes them:

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

A non-entry module may declare parameters in its header. A parameter typed by a `signature` accepts an explicit dictionary at instantiation; a parameter typed by a value type accepts an ordinary value:

```text
module Tally (S :: Stepper) where
  import Api

  bump :: Int -> Int
  bump x = step x
```

Instantiation occurs at the `import` site, passing the argument in parentheses before `as`:

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

`Up` and `Down` are distinct instantiations of `Tally`, each closed over a different `Stepper` dictionary. Value parameters work identically: `module AddK (k :: Int) where` can be instantiated with a literal (`import AddK (1) as ByLit`) or an in-scope binding (`import AddK (base) as ByName`). Parameter signatures are recorded in the module's checked interface. Entry modules declaring `main` cannot take module parameters.

# Import forms

| Form | Effect |
| --- | --- |
| `import Mod` | Loads `Mod` and brings all exported names into scope unqualified. |
| `import Mod (a, b)` | Loads `Mod` and brings only `a` and `b` into scope unqualified. |
| `import Mod (a as b)` | Loads `Mod` and binds `a` locally under the name `b`. |
| `import Mod hiding (a, b)` | Loads `Mod` and brings all exported names into scope except `a` and `b` (excluded both bare and as `Mod.a`). |
| `import Mod as M` | Loads `Mod` for qualified access only (`M.a`). |
| `import stage Mod` | Declares a compile-time dependency; see [staging and macros](14-staging-and-macros.md). |
| `import data "path" as name` | Converts a non-source asset at check time and binds it as `name`. |
| `import Mod when <clause>` | Conditionally includes the import for matching targets; see [conditional imports and declarations](#conditional-imports-and-declarations). |

Every unconditional `import` loads the target module; selection and `hiding` clauses only control which names enter unqualified scope. An import may either select or hide, never both. Hiding a name that `Mod` does not export is an error (`` `Mod` has no member `missing` to hide ``), preventing silent typos. Hiding an operator also excludes its fixity declaration.

Selective, qualified, and renaming imports:

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

## Module resolution

When resolving `import Mod`, the loader searches three sources in order and stops at the first match:

1. A module already loaded in the current compilation, including one declared earlier in the same file.
2. A file named `Mod.cinch` beside the entry file.
3. A unique `module Mod` declaration in any `.cinch` file recursively under the entry file's parent directory (skipping `.git`, `target`, `build`, and `node_modules`).

Step 3 is how vendored packages under `vendor/` and modules whose filenames differ from their module names are discovered. Because recursive search inspects the entry path's parent directory, invoking `cinch check Main.cinch` leaves the search directory empty and skips step 3; pass `./Main.cinch` or a path with an explicit directory component when importing modules declared in differently named files or subdirectories.

Missing and ambiguous modules produce distinct errors:

```text
cinch: module Thing is imported, but Thing.cinch does not exist and no other source file beside the entry declares `module Thing`
cinch: module Thing is imported, but several source files declare it (./B.cinch, ./A.cinch); imports resolve to a unique module
```

# Conditional imports and declarations

Target conditions use `when` clauses in two module-level positions: imports and declarations. Both evaluate against the active `--target` profile using the same atom vocabulary as `prim` bodies (`Available <capability>`, `arch`, `os`, `libc`, `endian`, `word-size`, `target`, and `not (<atom>)`; see [target conditions](15-ffi-layout-and-attributes.md#target-conditions)). Multiple `when` clauses on the same item form a conjunction.

On an import, `when` clauses follow the rest of the import form:

```cinch
import Prim.Sys.Darwin when os darwin
import Prim.Sys.Linux when os linux
import Data.Machine when arch aarch64 when not (os windows)
```

On a declaration, each `when` clause is written as a prefix immediately before the declaration with no separator:

```cinch
when os darwin sys-errno :: I64
when os darwin sys-errno = 2
```

## Inactive items

When a `when` clause evaluates to false for the active target, the annotated item is completely absent:

- An inactive import introduces no names, adds no module dependency, triggers no file lookup, and is omitted from the checked interface. Referencing one of its names fails with an ordinary unknown-name error.
- An inactive declaration is omitted from the module's exports, checked interface, and code generation.

When a clause holds, the item behaves identically to its unconditional form. Checked interfaces record only the active target's surface.

## Conditional re-exports

`export module A` and `export A.name` inherit the condition of the import they reference. If the underlying import is inactive on the current target, the re-export is dropped silently rather than reported as an unimported module. This lets a platform facade expose a uniform surface across targets—the same pattern used by `Prim.Sys` in the standard library:

<!-- cinch:project group=modules-conditional file=Main.cinch exit=0 -->
```cinch
module Main where
  import Platform (word)

  main :: Int
  main = if 0 < word then 0 else 1
```

<!-- cinch:project group=modules-conditional file=Platform.cinch -->
```cinch
module Platform where
  import Darwin when os darwin
  import Linux when os linux

  export module Darwin
  export module Linux
```

<!-- cinch:project group=modules-conditional file=Darwin.cinch -->
```cinch
module Darwin where
  export word

  word :: I64
  word = 2
```

<!-- cinch:project group=modules-conditional file=Linux.cinch -->
```cinch
module Linux where
  export word

  word :: I64
  word = 3
```

## Disjoint conditional definitions

A module may define the same name more than once only if the definitions' `when` clauses are provably disjoint—meaning no target profile can satisfy both. Two clauses are disjoint when they assign conflicting values to the same target component (`os darwin` vs. `os linux`) or negate one another (`os darwin` vs. `not (os darwin)`).

Overlap checking is purely syntactic over the conditions and does not depend on the active `--target`: a module with overlapping conditions (or a conditional definition alongside an unconditional one) is rejected on every target:

```text
cinch: 7:3: two definitions of `word` can apply to one target profile: `os darwin` and `os darwin and arch aarch64` overlap, because the atom `os darwin` holds under both.
```

With disjoint conditions, exactly one definition applies per target:

<!-- cinch:project group=modules-conditional-name file=Main.cinch exit=0 -->
```cinch
module Main where
  import Word (word)

  main :: Int
  main = if 0 < word then 0 else 1
```

<!-- cinch:project group=modules-conditional-name file=Word.cinch -->
```cinch
module Word where
  export word

  when not (os windows) word :: I64
  when not (os windows) word = 2

  when os windows word :: I64
  when os windows word = 3
```

# Export forms

A module with no `export` declaration exports every declaration it defines. Adding one or more `export` declarations replaces that default with exactly the listed items. Imported names are private unless explicitly re-exported.

| Form | Meaning |
| --- | --- |
| `export name` | Exports a local declaration or an unqualified imported name. |
| `export A.name` | Re-exports one member of an imported module (two segments only). |
| `export module A` | Re-exports the entire exported surface of imported module `A`. |

Both re-export forms can be combined in a single facade:

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

`export` items do not support `as` renaming (rename at the `import` site instead). Naming an out-of-scope identifier in an export list fails with `` `<name>` is not a name in scope to export ``.

# Surface and coherence errors

The module checker enforces unambiguous surfaces and an acyclic import graph:

- **Missing or private member.** Importing a name that does not exist or is not exported is reported at the import site:

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

- **Ambiguous unqualified import.** Importing the same unqualified name from two modules is always an error—import order never shadows an earlier import:

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

Resolve the collision by renaming one member (`import B (shared as b-shared)`), qualifying one module (`import B as B`), or hiding the duplicate from a whole-module import:

<!-- cinch:run exit=0 -->
```cinch
module A where
  export shared, other
  shared :: Int
  shared = 1
  other :: Int
  other = 10

module B where
  export shared
  shared :: Int
  shared = 2

module Main where
  import A hiding (shared)
  import B
  main :: Int
  main = if shared + other == 12 then 0 else 1
```

- **Import cycles.** Mutual imports are rejected by interface coherence checking:

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

- **Duplicate declarations.** Repeating a type signature for the same binding in one module reports `duplicate signature for <name>`. Declaring the same module name in two separate files triggers the ambiguous-module error during resolution; keep one declaration per module name per file.

# Signatures and instances

A `signature` declares a type class—its method signatures and optional default implementations. An `instance` defines a named dictionary for a concrete target type; `implicit instance` additionally registers that dictionary for automatic resolution.

An equation immediately beneath a method's signature provides its default body. Instances that omit the method inherit the default, checked at the instance's types; instances that supply an equation override it:

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

Here `Label_Box` defines only `label`; `level (Box 5)` evaluates through the default `label x + label x` to `10`.

## Resolution rules

- **Default method checks.** Default bodies are checked once at the `signature` declaration. Because defaults are serialized into module interfaces, they must stay within the first-order interface expression subset (an allocating call in a default body is rejected with `class default ... uses unsupported interface syntax`). Omitting a method that has no default is rejected at the `instance` declaration.
- **Transitive implicit collection.** Implicit instances are gathered across the entire loaded import closure, not merely from the importer's explicit selection list. Importing a module with any selection—or importing a module that transitively imports it—makes its exported `implicit instance` declarations reachable.
- **Missing and ambiguous instances.** Resolution checks for a unique reachable implicit instance at each use site, reporting either `no instance for <Class> <Type>` or `ambiguous instances for <Class> <Type>`. When multiple implementations of one class exist for the same type, declare them as plain `instance` (never selected implicitly) and pass the desired dictionary explicitly via a module parameter (`import Tally (StepUp) as Up`).
- **No superclasses.** A function requiring multiple classes lists each constraint directly in its `using` clause:

```cinch
explicit :: forall a. using (d :: Label a) => a -> Nat
explicit x = level x
```

`using` constraints for ordinary signatures are discharged by implicit instance resolution; calling `explicit (Box 5)` resolves `Label_Box` automatically. Call-site `@{...}` syntax supplies runtime capabilities (`Io r`), never type-class dictionaries (`explicit @{d = Label_Box}` fails with `explicit has no capability parameter d`).

# Stage and data imports

- **`import stage Other`** introduces a compile-time dependency on `Other`. The compiler checks and builds `Other` first so its macros and generators can execute while compiling the importer. The environment variable `CINCH_STAGE_MODE` governs external reads during staging: `refresh` (default) executes reads and records a snapshot, `replay` reads solely from the recorded snapshot (failing if none exists), and `live` performs unrecorded reads. See [staging and macros](14-staging-and-macros.md).
- **`import data "path" as name`** converts a non-source file at check time and binds the result as `name`. Built-in converters are chosen by file extension: `.txt` (string), `.bytes` (byte buffer), and `.json` (JSON, including direct decoding into a declared `record` type):

```cinch
import data "config.json" as config
import data "note.txt" as note
import data "blob.bytes" as blob
```

The build manifest records the converter, file path, and a 64-bit FNV-1a digest of the file's bytes—never the file contents themselves—so modifying the asset invalidates downstream build artifacts.

# Packages

A package pairs a `.cinchpkg` manifest with a tree of module sources. The standard library ships embedded as `cinch-base`; third-party and workspace packages are installed into `vendor/` with `cinch pkg install`, which writes a lockfile and source tree resolved automatically by step 3 of [module resolution](#module-resolution).

The implicit `Prelude` is a curated facade re-exporting core definitions rather than the entire standard library. Placing a source module with the same name beside your entry file shadows the embedded copy. For manifests, lockfiles, and registry publishing, see [packages, manifests, and the registry](29-package-management.md).

# Current boundaries

- <span class="support-tag support-tag--planned">Coming soon</span> **General parameterized-module composition.** Only signature-typed and value-typed module parameters are currently accepted. Passing parameterized instances as module arguments, nested parameterized instantiations, and dependent index references through parameterized functions are rejected. `Io` capabilities remain lexical and `Copy` remains structural; neither can be abstracted as an arbitrary module parameter.
- <span class="support-tag support-tag--planned">Coming soon</span> **Richer dependent module interfaces.** Signature blocks currently admit `opaque type` and `record` declarations alongside methods (spliced ahead of the signature so method types can reference them). Full dependent module signatures—packaging associated type families, semantic models, laws, and staged generators, as well as settled applicative/generative functor identity rules—are planned. The following schematic target syntax is not yet accepted:

```text
signature Filtering (c :: Type -> Type) where
  type Output :: forall a. (a -> Bool) -> c a -> Type
  filter :: forall a. (p :: a -> Bool) -> (xs :: c a) -> Output p xs
```
