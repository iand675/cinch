# 18. Compile-time code and hygienic macros

Staging runs computations during compilation and feeds their results into the program. It can remove repetitive declarations, but generated code still passes the ordinary parser and checker. A generator is not an exemption from type, effect, or ownership rules.

# Generate a declaration

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

`quoteDecls` constructs declaration syntax. `$(value)` splices a syntax argument into the template. `generate` emits the resulting declarations at the insertion point. The generated `answer` is available to `main` and is checked as an `Int` definition.

Templates are hygienic: a template's local binder must not accidentally capture a caller's variable with the same spelling. Free helpers in a template resolve at the definition site. A splice supplies caller syntax deliberately; it is not textual search-and-replace. Name quotations such as `'answer` and `''TypeName` refer to staging names in the relevant context; ordinary term-position `'answer` is an interned symbol.

# Keep compile-time inputs reproducible

File reads, subprocesses, and schema introspection require staging capabilities. The compiler records inputs and provenance. `CINCH_STAGE_MODE=refresh` captures inputs; `replay` uses recorded inputs; `live` follows the live staging mode. A dependency on a database schema needs a consistent snapshot and a recorded tool identity. Secrets may authorize access, but they cannot be embedded into emitted program constants.

`import stage Module` establishes a staging dependency, distinct from an ordinary runtime import. Cycles in that dependency graph are errors. A malformed expansion reports the expansion's provenance so that you can find the macro or call site responsible.

# Typed code is different from declaration syntax

`[| ... |]`, splicing, `Code a`, and `eval` cover typed code values. Target profiles determine whether runtime evaluation is admitted. Do not assume that a syntax tree can be executed everywhere, or that every backend has the same runtime evaluator. [Staging reference](../reference/14-staging-and-macros.md) gives the supported forms and limits.

# Exercise and answer

If the macro above emits `answer = True` while retaining `answer :: Int`, when should compilation fail?

<details><summary>Answer</summary>

After expansion, when the ordinary checker checks the generated declaration. A syntactically valid quotation does not establish that its generated program is well typed. The diagnostic should retain the generation provenance.

</details>
