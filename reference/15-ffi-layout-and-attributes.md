# Foreign code, representation attributes, and inline primitives

A `foreign` declaration or a `prim` body crosses a trust boundary. The compiler checks the parts it can see: the calling convention, the layouts of the values that cross, the declared effect row, the ownership roles, and (for `prim`) the containment grammar of the inline code. It does not prove the external implementation correct. The artifact manifest states the boundary explicitly, listing LLVM code generation and the runtime as `trusted` and the source-level phases as `checked`.

# The foreign declaration form

```text
foreign import ccall "symbol" local-name :: Type
foreign export ccall "symbol" local-name :: Type
```

The convention is `ccall` or `component`; any other word is refused by the parser with `expected `ccall` or `component`, found ...`. Only `ccall` is implemented:

```
cinch: 2:3: only the ccall ABI is supported; the component ABI needs the WIT adapter (P14)
```

A component boundary is entered through a WIT file, whose adapter generates the bindings ([targets and components](19-targets-and-components.md)).

A `foreign` declaration is not pure by default. The result arrow must carry an effect row naming at least one of the foreign vocabulary:

<!-- cinch:reject contains="a foreign import must declare its effects" -->
```cinch
module Main where
  foreign import ccall "cinch_clock_wall_ns" wall-ns :: () -> U64

  main :: Int
  main = 0
```

```
cinch: 2:3: a foreign import must declare its effects: the result arrow's row names at least one of `Foreign <boundary>`, `IO`, `Global`, `Blocking`, `Diverge`, `Fail`, `Reentrant`, `ThreadAffine` (foreign function interface specification section 4: an external function is not pure merely because its C signature has no explicit state parameter) (in Pure.cinch)
```

The boundary name after `Foreign` is yours to choose (`Lib`, `Codec`, `Fix`, `Component` in the specifications and the prelude; a new name is accepted and recorded). An entry point that performs the call declares the matching row, and the CLI accepts `main :: Unit ->{e} Int` for exactly this purpose; a nullary `main` must be pure.

`Int` and `Nat` are not C integers. Reaching for one is refused by name:

```
cinch: 5:3: Int has no C representation: language-native closures, dictionaries, vectors, dependent pairs and proofs have no implicit C representation (foreign function interface specification section 2)
```

Use the machine scalars (`I8`..`I64`, `U8`..`U64`, `Isize`, `Usize`, `F32`, `F64`), `Bytes`, `RawPtr t`, or an `opaque type`.

# A binding that links and runs

`cinch build` and `cinch run` link the program against the runtime library, so a symbol the runtime exports resolves without any extra flag. The runtime exports `cinch_clock_wall_ns`, which makes a complete linkable example possible:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  foreign import ccall "cinch_clock_wall_ns" wall-ns :: () ->{Foreign Lib, Diverge} U64

  main :: Unit ->{Foreign Lib, Diverge} Int
  main u = if eq-u64 (wall-ns ()) 0 then 1 else 0
```

`cinch emit-ir Main.cinch -o Main.ll` shows the declaration the backend wrote and the stub it generated. The symbol is declared with the C signature; the cinch-side unit takes the call's argument word even for a unit argument and forces the result through `freeze` so a hostile callee cannot poison later optimization.

```
declare i64 @"cinch_clock_wall_ns"()

define i64 @"Main.wall-ns"(i64 %p0) #0 {
entry:
  %t0 = call i64 @"cinch_clock_wall_ns"()
  %t1 = freeze i64 %t0
  ret i64 %t1
}
```

# Handing a buffer to C

A C function that takes a `char *` needs an address, and `Bytes` values are counted handles rather than pointers. `bytes-data` is the one conversion: it returns the address of the buffer's payload as a machine word, which is what a foreign declaration's first parameter expects when the callee reads memory.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  foreign import ccall "strlen" c-strlen :: I64 ->{Foreign Lib, Diverge} I64

  main :: Unit ->{Foreign Lib, Diverge} Int
  main u =
    let raw = string-to-bytes "note.txt"
        n = bytes-len raw
        path = bytes-new (n + 1)
        _ = bytes-copy-range path raw 0 0 n
        _ = bytes-set path n 0
    in if eq-i64 (c-strlen (bytes-data path)) 8 then 0 else 1
```

Three facts make this safe to write:

- **The payload does not move.** A buffer's address is fixed when it is made, so an address taken once stays valid while the handle is live. It is the handle, not the address, that the compiler tracks.
- **`bytes-data` does not release the buffer.** The ownership stays with the caller, which is what keeps the payload alive for the rest of the expression. A pointer that outlives the last use of its handle is a dangling read, and the checker will not invent a reference to save it.
- **`Bytes` is not a C string.** There is no terminator, and the length is carried beside the buffer, so a C string is a fresh buffer one byte longer with the last byte zeroed, as above.

The address is a word, not a typed pointer, so the type checker cannot tell you what it points at. That is deliberate: the only thing to do with it is pass it to something that already knows, a declaration whose signature you wrote or a `prim` body.

# The operating system as a library

A `foreign import ccall` whose symbol is a `libSystem` name needs no runtime export and no Rust in the path: the declaration is the binding, and `open`, `read`, `write` and `close` resolve to the C library exactly as `strlen` does. The prelude packages that up as `Prim.Sys.Darwin`, whose wrappers turn a `-1` result into a named error read from the thread's error slot, and as `Prim.Sys.Linux`, whose entries are `prim` bodies that issue `syscall` with the number pinned in the body. The two modules name the same `SysError` constructors, so a module that uses one switches with a single import line.

This program is the whole filesystem round trip on Darwin: `creat` (the fixed argument form of the creating open, since a variadic `open` cannot be declared), a write, `lseek` for the length, a read, and `unlink`.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.Bytes
  import Data.Nat
  import Data.Result
  import Data.String
  import Prim.Sys.Darwin

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    case sys-create "doc-fs-probe.txt" of
      Err e -> 1
      Ok fd -> case sys-write fd (string-to-bytes "hello") 0 5 of
        Err e -> 2
        Ok n -> case n == 5 of
          False -> 3
          True -> case sys-close fd of
            Err e -> 4
            Ok _ -> case sys-open-read "doc-fs-probe.txt" of
              Err e -> 5
              Ok rd -> case sys-lseek rd 0 sys-seek-end of
                Err e -> 6
                Ok size -> case size == 5 of
                  False -> 7
                  True -> case sys-lseek rd 0 sys-seek-set of
                    Err e -> 8
                    Ok _ ->
                      let buf = bytes-new 5 in
                      case sys-read rd buf 0 5 of
                        Err e -> 9
                        Ok got -> case got == 5 of
                          False -> 10
                          True -> case sys-close rd of
                            Err e -> 11
                            Ok _ -> case string-from-bytes buf of
                              None -> 12
                              Some text -> case text == "hello" of
                                False -> 13
                                True -> case sys-unlink "doc-fs-probe.txt" of
                                  Err e -> 14
                                  Ok _ -> 0
```

The path reaches C as an address: `sys-cpath` copies the text into a fresh buffer one byte longer and zeroes the last byte, because a `Bytes` value carries a length rather than a terminator. The buffer's handle is live for the whole call, which is what keeps its payload alive.

# Linking an object of your own

The CLI has no option that supplies an object or library. `--object`, `-l`, and `--lib` are not flags: a word after the entry file that is not one of the documented flags becomes a program argument, so the build succeeds and silently ignores it. The Rust API is the current route (`driver::run_source_objects` and `build_source_objects` link supplied object files, which is how the `ffi` and `llvm_contract` suites call real C fixtures).

A symbol that the linker cannot resolve fails the build, after the program has checked:

```
$ cinch check Main.cinch
$ cinch build Main.cinch -o prog
cinch: clang failed to link prog.ll: warning: overriding the module target triple with arm64-apple-macosx27.0.0 [-Woverride-module]
1 warning generated.
Undefined symbols for architecture arm64:
  "_docs_no_such_symbol", referenced from:
      _Main.absent in prog-c275ee.o
ld: symbol(s) not found for architecture arm64
clang: error: linker command failed with exit code 1 (use -v to see invocation)
```

State that dependency in your build instructions. A `foreign import` is a link-time obligation, so a program that reaches one is not standalone.

# Exports

`foreign export` wraps a definition that already exists. It needs the signature first:

```
cinch: 2:44: a foreign export wraps a defined function: declare `answer :: ...` before exporting it
```

<!-- cinch:run exit=0 -->
```cinch
module Main where
  answer :: I64 -> I64
  answer x = wrapping-add-i64 x 1

  foreign export ccall "cinch_docs_answer" answer :: I64 -> I64

  main :: Int
  main = if answer 41 == 42 then 0 else 1
```

The emitted symbol is the C name, with a wrapper that freezes the incoming word:

```
define i64 @"cinch_docs_answer"(i64 %c0) {
entry:
  %t0 = freeze i64 %c0
  %t1 = call i64 @"Main.answer"(i64 %t0)
  ret i64 %t1
}
```

# Aggregate crossings: `#[byval]` and `#[sret]`

An aggregate needs an explicit layout before it can cross. `#[repr(C)]` is that declaration; a record with no representation attribute is refused with the layout it computed instead:

```
cinch: 7:3: the recorded layout of Main.Big is `uniform-word` (no representation attribute), which has no C representation at the boundary (foreign function interface specification section 2: only `#[repr(C)]`, `#[repr(packed)]` and `#[repr(transparent)]` aggregates cross; boxed aggregates and fixed-discriminant data are rejected instead of guessed)
```

The default crossing for an aggregate parameter is a pointer to a call-scoped copy, not a by-value C argument. `#[byval]` and `#[sret]` spell the by-value forms. Both attach to the `foreign` declaration itself, and both are checked against the types they claim:

<!-- cinch:check -->
```cinch
module Main where
  #[repr(C)]
  record Big where
    a :: I64
    b :: I64
    c :: I64

  #[byval]
  foreign import ccall "sum_big" sumBig :: Big ->{Foreign Lib, Diverge} I64

  #[sret]
  foreign import ccall "mk_big" mkBig :: I64 ->{Foreign Lib, Diverge} Big

  main :: Unit ->{Foreign Lib, Diverge} Int
  main u = if sumBig (mkBig 5) == 18 then 0 else 1
```

This program checks but cannot link from the CLI, because `sum_big` and `mk_big` have no implementation in this repository. The declarations it emits are the point:

```
%"__sl_layout_0" = type [24 x i8]
declare void @"mk_big"(ptr sret(%__sl_layout_0) align 8, ptr noundef)
declare i64 @"sum_big"(ptr byref(%__sl_layout_0) align 8 nonnull align 8)
```

`sret` makes the result an explicit out-buffer and the LLVM result `void`. The parameter spelling is `byref(%layout)` rather than LLVM's `byval` keyword, because `byval` has a different machine meaning on some targets; the pointee type states the layout without changing the convention. Each attribute is refused where its claim is false:

| Claim | Refusal |
| --- | --- |
| `#[byval]` with no aggregate parameter | `cinch: 3:3: `#[byval]` spells aggregate parameters crossing by value; this binding marshals no aggregate parameter` |
| `#[sret]` with a non-aggregate result | `cinch: 9:3: `#[sret]` spells an aggregate result crossing by value; this binding's result is not an aggregate` |
| An aggregate result with neither | `cinch: 8:3: a foreign result cannot be an aggregate: `Main.Big` follows the platform's by-value struct ABI, which this slice does not guess ... return through an `Out` cell or an `Owned` pointer, or spell the by-value crossing `#[sret]` |
| Either attribute on a definition | `cinch: 2:3: `#[byval]` and `#[sret]` attach to a `foreign` import or export declaration ...` |

There is no `#[extern]` attribute. A foreign boundary is declared by the `foreign` keyword, and the only attributes that change its ABI are `#[byval]` and `#[sret]`. Any other name must be compiler-known or declared by a macro, and the rejection names both routes:

```
cinch: 2:3: unknown attribute `reader-note`: outside the `meta` namespace, an attribute name must be compiler-known or declared by a macro (`macro name attributes (reader-note) :: ...`)
```

# Representation attributes

`#[repr(...)]` is the layout vocabulary a boundary reads. It attaches to a type or to a field of a `repr(C)` record. Learn the byte offsets and packing rules from [type layout](24-type-layout.md); the boundary-relevant rejections are these:

| Written | Refusal |
| --- | --- |
| `#[repr(transparent)]` on two represented fields | `needs exactly one represented field, found 2` |
| `#[repr(C, align(3))]` | `repr(align(N))` takes a non-zero power of two |
| `#[repr(C, packed, align(8))]` | `repr(packed)` removes padding and `repr(align(N))` raises alignment; they conflict |
| `#[repr(Rust)]` | `Rust` is not a representation (expects `C`, `transparent`, `align(N)`, `packed`, `packed(N)`, or `u8`/`u16`/`u32`/`u64`) |
| `#[repr(C)]` on multi-constructor `data` | use `#[repr(u8\|u16\|u32\|u64)]` for a fixed discriminant |

The layout a type gets is part of its interface. Changing a `#[repr]` changes the recorded layout and the interface identity, which makes dependents built against the old identity stale (see [upgrades](17-upgrades-and-temporal-contracts.md)).

# Function attributes

| Attribute | Placement | Meaning |
| --- | --- | --- |
| `#[inline]`, `#[inline(always)]`, `#[inline(never)]`, `#[noinline]` | function declaration | Inlining hint. One hint per function. Refused on a `foreign` declaration: `` `#[inline]` is a hint on a function declaration `` |
| `#[cold]` | function declaration | Cold-function hint |
| `#[weight(n)]`, `#[likely]`, `#[unlikely]` | clause, alternative, guard, `if` | Decision-point weight; one weight per alternative |
| `#[meta(key = "value")]` | any supported position | Nonsemantic metadata, recorded and otherwise ignored |
| `#[profile "jpl"]`, `#[deny-warnings]` | module header | Module policy |

`#[meta]` is the one attribute a program can attach freely, and it does not change behavior:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  #[meta(owner = "docs")]
  answer :: Int
  answer = 1

  main :: Int
  main = if answer == 1 then 0 else 1
```

The complete list of names, arguments, placements, and conflicts is the [attribute index](23-attributes-and-pragmas.md).

# Inline primitives

A primitive has an ordinary type and one body per target-IR family. The family names are `llvm`, `wasm`, `asm`, `clr`, and `jvm`; only `llvm` and admitted `asm` have lowering paths today.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  prim add-word :: U32 -> U32 -> U32 where
    llvm {
      %sum = add i32 %0, %1
      ret i32 %sum
    }

  main :: Int
  main = if add-word 40 2 == 42 then 0 else 1
```

Bodies take clauses after the fragment: `when <atom>` selects a body by the compile target, `justify <term>` records why a representation choice is sound, `uses (name :: type), ...` declares the callee vocabulary the fragment may use, `immarg 0, 1` marks immediate arguments, and a leading `unsafe` on one body marks that body unverified so its declared row is trusted instead of checked.

## Target conditions

A `when` clause is one atom read off the target profile, and every atom is also usable negated, with the atom in parentheses:

| Atom | Holds when |
| --- | --- |
| `Available <capability>` | the profile's capability evidence names it, which is a `signature` (platform proofs and SIMD specification section 3) |
| `arch <name>` | the target's architecture is `<name>`: `x86_64`, `aarch64`, `wasm32` |
| `os <name>` | the target's operating system is `<name>`: `darwin`, `linux`, `windows` |
| `libc <name>` | the target links `<name>`: `glibc`, `musl`, `libSystem`, `none` |
| `endian <little\|big>` | the target's byte order is the one named |
| `word-size <bits>` | the target's `Usize` and `Isize` width is `<bits>`: `32`, `64` |
| `target <triple>` | the target's whole triple is `<triple>`, the exact-match escape hatch |

A body may carry several clauses, and they are a conjunction: `when os linux when arch aarch64` holds on Linux on aarch64 and nowhere else. `when not (os windows)` is the negation of `os windows`, so it holds on Darwin and Linux.

Every atom reads the compile target profile and never the build host, so a cross compile selects by the artifact's own target.

The same vocabulary and the same conjunction are available in two further positions, both covered in [the module page](07-modules-and-packages.md#conditional-imports-and-declarations): `import Mod when <clause>` (where a false clause makes the import contribute nothing) and `when <atom> <declaration>` (where a false clause removes the name from the interface, the exports and code generation). The atoms mean the same thing in all three positions.

A conditional body wins over the baseline body (no `when`), and a prim whose conditions no body satisfies refuses by name:

```
cinch: 3:3: no body applies for the target profile `x86_64-unknown-linux-gnu` (`x86_64-unknown-linux-gnu`): the `asm` body's `when os windows` does not hold (`os windows` fails: the target's os is `linux`). A `when` clause is one atom over the compile target: ...
```

The refusal names the atom that failed and what the target is instead, and a missing capability reads differently from a platform mismatch: the capability account lists the evidence the profile does carry.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  prim pick :: U32 -> U32 where
    llvm {
      %s = add i32 1, %0
      ret i32 %s
    } when not (os windows)
    llvm {
      %s = add i32 %0, 1
      ret i32 %s
    }

  main :: Int
  main = if pick 1 == 2 then 0 else 1
```

Containment parses the fragment before emission and refuses what an inline body may not do:

| Fragment | Refusal |
| --- | --- |
| `%out = call i32 @puts(ptr null)` | `containment (no I/O): `@puts` is I/O (a body performs no I/O)` |
| `unreachable` | `containment (no fault paths): `unreachable` is a fault path beyond a declared failure mode` |
| `invoke`, `resume`, `landingpad`, `catchswitch` | `containment (no-unwind): ... may raise or unwind out of the body` |
| `br`, `switch`, `indirectbr` | `containment (no-escape): ... is a jump` |

A reserved family is refused by name rather than half-supported:

```
cinch: 3:5: the `clr` target-IR family is reserved: its fragment grammar lands with its backend (primitive operations and inline target IR specification section 3)
```

A conditional body whose atoms do not hold in the target profile is not selected, and the selection with its reason lands in the manifest. A body's family must also lower through this build's emission path, which today means `llvm` and admitted `asm`; registering a family name is not backend support.

Implementation: `repr.rs` (layout admission), `check_decls.rs` (binding contracts, containment), `codegen.rs` (declarations, stubs, freeze), `interface.rs` (the recorded contract); suites `ffi.rs`, `attributes.rs`, `prim.rs`, `llvm_contract.rs`, `repr_classes.rs`, `safety.rs`. Intended semantics: [foreign function interface specification](../specifications/foreign-function-interface-specification.md) sections 1 to 4, [attribute annotations specification](../specifications/attribute-annotations-specification.md) sections 1 and 4, [inline target IR](../specifications/primitive-operations-and-inline-target-ir-specification.md). Design notes: [LLVM contract attributes](../design/llvm-contract-attributes.md), [runtime object linkage](../design/runtime-object-linkage.md).
