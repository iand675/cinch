# Verified primitives: a smaller trusted base than Lean

Status: design direction for review, 2026-10-29 (user direction: "more
powerful than Lean"). Concretes `specifications/platform-proofs-and-simd-specification.md`,
which already requires versioned instruction models (line 18), artifacts
that name "which lowering and instruction semantics were checked and which
backend stages remain trusted" (line 96), and prefers "proof certificates
checked by the language kernel" (line 98). Companion to
`design/dependent-core-kernel.md` (P7) and `design/inline-target-ir.md`
(P18). Bit-vectors are one domain of `design/proof-automation.md`, which
catalogues the other proof strategies (linear arithmetic, rewriting,
congruence closure, rings, induction, separation logic) and the shared
rule that search is untrusted and the kernel checks the result.

## 1 The problem

Representation instances and fast paths (`design/layouts-and-specialization.md`
section 7) are justified by proofs, and bit-level proofs (a bitset's
`view-cons`) bottom out in facts about shifts and masks on machine integers.
Today those facts are `prim` laws that are justified or assumed. The goal is
to prove them, and to prove that the code that runs computes them.

## 2 Where Lean extends its trust

- `bv_decide` bit-blasts a `BitVec` goal to SAT and checks the solver's LRAT
  certificate with a checker that has a soundness proof, but the check runs
  as compiled code and the tactic "adds the result of the compiled check as
  an axiom, thus adding the Lean compiler to the trusted code base"
  ([Lean.Elab.Tactic.BVDecide](https://lean-lang.org/doc/api/Lean/Elab/Tactic/BVDecide.html)).
- Lean's compiled code runs machine arithmetic (`UInt64` and friends)
  through `@[extern]` native implementations. The kernel reasons about the
  pure definitions; nothing proves the native code computes them. Lean's
  theorems are about the model, and the executed code is trusted to match it.

## 3 The chain, every link proven

1. **A model for every machine type.** `I64`, `U32`, ... are defined by pure
   bit-vector models, and each `prim`'s specification is its model function
   (`checkedAddI64` is addition modulo 2^64 with an overflow flag). Laws
   about shifts and masks are theorems over the models, proven in the kernel.
2. **A decision procedure whose certificate the kernel checks.** Bit-blast to
   SAT, let an external solver produce a certificate (LRAT), and check it in
   the kernel by evaluation, with no compiled-code axiom. Two explicit tiers:
   - kernel-checked: the trusted base stays minimal; the default, and
     required for anything a safety profile cites;
   - compiled-check: faster, with the Cinch compiler added to the trusted
     base and recorded in the manifest.
3. **`prim` bodies proven against their models.** The subset of LLVM IR that
   `prim` bodies use gets a formal semantics as Cinch definitions, versioned
   per the platform specification. "This `prim` body computes its model" is
   then a bit-vector theorem discharged by link 2. This is
   [Alive2](https://users.cs.utah.edu/~regehr/alive2-pldi21.pdf)'s
   translation validation for LLVM, ending in a kernel-checked proof instead
   of a trusted SMT verdict. `asm` bodies are proven against ISA models:
   [Sail](https://github.com/rems-project/sail) has
   [ARMv8-A (derived from ARM's specification) and RISC-V](https://dl.acm.org/doi/abs/10.1145/3290384)
   models to translate into Cinch definitions.
4. **Everything above rests on links 1-3.** Representation instances,
   specialized bodies, rewrites and SIMD proofs need no assumed arithmetic
   laws. The interface's `assumptions.declared` list shrinking to empty is the
   progress measure.

## 4 What stays trusted

| | Lean today | Cinch with links 1-3 |
| --- | --- | --- |
| The kernel | trusted | trusted |
| The compiler, for proofs | trusted (`bv_decide`'s axiom) | not trusted in kernel-checked mode |
| Native machine operations | trusted to match their model | proven against LLVM or ISA semantics |
| LLVM's lowering from IR to machine code | trusted | trusted, except `asm` bodies proven against an ISA model |
| Fidelity of the instruction models | not applicable | trusted (platform specification line 100); Sail's models are validated against hardware |
| Hardware | trusted | trusted |

A smaller trusted base than Lean for anything touching machine arithmetic,
not zero trust.

## 5 Costs and mitigations

- Kernel-checked certificates are slow (why Lean took the axiom): store each
  certificate in the content-addressed package store keyed by the goal's
  hash, so a fact is proven once and shipped with its package; trim
  certificates; keep any kernel arithmetic acceleration small and audited,
  since it joins the trusted base (as Lean's accelerated `Nat` arithmetic
  does).
- Bounded validation is not a theorem (the platform specification says a
  bounded result cannot be relabeled as one). `prim` bodies are almost all
  straight-line and fully decidable; loops (SIMD reductions) need the
  induction proofs the platform specification describes.
- The model semantics must be written: the LLVM instruction subset `prim`
  bodies use (transcribed from the LLVM language reference, for the
  instructions used), then ISA models (translated from Sail).

## 6 Sequencing

This belongs to the kernel slice (P7) and the `prim` slice (P18) and is
independent of reclamation:

1. Bit-vector models for the integer `prim`s.
2. The kernel's bit-blasting procedure and certificate checker.
3. LLVM semantics for the instructions existing `prim` bodies use.
4. Discharge the currently assumed `prim` laws one at a time.
5. ISA models for `asm` bodies.

## 7 Open decisions

- Certificate format (LRAT proposed, as Lean uses) and how certificates are
  stored and shipped.
- How far kernel evaluation is accelerated, and how the acceleration is
  audited.
- Whether the compiled-check tier exists at all, or kernel-checked only.
- The LLVM semantics' source of truth and versioning against LLVM releases.


## 8 Context for later agents

- The user's direction, verbatim: "I'd like this to be even more powerful
  than e.g. Lean, and I think you can prove that with them", in reply to the
  observation that bit-level representation proofs would otherwise rest on
  "a small, explicitly listed set of assumed arithmetic laws".
- What exists today (2026-10-29): a dependent core with `Eq`, `refl`,
  `congr`, `decide` and structural induction (`design/dependent-core-kernel.md`,
  status "design draft for implementation"); `rewrite ... justify` checked as
  kernel evidence or a citable class `law`, undischarged ones recorded in
  `assumptions.declared` (D55, `compiler/CONFORMANCE.md`);
  `examples/source/03-map-fusion.cinch` carries its own `mapFusion` proof and
  cites it; `examples/source/08-platform-simd.cinch` states an instruction law
  as an obligation (D228); `prim` bodies have `justify`/assumed tiers
  (`specifications/primitive-operations-and-inline-target-ir-specification.md`,
  `design/inline-target-ir.md`). Class laws are checked sample-wise by the
  P18 law layer.
- Where this plugs in: specialization's justification rule
  (`design/layouts-and-specialization.md` section 7) uses these proofs; the
  prims whose laws need discharging are the `cinch.prim.*` intrinsics
  (`PrimOp::Arith/Shift/Bit/Cmp` in `compiler/src/core.rs`), lowered inline to
  LLVM instructions and `llvm.*` intrinsics (`llvm.sadd.with.overflow.i64`
  and friends) in `compiler/src/codegen.rs`.
- Prior art to read first: Lean's `bv_decide` architecture (bit-blasting
  verified in Lean, CaDiCaL, a verified LRAT checker, the compiled-check
  axiom), [Alive2](https://users.cs.utah.edu/~regehr/alive2-pldi21.pdf)
  (its LLVM IR semantics is the closest existing formalization of the
  instruction subset), and the [Sail models](https://github.com/rems-project/sail)
  (ARM, RISC-V) for the `asm` path.
