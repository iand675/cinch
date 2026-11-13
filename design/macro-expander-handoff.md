# The macro expander: what the parse rung hands you

The parse rung for `name!(...)` is landed on `main` (`25bb2c83`, `4c885638`).
This note is for the lane that writes the expander. It records the
representation you receive, the seed's specification with line numbers, and
the four traps that cost this lane a full build cycle each.

## What is already true

`Bootstrap.Ast` has

```
EMacroCall :: Span -> String -> List LToken -> Expr
```

- `Span` covers the macro's name through the argument tree's closing
  delimiter (the `span-join` of the name's span and the tree's last token).
- `String` is the macro's name **as written**, before resolution: `wrap`,
  or `F.exposed` for a qualified spelling. It is not an encoded global.
- `List LToken` is the whole tree **including its delimiters**. `Text.Lex`
  exports `LToken`, `lt-token`, `lt-span`, `lt-virtual`, and the `Token`
  constructors.

`Bootstrap.Parse` produces it in `parse-postfix`:

```
else if is-punct-t (lt-token t) "!" then parse-macro-call value rest
```

`parse-macro-call` accepts only `EVar` as the head (`expr-name`), which is
the grammar's `var_path`. `parse-token-tree` / `tree-scan` collect the tree.

The grammar is `specifications/surface-syntax-and-grammar.md:182-183`:

```
macro_call = var_path, "!", token_tree ;
token_tree = ? balanced token tree in (), [] or {}; nested delimiters match ? ;
```

The lexer already does the hard half. `compiler/prelude/Text/Lex.cinch:511`
(`is-macro-bang`) ends a word at a `!` that is tight against `(`, `[` or
`{`, so `f!(x)` lexes as `TVar "f"`, `TPunct "!"`, `TPunct "("`, ... . A
name that itself ends in `!` invokes with one more bang (`f!!(x)`), and
`reserved-symbol` (line 739) spells `!` as `TPunct`. You do not need to
change the lexer.

## The seed is the specification

Read these before writing anything. Every diagnostic below goes in verbatim.

| What | Where |
|---|---|
| Ordering: expand before the ordinary check | `compiler/src/check_decls.rs:18958-18960` (`expand_remaining_macros`, with the comment "Macro calls in ordinary code expand before the ordinary check") |
| The term-position expansion | `check_decls.rs:20334-20360` (`expand_macros_in_term`, the `MacroCall` arm) |
| The call itself | `check_decls.rs:20556-20660` (`eval_macro_call`) |
| Splitting the tree into arguments | `compiler/src/ast.rs:456` (`arg_groups`) and `:500` (`macro_arg_groups`) |
| The declaration walk | `check_decls.rs:20501` (`expand_macros_in_decl`) |
| Expansion at emission | `check_decls.rs:20220-20224` (generated code expands where it is emitted) |

Diagnostics, verbatim from the seed:

- `` `{name}` is not a macro (only `macro` definitions take `!(...)` token-tree arguments) `` (`:20569`)
- `` macro `{name}` takes {n} token-tree argument(s), but the invocation supplies {m} `` (`:20582`)
- `a macro invocation in term position must expand to code (`Code t`), but this one produced {kind}` (`:20355`)
- `` a macro must produce `Code`, `Declarations`, `Syntax` or `Name`, but `{name}` produced {kind} `` (`:20651`)

The arity rule is the one that bites. `macro_arg_groups` (`ast.rs:500`):
`m!(a, b)` is two groups split on commas **outside** nested delimiters, and
for a macro of arity 0 an argument of exactly one empty group is **zero**
arguments, so `number!()` calls a nullary macro while `empty!()` calls a
unary macro with one empty `Syntax`. Get this wrong and
`nullary_native_macros_keep_definition_scope_and_unary_empty_input` fails
with a message about the argument count, not about arity.

## Where the expander goes in the self-hosted chain

`Bootstrap.Stage.expand-declaration` (`bootstrap/Bootstrap.Stage.cinch:449`,
the `DMacro` arm at `:451`) is the walk. That arm currently returns
`Ok (Nil, fuel)`, so a macro leaves no runtime trace, and every other
declaration returns `Ok ([declaration], fuel)`. That is the arm to extend: a
`DFun` whose clauses contain an `EMacroCall` must be rewritten before
`retain-resolved-source` runs.

The pipeline order is already right for you.
`Bootstrap.Pipeline.check-source-phase` (`:277`) runs
`expand-stages resolve program` and then `retain-resolved-source (resolve
expanded)`. So expansion sees unresolved source and its output is resolved
afterwards. That matches the seed, and it is why `EMacroCall`'s name is a
spelling rather than a global.

You will need the macro's *checked* body to run it, which is what
`Bootstrap.StageEval.evaluate-stage` provides, and a quotation's template,
which `Bootstrap.Stage` reads off the checked surface
(`surface-template`, `declarations-template`, `clause-template`). A
`macro`-bodied declaration whose body is `EQuote` has no stage computation
in it: the template is re-checked where the code lands, so it must leave the
stage namespace exactly as `quotation-bodied` already does for `DStage`.

## Four traps this lane paid for

1. **Push the opener's closer exactly once.** `parse-token-tree` seeds
   `tree-scan` with the opener already in `acc` and its closer already owed,
   and the scan begins at the opener's *successor*. The first version began
   at the opener, pushed its closer a second time, and every tree ended with
   one delimiter still owed, so `name!(...)` failed to parse at all while
   every module still typechecked and the chain still reached its fixed
   point. A parse bug here is invisible to the fixed point: `compiler.ll`
   was byte-identical, and only the target suite showed it. Measure with
   `tooling/run_stage_targets.py`, never with the build alone.

2. **`stage2` and `stage3` must agree byte for byte.** If any part of your
   expansion iterates a `HashMap`, the generated declarations can differ in
   order between stages and the fixed point breaks in a way that looks like
   a codegen bug. Sort, or use a spine with a defined order.

3. **Generated binders need the `#` spelling.** A binder the expansion
   invents must use the internal spelling so it cannot capture a caller's
   name. `Bootstrap.Stage.probe-name` (`:33`) is the existing example:
   `string-concat "#stage-probe-" (nat-to-string (span-start site))`.

4. **The name is never a value reference.** `Bootstrap.Resolve`,
   `Bootstrap.Contracts` and `Bootstrap.Infer`'s guard walker each have an
   explicit `EMacroCall` arm that refuses or ignores the call and
   deliberately does not rename its name. Do not "simplify" those arms into
   a rename: the expander resolves the name against the macros in scope, and
   a renamed spelling would give it a global the macro never had. Likewise
   `expr-unmodeled-count` returns 0 for a call, because the form is fully
   modelled; the argument is macro input, not a term the rung owes a model
   of.

## What will move the count, and what will not

Baseline `syntax_objects`: 26 passed, 52 failed. The 52 split as:

- 33 were `bootstrap.resolve: the parser cannot model this declaration`
  naming a `name!(...)` call. After the parse rung, 3 such failures remain
  and all are non-macro (`Sigma`, and `(let x :: T; x = e in ...)`), and 16
  now report `macro invocation `x!` was not expanded`. Those 16 are your
  work.
- 15 are `modified imports require stage/module-argument elaboration`
  (`import stage Library`, module arguments). That is a separate feature and
  a separate lane. Do not spend time on it.
- The rest are `has no exported member`, also not macro work.

So the ceiling in this target is 33 plus whatever expansion unblocks, not
52. Several of the 33 also need `Syntax.capture`/`Syntax.apply` and friends
(`unknown name (name is not in scope: Syntax.apply)`), which is the
`Syntax` stage API, not the expander; expect those to stay red.

The acceptance program, which the seed runs to 42:

```
module Main where
  macro wrap :: Syntax -> Code Int
  macro wrap body = [| 42 |]
  main :: Int
  main = wrap!(40)
```

`compiler/target/debug/cinch run` on that file exits 42. That binary is the
seed and the specification; use it to pin any behaviour before mirroring it.

## Fast checks while you iterate

Typechecking one edited module against the seed takes about 3 to 5 seconds,
against about 11 minutes for a chain rebuild. Copy the module and the
`Text.*` prelude files it imports into one directory, then:

```
compiler/target/debug/cinch check Bootstrap.Stage.cinch
```

That catches type errors and wrong constructor arities. It does not catch
the fixed-point or behaviour traps, which need the full build and the target
suite.
