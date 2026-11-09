# Staging, quotation, generation, and data imports

Staging runs code during compilation and feeds the result into the program. Two mechanisms cross that boundary, and they have separate spellings: declaration macros transform syntax, and typed quotations build `Code a` values that the checker types as ordinary terms. Generated declarations are parsed and checked like handwritten ones. A macro that emits a well-formed template with a wrongly typed body is rejected by the ordinary checker, not by the macro.

# Surface forms

| Form | Meaning |
| --- | --- |
| `stage name :: T` | A compile-time definition. It is evaluated during compilation and is not callable from runtime code. |
| `[\| term \|]` | A quotation. It has type `Code T` where `T` is the term's type. |
| `$(code)` | A splice inside a quotation. It inserts a `Code T` value where a `T` is expected. |
| `quoteDecls where { ... }` | A declaration template. Its value has type `Declarations`. |
| `macro name :: Syntax -> Declarations` | A declaration macro. |
| `name!(tokens)` | A macro invocation with a balanced token tree; the macro receives it as `Syntax`. |
| `generate do ...` | A checked generator run at the declaration's position. |
| `import stage Module` | A staging dependency on another module's generators and quotations. |
| `import data "file.json" as value` | Compile-time conversion of a data file into a constant. |

The fragments above are fragments, not programs: `T` and `term` stand for any admitted type and term.

# A macro that emits a checked declaration

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

`macro addVia arg = quoteDecls where ...` returns the quoted declaration group. `addVia!(x)` passes the token `x` as a `Syntax` value, and `$(arg)` splices it into the template body. `emit` appends the resulting declarations at the `generate` position, so `main` can see `result`.

The result is 42, not 42 + 1 and not 1 + 1, which is the hygiene property: the template's `let x = 1` binder is renamed apart from the caller's global `x`, so the spliced `x` still means the caller's 41. A free name in a template resolves at the macro's definition site. In the same file, a template that calls a helper defined next to the macro uses that helper even when the caller defines the same spelling.

# Typed quotations and `eval`

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

`addOne` composes two `Code Int` values at compile time; the generated `answer` is the checked result, 22. `quoted` is a `Code Int` value in the running program and `eval quoted` is the builtin `eval :: forall a. Code a -> a`, which lowers to a call to the runtime evaluator (`cinch_eval` in the emitted IR). `eval` carries no capability and no effect of its own, and it establishes no proof: the value it returns is a runtime computation. A target profile may forbid runtime code generation, in which case `eval` is rejected at check time with `forbids runtime code generation`. The CLI always selects the native profile, which admits `eval`.

A `stage` definition cannot be called from runtime code:

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

```
cinch: 6:12: addOne is a compile-time definition (`stage`/`macro`): it cannot appear in runtime code (in Main.cinch)
```

# Constant transport out of the compile-time world

Three builtins turn a compile-time value into generated code. Their signatures are `quoteNat :: Nat -> Code Nat`, `quoteInt :: Int -> Code Int`, and `quoteBool :: Bool -> Code Bool`. All three are pure.

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

There is no transport for any other constant. The obvious spelling for a text constant fails by name:

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

```
cinch: 8:16: unknown name quoteStr
```

A `Secret` cannot reach generated source either. `secret :: String ->{StageSecret} Secret` produces a compile-time-only value, no function converts it to text or an integer, and the compile-time evaluator refuses to stringify one with `a secret value must not be embedded in generated source, logs, or snapshots (modules specification section 10)`. The only consumer in the surface language is `withDatabase`.

# Compile-time inputs, capabilities, and replay

Compile-time I/O needs a capability value. The root operation is `snapshot`, which names the capture and hands a callback the `IoCap`:

| Builtin | Type | Effect |
| --- | --- | --- |
| `snapshot` | `String -> (IoCap ->{e} a) ->{e} a` | none of its own |
| `withReadFiles` | `IoCap -> (FileReadCap ->{e} a) ->{e} a` | none of its own |
| `withWriteFiles` | `IoCap -> (FileWriteCap ->{e} a) ->{e} a` | none of its own |
| `withProcess` | `IoCap -> (ProcCap ->{e} a) ->{e} a` | none of its own |
| `readFile` | `FileReadCap -> String ->{StageFile} String` | `StageFile` |
| `writeFile` | `FileWriteCap -> String -> String ->{StageFile} Unit` | `StageFile` |
| `run` | `ProcCap -> String -> List String ->{StageProcess} String` | `StageProcess` |
| `secret` | `String ->{StageSecret} Secret` | `StageSecret` |
| `emit` | `Declarations ->{StageEmit} Unit` | `StageEmit` |
| `withDatabase` | `IoCap -> Secret -> SchemaRead -> (Database ->{e} a) ->{e} a` | none of its own |
| `ReadSchema` | `SchemaRead` | none |
| `introspectConsistently` | `Database -> SchemaSnapshot` | none |
| `CodecGenerator.derive` | `SchemaSnapshot -> Declarations` | none |
| `strNat`, `natStr`, `strAppend`, `strLen` | text and `Nat` conversions | none |
| `name-to-sym`, `sym-to-name` | `Name -> Sym` and `Sym -> Name` | none |

The following project reads a file during compilation and emits the parsed number as a constant. Save `input.txt` containing `42` beside `Main.cinch`.

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

A subprocess capture works the same way and needs no second file:

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

`CINCH_STAGE_MODE` selects how inputs are obtained. `refresh` executes the external work and records it, `replay` uses recorded inputs, and `live` performs the current read. An absent or unrecognized value selects `refresh`.

```
$ CINCH_STAGE_MODE=refresh cinch run Main.cinch
$ cat build/Main.stage.snap
file	inputs	input.txt	3157674352912849835	recorded	42\n
$ rm input.txt
$ CINCH_STAGE_MODE=replay cinch run Main.cinch
$ echo $?
0
```

The snapshot file is tab separated: kind, snapshot name, input name, content hash, recording state, and the captured content. The build also records the capture in `build/Main.manifest.json`, in its `staging` section.

```json
{
  "compiler": "0.1.0",
  "generated": { "Main": ["answer"] },
  "inputs": [
    { "hash": "3157674352912849835", "kind": "file", "live": false,
      "name": "input.txt", "snapshot": "inputs" }
  ],
  "mode": "refresh"
}
```

Replay with no recorded input is a named failure, not a silent re-read:

```
$ CINCH_STAGE_MODE=replay cinch run Main.cinch
cinch: replay mode: no captured input for the file `input.txt` in snapshot `inputs` (run a refresh build to record it)
```

`CINCH_SCHEMA_TOOL` names the subprocess that `withDatabase` runs for schema introspection. The value of the secret is handed to the tool and never recorded: the manifest names the environment variable, not the credential. `introspectConsistently` rejects a document whose table names collide with `inconsistent schema` and the colliding name, a column type outside `INTEGER`, `REAL`, `BOOLEAN` and `TEXT` with `unsupported column type`, and `CodecGenerator.derive` reports that its encoders cover `INTEGER` and `BOOLEAN`.

# Expansion provenance

A generated body that fails a check reports the template location and the call site, so the error points into the macro and at its use.

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

```
cinch: Macros.cinch:8:14 (in expansion of macro `bad` called at 4:3): type mismatch: expected Bool, got Int (in Main.cinch)
```

# Stage dependencies

`import stage Module` adds a scheduling dependency. A genuine cycle is refused with the cycle printed, naming the generator block and the stage definition it needs.

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

```
cinch: stage dependency cycle: generate block in Main (at 5:3) -> stage definition `f` in Main -> generate block in Main (at 5:3)
```

# Data imports

`import data "file.ext" as name` converts a file at compile time and binds the result as a constant. The converters are keyed by extension: `json` produces a `Json` value unless the binding carries a declared type, `txt` produces a `String`, and `bytes` produces a `Bytes`. A declared type must be monomorphic, and a `json` import decodes into scalars, `String`, `List`, `Option`, and records by field name.

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

The manifest records the file, the converter, and the content hash:

```json
{
  "bindings": [
    { "converter": "json", "file": "n.json", "hash": "12638132224974231446",
      "module": "Main", "name": "value" }
  ]
}
```

An unknown extension is a build error naming the built-ins:

```
cinch: 2:15: no converter for `bogus`: the built-in converters are keyed by extension (`json`, `txt`, `bytes`) (in Bad.cinch)
```

# What is not available

<span class="support-tag support-tag--planned">Coming soon</span> **staged values beyond the constant family.** The intended language allows a wider set of values to cross from compile time to runtime, with declared transport and metadata rules rather than a generic cast (`modules-and-compilation-specification` section 9). Only `quoteNat`, `quoteInt` and `quoteBool` exist today, so a text, byte string, record, or function value cannot be lifted; `quoteStr` is not a name and the rejection above is what you get. Extending the family is a specification change: a new transport needs its own serialization, identity, and provenance rules.

Implementation: `driver.rs` (stage mode, snapshot files), `check_decls.rs` (stage builtins, capability refinements, macro expansion), `check_code.rs` (quotation typing), `provenance.rs`; suites `staging.rs`, `surface.rs`, `arrays.rs`. Intended semantics: [modules and compilation specification](../specifications/modules-and-compilation-specification.md) sections 7 to 11 and [surface syntax and grammar](../specifications/surface-syntax-and-grammar.md). Tutorial: [compile-time code](../learn/18-staging-and-macros.md).
