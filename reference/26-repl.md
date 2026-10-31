# Interactive programming with the REPL

<span class="support-tag support-tag--planned">Coming soon</span> **This chapter describes the intended terminal REPL.** The commands, prompts, editing behaviour, and project workflow below form the target user experience. They are not a claim that `cinch repl` works in today's executable. The Rust implementation already has a stateful REPL API; [current implementation](#current-implementation) records that boundary.

The REPL is an interactive Cinch session. Enter an expression to inspect its value and type, add declarations to use in later inputs, or load a project and explore its exported functions. It uses the compiler's analysis layer, so signatures, instances, effects, refinements, and ownership follow the same rules as source files.

# Start a session

The intended invocation, from the directory containing your project, is:

```sh
cinch repl
```

The working directory establishes the session's project root. Source files and their imports are resolved using the compiler's module rules. Starting a REPL does not install dependencies or download packages.

In the following **illustrative transcript**, `cinch>` is the input prompt and `...>` is a continuation prompt. Neither is part of Cinch source. Output wording and prompt spelling may change before the terminal frontend ships.

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

The intended terminal frontend accepts a pasted declaration block as one submission, including signatures, equations, and local `where` declarations. A data declaration with several constructors must be checked as a complete block, rather than committing after the first constructor and treating the rest as unrelated input. Explicit block delimiters and indentation retain their normal language meanings.

Terminal integration should provide line editing, history navigation, and Tab completion. At a fresh prompt, completion includes visible session, imported, and prelude names; after `:`, it offers session commands. Completion helps enter a name and does not make an otherwise out-of-scope name available.

**Coming soon: interruption and input control.** Ctrl-C should discard an unfinished submission or interrupt evaluation and return to a usable prompt. End-of-input at an empty prompt should close the session cleanly. Interruption must respect resource cleanup and structured task lifetimes; it must not fabricate an ordinary return value or proof of completion. These terminal behaviours are frontend requirements, not controls supplied by the current Rust API.

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

An omitted required argument should produce usage information. An unknown command should produce a diagnostic and leave the session usable. Command aliases in this table already exist in the Rust API; the terminal interface is still coming soon.

## Inspect types and kinds

```text
cinch> :type double
Int -> Int
cinch> :kind List
List :: Type -> Type
```

Type queries use the session's imports and instance environment. An unresolved name, ambiguous instance, or unmet proof obligation remains an error. `:kind` takes a type or class name; arbitrary type-expression queries are not part of this command's current API contract.

The intended type-query interface reports types without performing runtime effects. The existing implementation uses a shared expression probe which can attempt evaluation; a strict separation of query and execution remains part of completing the frontend.

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

The transcript illustrates the workflow; the actual affected set depends on the files and dependency changes. Loading follows source module identities and imports. A source-distributed library still needs its files available locally; loading a module is not a remote package installation operation.

The intended reload operation publishes one coherent checked project revision. If an edit breaks a signature or invalidates a session declaration, the REPL should report the affected source and prevent evaluation against a mixture of incompatible revisions. Repair the source and reload before continuing. Transactional reload, recovery from broken imports, and dependent-session invalidation are completion requirements; today's API should not be assumed to provide that entire experience.

For larger experiments, move declarations into a source module and reload it. That makes imports, signatures, and dependencies reproducible outside the session. Use [`cinch check`](01-using-the-compiler.md) to check the saved program and `cinch run` to exercise its native runtime path.

## History

History records submitted inputs, including commands and unsuccessful submissions. It is useful for retrieving an expression, not evidence that every entry checked or ran successfully. History is separate from the accepted declaration environment.

The current API keeps history in memory. Cross-session persistence, replay commands, and an on-disk history location are not established interfaces. A future persistent history facility needs explicit treatment of sensitive input; this guide does not invent a storage path or promise automatic persistence.

# Effects, ownership, and evaluation

The REPL must retain normal Cinch checking rules. An effectful expression needs its effect row, capability supply, and permitted execution context. A linear resource cannot become reusable just because it was entered at a prompt. Refinement evidence must remain attached to the values and assumptions that justified it, and must be invalidated when those assumptions change.

**Coming soon: complete runtime-backed interactive execution.** The intended session distinguishes inspecting a pure value from executing an effectful computation. An effectful action runs only as an explicit evaluation with the required capabilities; merely browsing or asking for its type must not execute it. Files, foreign calls, tasks, and other runtime facilities must use the same contracts and cleanup rules as compiled programs.

Until that path is complete, use `cinch run` for runtime I/O, foreign integration, concurrency, and native representation tests. The current REPL evaluates through the compiler's D92 expression evaluator. Support for a construct in the compiler does not by itself establish support for evaluating it interactively.

# Errors and recovery

A failed input should show its source location, the relevant input, and the checker or evaluator's explanation. A rejected declaration does not become an accepted session binding. After a pending signature and an invalid equation, a corrected equation can still complete the definition.

When diagnosing a failure, separate three cases:

| Failure | Next step |
| --- | --- |
| The input has a syntax, type, effect, ownership, or proof error. | Correct the input using the same rules as a source file. |
| A loaded module fails. | Fix the named file or import, then reload. |
| The expression checks but cannot be evaluated interactively. | Save a small program and use the native build/run path; retain the evaluator refusal in a bug report. |

Include the compiler revision, minimal sequence of inputs, required source files, and complete diagnostic when reporting a session bug. See [diagnostics](21-editors-and-diagnostics.md) for phase-specific checks.

# Current implementation

| Surface | Present boundary |
| --- | --- |
| `cinch repl` and terminal UI | **Coming soon.** The executable does not currently dispatch a REPL command. Prompts, line editing, paste handling, and interruption above describe the intended frontend. |
| Stateful session | **API only.** `cinch::repl::Repl` retains declarations, pending input, imports, and in-memory history. |
| Commands | The Rust API implements the command table above. They are API inputs, not currently available terminal commands. |
| Checking and pure expression evaluation | Implemented through shared analysis and the D92 evaluator, within that evaluator's admitted subset. |
| Completion | The API exposes command and visible-name completion; terminal completion UI is coming soon. |
| Complete project/runtime session | **Coming soon.** Coherent reload recovery and full runtime-backed evaluation must be completed before the idealized workflow is advertised as available. |

Frontend authors can create `Repl::with_root(path)` and feed submissions to `eval(&str)`. Its result is `ReplOutcome::Output(String)`, `Incomplete`, or `Quit`. `Incomplete` requests more input; it is not a successful declaration. The API also exposes `complete`, `history`, `session_source`, and `analysis`. A terminal adapter must handle input grouping and interruption itself.

Implementation: `compiler/src/repl.rs`, `compiler/src/analysis.rs`; behaviour suite: `compiler/tests/repl.rs`. The [language server guide](27-language-server.md) describes the other frontend sharing this analysis layer.
