# Editors, analysis APIs, and diagnostics

One analysis layer (`compiler/src/analysis.rs`) answers the checker's questions for three front ends: the command line, the language server, and the REPL. It keeps source identity, definition sites, diagnostics, and incremental recheck reports, so the three front ends do not run independent checkers that could disagree.

Three user-facing ways to see a diagnostic:

```sh
cinch check Main.cinch   # typecheck, no linker, fastest feedback
cinch run Main.cinch     # typecheck, build, run
cinch lsp                # the same analysis over stdio for an editor
```

`cinch check` returns exit status 1 when it reports a diagnostic. `cinch run` also returns the program's own exit status, so read the diagnostic text rather than the status alone.

# Message shape

The compiler prints three shapes, and which one you see tells you which part failed first.

| Shape | Producer | Example |
| --- | --- | --- |
| `cinch: LINE:COL: message (in FILE)` | the checker after parsing | `cinch: 4:8: type mismatch: expected String, got Int (in Main.cinch)` |
| `cinch: FILE: LINE:COL: message` | the parser and layout engine | `cinch: Main.cinch: 5:1: expected an expression, found end of input` |
| `cinch: message` | a whole-module or whole-program rule with no single span | `cinch: main must have type Int or Unit ->{e} Int in the bootstrap slice, got String` |

The checker form comes from `Error`'s `Display` in `compiler/src/lib.rs`, which writes `span: message`, plus `in_module` in `compiler/src/check_decls.rs`, which appends ` (in FILE)`. The REPL wrapper in `compiler/src/repl.rs` prints a fourth, more readable shape for a session input:

```text
error: no instance for Add Bool
  at 1:3
  1 + True
    ^
```

Line and column are one-based. The column points at the start of the failing expression, declaration, or token, not at a whole statement, so column 8 in `main = "abc"` is the opening quote.

The parser names tokens by their grammar class rather than by their source text, because it has not built an expression yet:

| Message | Cause |
| --- | --- |
| `expected an expression, found end of input` | the file ends in the middle of an expression |
| ``expected `else`, found RBrace`` | an `if` with no `else`, so the enclosing block closed first |
| ``expected `::`, found Where`` | `where` used where a signature was expected |
| ``expected `;` or `}` in the declaration block, found Equals`` | an explicit block was opened and never split or closed |

# What the checker reports

Every message below was produced by compiling a file in this section. The table groups them by the rule that failed, so a message you do not recognise can be placed before you read the rest of it.

| Category | Example message | The rule that failed |
| --- | --- | --- |
| Type mismatch | `type mismatch: expected String, got Int` | Two types were required to be equal and are not. The first type is what the expression produced; the second is what its position requires. |
| Literal type | `integer literal cannot have type String` | The literal's own defaulting meets an incompatible expected type. |
| Application | `cannot apply a value of type Int` | The head of an application is not a function. |
| Unknown name | `unknown name quad` | No declaration is visible under that name in this scope. |
| Unknown type | `unknown type Nope` | A type name in a signature is not declared or imported. |
| Missing module | `module Nonexistent.Module is imported, but Nonexistent.Module.cinch does not exist` | The import has no source to read. The full message continues with the sibling-file rule. |
| No instance | `no instance for Main.Pretty Main.Color` | A class constraint has no matching instance. |
| Effect row | `effect rows do not match: {} vs {IO r#844 \| ?r894}` | The body performs effects its declared row does not admit. See the note below on the leaked metavariables. |
| Multiplicity | `multiplicity %1: x must be used exactly once on every path, but is never used` | The use count violates the declared multiplicity. |
| Proof obligation | ``the obligation `n > 0` needs `Dec` evidence or a branch fact at this use`` | A refinement predicate is not discharged at the use. |
| Totality | `non-exhaustive patterns: constructor Green is not covered` | A `case` omits a constructor. |
| Fixity | ``there is no fixity declared for the operator `<>`, and there is no default for an undeclared operator`` | An operator was used with no `infix`, `infixl`, or `infixr` declaration in scope. |
| Duplicate declaration | `duplicate signature for f` | Two signatures, or two constructors of the same name, exist in one module. |
| Projection | `Main.P is not a record; field projection needs a record type` | `.field` was applied to something declared with `data`. |
| Attribute placement | `` `#[inline]` is a hint on a function declaration `` | An attribute sits on a declaration kind it does not apply to. |
| Entry point | `main must have type Int or Unit ->{e} Int in the bootstrap slice, got String` | The entry point's type is not one the runtime can start. |
| Missing entry point | ``this program has no `main`; `emit-ir`, `build`, and `run` need `main :: Int` in the entry module`` | Only `run`, `build`, and `emit-ir` need an entry point; `check` does not. |

An effect-row mismatch has its own shape, because the row is printed as a set:

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

The two names are inference variables in their internal form, not source names. They are stable for one compile, so match on the marker (`IO`) rather than on the digits. This is a rough edge in the message text, not in the check.

A repeated equation for one name is not a duplicate definition. `f 0 = 0` and `f n = n` are two clauses of one function, and the checker accepts them.

# Worked diagnostics

Each example below is a complete file followed by the exact message `cinch check` printed for it.

A declared result type that disagrees with the body:

<!-- cinch:reject contains="type mismatch: expected String, got Int" -->
```cinch
module Main where

main :: Int
main = "abc"
```

```text
cinch: 4:8: type mismatch: expected String, got Int (in Main.cinch)
```

The fix is to change the body or the declared type, not to add a coercion. Fix the side whose contract is wrong.

A name that is not in scope:

<!-- cinch:reject contains="unknown name show" -->
```cinch
module Main where

main :: Int
main = show 1
```

```text
cinch: 4:8: unknown name show (in Main.cinch)
```

`show` is not in the prelude facade. Import the module that declares the operation you meant, or write the conversion from the functions already in scope.

An `import` with no source file beside it:

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

This message has no position because the failure is the import edge, not a token. The prelude family is embedded in the compiler and needs no file; a module outside that family must exist as a sibling `.cinch` file or arrive through a package.

A `case` that omits a constructor:

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

Add the missing alternative, or match a value that cannot be `Green`.

A `signature` whose class has no instance for the type used:

<!-- cinch:reject contains="no instance for Main.Pretty Main.Color" -->
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
cinch: 13:9: no instance for Main.Pretty Main.Color (in Main.cinch)
```

Write `implicit instance PrettyColor :: Pretty Color where` beside the type, or add the constraint to `use` and let the caller supply it.

# Read diagnostics by phase

Use the narrowest command that reaches the failing phase.

| Command | What it runs | Use it when |
| --- | --- | --- |
| `cinch check Main.cinch` | lexer, layout, parser, checker | isolating a front-end failure, or a fast edit loop |
| `cinch emit-ir Main.cinch -o out.ll` | everything `check` runs, then code generation | separating an accepted program from a code-generation failure |
| `cinch build Main.cinch -o prog` | everything, including the linker | a link or external symbol problem |
| `cinch run Main.cinch` | everything, then the program | the program's own exit status matters |
| `cinch check Main.cinch --rebuild` | as `check`, and rewrite the package-store entry for this compile even when the existing entry is intact | the stored entry for the base package is suspect |

`--rebuild` is a store-repair switch, not a checker switch. The compile path keeps an existing entry only when `Store::load` succeeds and the entry agrees with exactly the artifact set this compile produced; anything else is repaired by writing, so an ordinary compile never fails because of a stale entry. `--rebuild` skips that comparison and writes unconditionally. The loader's hard errors (a truncated interface, a mismatched key) reach callers that consume the store, which no command-line mode does today.

`check` never invokes clang, so a clang or linker failure can only appear at `build` or `run`. A program with no `main` passes `check` and fails `build`, which is the standard way to tell "my module is fine, the entry point is not".

The [compiler guide](01-using-the-compiler.md) covers the subcommands and flags in full. The [support page](16-support-and-roadmap.md) lists the refusals that are deliberate boundaries rather than regressions, so a named refusal can be distinguished from a bug.

A diagnostic's span is attached to the failing source or to the provenance of a generated expansion. When a message appears inside generated or expanded code, fix the originating declaration rather than weakening a signature to silence it.

# Editor setup that exists

The repository contains syntax and language-client files, and nothing is published to an extension marketplace.

| Editor | Files | What they provide |
| --- | --- | --- |
| VS Code, Cursor | `tooling/editors/vscode/` | language id `cinch` for `.cinch`, a TextMate grammar, bracket and comment configuration, and a language client |
| Vim | `tooling/editors/vim/` | `ftdetect/cinch.vim` (the `.cinch` suffix) and `syntax/cinch.vim` |
| Emacs | `tooling/editors/emacs/cinch-mode.el` | `cinch-mode`, a `prog-mode` derivative with font-lock rules, `-- ` comments, and an `auto-mode-alist` entry for `.cinch` |
| Sublime Text | `tooling/editors/sublime/Cinch.sublime-syntax` | a syntax definition with `file_extensions: [cinch]` |
| Zed | `tooling/editors/zed/` | an extension manifest pointing at `tooling/tree-sitter-cinch`, plus indentation, bracket, and completion word rules |

The VS Code extension is the only one that starts a language client. Its `package.json` declares four contributions: the `cinch` language for `[".cinch"]`, the grammar `source.cinch`, the `cinch.lsp.command` setting, and the `cinch.lsp.trace` setting. `extension.js` reads `cinch.lsp.command`, falls back to `DEFAULT_COMMAND` (`bin/cinch-lsp`), and starts a `LanguageClient` with `documentSelector: [{ scheme: 'file', language: 'cinch' }]`.

`bin/cinch-lsp` is a shell launcher. It execs `cinch lsp` when `cinch` is on `PATH`, and otherwise runs the driver crate:

```sh
cargo run --quiet --manifest-path tooling/lsp-driver/Cargo.toml --release -- "$@"
```

`cinch.lsp.command` is one executable path. The client passes it as `{ command: command }` with no argument list, so a value containing a space is a single path and will not work. Point it at a launcher if you need arguments:

```sh
#!/bin/sh
exec /absolute/path/to/compiler/target/debug/cinch lsp
```

Two settings notes, both read from the files rather than assumed:

- The `cinch.lsp.command` description in `package.json` still says the compiler binary has no `lsp` subcommand. It does: `cinch lsp` is listed in `cinch --help` and is covered by a test. Trust the binary and the launcher, not that description.
- `package.json` declares `cinch.lsp.trace`, and `extension.js` never reads it. Changing the setting does not enable tracing.

The TextMate grammar declares `"fileTypes": ["lang"]` and the Zed language config declares `path_suffixes = ["lang"]`, while the VS Code language contribution, Vim detection, Emacs mode, and Sublime syntax all use `.cinch`. If highlighting does not attach in Zed or through the bare grammar, add `cinch` to that list in your copy.

# Analysis APIs

`cinch::analysis::Analysis` is the shared layer. It holds open documents, checked module state, definition sites, and the last `UpdateReport`. The language server and REPL both drive it:

| Call | Answer |
| --- | --- |
| `set_document`, `remove_document` | open or drop an in-memory buffer that wins over disk |
| `update` | recheck, returning an `UpdateReport` with `rechecked`, `kept`, and `chain_ok` |
| `file_diagnostics` | diagnostics for one workspace-relative file |
| `hover`, `definition`, `completions`, `kind_of`, `browse` | the queries behind the editor and REPL answers |
| `semantic_tokens` | classified spans for one file |

`UpdateReport` is the incremental report: `rechecked` names the modules the update re-ran, `kept` names the cached interfaces it reused, and `chain_ok` records that the merged interface chain was re-verified. Both front ends report the same lists, so a slow edit and a slow reload are explained by the same data.

Two boundaries that matter when writing a front end:

- The checker's spans carry a line and a column only, with no file. Each file is parsed behind a fixed line band and the attribution is removed at the output boundary, which is why `check` can name the file in `(in FILE)` and the analysis can answer with workspace-relative keys.
- Every path the analysis produces is workspace-relative. Absolute paths appear only in a document URI that the client itself supplied.

The [language server guide](27-language-server.md) covers the protocol; the [REPL guide](26-repl.md) covers the interactive front end.

# Reporting a compiler bug

Include the compiler checkout, the exact command, the minimal source and any sibling files the import graph needs, and the complete diagnostic including its `cinch:` line. For editor problems, add the client and version, the workspace root you selected, whether the buffer was saved, and the request or action that failed.

Staging input and process environments can carry secrets. Remove them from the source and from protocol traces before posting; protocol traces contain full document text.
