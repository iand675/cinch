# Interactive programming with the REPL

<span class="support-tag support-tag--current">Current</span> **`cinch repl` starts an interactive session.** In a terminal it runs a full-screen frontend; with standard input or standard output redirected it runs a plain line loop. Both drive the same session object, keep declarations across inputs, and check expressions with the compiler's own rules.

Use a session to try an expression, to find out what a function's signature actually is, or to check a declaration before committing it to a file. Start it from the directory that holds the project:

```sh
cinch repl
```

The working directory is the session's project root. Sources and imports resolve with the same module rules the command-line compiler uses. A session does not install packages or download anything.

# Which frontend you get

`cinch::repl::run` chooses once, at startup:

```sh
if stdin.is_terminal() && stdout.is_terminal() {
    repl_tui::run()   // full-screen
} else {
    run_with_root(current_dir, stdin, stdout, false)   // plain line loop
}
```

Two consequences are worth knowing before you pipe anything:

- The plain loop is chosen when either stream is redirected, not only when input is. `cinch repl > log.txt` gives the line loop.
- The loop runs with `interactive: false`, so it prints no banner and no prompts. The prompt strings `cinch> ` and `...> ` exist in the source only for the `interactive: true` path, which `cinch repl` never selects. A piped transcript is bare output.

The plain loop reads one line per call, evaluates it, writes any output, and continues. End of input ends the session. A normal session exits 0; a startup failure exits 1.

# A recorded session

Definitions, expressions, multi-line input, and the query commands, in one run:

```sh
printf 'x :: Int\nx = 5\nx + 2\ndouble :: Int -> Int\ndouble n = n + n\ndouble 21\nresult = if True then\n  10 else 20\nresult\ntotal = (1 +\n  2) * 3\ntotal\ndata Colour where { Red :: Colour; Green :: Colour }\n:type double\n:type x + 2\n:kind Colour\n:browse\n:history\n:help\n1 + True\n:quit\n' | cinch repl
```

The process printed exactly this, with no prompts:

```text
noted x (add its definition)
defined x :: Int
7 :: Int
noted double (add its definition)
defined double :: (Int -> Int)
42 :: Int
defined result :: Int
10 :: Int
defined total :: Int
9 :: Int
defined Colour :: Type
defined Green :: Colour
defined Red :: Colour
(Int -> Int)
Int
Colour :: Type
Colour :: type Colour :: data
Green :: Colour
Red :: Colour
double :: (Int -> Int)
result :: Int
total :: Int
x :: Int
1: x :: Int
2: x = 5
3: x + 2
4: double :: Int -> Int
5: double n = n + n
6: double 21
7: result = if True then
8:   10 else 20
9: result
10: total = (1 +
11:   2) * 3
12: total
13: data Colour where { Red :: Colour; Green :: Colour }
14: :type double
15: :type x + 2
16: :kind Colour
17: :browse
18: :history
:type <expr>, :kind <type>, :browse [module], :load <file>, :reload, :history, :quit
error: no instance for Add Bool
  at 1:3
  1 + True
    ^
```

Here is the same run with each input above its output. The `>` markers are this page's convention for an input line; the REPL printed none of them. The `:history` listing is shortened to its first and last entry here, and appears in full in the block above.

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
```

# Definitions and expressions

Three outcomes, and the first word tells you which one happened:

| Output | Meaning |
| --- | --- |
| `defined NAME :: TYPE` | The declaration was accepted and is now part of the session. |
| `noted NAME (add its definition)` | A signature was recorded and is waiting for its equation. |
| `VALUE :: TYPE` | An expression was evaluated. Nothing was bound. |
| `error: ...` | The input was rejected. Nothing was bound. |

A signature may lead its equation or arrive with it. A lone signature stays pending; the equation completes it. Give an explicit signature when the definition is polymorphic, constrained, effectful, or dependent, since the session has the same signature discipline as a file rather than a weaker one.

Rendered schemes appear in parentheses: `defined double :: (Int -> Int)`, where a file's own signature would read `double :: Int -> Int`. A monomorphic binding renders without them (`defined x :: Int`). The parentheses come from the rendered scheme form, so do not copy them into a declaration.

An expression does not create a reusable name. `x + 2` prints `7 :: Int`; to keep the result, bind it.

The same declarations as a file, which is where they belong once they matter beyond one session:

<!-- cinch:run exit=0 -->
```cinch
module Main where

double :: Int -> Int
double n = n + n

main :: Int
main = double 21 - 42
```

`cinch run Main.cinch` accepts it and exits 0. Nothing in the session is required for the file form, and the file form is what `cinch check` and `cinch run` can reproduce later.

# Multi-line input

A block continues while it is unfinished. The test is lexical: an unclosed `(` `[` `{` `|]` or `"`, or a last significant token that cannot end a declaration, such as `where`, `of`, `do`, `then`, `else`, `in`, `=`, `->`, `=>`, `<-`, `,`, `|`, a type or term operator, `forall`, `using`, `let`, `letrec`, or `if`.

Two continuations from the recorded session:

```text
result = if True then
  10 else 20
```

```text
total = (1 +
  2) * 3
```

Indentation inside the block keeps its meaning. The continuation decision is made after each line, so a block that becomes complete early commits early. A `data` declaration with several constructors is the case to watch:

```text
data Colour where
  Red :: Colour
  Green :: Colour
```

```text
defined Colour :: Type
defined Red :: Colour
error: unknown name Green
  at 1:1
  Green :: Colour
  ^
```

The first two lines form a complete declaration, so the third arrives as a new input. Write the declaration on one line with explicit blocks, which is what the recorded session does:

```text
data Colour where { Red :: Colour; Green :: Colour }
```

In the full-screen frontend, a paste inserts the newlines into the input buffer and `Enter` submits the whole buffer, so a multi-constructor block pasted from a file submits as one input.

# Commands

A command starts with `:` at a fresh prompt. Its argument is Cinch source or a module name, not a shell command.

| Command | Aliases | Meaning |
| --- | --- | --- |
| `:type EXPR` | `:t` | Elaborate an expression and print its type scheme, or its diagnostic. |
| `:kind NAME` | `:k` | Print the kind of a type constructor or class. |
| `:browse` | `:b` | List names and schemes from the session and the loaded modules. |
| `:browse MODULE` | `:b MODULE` | Restrict the listing to one module. |
| `:load FILE` | `:l FILE` | Load a source project into the session. |
| `:reload` | `:r` | Read the loaded sources again and recheck them. |
| `:history` | `:hist` | List this session's inputs with entry numbers. |
| `:help` | `:h` | Print the command line. |
| `:quit` | `:q` | End the session. |

The alias list is read from the dispatch in `compiler/src/repl.rs`. An unknown command prints `error: unknown command :frobnicate` and leaves the session usable:

```text
:frobnicate
3 + 3
```

```text
error: unknown command :frobnicate
6 :: Int
```

A required argument that is missing prints a usage line rather than an error:

```text
:type
:kind
```

```text
usage: :type <expression>
usage: :kind <type>
```

`:type` elaborates in the session's scope and does not start a native runtime probe, so asking for a type never executes a foreign call or an I/O action. `:type 1 + True` reports the same instance failure the expression would:

```text
error: no instance for Add Bool
  at 1:3
  1 + True
    ^
```

`:kind` answers for a type declared in the session or in a loaded module, and the answer for a type constructor includes its declaration keyword:

```text
:kind Colour
Colour :: Type
```

The listing for a polymorphic constructor keeps its arity. With `data Shape a where ...` in a loaded module, `:kind Shape` prints `Shape :: Type -> Type`. A prelude type name currently answers with a refusal rather than a kind:

```text
:kind List
```

```text
error: no type or class named List
```

`:browse` collects the session's own definitions and every loaded module. After the project in the next section is loaded, `:browse Util` restricts the listing, and `:browse` of a module that is not loaded prints nothing:

```text
:browse Util
```

```text
double :: (Int -> Int)
```

`:browse` is a type index, not a substitute for the [library reference](../library/00-index.md). A name appearing in a listing does not bypass the export and import rules of its module, because the listing only shows modules the session already loaded.

# Loading and reloading a project

With `Util.cinch` declaring `double :: Int -> Int` and `Main.cinch` importing it and declaring `quad n = double (double n)`:

```text
:load Main.cinch
quad 21
```

```text
loaded Main.cinch
rechecked Main
rechecked Util
84 :: Int
```

The `rechecked` lines name what the update re-ran. After a load, expression probes reuse the checked closure, so a repeat expression does not recheck the project.

Reload is transactional. Breaking `Util.cinch` and sending `:reload` in the same live session, then repeating the expression:

```text
:reload
quad 21
```

```text
error: reload failed; kept the previous checked session
Util.cinch:4: type mismatch: expected String, got Int (in Util.cinch)
84 :: Int
```

The evaluation after the failed reload still runs against the last checked revision. Repair the file and reload again:

```text
:reload
```

```text
rechecked Main
rechecked ReplSession
rechecked Util
```

`ReplSession` is the session's own module, which is rechecked with the project because loaded entries are part of one source set. A load failure has the same shape:

```text
:load Missing.cinch
```

```text
error: Missing.cinch: No such file or directory (os error 2); kept the previous checked session
```

When the previous session also fails to check, a third line says so. The run below loaded nothing first and then loaded a broken project:

```text
:load Main.cinch
:reload
quad 21
```

```text
error: load failed; kept the previous checked session
Util.cinch:4: type mismatch: expected String, got Int (in Util.cinch)
error: reload failed; kept the previous checked session
Util.cinch:4: type mismatch: expected String, got Int (in Util.cinch)
error: previous session also fails: Util.cinch:4: type mismatch: expected String, got Int (in Util.cinch)
error: unknown name quad
  at 1:1
  quad 21
  ^
```

`:load` and `:reload` read source files. A library that only exists as a package still has to be materialized as source or as checked interfaces first; loading a module is not a remote install.

# History and help

`:history` numbers every input, including commands and rejected inputs, so it is a record of what you typed rather than of what succeeded. The numbers match the recorded session above: entry 17 is `:browse` and entry 18 is `:history` itself.

`:help` prints one line, and it does not list itself:

```text
:type <expr>, :kind <type>, :browse [module], :load <file>, :reload, :history, :quit
```

History is in memory for one process. There is no on-disk history file and no replay command.

# Failures and recovery

A rejected input prints the message, the position, the offending source line, and a caret:

```text
error: integer literal cannot have type Bool
  at 1:1
  9
  ^
```

The position is relative to the submitted block, not to the session as a whole.

One behaviour to know before you spend time on a confusing second error: after a failed input, the session's literal defaulting state can keep a constraint the failure introduced. The run below sends only two lines:

```text
1 + True
9
```

```text
error: no instance for Add Bool
  at 1:3
  1 + True
    ^
error: integer literal cannot have type Bool
  at 1:1
  9
  ^
```

`9` is valid on its own. Start a fresh session after a failure of that shape while the session is under development; the failure does not otherwise corrupt accepted declarations, and an unrelated name error leaves later inputs working:

```text
nope
9
```

```text
error: unknown name nope
  at 1:1
  nope
  ^
9 :: Int
```

# The full-screen frontend

Everything in this section is read from `compiler/src/repl_tui.rs`. The recorded sessions above exercised the line loop, not the terminal frontend, so treat the layout and key table as a description of the code rather than as output you have seen on this page.

Startup takes the terminal: raw mode, the alternate screen, and bracketed paste. All three are undone on exit and after a panic.

The screen is five vertical bands:

| Band | Height | Contents |
| --- | --- | --- |
| header | 3 rows | `cinch / interactive`, the session root path, and a status dot with `ready` or `error` plus a definition count |
| body | fills | transcript on the left, `Session` sidebar 34 columns wide on the right |
| input | 3 to 8 rows | the editor, titled `Input`, with `cinch> ` or `...> ` in the first line |
| completion | 0 or 2 to 8 rows | the `Complete` popup, one row per candidate up to 6, shown only while completing |
| status | 1 row | the key hint line |

The transcript colours its own output by prefix: a line starting with `error` is red, `defined`, `loaded`, and `rechecked` are green, `noted` is yellow. Input lines and their results are syntax coloured with the tree-sitter token classes from `compiler/src/highlight.rs`, the same classification the generated documentation uses.

The sidebar lists accepted bindings with their schemes, or `No definitions yet.`, then a `Commands` list naming `:type expr`, `:kind Type`, `:browse [module]`, `:load file`, `:reload`, `:history`, `:help`, and `:quit`.

The status line reads:

```text
 Enter run · Tab complete · ↑↓ history · Ctrl+J newline · Ctrl+C clear · Ctrl+D quit
```

Key bindings, from the handler:

| Key | Action |
| --- | --- |
| `Enter` | Submit the buffer, or accept the highlighted completion |
| `Alt+Enter`, `Ctrl+J` | Insert a newline |
| `Tab`, `Shift+Tab` | Start completion, or cycle candidates forward and backward |
| `Up`, `Down` | History when no completion popup is open; otherwise cycle candidates |
| `Esc` | Close the completion popup |
| `Left`, `Right`, `Home`, `End` | Move the input cursor |
| `Ctrl+A`, `Ctrl+E` | Move to the start or the end of the input |
| `Ctrl+U` | Clear the input |
| `Backspace`, `Delete` | Delete before or at the cursor |
| `PageUp`, `PageDown` | Scroll the transcript by 8 lines |
| `Ctrl+L` | Clear the transcript and reset the scroll |
| `Ctrl+C` | With empty input and no continuation, quit; otherwise discard the pending input and any unfinished block |
| `Ctrl+D` | Quit |

Completion replaces the current word from `Repl::complete`. One candidate is inserted directly; several open the popup, which shows at most 6 at a time and applies the selected candidate to the input as you cycle. After `:` the candidates are the command names; otherwise they are session, imported, and prelude names.

Evaluation is synchronous. The event loop polls for 250 ms, then handles one event, and an evaluation runs to completion before the next draw. `Ctrl+C` therefore discards pending input rather than interrupting a check or a native probe already running.

# The session behind the frontend

`cinch::repl::Repl` is the session object both frontends use:

| Call | Effect |
| --- | --- |
| `Repl::with_root(path)` | A session rooted at a directory |
| `Repl::with_compiler(root, binary)` | The same, with an explicit executable for native probes |
| `eval(&str)` | Submit one input; returns `ReplOutcome` |
| `complete(partial)` | Completion candidates for a prefix |
| `history()` | The inputs submitted so far |
| `session_source()` | The accumulated session module as source text |
| `abort_pending()` | Drop an unfinished block |
| `analysis()` | The shared analysis layer |

`ReplOutcome` is `Output(String)`, `Incomplete`, or `Quit`. `Incomplete` requests more input and is not a successful declaration. A frontend that ignores it will report a half-typed block as an error.

# Current implementation

| Surface | Present boundary |
| --- | --- |
| `cinch repl` and the terminal frontend | **Available.** Full-screen frontend with transcript, session sidebar, input editor, completion popup, history, and token colours. Redirected streams use the plain line loop. |
| Session state | **Available.** Declarations, pending signatures, imports, and in-memory history persist for the process. |
| Queries | **Available.** `:type`, `:kind`, `:browse`, `:history`, `:help` behave as recorded above; `:kind` refuses prelude type names. |
| Project load and reload | **Available and transactional.** A broken, missing, or failing revision is rejected and the previous checked revision stays live. |
| Expression evaluation | **Available** through the compiler's evaluator, with a native probe for constructs that need target code. |
| Completion | **Available** over session, imported, prelude, and command names. |
| Literal defaulting after a failure | **Partial.** A failed input can leave a constraint that rejects later numeric literals; start a fresh session. |
| Cross-session history and replay | **Not implemented.** History is in memory for one process. |
| Interrupting a running probe | **Not implemented.** `Ctrl+C` cancels pending input only. |

Implementation: `compiler/src/repl.rs` and `compiler/src/repl_tui.rs`; behaviour suite: `compiler/tests/repl.rs`. The [language server guide](27-language-server.md) covers the other frontend that shares the same analysis layer, and [the compiler guide](01-using-the-compiler.md) covers saving a session's declarations to a file where `cinch check` and `cinch run` take over.
