# 1. Build and run your first program

Cinch is a strict functional language. You write expressions, every top-level declaration carries a type, and the checker verifies the program before any machine code exists. In this chapter you build the Rust compiler, write one program, and drive it with the commands the rest of the book uses:

```sh
cargo build --manifest-path compiler/Cargo.toml
cinch check Main.cinch
cinch run Main.cinch
cinch build Main.cinch -o answer
```

Prerequisites: none. You need a terminal, a checkout of the Cinch repository, and Rust with Cargo installed. Every chapter after this one assumes the compiler is already on `PATH`.

# Build the compiler

From the repository root:

```sh
cargo build --manifest-path compiler/Cargo.toml
export PATH="$PWD/compiler/target/debug:$PATH"
cinch --help
```

`cinch --help` prints every subcommand. The three this book's chapters use constantly are `check`, which typechecks only, `run`, which builds and runs in one step, and `build`, which produces an executable you start yourself. `emit-ir` writes the LLVM intermediate representation as text when you want to read it.

A `clang` executable must be on `PATH` for `build` and `run`, because the compiler emits LLVM IR and asks clang to link it against the Rust runtime. `check` never reaches the linker. With a deliberately broken clang configured, `check` still exits 0 and `build` fails with:

```
cinch: failed to run /nonexistent/clang: No such file or directory (os error 2)
```

Set the environment variable `CINCH_CLANG` to a clang path when the default on `PATH` is not the one you want.

# A complete program

Create an empty directory, save this as `Main.cinch`, and run every command in that directory:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  double :: Int -> Int
  double n = n + n

  main :: Int
  main = if double 21 == 42 then 0 else 1
```

# Check, run, and build

```sh
cinch check Main.cinch
cinch run Main.cinch
cinch build Main.cinch -o answer
./answer
```

`check` prints nothing and exits 0. `run` prints nothing and the program's own exit status becomes the status of `cinch`. `build` writes the executable named by `-o` and leaves it for you to start; `./answer` also exits 0.

This program prints nothing at all. It reports its result through its exit status, which is the only thing a program with no I/O capability can do. Producing output requires an effect, and effects arrive in [chapter 10](10-effects-and-io.md). Everything up to that point is checked with exit statuses.

Without `-o`, `build` names the executable after the entry module, so `cinch build Main.cinch` writes `./Main`, with `Main.ll` beside it. The `build` directory next to the source holds one interface per module, a manifest, and a stage snapshot. See [installation and invocation](../reference/01-using-the-compiler.md) for the command surface, the artifact table, and platform caveats.

# Read the program in source order

`module Main where` starts a module named `Main`. The declarations indented below it belong to that module. A file must start with a module header; the compiler answers ``cinch: Main.cinch: no `module` declaration`` when it does not.

`double :: Int -> Int` is a type signature. It states that `double` takes one `Int` and returns an `Int`, and it is the only way to give `double` a type. The next line, `double n = n + n`, is the definition: the name before `=` is the parameter, and the body `n + n` uses it.

`main :: Int` declares the entry point with that signature. Its body, `if double 21 == 42 then 0 else 1`, applies `double` to `21` and compares with `==`. Application is written with a space; `double 21` is a call, not multiplication. `if` takes a `Bool` condition and two expressions, and its result is whichever branch the condition selects. This program returns `0` because `double 21` is `42`.

# Entry points

Two signatures are accepted for `main`.

| Declaration | Meaning |
| --- | --- |
| `main :: Int` | Pure entry point. The value becomes the exit status. |
| `main :: Unit ->{e} Int` | Effectful entry point, applied to `()`. The row `{e}` is the program's effect envelope. |

Both forms are checked and run today. This book uses `main :: Int` until effects appear, then switches to the effectful form. A file may declare several modules, but only one of them may define `main`:

```
cinch: several modules of the entry file define `main` (Main, Other); the entry file's `main` is unique
```

# Exit statuses

The exit status is one byte. The compiler converts the `Int` result to a machine integer and the operating system keeps the low eight bits, so a value outside 0 to 255 wraps:

| `main` returns | Status |
| --- | --- |
| `0` | 0 |
| `3` | 3 |
| `255` | 255 |
| `256` | 0 |
| `300` | 44 |
| `-1` | 255 |

Return `0` for success and small positive values for distinct failure reasons. A value that does not fit in a byte is silently truncated, so `300` and `44` are indistinguishable to a caller.

# What the compiler enforces

- The file begins with `module NAME where`, and declarations sit inside that module.
- Every top-level value has a type signature. An omitted one is rejected rather than inferred.
- `main` has type `Int` or `Unit ->{e} Int`, and only one module of the entry file defines it.
- An expression has exactly one type. Supplying more arguments than a function's type allows is a type error, not a runtime failure.
- Integer literals, operators, and `if` agree on their types. `if 1 then 0 else 1` is rejected because `1` is not a `Bool`.

# Diagnostics you will meet

Every message has the shape `cinch: LINE:COL: message (in FILE)`.

| Message | What produced it |
| --- | --- |
| `cinch: 3:10: type mismatch: expected Bool, got Int (in Main.cinch)` | `main :: Int` with the body `True` |
| `cinch: 6:20: type mismatch: expected Bool, got Int (in Main.cinch)` | `double True` where `double :: Int -> Int` |
| `cinch: main must have type Int or Unit ->{e} Int in the bootstrap slice, got Nat` | `main :: Nat` |
| ``cinch: this program has no `main`; `emit-ir`, `build`, and `run` need `main :: Int` in the entry module`` | `build` or `run` on a file with no `main` |
| ``cinch: Main.cinch: no `module` declaration`` | declarations with no module header |

The type-mismatch message names the type of the expression first, then the type the context requires. In `main = True` the expression `True` has type `Bool` and `main` requires `Int`.

# A type error and a failed check

These are two different failures, and confusing them wastes time. A type error stops compilation: nothing runs and no executable is produced.

<!-- cinch:reject contains="type mismatch: expected Bool, got Int" -->
```cinch
module Main where
  main :: Int
  main = True
```

A failed check is a correct program that ran and reported failure through its exit status. Here the program typechecks, links, and exits 1 because `double 20` is `40`, not `42`.

<!-- cinch:run exit=1 -->
```cinch
module Main where
  double :: Int -> Int
  double n = n + n

  main :: Int
  main = if double 20 == 42 then 0 else 1
```

The checker proves that the program is well typed. It does not prove that your numbers or comparisons are the ones you intended, which is why a passing `check` and a failing run are both normal during development.

# Exercises

1. Add `triple :: Int -> Int` with its definition, and make `main` succeed only when `triple 14 == 42`.

<details><summary>Answer</summary>

Write `triple n = n + n + n` (or `triple n = n * 3`) and `main = if triple 14 == 42 then 0 else 1`. Both declarations stay indented under `module Main where`, and `triple` needs its own signature line directly above its definition.

</details>

2. Make the program exit with status `3` when `double 5 == 10`, and `0` otherwise. Then change `3` to `259` and explain the status you observe.

<details><summary>Answer</summary>

`main = if double 5 == 10 then 3 else 0` exits 3. Changing the literal to `259` still exits 3, because the operating system keeps the low eight bits of the result: 259 - 256 is 3.

</details>

Next: [values, expressions, and layout](02-values-and-layout.md).
