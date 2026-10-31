# 1. Build and run your first program

Cinch is a strict functional language. You describe values with expressions, give functions types, and use those types to state which effects and resources a computation requires. This book starts with ordinary functions and data. Proofs, ownership, concurrency, and code generation come after you have a working program to extend.

You need basic familiarity with files and a terminal. No Haskell knowledge is assumed. Read the chapters in order on a first pass; use the [compiler guide](../reference/00-guide.md) when you need the exact rule behind an example.

# Build the compiler

From a checkout of the Cinch repository, build the **Rust implementation**, which is the implementation used throughout this book:

```sh
cargo build --manifest-path compiler/Cargo.toml
export PATH="$PWD/compiler/target/debug:$PATH"
```

Rust and Cargo build the compiler. A compatible `clang` must also be on `PATH` to build or run Cinch programs: the compiler emits LLVM IR and invokes clang to link it with the Rust runtime. Set `CINCH_CLANG` to select another clang executable. `cinch check` stops before native linking. See [installation and invocation](../reference/01-using-the-compiler.md) for the complete command surface and platform caveats.

The [downloads page](/downloads/) currently contains explicitly labelled sample bundles. Those bundles do not install a compiler. Build from source for this tutorial.

# A complete program

Make an empty directory for your exercises. Save this as `Main.cinch`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  double :: Int -> Int
  double n = n + n

  main :: Int
  main = if double 21 == 42 then 0 else 1
```

Run these commands inside that directory:

```sh
cinch check Main.cinch
cinch run Main.cinch
printf 'exit: %s\n' "$?"
cinch build Main.cinch -o answer
./answer
```

The successful run produces no standard output and exits with status `0`. This first program tests a computation using its exit status. Printing requires an I/O capability, which we introduce in [chapter 10](10-effects-and-io.md).

`module Main where` starts a module. The indented declarations belong to it. `double :: Int -> Int` says that `double` takes an integer and returns an integer. The equation on the next line gives its implementation. Function application is written with a space: `double 21`. `==` compares values, and `if` chooses a result expression.

`main :: Int` is a supported entry point: the integer becomes the process exit status. Use `0` for success and small positive values for failed checks. An effectful entry point has a different shape, introduced later. This is an exit code, not a value printed by an interpreter.

# Make a mistake on purpose

Change `double 21 == 42` to `double 20 == 42`. The program still typechecks, but running it exits `1`: typechecking does not prove your business requirement. Next replace `21` with `True`. This time `cinch check` fails because a `Bool` is not an `Int`.

These are different kinds of feedback. A type error prevents compilation. A failed application check is a valid program reporting that its expected result did not occur.

# Exercise and answer

Add `triple :: Int -> Int`, implement it, and make `main` succeed only when `triple 14 == 42`.

<details><summary>Answer</summary>

Use `triple n = n + n + n` and `main = if triple 14 == 42 then 0 else 1`. Keep both declarations indented beneath the module header. You can also write `triple n = n * 3`.

</details>

Next: [values, expressions, and layout](02-values-and-layout.md).
