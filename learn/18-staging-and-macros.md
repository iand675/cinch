# 18. Compile-time code and hygienic macros

Staging runs part of your program while the compiler is compiling it. A generator builds syntax, the compiler inserts that syntax into the module, and the ordinary checker then checks the result. That last step is the point: generated code is not privileged, so a generator that emits a well-typed declaration gets what it asked for, and one that emits a body of the wrong type is refused like any hand-written mistake.

Cinch has two staging forms, and the difference is what the generator produces.

| Form | Written as | Produces | Consumed by |
| --- | --- | --- | --- |
| typed quotation | `[\| e \|]`, a `stage` function over `Code a` | `Code a`, a checked fragment | a `$( ... )` splice, or `eval` at runtime |
| declaration macro | `macro m :: ... -> Declarations`, body `quoteDecls where ...` | `Declarations`, a group of declarations | `emit` inside a `generate do` block |

Prerequisites: [modules](08-modules.md) for `import` and multi-module files, [signatures and instances](09-signatures-and-instances.md) for the type of a generator. The normative account is [staging and macros](../reference/14-staging-and-macros.md).

# Quote typed code and splice it

`[| e |]` is a typed quotation: it builds a `Code a` whose type is checked where the quotation is written. `$( ... )` splices a `Code` value into a surrounding quotation.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  stage addOne :: Code Int -> Code Int
  stage addOne c = [| $(c) + 1 |]

  generate do
    emit (quoteDecls where
      base :: Int
      base = $(addOne [| 20 |])
      answer :: Int
      answer = base + 22)

  main :: Int
  main = if answer == 43 then 0 else 1
```

- `stage addOne :: Code Int -> Code Int` marks a definition as compile-time only. Calling it from runtime code is refused with ``addOne is a compile-time definition (`stage`/`macro`): it cannot appear in runtime code``, so a `stage` definition never reaches the program.
- `[| 20 |]` is a `Code Int`, checked as an `Int` expression at the point it is written.
- `$(c)` inside `[| $(c) + 1 |]` splices a `Code Int` into a position expecting an `Int`, and the result is another `Code Int`.
- `quoteDecls where ...` builds `Declarations`. The `where` body is a declaration group in surface syntax, so it is parsed and checked as declarations rather than as an opaque string.
- `emit` inside `generate do` inserts the declarations at that point in the module.

Both emitted declarations are ordinary declarations afterwards, so `main` reads `answer` with nothing special about it. The generated `base` evaluates to `21` and `answer` to `43`.

# Define a macro and use it

A macro takes syntax and returns declarations. `!` is the macro application form, and it passes syntax rather than a value.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  macro make-answer :: Syntax -> Declarations
  macro make-answer value = quoteDecls where
    answer :: Int
    answer = $(value)

  generate do
    emit (make-answer!(42))

  main :: Int
  main = if answer == 42 then 0 else 1
```

- `macro make-answer :: Syntax -> Declarations` declares a macro. `Syntax` is an untyped syntax tree and `Declarations` is a group of declarations.
- `make-answer!(42)` passes `42` as `Syntax`. That is why the parameter is `Syntax` and not `Int`: a macro sees the caller's syntax, not a value.
- `$(value)` in the template splices that caller syntax into the `answer = ...` position. Splicing is deliberate substitution at a chosen point, not textual search and replace.
- `generate do ... emit ...` at module level is the insertion point.

The generated `answer` is checked as an `Int` definition, so `main` reads it directly. When the emitted body does not fit the declared type, the failure arrives after expansion from the ordinary checker, with the expansion's provenance attached:

<!-- cinch:reject contains="in expansion of macro `bad` called at" -->
```cinch
module Main where
  wrong :: Bool
  wrong = True

  macro bad :: Syntax -> Declarations
  macro bad arg = quoteDecls where
    broken :: Int
    broken = wrong

  generate do
    emit (bad!())

  main :: Int
  main = 0
```

The report names the template line first and the call site second: ``cinch: Main.cinch:8:14 (in expansion of macro `bad` called at 10:3): type mismatch: expected Bool, got Int (in Main.cinch)``. That context is part of the diagnostic because the failing line exists in no file you wrote: it exists in the expansion.

# Templates are hygienic

A template's binders do not capture a caller's binding of the same spelling, and a free name in a template resolves where the macro was defined.

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

The generated `result` is `1 + 41`. The template's `let` binds `x` to `1` and uses it in `x + ...`, and the spliced `x` keeps the caller's binding worth `41`. Written without hygiene this template would compute `2` and print nothing suspicious. The generated binder is renamed apart from any caller binder it would otherwise capture, so you do not maintain a naming convention by hand. Definition-site resolution runs the other way: write a helper in another module, splice the caller's same-spelled name as the argument, and the bare name in the template still means the defining module's helper while the spliced one means the caller's.

A macro that chooses the name of the declaration it emits writes `$(nm)` in the declaration's name position, as in a template body of `$(nm) :: Int` and `$(nm) = let y = 1 in y + $(bump)` with `nm` a `Syntax` argument. That splice reifies the caller's `'result` quote as the declared name. Outside a `quoteDecls` template there is nothing to name, and the form is refused: ``a name splice (`$( e )`) names a generated declaration and is only meaningful inside a `quoteDecls` template``.

`import stage Macros` is what makes such a dependency a staging dependency rather than a runtime import. Cycles in that graph are errors, and the diagnostic prints the cycle:

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

Compiling `f` needs the generated `n`, and executing the generator needs `f`, so the report is ``cinch: stage dependency cycle: generate block in Main (at 5:3) -> stage definition `f` in Main -> generate block in Main (at 5:3)``.

# Compile-time input is a capability

A generator that reads a file, runs a tool, or inspects a schema does so under a capability handed to it, and `snapshot` records the result. The record is what makes a later build reproducible.

<!-- cinch:project group=stage-file file=input.txt -->
```cinch
42
```

<!-- cinch:project group=stage-file file=Main.cinch exit=42 -->
```cinch
module Main where
  generate do
    n <- snapshot "inputs" (\io ->
      withReadFiles io (\fs -> strNat (readFile fs "input.txt")))
    emit (quoteDecls where
      answer :: Nat
      answer = $(quoteNat n))
  main :: Int
  main = nat-to-int answer
```

- `snapshot "inputs"` names the snapshot and hands its callback an `IoCap`, which carries no file or process authority by itself. `withReadFiles io (\fs -> ...)` refines that to a `FileReadCap`, and `readFile fs "input.txt"` reads the file through it.
- `strNat` parses the text `42` into a `Nat`, and `quoteNat` turns it into a `Code Nat` for the splice.
- `main` returns `42`, the recorded value. The captured input, the snapshot name, and the mode that produced them are written into the build's manifest and snapshot file.

The capability set is exactly what the compiler admits, and a generator has no authority beyond it. `withReadFiles`, `withWriteFiles`, `withProcess`, `withDatabase` and `secret` are the refinements. `withProcess io (\p -> run p "/bin/echo" ["hi"])` runs a tool and returns its output, so `strLen` of that output is `3`.

A `Secret` is not a `String`, and that is how a credential stays out of emitted constants: passing one where text is expected is a type error, and a name that is not set fails by name.

<!-- cinch:reject contains="is not available in the build environment" -->
```cinch
module Main where
  generate do
    snap <- snapshot "schema" (\io ->
      withDatabase io (secret "CINCH_DOC_DSN_ABSENT") ReadSchema
        (\db -> introspectConsistently db))
    emit (quoteDecls where
      answer :: Nat
      answer = 0)
  main :: Int
  main = nat-to-int answer
```

The message is ``cinch: the secret `CINCH_DOC_DSN_ABSENT` is not available in the build environment (it is referenced by name only; its value never enters generated source or snapshots)``. The same program with the name set would still refuse, because a schema needs an introspection tool that a command-line build does not configure: ``cinch: no schema introspection tool is configured for `Stage.withDatabase` (modules specification section 11 delegates introspection to a tool subprocess; decision D65)``.

# CINCH_STAGE_MODE selects where compile-time input comes from

`CINCH_STAGE_MODE` has three values and defaults to `refresh`. Run the project above once in each mode, deleting the input after the first run.

```text
$ cinch run Main.cinch ; echo $?
42
$ rm input.txt
$ CINCH_STAGE_MODE=replay cinch run Main.cinch ; echo $?
42
$ CINCH_STAGE_MODE=live cinch run Main.cinch
cinch: cannot read input.txt: No such file or directory (os error 2)
```

| Mode | Where input comes from | When the input is missing |
| --- | --- | --- |
| `refresh` | the live world, recorded into the snapshot | the read fails |
| `replay` | the recorded snapshot only | fails naming the missing input |
| `live` | the live world, nothing reused | the read fails |

`replay` produced `42` with `input.txt` deleted, because it never touched the file. `live` failed on the same program because it insists on the live world. A replay with nothing recorded fails with ``cinch: replay mode: no captured input for the file `input.txt` in snapshot `inputs` (run a refresh build to record it)``, which names both the input and the way to produce one.

# Where the boundary still holds

`eval` runs a typed quotation at runtime. The native profile admits it, so this fragment exits with status `42`:

```cinch
  main :: Int
  main = eval [| 42 |]
```

The admission is a profile property rather than a language one, because a target profile may forbid runtime code generation. Laziness is refused at compile time in the other direction: `delay` and memo cells inside a `generate` block or a `stage` definition fail with ``cinch: primitive `cinch.prim.delay` is not available at compile time in this slice``.

# What the compiler enforces

- A quotation `[| e |]` is type checked where it is written, so a malformed quotation fails before any splice happens.
- A macro argument arrives as `Syntax` and must be spliced deliberately; nothing converts caller syntax into a typed value for you.
- Generated declarations pass the ordinary checker after expansion. Types, effects, multiplicities and refinements all apply, and a failure carries the expansion's provenance.
- Template binders are hygienic, and a free name in a template resolves at the macro's definition site.
- A name splice is only meaningful inside a `quoteDecls` template.
- `import stage` records a staging dependency, and a cycle in that graph is reported with the cycle.
- Compile-time file, process, schema and secret access is capability-scoped. A `Secret` cannot be converted to text, and its value never reaches generated source, a snapshot, or a diagnostic.
- `CINCH_STAGE_MODE` decides whether compile-time input is captured, replayed, or read live, and a replay never falls back to the live world.

# Diagnostics you will meet

| You wrote | The compiler says |
| --- | --- |
| a macro that emits a body of the wrong type | ``(in expansion of macro `bad` called at 10:3): type mismatch: expected Bool, got Int`` |
| a bare `quoteDecls` block emitting a body of the wrong type | ``(in expansion of definition `$gen0` called at 2:3): type mismatch: expected Bool, got Int`` |
| `$(x) :: Int` outside a `quoteDecls` template | ``a name splice (`$( e )`) names a generated declaration and is only meaningful inside a `quoteDecls` template`` |
| a stage definition that needs a declaration its own generator emits | ``stage dependency cycle: generate block in Main (at 5:3) -> stage definition `f` in Main -> generate block in Main (at 5:3)`` |
| `secret "NAME"` with `NAME` unset | ``the secret `NAME` is not available in the build environment (it is referenced by name only; its value never enters generated source or snapshots)`` |
| `withDatabase` in a command-line build | ``no schema introspection tool is configured for `Stage.withDatabase` `` |
| `CINCH_STAGE_MODE=replay` with no snapshot | ``replay mode: no captured input for the file `input.txt` in snapshot `inputs` (run a refresh build to record it)`` |
| `delay` in a `generate` block | ``primitive `cinch.prim.delay` is not available at compile time in this slice`` |

The first two rows are the same failure at two insertion points: a macro call reports the macro that produced the code, and a bare `quoteDecls` block reports the synthesised definition `$gen0` that the insertion point became.

# Exercises

**Exercise 1.** `make-answer` emits `answer :: Int` and `answer = $(value)`. A colleague calls it as `make-answer!("hello")`. Where does compilation fail, and what does the report name?

<details><summary>Answer</summary>

After expansion, in the ordinary checker. `"hello"` is valid `Syntax` and splices cleanly into the `answer = ...` position, so the macro succeeds; what fails is the emitted declaration, whose body is a `String` where an `Int` is required. The report is a `type mismatch` carrying the expansion provenance, in the shape ``(in expansion of macro `...` called at LINE:COL): type mismatch: ...``. The fix belongs in the macro's signature: a macro that wants a type guarantee takes a typed argument such as `Code Int` rather than `Syntax`, and then the caller's quotation is checked before the macro ever runs.

</details>

**Exercise 2.** A generator reads `version.txt` to emit `version :: Nat`. A release build runs where `version.txt` is absent, but an earlier build recorded it. Which `CINCH_STAGE_MODE` should the release use, and what happens with each of the other two?

<details><summary>Answer</summary>

`replay`. The earlier `refresh` build recorded the file's contents in the snapshot, so replay reproduces that value without reading the file, and the release is built from the same input the earlier build saw. With `refresh` or `live` the generator reads the missing file and fails with `cannot read version.txt: No such file or directory (os error 2)`. Replay also fails when no record exists, with `no captured input for the file `version.txt` in snapshot ...`, because a replay never falls back to the live world. That refusal is the guarantee: a replay build either reproduces recorded input or stops.

</details>

Next: [rewrites and performance](19-rewrites-and-performance.md).
