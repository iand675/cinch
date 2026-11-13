# Debug info, unwind information, and the frame chain

Status: implemented for the seed emitter (2026-11-13). The self-hosted
emitter's share is section 7, which is not implemented and states exactly
what it needs.
Companions: `design/no-faults.md` (the fault paths), the foreign function
interface specification (the boundary), `design/compilation-speed.md`
(the stage link). No em dashes.

## 1 The obligation, in the user's words

"We don't throw exceptions at all from native code, but can do from C code
or whatever that we call, after all. So we should definitely support
emitting frame pointers, CFI. And debug info."

The first half is the correctness obligation and the second is the
diagnostic one. They are separate mechanisms and are argued separately
below:

- A foreign unwind crossing a Cinch frame with no CFI is undefined: the
  unwinder cannot compute the caller's frame, so it either produces a
  wrong trace or aborts the process. This is not a convenience.
- A fault's message naming a Cinch source line needs DWARF, and a trace
  naming Cinch units needs either symbols (which exist) or line tables
  (which did not).

## 2 What the backend already gave us

Measured on `main` at `72d8264b` before any change, with a two-level
program built by the seed:

| question | answer |
|----------|--------|
| is there `__compact_unwind` in an object? | yes, one entry per function |
| is there `__eh_frame` in the linked binary? | yes, 6327 FDEs on the probe |
| is there `__unwind_info`? | yes, built by the linker from `__compact_unwind` |
| are symbol names present? | yes, `_Chain2.outer` and friends, unmangled |
| is there any DWARF? | no: no `__DWARF` segment, `llvm-dwarfdump --debug-line` empty |
| is the frame pointer established? | no: `stp x29, x30` saves it, nothing sets `x29` |

So the unwind tables were already there, symbols were already there, and
two things were missing: the frame chain, and all of DWARF. That is why
this work is small on the CFI side and larger on the DWARF side.

## 3 Frame pointers

Every native unit's attribute group now carries
`"frame-pointer"="all"`. Measured:

- Before: `sub sp, sp, #0x80; stp x29, x30, [sp, #0x70]` and no
  `add x29, sp`, so `[x29]` is not a chain and a frame-pointer walk
  cannot work.
- After: `sub sp, sp, #0x80; stp x29, x30, [sp, #0x70]; add x29, sp, #0x70`,
  which is a chain.

`all` rather than `non-leaf` because a leaf that faults (an inline
`llvm.trap` path, a bounds check the backend lowers itself) still has to
be walkable, and the difference in code size is one instruction per leaf.

The **module flag does not work**: `!{i32 7, !"frame-pointer", i32 2}` in
`!llvm.module.flags` is accepted and ignored by Apple clang 21 and by
`llc` 21. Only the per-function attribute establishes the register. This
was measured both ways and is why the attribute is on the group rather
than a module directive.

The runtime is built with `-Cforce-frame-pointers=yes` from
`compiler/runtime/.cargo/config.toml`, because the trace's walk needs the
Rust frames between a Cinch frame and the fault to be a chain too.

## 4 Unwind information, and what `nounwind` does to it

The rule at `compiler/src/codegen.rs` refuses to claim `nounwind` on a
unit that reaches an extern the scan cannot see through, and claims it
otherwise. Two facts decide whether that rule is sound.

**Fact 1: a `nounwind` frame has no unwind information at all.** Measured
on Mach-O by reading the encoding the linker records for the frame:

| attributes | encoding for the frame |
|------------|------------------------|
| `uwtable "frame-pointer"="all"` | `0x03000040` (an unwind encoding) |
| `nounwind "frame-pointer"="all"` | `0x00000000` (`DW_EH_PE_omit`: none) |
| `nounwind uwtable "frame-pointer"="all"` | `0x00000000` (none) |

`uwtable` does not rescue it. The backend trusts `nounwind` and omits the
frame from `__compact_unwind`, and the linker then has nothing to put in
`__unwind_info`.

**Fact 2: an unwind across such a frame terminates the process.** Measured
with a C++ `throw` in a foreign callee called by a Cinch unit, caught in a
C++ frame above it:

| Cinch frame's attributes | result |
|--------------------------|--------|
| none (today's emission for that unit) | `caught: foreign C++ throw`, exit 7 |
| `uwtable` | `caught`, exit 7 |
| `nounwind` | `libc++abi: terminating due to uncaught exception`, exit 134 |
| `nounwind uwtable` | `terminating`, exit 134 |

So today's configuration is correct: a unit that reaches a foreign call
keeps its unwind information, and a foreign unwind traverses it. The
attribute is the thing that would break it, and the rule already declines
to claim it exactly where a foreign unwind could arrive.

**Conclusion for the rule.** Do not weaken `nounwind` on units that
genuinely cannot unwind: there the absence of CFI is honest, nothing can
unwind through them, and the attribute is what lets the optimizer drop
landing pads. Do not add `uwtable` either: it does not override
`nounwind`, so it would pay for CFI that the frame will not get.

### 4.1 What the rule should become

The current rule is a *reachability heuristic*: "the scan could not see
through an extern". That is sound in the conservative direction (it never
claims `nounwind` where it might be wrong) but it is not a fact, and it is
coarse: a unit calling `sin` is treated exactly like a unit calling
something that might throw.

The user's proposal is to make it a declaration. An import says whether it
may unwind, in LLVM's vocabulary rather than `safe`/`unsafe`:

```cinch
foreign import ccall "foo" nounwind foo :: Int -> Int   -- cannot unwind; callers may claim nounwind
foreign import ccall "bar" unwind   bar :: Int -> Int   -- may unwind; callers may not claim it
```

`safe`/`unsafe` is the wrong pair for two reasons. Haskell's FFI already
uses them for "may block / may call back", which is a different question;
and in this language "safe" is the name of the property the language
exists to provide, so `unsafe` would read as "may corrupt your values" as
much as "may throw".

The default is the conservative one: an import with no marker is treated
as **may unwind**, because a wrong optimistic claim is a soundness bug and
a wrong conservative claim is a missed optimisation.

Then the rule is decidable: a unit may claim `nounwind` iff every extern
it reaches is *declared* `nounwind`. The `unwind` marker is also what
makes "the frame stays traversable" a per-unit decision rather than a
global flag.

**This is not implemented in this lane.** It needs the marker in the
parser, in `ForeignPlan`, and in the scan; the parser change alone touches
the foreign-declaration grammar that the self-hosted frontend also parses
(`bootstrap/Bootstrap.Parse.cinch`), so it is its own slice. The rule the
code should implement is the one above, and the current heuristic is a
sound approximation of it.

### 4.2 One hole in the heuristic, found and left alone

`Callee::Indirect` (`__apply`, `__cinch_demand`) is classified as *not* an
extern, so a unit whose only "opaque" call is an indirect application can
still claim `nounwind`. Whether that is reachable depends on whether a
unit that applies an unknown closure can be reached with an unwinding
closure in the word.

Every probe built to demonstrate it was folded by the simplifier before
emission (the inliner copies the whole chain into `main.exit`), so this is
**unconfirmed**. The fix, when the declaration lands, is the same one:
`__apply` calls a word, so a unit that reaches `__apply` reaches something
undeclared, and must not claim `nounwind` until the declaration covers
closures too. That is a one-line change to `classify_callee`'s
`Indirect` arm plus the class rule, and it is the conservative direction.

## 5 DWARF, and where the seed stops

The metadata a module needs is small and is now emitted:

- `!llvm.dbg.cu` naming one `DICompileUnit`,
- `!llvm.module.flags` with `Dwarf Version` (5) and `Debug Info Version`
  (3),
- one `DIFile` per module,
- a `DISubprogram` per unit with its start line,
- `DILocation`s on the instructions that matter.

A line table produced this way maps a PC to a Cinch file and line
correctly at `-O2`; verified with a hand-built module through
`llvm-dwarfdump --debug-line`, which reports the expected line and column
per address and an `end_sequence` at the right place.

### 5.1 The blocker, stated exactly

**The seed's `Core` has no spans.** `compiler/src/core.rs`'s `Core` enum
has 40 node shapes and zero `Span` fields; the frontend's `ECore` has the
same shape. The only positions that survive lowering are the ones the
declaration table keeps.

The self-hosted emitter is in the opposite state:
`bootstrap/Bootstrap.Core.cinch`'s `CoreTerm` carries `Span` on **every**
node (`CApply :: Span -> UType -> CoreTerm -> CoreTerm -> CoreTerm` and so
on). So the information exists in the self-hosted pipeline and is thrown
away by the seed's.

That is why DWARF is not in this lane. Getting `!dbg` on "every call and
every instruction that can fault" needs a span on the node that lowers to
that call, which means either:

1. **Add `Span` to the seed's `Core` nodes that can fault** (about 12 of
   the 40: `App`, `Proj`, `InstDict`, `Case`, `Demand`, `BoxGet`,
   `RefSet`, `Fault`, the arithmetic prims), and thread it through the
   checker's `lower` and through every rewriter (`simplify`, `rc`,
   `specialize`, `stack`, `coalesce`). The mechanical cost is 145
   `Core::App` construction sites outside tests plus the same for the other
   faulting nodes; the real cost is that a span in `Core` changes `Core`
   equality and hashing, which the optimizer artifacts and the Core
   goldens depend on.
2. **Keep spans out of `Core` and recover them per unit instead**: emit a
   `DISubprogram` per unit with its declaration line (which needs only a
   span on `FnDef`, one field) and attach one `DILocation` per unit. That
   gives a symbolizer the unit and its source line, not the exact call
   site. It is much cheaper and it is the honest intermediate step: a
   trace then says `Main.level3 at Main.cinch:7` rather than `:9:14`.

Option 2 is what this lane's metadata plumbing is shaped for, and it is
the recommendation for the next slice. Option 1 is what the acceptance
criterion ("every call and every instruction that can fault") literally
asks for.

### 5.2 What the Cinch-side emitter must build

For `bootstrap/Bootstrap.RuntimeIr.cinch`, the metadata it must construct
from its own span table is:

- one `DIFile` per module (it already knows the module name and the source
  file through `Bootstrap.Check`'s module table),
- one `DISubprogram` per emitted unit, `scope` the `DIFile`, `line` the
  unit's declaration span's line, `spFlags: DISPFlagDefinition`, `unit`
  the compile unit,
- one `DILocation` per instruction, `scope` the enclosing `DISubprogram`,
  `line`/`column` from the `CApply`/`CApplyEff` span it lowered from,
- the `!llvm.dbg.cu` and `!llvm.module.flags` header nodes.

The spans are `Text.Lex.Span`, which is the same interned handle the seed
uses, so `span-line` and `span-column` are the whole accessor surface. The
emitter's metadata-id counter is the one it already keeps for `!range` and
`!prof`; `DICompileUnit`/`DIFile`/`DISubprogram` nodes must be registered
in the same table so their ids do not collide.

The order matters for the fixed point: if the self-hosted emitter emits
DWARF and the seed does not, stage 2 and stage 3 still agree with each
other (both are built by a Cinch compiler), so the fixed point holds. What
breaks is the seed's own IR tests, which compare against the seed's
emission.

## 6 The trace at the failure point

`panic_exit` (`compiler/runtime/src/abi.rs`) writes its one line and exits
71. It now optionally writes a trace after that line, from
`compiler/runtime/src/trace.rs`.

The walk is the **frame chain**, not the platform unwinder, deliberately:
an unwind needs CFI for every frame between the fault and the catcher, and
section 4 shows that a legitimately `nounwind` frame has none. The chain
needs only that each frame stored its caller's frame pointer, which
section 3 makes true.

`CINCH_BACKTRACE` controls it:

| value | effect |
|-------|--------|
| unset, `0`, `false`, `no`, empty | no trace (the default) |
| anything else (`1`, `full`, `yes`) | the trace |

Off by default because the message is the program's contract and the
tests assert on it exactly.

Each line prints the absolute address **and** the file address
(`pc - slide`), because the absolute address of a position-independent
binary names nothing to a symbolizer given the file. `CINCH_SYMBOLIZER`
may name a program (`llvm-symbolizer --obj=<exe>`) that turns an address
into a name and a `file:line`; when it is absent the trace still prints
the addresses, which are what a DWARF build resolves.

The walk is bounded by a depth cap, a monotonicity check and a per-step
limit, because a fault path cannot trust the stack it is reading.

Measured, on a program whose deepest Cinch frame calls a C function that
faults through `cinch_panic`:

```
cinchruntime: fault deep in the chain
cinch-backtrace: 23 frame(s)
  #0  0x0000000102b21c34 +0x1000b1c34
  ...
  #4  0x0000000102a70b40 +0x100000b40
  #5  0x0000000102a70980 +0x100000980
  #6  0x0000000102a709e0 +0x1000009e0
```

and those three file addresses are inside `_Deep3.main.exit` (0x9a0),
`_Deep3.id3` (0x98c) and `_Deep3.main.exit` (0x9a0) respectively: the walk
crossed the C frame and named the Cinch frames above it.

## 7 What is not done

- **DWARF emission** (section 5.1): blocked on spans in `Core`, with the
  two options and the recommendation stated.
- **The `nounwind`/`unwind` declaration** (section 4.1): the rule the code
  should implement is stated; the parser, `ForeignPlan` and the self-hosted
  frontend changes are not made.
- **The `Indirect` hole** (section 4.2): unconfirmed by construction, with
  the fix stated.
- **Unwind tables at the stage link**: `__compact_unwind` is emitted by
  clang regardless of `-funwind-tables` on Mach-O (measured: the flag
  changes nothing), so the stage link needs no flag. On ELF the equivalent
  is `-fasynchronous-unwind-tables`, which is clang's default for a hosted
  target; a freestanding target would need it spelled.
