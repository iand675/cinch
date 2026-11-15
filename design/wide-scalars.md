# Wide scalars: native unboxed `I128` and `U128`

Status: phase 1 implemented in the seed compiler, 2026-11-15 (section
11 records what landed and where it departs from the text above it).
Decided by the user the same day: native
unboxed `i128` (not a tagged or boxed word), the whole primitive-types
surface for both types, an opt-in extension module rather than the core
prelude, and the stage compiler at parity with the seed compiler. No em
dashes.

Companions: `design/layouts-and-specialization.md` (the class model and
the expanding-edge rule this note extends), `design/narrow-value-
representation.md` (the boundary slice; its section 8 deferred wide
integers to "its own argument", which is this note), and
`specifications/primitive-types-specification.md` (section 2 reserves
`I128`/`U128` as an optional extension; section 12 left their placement
open).

## 1 The constraint

The primitive types specification, section 1: a primitive machine scalar
"occupies its stated fixed layout, is passed, stored and compared inline,
and involves no indirection, allocation or ownership transfer". Both
compilers hold every value in one `i64` word (`compiler/src/repr.rs`
header; `bootstrap/Bootstrap.RuntimeIr.cinch` header). A 128-bit scalar
does not fit, so it needs a representation of its own, and that
representation has to reach every place a value is held.

## 2 The ontology: a fourth representation class

A representation class says how a value is physically held. Today there
are three, all one word: `Raw` (never counted), `Obj` (a counted pointer),
`TObj` (an immediate with low bit 1, or a counted pointer). The class is
already the axis the collector specialises on, and it is already carried
on every `Core` binder, constructor field, unit parameter and result.

`Wide` joins them: **a 128-bit scalar, held as LLVM `i128`, never counted,
two words wherever a layout counts words.** Its letter is `W`.

| Class | Words | Counted | LLVM value | Map words |
|---|---|---|---|---|
| `Raw` | 1 | no | `i64` | `raw` |
| `Obj` | 1 | yes | `i64` (a pointer) | `obj` |
| `TObj` | 1 | if not immediate | `i64` | `tobj` |
| `Wide` | 2 | no | `i128` | `raw`, `raw` |

`repr_of` maps `Ty::Prim(I128 | U128)` to `Wide`; nothing else is `Wide`.
Making width a class, rather than a second axis beside it, is deliberate:
a `Wide` value is never counted, so the reclamation half of the class
says nothing new, and the width half is exactly what a class vector
specialises. A generic unit reached at `I128` gets its own `@W` copy whose
binders are `i128`, so `map`, `fold` and `Option I128` stay unboxed with
no new machinery. `Wide` is a leaf like `Raw` (no type contains it as a
wrapper), so the expanding-cycle argument of
`layouts-and-specialization.md` section 4 is unchanged and copies stay
finite: at most `4^k` for `k` type variables, and only reached ones.

The pointer map needs no change. Part II section 14.4 made maps
word-granular; a `Wide` field is two consecutive `raw` words, stored at an
8-aligned payload offset (`load i128, ptr %f, align 8`).

## 3 Positions: where a value may be wide

Every place a value is held is one of two kinds, and the kind is a fact
about the position, never about the value.

**Wide positions** hold a `Wide` value as an `i128`:

- locals: `Let`, `CPat::Var`, `Join` parameters (slots are
  `alloca i128, align 16`);
- a unit's parameters and result (`FnDef.repr`): `define i128 @f(i128 %p0)`;
- direct calls to a known unit, including prim units;
- constructor fields of a class-vector copy (`Some@W`): two payload words;
- `Proj` of a wide field.

**Word positions** hold exactly one word whatever the value's class,
because the code that reads them is shared across classes or arities:

- the apply frame (`__apply`'s `argv`, the stub's merged frame) and the
  result word of an indirect call;
- a closure's saved environment words;
- a dictionary's strict value slots;
- `Demand`, lazy and knot cells, `BoxInit`/`RefSet` cells;
- task, `Par` and STM payload words, and quotation captures.

**The edge rule.** A `Wide` value entering a word position is boxed: one
allocation of a 16-byte payload (two `raw` words, tag `WIDE_BOX`), and the
position holds the `Obj` word. Leaving a word position into a wide one
unboxes: two loads and a drop of the box. This is
`layouts-and-specialization.md` section 4's expanding-edge boxing, for
the one layout that does not fit a word.

The edges are made explicit in `Core`, not left as conventions inside the
emitter. A pass after specialisation (`compiler/src/wide.rs`) walks every
unit with one fact per value position, `Wide` or `Word`, and inserts
`Core::WideBox(e)` and `Core::WideUnbox(e)` where the two disagree. The
emitter then has two total rules: a wide position's operand is `i128`, a
word position's is `i64`, and the only conversions are the two explicit
nodes. Reference counting runs after the pass, so the boxes are counted
like any other `Obj` word.

The apply stub is the one edge the pass cannot see from `Core`, because
it is generated per unit: `@"U.app"` unboxes each `Wide` parameter it
takes from the frame and boxes a `Wide` result, from `unit_params`, the
same vector it already reads to count saved words. A closure call site
boxes its `Wide` arguments (the pass sees the `App` of a non-known
callee), so the two halves agree on "a wide argument crosses the frame
boxed".

## 4 Literals and the refinement solver

`Core::Int(i64)` cannot carry a 128-bit literal. `Core::Wide(u128)`
carries the bit pattern (a `U128` value, or an `I128` value in two's
complement), and emits as a decimal `i128` operand.

The checker's literal range check computes in `i128` (`literal_i128`), so
`U128`'s maximum, `2^128 - 1`, is not representable there. Literals for
the wide types are parsed into a sign and a `u128` magnitude, then
checked against the type's range, which fits that pair exactly.

The refinement solver's integer sorts are `IntSort::Bounded(i128, i128)`,
which cannot state `U128`'s range. The bounds are therefore declared
without refinements (`max-bound-u128 :: U128`, not `refined (r == ...)`)
until the solver carries a wider sort. Section 2 of the specification
does not require the refinements; the narrow types' bounds have them
because the solver can state them.

## 5 Operations

`Scalar` gains `I128` and `U128`, so the `PrimOp` families parameterised
by `Scalar` (`Arith`, `Neg`, `CheckedDiv`/`Rem`, `Shift`, `Rotate`,
`Bit`, `BitNot`, `BitScan`, `Cmp`, `Conv`, `Bound`) cover them by
construction, and every `cinch.prim.<op>I128` name resolves through the
existing suffix parse. Their prim units take and return `i128` (a `Wide`
unit), so their bodies are the plain LLVM instruction at width 128:
`add i128`, `@llvm.sadd.with.overflow.i128`, `sdiv i128`,
`@llvm.ctpop.i128`, `@llvm.fshl.i128` for rotates. There is no canonical
word form to restore after the operation, because the value is its own
width.

Conversions between a wide and a word scalar are `trunc`, `sext` and
`zext` with the same checked, saturating and wrapping rules as every
other pair; conversions with floats are `sitofp`/`uitofp`/`fptosi`/
`fptoui` at `i128` with section 7's checks. `Int` and `Nat` conversions
go through new runtime entries (`cinch_int_from_i128`,
`cinch_int_to_i128`, the `Nat` pair, and their fit tests), whose Rust
signatures use `i128`/`u128`, which Rust passes with the C ABI's
`__int128` convention.

The instances (`Eq`, `Ord`, `Bits`, `Show`) are ordinary prelude
instances over these prims; their method units are `Wide` units, reached
through a dictionary by the apply stub's edge rule.

## 6 The module

`Data.Wide`, embedded in `cinch-base` and ordered after `Prelude`, so it
is reachable only by `import Data.Wide` (the tier `Data.Set` uses). The
core prelude, `Data.Machine`, and the bootstrap sources do not change, so
no existing program sees a new name. `Show` instances live beside the
types in `Data.Wide`, which imports `Text.Show`; the cycle that keeps the
core types' `Show` instances in `Text.Show` does not exist for an opt-in
module.

## 7 The foreign boundary

`I128` and `U128` are C's `__int128` and `unsigned __int128`, crossing as
LLVM `i128`, which is clang's own spelling on x86-64 and AArch64. Inside
a by-value aggregate (`compiler/src/cabi.rs`) a 16-byte integer leaf is
two System V INTEGER eightbytes at a 16-aligned offset, one `i128`
register pair on AAPCS64, and by reference on Microsoft x64, as clang
classifies `__int128` members; the current refusal of wide leaves is
replaced by those rules.

wasm32 lowers `i128` multiply, divide and remainder to compiler-rt calls
(`__multi3`, `__divti3`, `__udivti3`, `__modti3`, `__umodti3`), which the
runtime archive provides through Rust's `compiler_builtins`.

## 8 Interfaces and ABI

`W` appears wherever class letters are written: unit names (`f@W`),
`.cinchif` representation records, the object ABI's class JSON
(`object_abi`, `parse_object_roots`), and `coretext`. A reader that
predates `W` must refuse such an interface rather than misread it, so
`RUNTIME_ABI_VERSION` moves once, with this change.

## 9 The stage compiler

The stage compiler (`bootstrap/*.cinch`) checks and lowers its own
programs; it does not compile the base library, which comes from the
pinned producer (`bootstrap/library-provider.lock.json`). Parity
therefore has two halves:

- **its own checker and backend**: `I128`/`U128` as kernel types with
  128-bit literal bounds (`Bootstrap.Type.integer-bounds`), a fourth
  representation code for `Wide` beside its `0`/`1`/`2`, wide locals,
  parameters, results and fields in `Bootstrap.RuntimeIr`, the edge rule
  at its word positions, and the foreign marshal at `i128`;
- **its view of the library**: `Data.Wide` reaches stage programs once the
  provider lock and the seed pin move to a revision that contains it, with
  `bootstrap/library-object-roots.json` entries for the units stage
  programs reach.

The fixed point (`tooling/bootstrap.sh check`) is the gate for both.

## 10 Phases

Each phase lands green: `cargo test --lib`, the affected integration
suites on clang 19 and 20, and from phase 4 the fixed point.

1. **The class and the core path (seed).** `Scalar::{I128, U128}`,
   `Class::Wide` and its letter, `Core::Wide`, wide literals, `repr_of`,
   the collector at four classes, the emitter's wide positions (slots,
   headers, direct calls, constructor fields, `Proj`), the `wide.rs` edge
   pass with `WideBox`/`WideUnbox` and the apply stub's half, the ABI
   version bump, and `Data.Wide` with bounds, the arithmetic families,
   division and comparison, plus `Eq`/`Ord`.
2. **The whole surface (seed).** Shifts, rotates, bitwise and
   bit-inspection families, `Bits`, every conversion with the other
   twelve numeric types and the floats, `Int`/`Nat` conversions and their
   runtime entries, `Show`, and the constant folder at 128 bits.
3. **The boundary (seed).** `__int128` imports and exports, the `cabi`
   leaf rules, and the wasm32 link.
4. **The stage compiler.** Section 9's two halves, and the fixed point.

## 11 What landed in phase 1, and where it departs

Phase 1 is in the seed compiler (`compiler/src`), with `tests/wide.rs`
running every lowered position against real hardware under the leak
checker. Five decisions differ from, or refine, the text above.

**The box is a constructor, not two new `Core` nodes.** Section 3 named
`Core::WideBox` and `Core::WideUnbox`. The edge pass (`compiler/src/wide.rs`)
uses the constructor machinery instead: a box is an application of the
builtin one-field constructor `<builtin>.WideBox`, whose field is `Wide`,
and an unbox is `let b = e in b.0`, a projection of that field. The pass
runs at the head of `crate::rc::insert_with`, before counting, so the
unbox's binder is an ordinary owned `Obj` word that reference counting
reads once and drops, and a box's allocation, layout and descriptor are
the constructor path's. No emitter rule is special to boxes. The apply
stub, the one edge outside `Core`, boxes and unboxes with the same
two-raw-word layout (`codegen::wide_box`/`wide_unbox`). The pass is
idempotent, so code generation also applies it to a program that never
reached reference counting.

**A wide boundary is a `Cross::WIDE` crossing.** Headers, direct calls,
returns, `call_unit` and `musttail` already read a unit's narrow boundary
signature; a wide position is a crossing at 128 bits whose widen and
narrow are the identity. Unlike a narrowing it is the unit's type, so the
registry decides it first and never drops it for a tail group's
prototype mismatch, and a tail group with a wide member is not widened
through the word-typed `#tail` worker (a self tail call is still a loop,
and identical `i128` prototypes still `musttail`).

**A projection carries its predecessors' classes.** A field's word offset
is its slot plus the wide fields before it, and for a generic record those
classes are `Param` until the collector substitutes them. So
`ProjField::before` records the classes of the runtime fields before the
projected one (filled at lowering from the types `ECore::Proj` carries)
and is a class site the collector rewrites. An empty list means every
earlier field is a word, which keeps every existing projection's offset.

**The type names are kernel; only the operations are opt-in.** `I128` and
`U128` are registered like every scalar, so the names resolve everywhere
and type identity needs no module; `Data.Wide` holds every operation and
instance, so a program sees none of them without `import Data.Wide`.

**The ABI version does not move yet.** Section 8 planned the bump with
this change. Every reader of a class letter refuses an unknown one by
name rather than misreading it, so an interface or object root carrying
`W` is refused, not misread, by an older reader. The bump becomes
necessary when wide interfaces cross between pinned builds, which is the
stage compiler's provider-pin move in phase 4.

What phase 1 still refuses, by name, in one place
(`codegen::refuse_unlowered_wide`): a wide field of a boundary layout
(`repr(C)`, phase 3), and a wide value in a quotation. Two optimisations
decline a wide constructor rather than learn its layout: in-place reuse
(`rc`, as `planned`) and frame allocation (`stack`, as a layout refusal).
The constant folder and the compile-time evaluator refuse wide
operations; a `U128` has no solver sort.

