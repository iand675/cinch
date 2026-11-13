# Narrow value representation: beyond one word per value

Status: section 4's first slice implemented, 2026-11-12 (section 6a, "The
boundary slice, taken"). User direction: Cinch must eventually run on
microcontrollers, so "every value is a uniform machine word" cannot be the
only representation. No em dashes.

## 1 What is true today

Every `Core` value is one machine word. `compiler/src/repr.rs` states it as
the premise of the whole reclamation design:

> Every `Core` value is one `i64` word. A word's class says what a
> reclamation pass may do with it.

The class system (`Raw`, `Obj`, `TObj`, `Param`) is the *reclamation*
axis: it says whether a word is counted. It is not a width axis, and
`Class::Raw` covers everything from `Bool` to `I64` to a foreign handle.

At the function boundary this is unconditional. `codegen.rs::fn_header`
builds the parameter list from arity alone:

```rust
let params: Vec<String> = (0..arity)
    .map(|i| format!("i64 %p{i}"))
    .collect();
```

The only per-parameter variation is `noundef`, and the only per-result
variation is a `range(...)` attribute. A `U8` parameter is a full `i64`
that happens to hold 0..255.

The one exception is `prim` declarations, whose units carry the declared
ABI (`i8`..`i64`, `float`, `double`) through `abi_word`. So narrow types
already exist, but only *inside* a prim body; they are widened at the
boundary into and out of ordinary code.

`word_size` is already a target property (64 and 32 both exist in
`core.rs`), so the *word* is parameterised. The *width* is not.

## 2 Why the uniform word is load-bearing today

Four things depend on it, and each has to be answered before a narrow
representation can land:

1. **The apply stub.** `stub_def` merges a closure's saved words with the
   incoming ones into a fixed `[arity x i64]` frame and does `i64`
   arithmetic on the count. One stub serves every function of an arity
   because every argument is one word. Mixed widths need either
   per-signature stubs or a marshal step.
2. **`__apply` and `__cinch_demand`.** Indirect application takes
   `(i64 f, i64 argc, ptr argv)` and pushes excess words at the result.
   The argv block is a word array.
3. **Dictionaries.** A dictionary is a block of method closure pointers;
   `InstDict`, `DictProj` and `MethodFamily` all index it by word.
4. **Reclamation.** The pointer map, `SCAN_RAW`/`SCAN_OBJ`/`SCAN_TOBJ`
   and every count operation assume a word either is or is not a pointer.
   A narrow field changes what a slot *is*, not just what it holds.

That is why this is a calling-convention change and not an attribute
change. The `i64` appears 804 times in `codegen.rs`.

## 3 The design already written, and its gap

`design/layouts-and-specialization.md` already specifies most of the
answer:

- Every type has a layout (size, alignment, field offsets, pointer map).
- The default layout is compiler-chosen, starting from one word per field.
- Multi-word and narrow layouts are specialised **on demand**, with the
  compiler boxing at expanding edges (section 4).
- `repr(unboxed)` value types are stored inline wherever used, as a C#
  `struct` or an OxCaml unboxed record is.
- `#[unpack]` inlines a fixed-layout field into its parent.

What it does **not** specify is the function boundary: the document talks
about *fields and storage*, and stops before *parameters and results*.
`repr(unboxed)` is documented and, in the compiler today, essentially
unimplemented (`unboxed` occurs once, in a checker name list).

So the gap is narrow and specific: **the boundary**, not the layout model.

## 4 Proposal: narrow at the boundary where it is proved

A value crosses the boundary narrow when its type has a fixed narrow
layout and the callee's signature is monomorphic at that position. The
rule, in order of decreasing certainty:

1. **Monomorphic narrow scalars cross narrow.** A function whose
   parameter is declared `U8`/`I16`/`F32` and which is not a type
   variable's instantiation takes that width directly. This is the
   microcontroller case and the one that matters most.
2. **`repr(unboxed)` value types cross by value** under the existing ABI
   rules, which the FFI layer already states for `repr(C)`.
3. **Anything polymorphic or variable-width stays one word.** A `Class`
   parameter, a dictionary, a closure, a boxed ADT: unchanged.
4. **Narrowing is proved, never guessed.** A position whose width the
   compiler cannot state keeps the word form. This is the same discipline
   as the contract attributes: a fact the analysis cannot support is
   omitted, never guessed.

Mixed-width signatures then need the apply stub to convert, and the
narrow path must not bring back the per-count fan-out that
`design/stack-allocation.md` section 7.8 collapsed (the `caml_applyN`
shape was 973 per-count entries). This section first said "one stub per
arity" was current; that was stale. Since section 7.8 there is one
`@"U.app"` per function whose closure is taken, and it calls `U`
directly, so a narrowed `U`'s conversion lives inside the stub it
already has and costs no function at all (measured below, "The boundary
slice, taken"). Sharing one stub across a signature class vector would
instead need the closure to carry a code pointer and the stub to call
through it, trading every closure application's direct call for fewer
functions; the user chose the per-function stub.

## 5 What this unlocks

- `noalias` and `readonly`/`writeonly` on `Ref`/`Mut` become expressible,
  because a pointer can be a `ptr` rather than an `i64`.
- Narrow fields stop costing a word each, which is the microcontroller
  size case.
- `range` metadata stops carrying facts that the type could carry
  directly.

## 6 What must not break

- The uniform-word path stays the default and stays correct. Every
  existing test, interface and object must keep working unchanged.
- `runtime_abi` gates the layout algorithm. A layout change is an
  interface change, so this needs a version bump and a store
  invalidation, exactly as `layouts-and-specialization.md` says.
- The measured apply-stub win must survive: no return to per-arity
  fan-out.

## 6a What the survey found

Measured on `main` before any change:

- **The word is `i64` on every target, including 32-bit ones.** A wasm32
  module emits `define i64 @"Main.main"` and `inttoptr i64 %f to ptr`.
  The datalayout says `p:32:32`, so the pointer truncates on the way to
  a real pointer. clang accepts this at `-O2`: it is correct, not
  merely tolerated.
- **So the `Nat`/`Int` immediate encoding is not a live bug.** Its
  `(1 << 63) - 1` payload claim is consistent with a word that is
  actually `i64`. It becomes wrong only once the word itself becomes 32
  bits.
- **Therefore the word width and the encoding are one change.** Making
  the word 32 bits while the tag still claims 63 payload bits would be
  the bug. They must move together.
- **The runtime refuses rather than miscompiles.** Building for
  `wasm32-unknown-unknown` fails on the syscall layer having no wasm
  backend; `i686-unknown-linux-gnu` failed on the syscall numbers being
  pinned for x86_64 and aarch64. That honesty is why the targeted route
  works: each step is forced by a specific refusal.

### The first slice, taken

The i686 syscall column is in. The trap is `int 0x80`, whose sixth
argument register is `ebp`, the frame pointer; `esi` cannot be an
inline-asm operand on i686 at all because LLVM reserves it. Both travel
through memory and the trap loads them once the compiler is done with
them, `ebp` is saved and restored around the trap, and the result
sign-extends because the kernel's `-errno` convention has to survive a
32-bit register. `cargo check` for `i686-unknown-linux-gnu` now succeeds.

### wasm32: the front half already works

Measured, not assumed. The compiler emits wasm32 IR through
`TargetProfile::wasm32` (reachable from the test harness, not yet from
the CLI), and clang assembles it to a valid module:

```
w32.o: WebAssembly (wasm) binary module version 0x1 (MVP)
```

The module declares 10 imports and no exports. Six are the toolchain's
own (`__linear_memory`, `__indirect_function_table`, `exit`,
`__stack_pointer`, and the table and memory of a freestanding build);
the runtime surface a `main = 0` program actually needs is six functions:

| Import | Why |
| --- | --- |
| `cinch_int_to_i64` | the `Int` result of `main` |
| `cinch_drop_slow`, `cinch_drop_mt` | reference counting |
| `cinch_exit_check` | the exit-status check |
| `cinch_reclaim_enable` | reclamation |
| `cinch_layouts_register` | the layout table |

No `open`, no `mmap`, no `pthread`, no signals. So wasm needs a
**minimal runtime subset**, not a syscall backend: the 26-function `sys`
contract is what a *native* program reaches, and a wasm program reaches
six functions of the runtime proper.

That is the shape of the first wasm slice: gate the OS-only modules
(`pty`, `net`, `task`, `platform`'s unix paths) so they are absent on
wasm, and build the core (`block`, `rc`, `arena`, `bytes`, `nat`, `heap`)
against a wasm `sys` that answers `getenv` from the WASI environment and
nothing else.

### The wasm slice, taken

The runtime builds and a real program runs, so this is measured end to
end rather than assumed: `cinch emit-ir --target wasm32-unknown-unknown`,
clang, `wasm-ld`, and Node executing the module. `main = 0`,
`fib 20 == 6765`, a 100000-iteration accumulator, and `map`/`replicate`/
`length` over a named function all answer 0.

What it took, in the order the compiler and the runtime refused:

- **Gating by item, not by section.** `platform.rs`'s OS surface is
  ~37 items (filesystem, signals, processes, piped children, the thread
  and stack reservation). Wrapping each section in a `mod` and
  re-exporting it hid `env_words`/`nul_words`/`exit_code` from `pty.rs`
  and `program_stack_bytes` from `par.rs`; gating the individual items
  leaves every path a native build uses untouched.
- **A freestanding `sys` backend.** `mmap_reserve`/`mprotect` are the
  small-object allocator's only OS dependency, and they map exactly onto
  WebAssembly's reserve/commit split: `memory.grow` extends linear
  memory by whole pages and is contiguous by construction, so a
  reservation *is* a growth and a commit is already done. Everything
  else answers `ENOSYS`; `exit` traps, because a freestanding module has
  no process to leave.
- **A reservation the host will grant.** The allocator's list started at
  64 GiB, which no wasm host gives (`memory.grow` answers -1; Node caps
  around 1 GiB). A failed reservation leaves the allocator with no range
  at all, so every allocation goes to the `Exhausted` exit. The
  freestanding list is 256/64/16 MiB.
- **`musttail` needs the tail-call feature.** A `D104` group emits
  `musttail`, which WebAssembly has outside the MVP, and the assembler
  refuses it without the feature. It is a per-function attribute
  (`"target-features"="+tail-call"`), not a module directive, and every
  function carries it because a callee without it is not a legal
  `musttail` target.
- **A static initializer holds a pointer at the pointer width.** A
  linker relocates an address, not a widened copy of one, so
  `i64 ptrtoint (ptr @f to i64)` in a constant is refused
  (`unsupported expression in static initializer`). A closure's code
  word is exactly that. Where the pointer is 4 bytes the block storage
  is emitted `[2n x i32]` (each word its low half then its high half),
  which is the same bytes on a little-endian target, so the runtime
  reads the block unchanged.
- **`memcpy`/`memset` take a `size_t`.** The declarations and the call
  sites both carried an `i64` length, which wasm-ld reports as a
  signature mismatch against the runtime's own `(i32, i32, i32)`
  `memcpy` and which traps at run time.

- **A freestanding module registers its own layout table.** This one
  cost the most and was the last thing found. `llvm.global_ctors` is how
  a native process registers the table: libc runs the constructors
  before `main`. A bare Wasm module has no `_start` and no libc, so
  nothing ran them, the table stayed empty, and every shaped block
  answered `shaped block with no registered layout`. The symptom was a
  trap inside the runtime's `release` on the first `map` over a closure
  with saved words: five separate wrong theories followed before the
  message itself was made readable. The Wasm entry now calls the
  initialiser itself, and a host that also calls `__wasm_call_ctors`
  runs it twice, which is free because `cinch_layouts_register` is a
  plain store.

The lesson worth keeping: a trap on a target with no stderr says only
`unreachable`, so the first move is to make the runtime's message
readable (an exported buffer `panic_exit` fills, which is what found
this), not to reason from the stack. Every one of the six items above was
invisible in review and obvious once the target was actually run.

### The limit of what is verifiable here

This host is aarch64 macOS. `cargo check --target i686-unknown-linux-gnu`
type-checks the whole crate, so the code is known to compile; linking and
running need an i686 Linux toolchain that is not present. So the i686
path is compile-verified and not execution-verified, and the design note
says so rather than implying a test that does not exist. Adding the
target to `SUPPORTED_TARGETS` needs that toolchain first.

wasm32 is different: the toolchain is here. clang assembles
`--target=wasm32-unknown-unknown`, `wasm-ld` links, and Node runs the
module, so `wasm32-unknown-unknown` is in `SUPPORTED_TARGETS` and every
claim above was observed rather than inferred. The runtime's wasm backend
itself cannot be unit-tested from this host (it only compiles for the
wasm target), so its proof is the end-to-end run, not a `#[test]`.

### The boundary slice, taken

All three parts of section 4's first slice are in, with the uniform word
settled by the user's decisions of 2026-11-12: a 64-bit scalar stays 8
unboxed bytes, and a heap slot stays 8 bytes. Polymorphic code moves a
whole slot (a `List I64` element is 8 bytes), so the uniform word stays
the slot on every target. The eventual design specialises polymorphic
code by layout (`List@unboxed_1_bytes` through `List@unboxed_8_bytes`,
`List@boxed_Maybe`), emitted lazily as section 2 of
`layouts-and-specialization.md` does for classes; this slice narrows the
positions a type proves, and leaves that specialisation axis for later.

- **Machine scalars cross at their own width** (`ParamContract::narrow`,
  `Crossing::Bits { bits, signed }`). `U8` is `i8`, `I16` is `i16`,
  `U32`/`I32`/`F32` are `i32`, on every target. The body is unchanged:
  the entry widens, every return narrows, and the caller mirrors it.
  A crossing carries a width **and** a signedness, because the width is
  not enough on its own: the extension direction has to match the word
  convention, or the crossing silently changes the value. The only
  correct widening of a signed narrow scalar is sign extension, because
  the canonical word for a signed scalar is sign-extended
  (`cinch.prim.ltI8` is `icmp slt i64` over its operands,
  `minBoundI8` returns `i64 -128`), so `-2 :: I8` must cross back as
  `-2` and not as `254`. An unsigned scalar, a float's bits, a pointer
  and a `Nat` zero-extend, and an `Int` sign-extends; `signed` rides
  `Crossing::Bits` exactly as it rides `Crossing::Csize`.
  Every boundary conversion goes through `Cross::sext` (the entry, a
  direct call, the apply stub, a thunk's value, a foreign export's
  wrapper), which is why the direction is one flag on that struct rather
  than a change at five call sites.
- **A word that fits the pointer crosses at `csize`**
  (`Crossing::Csize`): a counted pointer (an ADT, a closure, `Bytes`, a
  tuple), a `Nat` and an `Int`. That is the slot on a 64-bit target, so
  native output is byte-identical to the tree before this slice (61
  programs including the 44K-line compiler bundle and the toolkit bundle,
  same SHA-256 through both binaries). On wasm32 it is `i32`: a pointer
  and a `Nat` zero-extend back into the slot, an `Int` sign-extends.
- **The immediate payload follows `csize`.** That is what lets a `Nat` or
  `Int` cross at `csize` at all: a `Nat` immediate is `0..=2^(csize-1)-1`
  and an `Int` immediate `-2^(csize-2)..=2^(csize-2)-1`, so the encoded
  word fits and extends back exactly. The runtime's constants derive from
  `usize::BITS`, `cinch_nat_from_i64` boxes above the bound instead of
  assuming every `i64` fits, and the compiler's literal folding and the
  inline `Nat` add's carry check take the same bound from the target. On
  64-bit targets the numbers are unchanged (`2^63 - 1`, `2^62`).
- **One registry answer.** The header, every direct call (including a
  dictionary factory's strict method, a thunk's value, a foreign export's
  wrapper and another module's declaration) and the apply stub all read
  `Registry::narrow_sigs`, so none can disagree with the callee. Units
  whose boundary is fixed by something else keep the word: primitives,
  constructors, `prim` declarations, widened tail-group members, and
  `main`, which the runtime calls through an `i64 ()` pointer.
- **`musttail` survives.** A tail call is `musttail` when the two LLVM
  prototypes agree (an `Int` and a `Nat` position share `i32`), and a
  recursive group whose members would narrow to different prototypes
  keeps the word for all of them, because constant stack is a language
  guarantee (D104) and a narrower boundary is not.

Two earlier defects surfaced on the way and are fixed: a widened unit's
worker read the contract directly and would have widened an `i64`
parameter as an `i8`, and a narrowed call carried `!range` metadata typed
for the word, which the verifier refuses.

Measured on wasm32, end to end through clang, `wasm-ld` and Node: `fib`,
`Nat` arithmetic across `2^30`, `2^31`, `2^32` and `2^40`, `Int`
arithmetic across `-2^30`, a closure mapped over a list of `3 * 10^9`
values, a 12-deep tree of ADTs, a narrowed `U8` closure through its
stub, and three mutual tail groups 3,000,000 calls deep all answer 0,
with and without `--no-inline`. The 44K-line compiler bundle emits 3,802
narrowed functions for wasm32 and clang assembles the module.

The apply stubs do not fan out:

| module | target | tree | defines | apply stubs | narrowed defines |
| --- | --- | --- | --- | --- | --- |
| compiler bundle, 43,959 lines | native | before | 7,128 | 128 | 0 |
| compiler bundle | native | after | 7,128 | 128 | 0 |
| compiler bundle | wasm32 | before | 7,091 | 128 | 0 |
| compiler bundle | wasm32 | after | 7,091 | 128 | 3,802 |

Narrowing adds no function on either target. The wasm32 module is 3.3%
more IR lines (5.79M to 5.98M), which is the boundary conversions.

## 7 Open questions

1. **Where does the width live?** `Class` is the reclamation axis. A
   second axis (a width vector, as `Param(u16)` is a class parameter)
   would specialise units the same way the collector already does.
2. **Does `__apply` learn widths, or marshal?** Marshalling is simpler
   and slower; teaching it widths duplicates the stub logic at the
   indirect site.
3. **How does the FFI boundary interact?** A `repr(C)` narrow struct
   already crosses by value; the question is whether a *bare* `U8`
   parameter should too.
4. **What is the smallest first slice?** Taken, and wider than this
   question first proposed: narrow scalars and `csize` positions at every
   direct call, with `__apply` still word-based and the marshal at the
   apply stub ("The boundary slice, taken"). It touches neither
   dictionaries nor reclamation.

## 8 Not in scope

Wide integers, vector types, and multi-word returns. Each needs its own
argument; this note is about not paying a word for a byte.
