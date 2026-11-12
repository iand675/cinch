# Narrow value representation: beyond one word per value

Status: proposal, 2026-11-12. User direction: Cinch must eventually run on
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

Mixed-width signatures then need a per-signature apply stub. The current
"one stub per arity" is what makes `stub_def` simple, and it is what
`design/stack-allocation.md` section 7.8 measured as worth having (the
`caml_applyN` fan-out was 973 functions and 7.4% of a check). So the
narrow path must not regress that: a stub is shared by every function of
one *signature class vector*, not one arity.

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
4. **What is the smallest first slice?** My reading: monomorphic narrow
   scalars at direct call sites only, with `__apply` still word-based and
   a marshal at the indirect boundary. That is provable, small, and it
   delivers the microcontroller case without touching dictionaries or
   reclamation.

## 8 Not in scope

Wide integers, vector types, and multi-word returns. Each needs its own
argument; this note is about not paying a word for a byte.
