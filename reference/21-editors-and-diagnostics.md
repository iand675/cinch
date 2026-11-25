# Editors, analysis APIs, and diagnostics

A single analysis layer serves all three compiler front ends—the CLI (`cinch`), the language server (`cinch lsp`), and the interactive REPL (`cinch repl`). Because they share the same module graph, typechecker, and incremental cache, diagnostics and type queries cannot diverge between terminal builds and editor sessions.

```sh
cinch check Main.cinch   # typecheck without codegen or linking (fastest loop)
cinch run Main.cinch     # typecheck, compile, link, and execute
cinch lsp                # run the shared analysis over LSP stdio
```

`cinch check` exits with status `1` on a diagnostic (`2` on CLI argument errors). `cinch run` returns the compiled program's own exit code once the program starts, so inspect stderr rather than exit status alone.

# Diagnostic ordering and message formats

Diagnostics are emitted in strict pipeline order—each phase must succeed before the next runs:

1. **Front end (lexer, layout engine, parser):** Reports syntax, layout, and attribute-lexing errors first, stopping before name resolution or typechecking.
2. **Module loader & checker (elaboration, types, effects, ownership, totality, proofs, safety assertions):** Reports whole-module import failures and span-located semantic diagnostics.
3. **Lowering, codegen, and linking (`emit-ir`, `emit-object`, `build`, `run`):** Reports entry-point requirements, target-profile or capability admission errors, and native/Wasm linker failures.

Each producer formats its messages distinctly, so the prefix immediately tells you which phase rejected the program:

| Format | Producer | Example |
| --- | --- | --- |
| `cinch: FILE: LINE:COL: message` | Lexer, layout engine, and parser | `cinch: Main.cinch: 5:1: expected an expression, found end of input` |
| `cinch: LINE:COL: message (in FILE)` | Checker and elaborator | `cinch: 4:8: type mismatch: expected String, got Int (in Main.cinch)` |
| `cinch: message` | Whole-module, import-graph, or driver rule without a single token span | `cinch: module Nonexistent.Module is imported, but Nonexistent.Module.cinch does not exist...` |

In interactive sessions, `cinch repl` wraps the same diagnostic with a source excerpt and caret:

```text
error: no instance for Add Bool
  at 1:3
  1 + True
    ^
```

Line and column numbers are **1-based**. Columns point to the start of the offending token, expression, or declaration (for example, the opening `"` of `"abc"`).

## Common parser diagnostics

Because the parser runs before name or type resolution, it names unexpected tokens by grammar class:

| Message | Typical cause |
| --- | --- |
| `expected an expression, found end of input` | Truncated expression or missing right-hand side at EOF. |
| ``expected `else`, found RBrace`` | An `if ... then ...` missing its `else` branch before the enclosing layout block ends. |
| ``expected `::`, found Where`` | `where` written where a type signature was expected. |
| ``expected `;` or `}` in the declaration block, found Equals`` | Explicit `{ ... }` block missing a separator or closing brace. |

# Checker diagnostics by category

| Category | Example message | Rule violated |
| --- | --- | --- |
| Type mismatch | `type mismatch: expected String, got Int` | Two unified types disagree (see the `expected`/`got` order note below). |
| Literal type | `integer literal cannot have type String` | Numeric literal does not inhabit the expected type. |
| Application | `cannot apply a value of type Int` | Non-function value in callee position. |
| Unknown name | `unknown name quad` | Identifier is not declared or imported in scope. |
| Unknown type | `unknown type Nope` | Type constructor is not declared or imported in scope. |
| Missing module | `module Nonexistent.Module is imported, but Nonexistent.Module.cinch does not exist` | Import cannot be resolved in sibling files, vendored packages, or `cinch/stdlib`. |
| Missing instance | `no instance for Pretty Color` | Required type-class constraint has no matching instance in scope. |
| Effect row | `effect rows do not match: {} vs {IO r#844 \| ?r894}` | Expression performs effects not permitted by its declared effect row. |
| Multiplicity | `multiplicity %1: x must be used exactly once on every path, but is never used` | Linear (`%1`) or affine (`%aff`) usage count violated. |
| Proof obligation | ``the obligation `n > 0` needs `Dec` evidence or a branch fact at this use`` | Refinement or index predicate not discharged by static facts or `decide`. |
| Exhaustiveness | `non-exhaustive patterns: constructor Green is not covered` | `case` expression or function clauses omit a reachable constructor. |
| Fixity | ``there is no fixity declared for the operator `<>`, and there is no default for an undeclared operator`` | Infix operator used without an `infix`/`infixl`/`infixr` declaration in scope. |
| Duplicate declaration | `duplicate signature for f` | Duplicate type signature or constructor name in one module (multiple defining equations for one function are valid clauses, not duplicates). |
| Field projection | `Main.P is not a record; field projection needs a record type` | `.field` applied to a `data` type rather than a `record`. |
| Attribute placement | `` `#[inline]` is a hint on a function declaration `` | Attribute attached to an unsupported construct. |
| Entry point | ``this program has no `main`; `emit-ir`, `build`, and `run` need `main :: Int` in the entry module`` | Executable/IR mode invoked on a library module, or `main` has an unsupported entry type. |

## Reading `expected` / `got` order

The unifier prints `type mismatch: expected <A>, got <B>` using the argument order passed at the checking site:

- At **function argument sites**, the parameter's declared domain is unified first and the argument's type second (`expected <param-type>, got <arg-type>`).
- At **bidirectional checking sites** (such as checking a definition body or expression against its declared type signature), the checker passes `(inferred, contextual)` to the unifier—so `main :: Int; main = "abc"` prints `expected String, got Int`.

Read both types alongside the column span rather than assuming `expected` is always the signature type.

## Internal metavariables in effect-row errors

Effect-row mismatches print internal region and row-tail metavariables (such as `r#844` and `?r894`) whose numeric suffixes are per-compile identifiers:

<!-- cinch:reject contains="effect rows do not match" -->
```cinch
module Main where

f :: forall r. (Unit ->{IO r} Unit) -> Unit -> Unit
f g x = g x

main :: Int
main = 0
```

```text
cinch: 4:1: effect rows do not match: {} vs {IO r#844 | ?r894} (in Main.cinch)
```

Focus on the effect labels (`IO`, `Async`, `Diverge`, etc.) rather than the generated suffixes.

# Worked diagnostics

## Type mismatch against a signature

<!-- cinch:reject contains="type mismatch: expected String, got Int" -->
```cinch
module Main where

main :: Int
main = "abc"
```

```text
cinch: 4:8: type mismatch: expected String, got Int (in Main.cinch)
```

Column `8` points to `"abc"`. Fix whichever side carries the wrong contract—either the signature or the body.

## Unknown name

<!-- cinch:reject contains="unknown name show" -->
```cinch
module Main where

main :: Int
main = show 1
```

```text
cinch: 4:8: unknown name show (in Main.cinch)
```

`show` is not re-exported by `Prelude`. Import the module defining the operation you want or use an explicit formatting function already in scope.

## Missing imported module

<!-- cinch:reject contains="is imported, but Nonexistent.Module.cinch does not exist" -->
```cinch
module Main where

import Nonexistent.Module

main :: Int
main = 0
```

```text
cinch: module Nonexistent.Module is imported, but Nonexistent.Module.cinch does not exist and no other source file beside the entry declares `module Nonexistent.Module`
```

All `cinch/stdlib` modules resolve automatically through the toolchain's package manifest; any other imported module must exist as a sibling `.cinch` file or come from an installed package under `vendor/`.

## Non-exhaustive pattern match

<!-- cinch:reject contains="non-exhaustive patterns: constructor Green is not covered" -->
```cinch
module Main where

data Color where
  Red :: Color
  Green :: Color

main :: Int
main = case Red of
  Red -> 0
```

```text
cinch: 8:8: non-exhaustive patterns: constructor Green is not covered (in Main.cinch)
```

Cover every reachable constructor (or match a refined value whose type excludes the missing branch).

## Missing class instance

<!-- cinch:reject contains="no instance for Pretty Color" -->
```cinch
module Main where

signature Pretty a where
  pretty :: a -> Int

data Color where
  Red :: Color

implicit instance PrettyInt :: Pretty Int where
  pretty n = n

use :: Color -> Int
use c = pretty c

main :: Int
main = 0
```

```text
cinch: 13:9: no instance for Pretty Color (in Main.cinch)
```

Declare an `implicit instance` for `Pretty Color`, or abstract over the constraint (`using (p :: Pretty Color) => ...`) so callers supply it.

# Isolating failures by compiler phase

Run the narrowest subcommand that reaches the failing stage:

| Command | Phases executed | Best used for |
| --- | --- | --- |
| `cinch check Main.cinch` | Lexer, layout, parser, checker | Syntax, imports, types, effects, ownership, and safety assertions (never invokes `clang`). |
| `cinch emit-ir Main.cinch -o out.ll` | `check` + specialization, RC lowering, LLVM emission | Separating typechecking from code-generation failures. |
| `cinch build Main.cinch -o prog` | `emit-ir` + `clang` / `wasm-ld` linking | External C/Wasm symbol and archive link issues. |
| `cinch run Main.cinch` | `build` + execution | Runtime exit codes and stdout/stderr behavior. |
| `cinch check Main.cinch --rebuild` | `check` + unconditional package-store rewrite | Repairing a suspect package-store cache entry (see [Chapter 18](18-artifacts-and-incremental.md)). |

Diagnostic spans point either to your source token or to the provenance site of a macro/staging expansion. When an error originates inside generated code, fix the originating generator or declaration rather than weakening downstream signatures.

See [Chapter 01](01-using-the-compiler.md) for all CLI flags and [Chapter 16](16-support-and-roadmap.md) for intentional language and target boundaries.

# Editor integration and analysis APIs

## Bundled editor support

The repository ships syntax grammars and editor configurations under `tooling/editors/` (not yet published to external marketplaces):

| Editor | Path | Provided features |
| --- | --- | --- |
| VS Code, Cursor | `tooling/editors/vscode/` | `.cinch` language definition, TextMate grammar, bracket/comment config, and LSP client |
| Vim | `tooling/editors/vim/` | Filetype detection (`ftdetect/cinch.vim`) and syntax highlighting (`syntax/cinch.vim`) |
| Emacs | `tooling/editors/emacs/cinch-mode.el` | `cinch-mode` (`prog-mode` derivative) with font-lock and `.cinch` `auto-mode-alist` |
| Sublime Text | `tooling/editors/sublime/Cinch.sublime-syntax` | Syntax definition for `.cinch` |
| Zed | `tooling/editors/zed/` | Extension manifest using `tooling/tree-sitter-cinch` |

For LSP launch commands (`cinch lsp` and `tooling/lsp-driver`), client configuration, supported JSON-RPC methods, and open-document behavior, see [Chapter 27](27-language-server.md). For interactive evaluation and `:t`/`:i`/`:browse` queries, see [Chapter 26](26-repl.md).

## Shared Rust analysis API (`cinch::analysis::Analysis`)

Custom tools and front ends can drive the incremental analyzer directly through `cinch::analysis::Analysis`:

| Method | Behavior |
| --- | --- |
| `set_document`, `remove_document` | Register or drop an in-memory buffer that overrides the on-disk file. |
| `update` | Incrementally recheck dirty modules and return an `UpdateReport` (`rechecked`, `kept`, `chain_ok`). |
| `file_diagnostics` | Return current diagnostics for a workspace-relative path. |
| `hover`, `definition`, `completions`, `kind_of`, `browse` | Type, symbol, and module queries used by the LSP and REPL. |
| `semantic_tokens` | Return classified token spans for one document. |

All paths returned by `Analysis` are workspace-relative; absolute paths appear only in URIs supplied by the caller.

# Reporting compiler bugs

Include the compiler commit/version, the exact command line, a minimal self-contained reproducer (including any sibling modules or `.wit` files), and the full diagnostic output. For LSP issues, include the editor client, workspace root, buffer save state, and failing request—redacting any staging secrets or private source text from protocol logs before sharing.
