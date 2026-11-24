# Staging, quotation, generation, and data imports

Staging runs compile-time code and splices its results into the program before normal typechecking and code generation. Cinch separates compile-time metaprogramming into three explicit mechanisms:

- **Hygienic declaration macros** (`macro`, `quoteDecls`, `generate`) transform balanced token trees (`Syntax`) into declaration groups (`Declarations`).
- **Typed quotations** (`[| ... |]`, `$(...)`, `Code a`) construct and compose typed code fragments at compile time or pass them to runtime `eval`.
- **Compile-time data imports** (`import data`) embed validated external files as typed constants.

Every generated declaration is parsed and checked through the same type, effect, and ownership pipeline as handwritten code. Compile-time and runtime phases are strictly separated: `stage` and `macro` definitions cannot be called from runtime code, and compile-time I/O requires explicit capabilities with reproducible snapshot recording.

# Surface forms

| Form | Meaning |
| --- | --- |
| `stage name :: T` | Compile-time definition; evaluated during compilation and unreachable from runtime code. |
| `[\| term \|]` | Typed quotation with type `Code T` when `term :: T`. |
| `$(code)` | Splice inside a quotation or declaration template; inserts `Code T` where `T` is expected. |
| `quoteDecls where { ... }` | Hygienic declaration template producing `Declarations`. |
| `macro name :: Syntax -> Declarations` | Declaration macro receiving a balanced token tree as `Syntax`. |
| `name!(tokens)` | Macro invocation passing balanced tokens to `name`. |
| `generate do ...` | Compile-time generator block executed at its declaration position. |
| `import stage Module` | Compile-time staging dependency on another module's macros and `stage` definitions. |
| `import data "file.ext" as name` | Compile-time conversion of a data file into a constant binding. |

# Hygienic declaration macros

A declaration macro builds a `quoteDecls` template and returns `Declarations`, which a `generate` block splices into the module via `emit`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  x :: Int
  x = 41

  macro addVia :: Syntax -> Declarations
  macro addVia arg = quoteDecls where
    result :: Int
    result = let x = 1 in x + $(arg)

  generate do
    emit (addVia!(x))

  main :: Int
  main = if result == 42 then 0 else 1
```

Templates enforce lexical hygiene in both directions:

- **No accidental capture of caller names.** The template's local `let x = 1` is renamed apart from the spliced argument `x`, so `$(arg)` still resolves to the caller's top-level `x = 41`, yielding `42`.
- **Definition-site resolution for free names.** Free identifiers inside a `quoteDecls` body resolve in the macro's defining module, even if the call site shadows the same name.

# Typed quotations and `eval`

Quotations compose `Code a` values at compile time via `stage` functions and may also appear as runtime values evaluated by `eval :: forall a. Code a -> a`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  stage addOne :: Code Int -> Code Int
  stage addOne c = [| let x = $(c) in x + 1 |]

  generate do
    emit (quoteDecls where
      answer :: Int
      answer = $(addOne [| 21 |]))

  quoted :: Code Int
  quoted = [| 6 * 7 |]

  main :: Int
  main = if answer == 22 then (if eval quoted == 42 then 0 else 1) else 2
```

`addOne` runs during compilation and emits `answer = 22` as ordinary checked code. `eval quoted` lowers to the runtime evaluator (`cinch_eval`), carries no effect or capability of its own, and establishes no compile-time proof.

A target profile may disallow runtime code generation, in which case `eval` is rejected at check time with `forbids runtime code generation`. All five CLI `--target` triples ([Chapter 01](01-using-the-compiler.md#targets-and-cross-compilation)) enable `eval`; restricted profiles are configured through the compiler's Rust API.

Calling a `stage` or `macro` definition from runtime code is rejected during checking:

<!-- cinch:reject contains="cannot appear in runtime code" -->
```cinch
module Main where
  stage addOne :: Code Int -> Code Int
  stage addOne c = [| let x = $(c) in x + 1 |]

  answer :: Code Int
  answer = addOne [| 20 |]

  main :: Int
  main = if eval answer == 21 then 0 else 1
```

```text
cinch: 6:12: addOne is a compile-time definition (`stage`/`macro`): it cannot appear in runtime code (in Main.cinch)
```

# Constant transport across stages

Three pure builtins lift compile-time scalars into `Code` values:

- `quoteNat :: Nat -> Code Nat`
- `quoteInt :: Int -> Code Int`
- `quoteBool :: Bool -> Code Bool`

<!-- cinch:run exit=0 -->
```cinch
module Main where
  stage offset :: Int
  stage offset = 0 - 7

  stage width :: Nat
  stage width = 1000

  stage enabled :: Bool
  stage enabled = width == 1000

  generate do
    emit (quoteDecls where
      delta :: Int
      delta = $(quoteInt offset)
      size :: Nat
      size = $(quoteNat width)
      flag :: Bool
      flag = $(quoteBool enabled))

  main :: Int
  main = if delta + nat-to-int size == 993 then (if flag then 0 else 1) else 2
```

No other constant transport exists today. Attempting to lift a string with a nonexistent helper fails name resolution:

<!-- cinch:reject contains="unknown name quoteStr" -->
```cinch
module Main where
  stage greeting :: String
  stage greeting = "hello"

  generate do
    emit (quoteDecls where
      text :: String
      text = $(quoteStr greeting))

  main :: Int
  main = 0
```

Values of type `Secret` (produced by `secret :: String ->{StageSecret} Secret`) are compile-time-only credentials. No conversion turns a `Secret` into text or an integer, and the staging evaluator refuses to serialize one (`a secret value must not be embedded in generated source, logs, or snapshots`). The only built-in consumer is `withDatabase`.

# Compile-time I/O, capabilities, and replay

External compile-time inputs are gated behind scoped capabilities introduced by `snapshot`:

| Builtin | Type | Staging effect |
| --- | --- | --- |
| `snapshot` | `String -> (IoCap ->{e} a) ->{e} a` | — |
| `withReadFiles` | `IoCap -> (FileReadCap ->{e} a) ->{e} a` | — |
| `withWriteFiles` | `IoCap -> (FileWriteCap ->{e} a) ->{e} a` | — |
| `withProcess` | `IoCap -> (ProcCap ->{e} a) ->{e} a` | — |
| `withDatabase` | `IoCap -> Secret -> SchemaRead -> (Database ->{e} a) ->{e} a` | — |
| `readFile` | `FileReadCap -> String ->{StageFile} String` | `StageFile` |
| `writeFile` | `FileWriteCap -> String -> String ->{StageFile} Unit` | `StageFile` |
| `run` | `ProcCap -> String -> List String ->{StageProcess} String` | `StageProcess` |
| `secret` | `String ->{StageSecret} Secret` | `StageSecret` |
| `emit` | `Declarations ->{StageEmit} Unit` | `StageEmit` |
| `ReadSchema` | `SchemaRead` | — |
| `introspectConsistently` | `Database -> SchemaSnapshot` | — |
| `CodecGenerator.derive` | `SchemaSnapshot -> Declarations` | — |
| `strNat`, `natStr`, `strAppend`, `strLen` | Compile-time string and `Nat` utilities | — |
| `name-to-sym`, `sym-to-name` | `Name -> Sym`, `Sym -> Name` | — |

This project reads `input.txt` at compile time and emits the parsed integer as `answer`:

<!-- cinch:project group=14-staging-and-macros-refresh file=Main.cinch exit=0 -->
```cinch
module Main where
  generate do
    n <- snapshot "inputs" (\io ->
      withReadFiles io (\fs -> strNat (readFile fs "input.txt")))
    emit (quoteDecls where
      answer :: Nat
      answer = $(quoteNat n))

  main :: Int
  main = if answer == 42 then 0 else 1
```

<!-- cinch:project group=14-staging-and-macros-refresh file=input.txt -->
```cinch
42
```

Subprocess output is captured with `withProcess` and `run`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  generate do
    out <- snapshot "tools" (\io ->
      withProcess io (\p -> run p "/bin/echo" ["hi"]))
    emit (quoteDecls where
      size :: Nat
      size = $(quoteNat (strLen out)))

  main :: Int
  main = if size == 3 then 0 else 1
```

## Snapshot modes (`CINCH_STAGE_MODE`)

| Mode | Behavior |
| --- | --- |
| `refresh` (default) | Executes compile-time I/O and writes captured outputs to `build/<entry>.stage.snap`. Unrecognized values fall back to `refresh`. |
| `replay` | Reads captured inputs strictly from `build/<entry>.stage.snap` without touching external files or processes. Fails if an input is missing (`replay mode: no captured input for the file ...`). |
| `live` | Executes compile-time I/O live without relying on recorded snapshots. |

```sh
CINCH_STAGE_MODE=refresh cinch run Main.cinch
cat build/Main.stage.snap
# file	inputs	input.txt	3157674352912849835	recorded	42\n
rm input.txt
CINCH_STAGE_MODE=replay cinch run Main.cinch   # exits 0 from recorded snapshot
```

Each line in `build/<entry>.stage.snap` is tab-separated: `kind`, `snapshot`, `input`, `hash`, `state`, and captured content. `build/<entry>.manifest.json` records the same captures under `staging` (including `mode`, `generated` declarations per module, and hashed `inputs`).

For database schema introspection, `CINCH_SCHEMA_TOOL` names the subprocess invoked by `withDatabase`. The credential value is passed to the tool and never persisted: the manifest records only the environment variable name. `introspectConsistently` rejects duplicate table names (`inconsistent schema`) and column types outside `INTEGER`, `REAL`, `BOOLEAN`, and `TEXT` (`unsupported column type`); `CodecGenerator.derive` currently generates encoders for `INTEGER` and `BOOLEAN`.

# Expansion provenance

When generated code fails checking, diagnostics report both the macro template location and the expansion site:

<!-- cinch:project group=14-staging-and-macros-provenance file=Macros.cinch -->
```cinch
module Macros where
  wrong :: Bool
  wrong = True

  macro bad :: Syntax -> Declarations
  macro bad arg = quoteDecls where
    broken :: Int
    broken = wrong
```

<!-- cinch:project group=14-staging-and-macros-provenance file=Main.cinch mode=reject contains="in expansion of macro" -->
```cinch
module Main where
  import stage Macros

  generate do
    emit (bad!())

  main :: Int
  main = 0
```

```text
cinch: Macros.cinch:8:14 (in expansion of macro `bad` called at 4:3): type mismatch: expected Bool, got Int (in Main.cinch)
```

# Stage dependencies

`import stage Module` establishes a compile-time evaluation dependency. Cyclic dependencies between `stage` definitions and `generate` blocks are rejected with the full cycle trace:

<!-- cinch:reject contains="stage dependency cycle" -->
```cinch
module Main where
  stage f :: Nat
  stage f = n

  generate do
    let x = f
    emit (quoteDecls where
      n :: Nat
      n = 1)

  main :: Int
  main = 0
```

```text
cinch: stage dependency cycle: generate block in Main (at 5:3) -> stage definition `f` in Main -> generate block in Main (at 5:3)
```

# Data imports

`import data "file.ext" as name` converts an external file at compile time and binds it as a constant. Built-in converters are selected by filename extension:

| Extension | Default type | Typed decoding |
| --- | --- | --- |
| `json` | `Json` | When a monomorphic type signature is declared for `name`, decodes scalars, `String`, `List`, `Option`, and records by field name. |
| `txt` | `String` | — |
| `bytes` | `Bytes` | — |

<!-- cinch:project group=14-staging-and-macros-import file=Main.cinch exit=0 -->
```cinch
module Main where
  import data "n.json" as value

  value :: Int

  main :: Int
  main = if value == 7 then 0 else 1
```

<!-- cinch:project group=14-staging-and-macros-import file=n.json -->
```cinch
7
```

Each data import is recorded in `build/<entry>.manifest.json` with its module, binding name, file path, converter, and content hash. An unrecognized extension fails with `no converter for ...: the built-in converters are keyed by extension (json, txt, bytes)`.

# Current limits

<span class="support-tag support-tag--planned">Coming soon</span> **Cross-stage lifting beyond primitive scalars.** Only `quoteNat`, `quoteInt`, and `quoteBool` currently transport compile-time values into `Code`. Lifting `String`, `Bytes`, records, and higher-order values with explicit serialization and provenance rules is planned.

See [modules and compilation specification](../specifications/modules-and-compilation-specification.md) §§7–11, [surface syntax and grammar](../specifications/surface-syntax-and-grammar.md), and the [compile-time code tutorial](../learn/18-staging-and-macros.md).
