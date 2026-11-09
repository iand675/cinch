# 20. Foreign code, layouts, and target profiles

A foreign boundary joins two independently compiled ideas about what a word is. Matching the symbol name is the least of it. The declaration must agree with C on the calling convention, the width and signedness of each argument, the byte layout of every aggregate, which side owns each pointer, and whether the call can block, fail, or unwind. The compiler checks what a compiler can check: effect honesty, representation, ownership roles, and the no-unwind rule. It does not check that the C function does what its name suggests.

Prerequisites: [effects and IO](10-effects-and-io.md), [values and layout](02-values-and-layout.md), and [numbers, text, and bytes](07-numbers-text-and-bytes.md).

# Declare a boundary

A `foreign import ccall "symbol"` binds a C symbol to a Cinch type. These declarations are a fragment of a module:

```text
  foreign import ccall "cinch_clock_monotonic_ns" monotonic :: Unit ->{Foreign Clock, IO} I64
```

The effect row is not decoration. The result arrow's row must name at least one boundary, `IO`, `Global`, `Blocking`, `Diverge`, `Fail`, `Reentrant`, or `ThreadAffine`, because an external function is not pure just because its C signature has no explicit state parameter. `Foreign Clock` names the boundary this call belongs to; `IO` says the call observes the world.

The embedded library declares the same two clock words itself, in `Effect/Clock.cinch`. Its form is worth copying because the refinement carries the runtime's own guarantee:

```text
  foreign import ccall "cinch_clock_monotonic_ns" raw-monotonic-ns :: forall r. Unit ->{IO r} InRange 0 9223372036854775807 I64
  clock-monotonic-ns :: forall r. using (io :: Io r) => Unit ->{IO r} Nat
  clock-monotonic-ns u = nat-from-i64 (raw-monotonic-ns u)
```

Writing the declaration yourself, as the next example does, shows the mechanism the library uses.

# A foreign declaration you link and run

<!-- cinch:run exit=0 -->
```cinch
module Main where
  foreign import ccall "cinch_clock_monotonic_ns" monotonic :: Unit ->{Foreign Clock, IO} I64

  main :: Unit ->{Foreign Clock, IO} Int
  main u =
    let before = monotonic u
    in let after = monotonic u
    in if before <= after then 0 else 1
```

`cinch run Main.cinch` typechecks the program, emits IR, links it with `clang`, and runs it; this example exits `0` because two monotonic readings never go backwards. `cinch check Main.cinch` is not enough to prove a link: it never invokes the linker. The IR carries an external declaration and a call:

```text
declare i64 @"cinch_clock_monotonic_ns"()
  %t0 = call i64 @"cinch_clock_monotonic_ns"()
```

The linker resolves that symbol against the Cinch runtime archive, which supplies the `cinch_clock_*` externs. If the symbol is absent, the failure is the system linker's, and it names the symbol:

```text
$ cinch run Missing.cinch
cinch: clang failed to link /var/folders/.../cinch-run-32619-9/program.ll: warning: overriding the module target triple with arm64-apple-macosx27.0.0 [-Woverride-module]
1 warning generated.
Undefined symbols for architecture arm64:
  "_no_such_c_symbol", referenced from:
      _Main.missing in program-b432c4.o
ld: symbol(s) not found for architecture arm64
clang: error: linker command failed with exit code 1 (use -v to see invocation)
```

That command exits `1`. The public CLI takes `--package-object <path>` for an installed Cinch package object, not an arbitrary C object. The Rust API links supplied objects through `run_source_objects`, which is how `compiler/tests/ffi.rs` exercises a real C fixture. Document the object you depend on rather than presenting a bare `foreign import` as a standalone program.

# Machine types, not mathematical ones

`Int` and `Nat` are mathematical: they are bignum capable and their representation is the compiler's business. A boundary argument must be an admitted machine type such as `I64`, `U32`, or `CInt`, or a checked `RawPtr`, so the call's convention is fixed by the declaration. An aggregate result is refused because the platform's by-value struct ABI is not this slice's guess:

<!-- cinch:reject contains="a foreign result cannot be an aggregate" -->
```cinch
module Main where
  #[repr(C)]
  record P where
    a :: I32
  foreign import ccall "c_fix_make" make :: () ->{Foreign Fix, Diverge} P
```

Return through an `Out` cell or an `Owned` pointer instead, or spell a by-value crossing `#[sret]`. Opaque handles take ownership roles: `Owned FixHeap H` is transferred once and `Borrowed H` is call scoped, and an owned foreign allocation needs its own deallocator. An integer-to-pointer cast is refused, because an address does not acquire a safe lifetime by being placed in a record.

# Representation choices that change bytes

A record crossing the boundary needs an admitted representation. Without one, its layout is `uniform-word`, which is a boxed aggregate and has no C representation:

| Declaration | Recorded layout | At the boundary |
| --- | --- | --- |
| `record R where x :: I32` | `uniform-word` | Refused: no C representation |
| `#[repr(C)] record CP where a :: U8; b :: U32` | C layout with padding | Both fields cross at their C offsets |
| `#[repr(C, packed(2))]` | C layout, alignment capped at 2 | Fields cross at packed offsets, copied byte-exactly |
| `#[repr(transparent)]` | One representation field | Crosses as that field |
| `#[repr(u8)] data Color where ...` | Fixed discriminant tag | Refused: a tag needs a generated shim |

The emitted marshalling wrapper shows the offsets. For `#[repr(C)]`, `b` lands at offset 4 and the buffer is aligned 4:

```text
  %t0 = alloca [8 x i8], align 4
  %t5 = getelementptr inbounds i8, ptr %t1, i64 4
  %t6 = load i32, ptr %t5, align 1
```

For `#[repr(C, packed(2))]` with fields `a :: U8`, `b :: U32`, `c :: U16`, the same wrapper compacts the record to 8 bytes aligned 2 and reads each field at its own width:

```text
  %t0 = alloca [8 x i8], align 2
  %t5 = getelementptr inbounds i8, ptr %t1, i64 2
  %t6 = load i32, ptr %t5, align 1
  %t8 = getelementptr inbounds i8, ptr %t1, i64 6
  %t9 = load i16, ptr %t8, align 1
```

Packing has a cost inside the language. A field at an offset that is not a multiple of its natural alignment cannot be borrowed, because a reference asserts alignment. Copying it is fine, and this program runs:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  #[repr(packed)]
  record R where
    a :: U8
    b :: U32

  copyB :: R -> U32
  copyB r = r.b

  main :: Int
  main = if copyB (R { a = 1, b = 7 }) == 7 then 0 else 1
```

The borrow form of the same field is refused, with the offsets printed:

<!-- cinch:reject contains="cannot be borrowed" -->
```cinch
module Main where
  #[repr(packed)]
  record R where
    a :: U8
    b :: U32

  borrowB :: forall region. R -> Ref region U32
  borrowB r = borrow r.b

  main :: Int
  main = 0
```

The message reads ``field b of R sits at offset 1 but its alignment is 4: a field whose offset is not a multiple of its natural alignment cannot be borrowed, since `borrow` asserts alignment; copy the field instead``.

# Effect honesty at the boundary

A declaration that hides its effects is refused before any code is generated:

<!-- cinch:reject contains="a foreign import must declare its effects" -->
```cinch
module Main where
  foreign import ccall "cinch_clock_wall_ns" wall :: Unit -> I64
```

A `foreign export ccall "symbol" name :: Type` goes the other way: it exposes a checked Cinch definition under a C symbol, and the export's contract records the symbol, type, layout, and relevant effects in the compiled interface. Neither direction may unwind across the boundary. A foreign function that can block must say so, and one that can fail or diverge must not be declared effect free to make a call site convenient.

# Target profiles

A target profile fixes the pointer width, the features an artifact may assume, the admitted runtime evaluation, and the optimizer's driving bounds. The build records it in `build/Main.manifest.json`:

```text
"target": {"admission": "the deployment contract must satisfy the recorded feature assumptions",
           "conditions": [], "endian": "little", "features": ["u32.wrappingAdd"],
           "profile": "native64", "vector_shape": "fixed(8)", "word_size": 64}
```

`Usize` and `Isize` follow `word_size`, never the build host, so a `Wasm32` profile makes them 32 bits. The Rust API exposes `TargetProfile::native()`, `TargetProfile::wasm32(artifact)`, and the stricter `TargetProfile::jpl()`.

<span class="support-tag support-tag--planned">Coming soon</span> **target selection in the CLI.** There is no `--target`, `--component`, or cross-target packaging flag today; `cinch --help` lists `check`, `emit-ir`, `emit-object`, `build`, `run`, `lsp`, `doc`, `repl`, `jit`, and `pkg`. The build-property switches the CLI does expose are the runtime ones, `CINCH_RECLAIM`, `CINCH_HEAP_LIMIT`, `CINCH_STACK_SIZE`, and the runtime's diagnostic variables.

# The component path today

A `.wit` file beside the source is parsed into canonical-ABI adapter vocabulary, and the vocabulary becomes importable. The adapters are read from the source file's own directory, so pass a path with a directory component:

```text
$ ls
Main.cinch  codec.wit
$ cinch check ./Main.cinch
```

Run against `Main.cinch` with no directory part, the same command reports `unknown type DecodeError`, because the adapter is never loaded. With the adapter in scope, a module that defines the world's implementation checks:

```text
$ cinch build ./Main.cinch -o prog
cinch: the canonical ABI glue for example:codec@0.1.0/decoder#decode is defined for the wasm32 target (WebAssembly and WASI specification section 4); emit under a Wasm artifact profile
```

The source is checked; the native artifact is refused, and the message names the reason. Declaring the import without an implementation of the export is refused one step earlier:

```text
cinch: the world exports example:codec@0.1.0/decoder#decode but the program defines no implementation `Decoder.decode` (WebAssembly and WASI specification section 9)
```

A `WasiStream` or `WasiFuture` from such an adapter is an external handle with its own failure contract. It is not the productive `Stream` of chapter 15, and importing one establishes no productivity theorem.

# What the compiler enforces

- A foreign import declares at least one effect from the boundary vocabulary; a pure signature is refused.
- Boundary argument and result types are admitted machine types, `Out` cells, ownership roles, or aggregates with `#[repr(C)]`, `#[repr(transparent)]`, or a packed C layout. Boxed aggregates and fixed-discriminant data are refused instead of guessed.
- A field at a misaligned offset cannot be borrowed; copying it is allowed.
- Owned foreign allocations carry their deallocator contract, and integer-to-pointer casts are refused.
- No unwinding crosses the boundary, and blocking must be declared.
- The target profile's feature assumptions are checked at artifact admission, and the recorded profile, word size, endianness, and features travel in the manifest.

# Diagnostics you will meet

| You wrote | The compiler says |
| --- | --- |
| `foreign import ccall "f" g :: I64 -> I64` | ``a foreign import must declare its effects`` |
| A plain `record` as a foreign argument | ``the recorded layout of Main.R is `uniform-word` (no representation attribute), which has no C representation at the boundary`` |
| `#[repr(u8)] data Color` at the boundary | ``which has no C representation at the boundary`` |
| A record as a foreign result | ``a foreign result cannot be an aggregate`` |
| `borrow r.b` on a packed field | ``cannot be borrowed, since `borrow` asserts alignment; copy the field instead`` |
| A missing C symbol | ``Undefined symbols for architecture arm64`` naming the symbol |

# Exercises

**Exercise 1.** Why is replacing an `I64` foreign parameter with `Int` wrong, even when every value in your tests fits in 64 bits?

<details><summary>Answer</summary>

The calling convention depends on representation, not on the set of mathematical values. `Int` can be an immediate word or a heap-backed bignum, so a C function expecting `long long` would receive either a different register content or a pointer. The boundary needs an admitted machine type; when a mathematical value has to cross, convert with a checked function such as `int-from-i64` or `nat-from-i64` and keep the machine type in the foreign signature. The library's `clock-monotonic-ns` is exactly that shape: a machine `I64` crosses, and `nat-from-i64` produces the mathematical `Nat`.

</details>

**Exercise 2.** A packed record with `a :: U8` and `b :: U32` computes correctly in your tests but a colleague's `borrow r.b` is rejected. Which choice do you change, and what do you tell them?

<details><summary>Answer</summary>

The compile-time rejection is not a bug in the borrow: offset 1 is not a multiple of 4, so a reference to `b` would assert an alignment the memory does not have. Either copy the field (`r.b` as a value), or remove the packing if the C side does not require it and the record needs borrows. Changing the field type to force alignment changes the C layout and breaks the boundary contract, so it is the wrong fix. The borrow rule and the layout are the same fact seen from two sides, which is why `#[repr(C)]` (b at offset 4) accepts the borrow that `#[repr(C, packed(2))]` (b at offset 2) refuses.

</details>

Next: [state evolution and temporal contracts](21-upgrades-and-contracts.md).
