# Interactive programming with the REPL

<span class="support-tag support-tag--current">Current</span> **`cinch repl` starts an interactive session.** Connected to a terminal, it launches a full-screen TUI; with standard input or output redirected, it runs a plain line loop. Both frontends drive the same `Repl` session backed by the compiler's own type checker, evaluator, and module loader.

Start a session from your project root:

```sh
cinch repl
```

The current working directory is the session's project root. Imports and `:load` resolve files using the same module rules as `cinch check` and `cinch run`. The REPL never downloads or installs packages.

# Frontend selection

At startup, `cinch repl` chooses its frontend once:

```rust
if stdin.is_terminal() && stdout.is_terminal() {
    repl_tui::run()                                    // full-screen TUI
} else {
    run_with_root(current_dir, stdin, stdout, false)   // plain line loop
}
```

Two details matter when scripting or redirecting output:

- Redirecting **either** stream selects the plain line loop (for example, `cinch repl > log.txt`).
- The plain line loop runs with `interactive: false`, printing bare responses with no banner and no `cinch> ` prompts. It reads until EOF and exits `0` on normal shutdown (or `1` on startup failure).

# Walkthrough session

The transcript below illustrates definitions, expressions, multi-line continuations, data declarations, query commands, and a type error. Input lines are prefixed with `>` for readability (piped sessions emit only the output lines):

```text
> x :: Int
noted x (add its definition)
> x = 5
defined x :: Int
> x + 2
7 :: Int
> double :: Int -> Int
noted double (add its definition)
> double n = n + n
defined double :: (Int -> Int)
> double 21
42 :: Int
> result = if True then
>   10 else 20
defined result :: Int
> result
10 :: Int
> total = (1 +
>   2) * 3
defined total :: Int
> total
9 :: Int
> data Colour where { Red :: Colour; Green :: Colour }
defined Colour :: Type
defined Green :: Colour
defined Red :: Colour
> :type double
(Int -> Int)
> :type x + 2
Int
> :kind Colour
Colour :: Type
> :browse
Colour :: type Colour :: data
Green :: Colour
Red :: Colour
double :: (Int -> Int)
result :: Int
total :: Int
x :: Int
> :history
1: x :: Int
...
18: :history
> :help
:type <expr>, :kind <type>, :browse [module], :load <file>, :reload, :history, :quit
> 1 + True
error: no instance for Add Bool
  at 1:3
  1 + True
    ^
> :quit
```

# Definitions and expressions

Each submitted input produces one of four outcomes:

| Output | Meaning |
| --- | --- |
| `defined NAME :: TYPE` | A declaration was checked and added to the session. |
| `noted NAME (add its definition)` | A type signature was recorded and awaits its defining equation. |
| `VALUE :: TYPE` | An expression was evaluated; no binding was added. |
| `error: ...` | The input was rejected; no binding was added. |

Provide explicit signatures for polymorphic, constrained, effectful, or dependent definitions—the REPL enforces the same signature rules as source files. Note that rendered arrow schemes include outer parentheses (such as `(Int -> Int)`); omit them when copying signatures into source files.

To move a session's definitions into a reproducible program, save them in a `.cinch` file for `cinch check` and `cinch run`:

<!-- cinch:run exit=0 -->
```cinch
module Main where

double :: Int -> Int
double n = n + n

main :: Int
main = double 21 - 42
```

# Multi-line input

The line loop continues reading when the current buffer is lexically incomplete: an unclosed `(`, `[`, `{`, `|]`, or `"`, or a trailing token that cannot finish a declaration (`where`, `of`, `do`, `then`, `else`, `in`, `=`, `->`, `=>`, `<-`, `,`, `|`, an operator, `forall`, `using`, `let`, `letrec`, or `if`).

```text
result = if True then
  10 else 20
```

Because completeness is tested after every line, a multi-constructor `data` block written with indentation alone commits after its first constructor:

```text
> data Colour where
>   Red :: Colour
defined Colour :: Type
defined Red :: Colour
>   Green :: Colour
error: unknown name Green
  at 1:1
  Green :: Colour
  ^
```

In the line loop, use explicit braces for multi-constructor declarations:

```text
data Colour where { Red :: Colour; Green :: Colour }
```

In the full-screen TUI, bracketed paste inserts newlines into the input buffer and `Enter` submits the entire block at once.

# Commands

Commands begin with `:` at the start of an input:

| Command | Alias | Action |
| --- | --- | --- |
| `:type EXPR` | `:t` | Elaborate `EXPR` and print its type scheme without executing it. |
| `:kind NAME` | `:k` | Print the kind of a type constructor or class declared in the session or loaded modules. |
| `:browse` | `:b` | List names and schemes in scope across the session and loaded modules. |
| `:browse MODULE` | `:b MODULE` | List names and schemes exported by `MODULE`. |
| `:load FILE` | `:l FILE` | Load a source entry file and its local module closure. |
| `:reload` | `:r` | Re-read and recheck all loaded source files. |
| `:history` | `:hist` | Print numbered session inputs (including commands and errors). |
| `:help` | `:h` | Print the command summary line. |
| `:quit` | `:q` | Exit the session. |

Unknown commands report `error: unknown command :name` without disturbing session state; missing required arguments print a `usage:` line (for example, `usage: :type <expression>`).

Notes on query commands:

- `:type` elaborates without running code, so it never triggers I/O or foreign calls.
- `:kind` reports arity for polymorphic constructors (e.g. `Shape :: Type -> Type`), but currently only resolves types declared in the session or loaded modules; querying a prelude type such as `:kind List` returns `error: no type or class named List`.
- `:browse` lists only loaded modules and session bindings; querying an unloaded module prints nothing.
- `:history` is kept in memory for the current process only; there is no history file or replay command.

# Loading and reloading projects

`:load` loads an entry file together with the local modules it imports, printing each module checked:

```text
> :load Main.cinch
loaded Main.cinch
rechecked Main
rechecked Util
> quad 21
84 :: Int
```

Subsequent expression probes reuse the checked project without rechecking unchanged files.

Reloading is **transactional**. If an edit introduces a type error, `:reload` reports the diagnostic and leaves the last valid session live:

```text
> :reload
error: reload failed; kept the previous checked session
Util.cinch:4: type mismatch: expected String, got Int (in Util.cinch)
> quad 21
84 :: Int
```

Once the file is repaired, `:reload` rechecks the project alongside `ReplSession` (the synthetic module holding interactive bindings):

```text
> :reload
rechecked Main
rechecked ReplSession
rechecked Util
```

Missing files follow the same transactional rule (`error: Missing.cinch: No such file or directory (os error 2); kept the previous checked session`). If the previous session also fails to recheck after a failed load, the REPL prints an additional `error: previous session also fails: ...` line.

`:load` and `:reload` operate on local source files; external packages must be installed first with `cinch pkg`.

# Diagnostics and recovery

Rejected inputs print the diagnostic message, block-relative `line:column`, offending source line, and caret:

```text
error: integer literal cannot have type Bool
  at 1:1
  9
  ^
```

**Known limitation (literal defaulting after a type error):** an input that fails during numeric/instance constraint solving (such as `1 + True`) can leave a stale literal-defaulting constraint in the session, causing subsequent bare numeric literals like `9` to fail with `integer literal cannot have type Bool`. Unrelated errors (such as unknown names) recover cleanly. If you hit stale literal defaulting, start a fresh session.

# Full-screen terminal frontend

When attached to a TTY, `cinch repl` enters raw mode, the alternate screen, and bracketed paste (restoring all three on normal exit and on panic).

## Layout

| Region | Height | Contents |
| --- | --- | --- |
| Header | 3 rows | `cinch / interactive`, session root path, status indicator (`ready` or `error`), and definition count |
| Body | flexible | Scrollable transcript on the left; 34-column `Session` sidebar on the right listing active bindings and command hints |
| Input | 3 to 8 rows | Multi-line editor titled `Input` with `cinch> ` / `...> ` prompts |
| Completion | 0 or 2 to 8 rows | `Complete` popup showing up to 6 candidates while completing |
| Status | 1 row | Key-hint bar (`Enter run · Tab complete · ↑↓ history · Ctrl+J newline · Ctrl+C clear · Ctrl+D quit`) |

Transcript output is colour-coded by outcome (`error` in red; `defined`, `loaded`, and `rechecked` in green; `noted` in yellow), and Cinch expressions use the compiler's tree-sitter syntax highlighting.

## Key bindings

| Key | Action |
| --- | --- |
| `Enter` | Submit input buffer, or accept the highlighted completion |
| `Alt+Enter`, `Ctrl+J` | Insert a newline |
| `Tab`, `Shift+Tab` | Start completion or cycle candidates forward/backward |
| `Up`, `Down` | Navigate input history (or cycle completions when popup is open) |
| `Esc` | Dismiss the completion popup |
| `Left`, `Right`, `Home`, `End`, `Ctrl+A`, `Ctrl+E` | Move the cursor |
| `Ctrl+U` | Clear the current input buffer |
| `Backspace`, `Delete` | Delete character before or at the cursor |
| `PageUp`, `PageDown` | Scroll the transcript by 8 lines |
| `Ctrl+L` | Clear the transcript and reset scroll |
| `Ctrl+C` | Discard current input and any pending multi-line block (or quit if empty) |
| `Ctrl+D` | Quit |

`Tab` completion queries commands after `:` and session, imported, and prelude identifiers otherwise. A single match inserts inline; multiple matches open the completion popup.

Evaluation runs synchronously on the UI loop, so `Ctrl+C` cancels pending input rather than interrupting an already-running type check or native evaluation probe.

# Embedding API (`cinch::repl::Repl`)

Both frontends wrap `cinch::repl::Repl`:

| Method | Purpose |
| --- | --- |
| `Repl::with_root(path)` | Create a session rooted at `path` |
| `Repl::with_compiler(root, binary)` | Create a session using an explicit compiler binary for native probes |
| `eval(&str)` | Submit an input; returns `ReplOutcome::Output(String)`, `Incomplete`, or `Quit` |
| `complete(prefix)` | Return completion candidates for `prefix` |
| `history()` | Access submitted inputs |
| `session_source()` | Retrieve the synthesized `ReplSession` source text |
| `abort_pending()` | Discard an unfinished multi-line buffer |
| `analysis()` | Access the underlying shared analysis state |

Callers must treat `ReplOutcome::Incomplete` as a request for additional lines rather than a completed declaration or error.

# Feature status

| Capability | Status |
| --- | --- |
| Full-screen TUI and piped line loop | **Available** |
| Persistent in-session bindings, pending signatures, imports, history | **Available** |
| `:type`, `:kind`, `:browse`, `:history`, `:help` | **Available** (`:kind` excludes prelude types) |
| Transactional `:load` and `:reload` | **Available** |
| Expression evaluation (interpreter + native probe) | **Available** |
| Prefix completion (commands, session, imports, prelude) | **Available** |
| Literal defaulting recovery after failed numeric constraint | **Partial** (stale constraint can require a fresh session) |
| Persistent cross-session history / replay | **Not implemented** |
| Interrupting a running evaluation with `Ctrl+C` | **Not implemented** |

See [language server and editor integration](27-language-server.md) for the editor frontend over the same analysis layer, and [using the compiler](01-using-the-compiler.md) for file-based builds.
