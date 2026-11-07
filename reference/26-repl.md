# Interactive programming with the REPL

<span class="support-tag support-tag--current">Available</span> **`cinch repl` starts an interactive session.** In a terminal it provides a full-screen frontend; when input or output is redirected it uses a plain line loop. Both use the compiler's shared analysis layer, keep declarations across inputs, and run expressions through the D92 evaluator. Foreign and IO expressions fall back to a temporary native probe; reload rolls back broken revisions. [Current implementation](#current-implementation) records the remaining limits.

The REPL is an interactive Cinch session. Enter an expression to inspect its value and type, add declarations to use in later inputs, or load a project and explore its exported functions. It uses the compiler's analysis layer, so signatures, instances, effects, refinements, and ownership follow the same rules as source files.

# Start a session

Start from the directory containing your project:

```sh
cinch repl
```

The working directory establishes the session's project root. Source files and their imports are resolved using the compiler's module rules. Starting a REPL does not install dependencies or download packages.

# Terminal and piped sessions

When both standard input and standard output are terminals, `cinch repl` starts the full-screen frontend. It uses raw mode, the alternate screen, and bracketed paste, and restores the terminal on exit or panic. The terminal must support those controls; the frontend is not a fallback for a line-only console.

When either stream is redirected, `cinch repl` keeps the same session loop but prints no prompts and uses no alternate screen. This mode is useful for scripted transcripts and tests:

```sh
printf '1 + 2\n:quit\n' | cinch repl
```

```text
3 :: Int
```

Piped input has no line editing or completion popup. Each line is submitted as it arrives, and an unfinished block continues on later lines exactly as in the terminal. End-of-input ends the session. The process exits 0 after a normal session; a startup failure exits 1.

In the following **illustrative transcript**, `cinch>` is the input prompt and `...>` is a continuation prompt. Neither is part of Cinch source. Output wording and prompt spelling may change.

```text
cinch> 1 + 2
3 :: Int
cinch> x :: Int
noted x (add its definition)
cinch> x = 5
defined x :: Int
cinch> x + 2
7 :: Int
cinch> :type x + 2
Int
cinch> :quit
```

Evaluation shows a value and its type when the evaluator can produce a printable result. Type checking and evaluation are distinct outcomes: an accepted type does not guarantee that the interactive evaluator supports the expression's runtime operations. A refusal should explain that limit without presenting a type as a computed value.

# Expressions and declarations

An expression evaluates against the accepted declarations and imports in the current session. An expression result does not implicitly introduce a reusable name; give a result an explicit binding if you need it later.

Declarations extend the session. A signature can be entered before its defining equation, or both can be submitted as one block. The signature is pending until its definition arrives. Prefer explicit signatures for polymorphic functions, constrained definitions, effects, and dependent types; the REPL is not an alternative set of inference rules.

```text
cinch> double :: Int -> Int
noted double (add its definition)
cinch> double n = n + n
defined double :: Int -> Int
cinch> double 21
42 :: Int
```

Convenient inference for simple bare equations is intended to remain available. Where the checker cannot synthesize a stable surface signature, supply one rather than relying on a hidden metavariable or an ambiguous constraint.

Ordinary duplicate-definition and scope rules still apply. A fresh session is the reliable way to discard exploratory declarations. Rebinding a name must not silently change the meaning of earlier checked declarations or leave their dependent evidence stale.

# Multiline input and terminal editing

An unfinished expression continues at the next prompt. Indentation within the input remains meaningful.

```text
cinch> result = if True then
...>   10 else 20
defined result :: Int
cinch> result
10 :: Int
cinch> total = (1 +
...>   2) * 3
defined total :: Int
cinch> total
9 :: Int
```

The terminal frontend accepts a pasted declaration block as one submission, including signatures, equations, and local `where` declarations. A data declaration with several constructors must be checked as a complete block, rather than committing after the first constructor and treating the rest as unrelated input. Explicit block delimiters and indentation retain their normal language meanings.

At a fresh prompt, completion includes visible session, imported, and prelude names; after `:`, it offers session commands. Completion helps enter a name and does not make an otherwise out-of-scope name available.

The full-screen frontend provides line editing, history, paste, Tab completion, and syntax highlighting. Its key bindings are:

| Key | Action |
| --- | --- |
| `Enter` | Submit the input, or accept the highlighted completion |
| `Ctrl+J`, `Alt+Enter` | Insert a newline |
| `Tab`, `Shift+Tab` | Complete; cycle completion candidates |
| `Up`, `Down` | History, or cycle completion candidates |
| `Left`, `Right`, `Home`, `End` | Move the input cursor |
| `PageUp`, `PageDown` | Scroll the transcript |
| `Ctrl+L` | Clear the transcript |
| `Ctrl+C` | Discard pending input and any unfinished block; at an empty prompt, quit |
| `Ctrl+D` | Quit |

Evaluation is synchronous. `Ctrl+C` cancels pending input, not a check or evaluation already running.

# Command reference

Commands start with `:` at a fresh prompt. Their arguments are not shell commands. Enter `:help` to inspect the commands supported by the running version.

| Command | Alias | Meaning |
| --- | --- | --- |
| `:type expression` | `:t` | Elaborate an expression in the session and display its type scheme, or its diagnostic. |
| `:kind name` | `:k` | Display the kind of a named type constructor or class, such as `List`. |
| `:browse` | `:b` | List names and their type schemes from the session and known modules. |
| `:browse Module` | `:b Module` | Restrict the listing to a named module. |
| `:load file.cinch` | `:l file.cinch` | Load a source project and make the loaded modules available to the session. |
| `:reload` | `:r` | Read loaded sources again and recheck the affected project. |
| `:history` | `:hist` | Display this session's submitted inputs with entry numbers. |
| `:help` | `:h` | Display command help. |
| `:quit` | `:q` | End the session. |

An omitted required argument should produce usage information. An unknown command produces a diagnostic and leaves the session usable. These commands and aliases are available in the terminal frontend.

## Inspect types and kinds

```text
cinch> :type double
Int -> Int
cinch> :kind List
List :: Type -> Type
```

Type queries use the session's imports and instance environment. An unresolved name, ambiguous instance, or unmet proof obligation remains an error. `:kind` takes a type or class name; arbitrary type-expression queries are not part of this command's current API contract.

`:type` uses the shared probe and does not start a native runtime probe, so asking for a type never executes a foreign or IO action. The D92 probe may reduce pure expressions while elaborating them; that is checking work, not an effectful run.

## Browse names

Use `:browse` to discover a loaded function's actual signature before calling it. A module listing is a type-oriented index, not a replacement for the [library reference](../library/00-index.md), which also explains declarations and dependencies. Seeing a name in a known module's listing does not bypass that module's export/import rules.

## Load and reload a project

Suppose `Util.cinch` contains a module `Util` exporting `double :: Int -> Int`, and `Main.cinch` imports it and exports `quad :: Int -> Int`, where `quad n = double (double n)`.

```text
cinch> :load Main.cinch
loaded Main.cinch
rechecked Util
rechecked Main
cinch> quad 21
84 :: Int
cinch> :reload
rechecked Util
rechecked Main
```

The transcript illustrates the workflow; the actual affected set depends on the files and dependency changes. Loading follows source module identities and imports. A source-distributed library still needs its files available locally; loading a module is not a remote package installation operation. `:load` is transactional too: a missing import, parse error, or check error restores the previous workspace instead of leaving a partial project loaded.

Reload is transactional. If the new revision has a type error, a broken import edge, or a missing loaded source, `:reload` reports the failure and restores the previous in-memory revision instead of leaving the session on a half-replaced interface chain. Evaluation continues against the last coherent project until the source is repaired and reloaded.

For larger experiments, move declarations into a source module and reload it. That makes imports, signatures, and dependencies reproducible outside the session. Use [`cinch check`](01-using-the-compiler.md) to check the saved program and `cinch run` to exercise its native runtime path.

## History

History records submitted inputs, including commands and unsuccessful submissions. It is useful for retrieving an expression, not evidence that every entry checked or ran successfully. History is separate from the accepted declaration environment.

The current API keeps history in memory. Cross-session persistence, replay commands, and an on-disk history location are not established interfaces. A future persistent history facility needs explicit treatment of sensitive input; this guide does not invent a storage path or promise automatic persistence.

# Effects, ownership, and evaluation

The REPL must retain normal Cinch checking rules. An effectful expression needs its effect row, capability supply, and permitted execution context. A linear resource cannot become reusable just because it was entered at a prompt. Refinement evidence must remain attached to the values and assumptions that justified it, and must be invalidated when those assumptions change.

Evaluation starts with the compiler's D92 evaluator. When that evaluator cannot run a construct because it needs target code (a foreign import, an I/O operation, or a runtime-only value), the REPL writes a temporary `ReplProbe` program, copies the loaded project sources, and runs the project through the same native compiler. The native probe has a 60 second timeout and does not inherit the terminal's stdin, so a probe that needs input sees end-of-input. It runs with the project root as its working directory, so a relative filesystem path in a probe means the project, not the temporary probe directory. Imports accepted in the session are imported into the probe as well.

The probe prints the result when the type has an obvious printer: `Int`, `Nat`, `Bool`, `Char`, `String`, `Unit`, and `Bytes`. Other result types report `<value>` after running, because the REPL will not invent a `Show` instance; a must-use result is discarded with `ignore`. An effectful action still needs its capability, exactly as in a compiled program; supply it lexically with `with (io = ...) in ...`. Asking for a type with `:type` does not execute the expression.

Runtime-backed evaluation is deliberately bounded. It does not yet cover arbitrary effect capabilities, interactive stdin, or interruption of a running probe; use `cinch run` for a broader runtime workflow. Support for a construct in the compiler does not by itself establish support for evaluating it interactively.

# Errors and recovery

A failed input should show its source location, the relevant input, and the checker or evaluator's explanation. A rejected declaration does not become an accepted session binding. After a pending signature and an invalid equation, a corrected equation can still complete the definition.

When diagnosing a failure, separate three cases:

| Failure | Next step |
| --- | --- |
| The input has a syntax, type, effect, ownership, or proof error. | Correct the input using the same rules as a source file. |
| A loaded module fails. | Fix the named file or import, then reload. |
| The expression checks but the native probe also fails. | Read the probe diagnostic; save a small program and use `cinch run` for a broader runtime workflow. |

Include the compiler revision, minimal sequence of inputs, required source files, and complete diagnostic when reporting a session bug. See [diagnostics](21-editors-and-diagnostics.md) for phase-specific checks.

# Current implementation

| Surface | Present boundary |
| --- | --- |
| `cinch repl` and terminal UI | **Available.** Full-screen ratatui frontend with transcript, session sidebar, input editor, completion popup, history, and tree-sitter token colours. Piped stdin uses a plain line loop. |
| Stateful session | **Available.** `cinch::repl::Repl` retains declarations, pending input, imports, and in-memory history. |
| Commands | The command table above is available in the terminal frontend. |
| Checking and pure expression evaluation | Implemented through shared analysis and the D92 evaluator, within that evaluator's admitted subset. |
| Completion | `Tab` cycles session, imported, prelude, and `:` command names. |
| Runtime-backed evaluation | **Available for target-code probes.** D92 is the fast path; foreign/IO/runtime-only expressions fall back to a temporary native `ReplProbe` with a 60 second timeout and project source copy. |
| Project reload | **Transactional.** A broken or missing revision is rejected and the previous checked revision is restored. |
| Complete project/runtime session | **Partial.** Arbitrary effect capabilities, interactive stdin, full value printers for every ADT, and interruption of a running probe remain ahead. |

`cinch repl` is the terminal adapter. Frontend authors can also create `Repl::with_root(path)` and feed submissions to `eval(&str)`; `Repl::with_compiler(root, cinch_binary)` selects the executable used by runtime probes. Its result is `ReplOutcome::Output(String)`, `Incomplete`, or `Quit`. `Incomplete` requests more input; it is not a successful declaration. The API also exposes `complete`, `history`, `session_source`, `abort_pending`, and `analysis`.

Implementation: `compiler/src/repl.rs`, `compiler/src/analysis.rs`; behaviour suite: `compiler/tests/repl.rs`. The [language server guide](27-language-server.md) describes the other frontend sharing this analysis layer.
