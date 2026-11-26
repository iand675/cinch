# References and related work

Primary literature, languages, and standards that informed Cinch's design.

## Dependent types, refinements, and verification

| Source | Relevant topic |
| --- | --- |
| [F* — Proof-oriented programming](https://fstar-lang.org/tutorial/book/intro.html) | Dependent and refinement types, effects, proof automation |
| [Flux — Liquid types for Rust](https://doi.org/10.1145/3591283) | Refinement types combined with ownership |
| [Verus modes](https://verus-lang.github.io/verus/guide/modes.html) | Separating specification, proof, and executable code |
| [Idris 2 — Quantitative type theory in practice](https://drops.dagstuhl.de/storage/00lipics/lipics-vol194-ecoop2021/LIPIcs.ECOOP.2021.9/LIPIcs.ECOOP.2021.9.pdf) | Dependent types, erasure, and usage multiplicities |
| [Lean 4](https://lean-lang.org/doc/reference/latest/) | Proof kernel, tactics (`grind`, `omega`, `bv_decide`), small-object runtime |
| [Sound gradual verification with symbolic execution](https://arxiv.org/abs/2311.07559) | Sound interaction between partial specifications and runtime checks |
| [Agda coinduction](https://agda.readthedocs.io/en/stable/language/coinduction.html) | Guarded corecursion and stream bisimulation |
| [Alive2 — Bounded translation validation](https://users.cs.utah.edu/~regehr/alive2-pldi21.pdf) | Formal LLVM IR semantics and verifying compiler lowerings |
| [Sail ISA specifications](https://github.com/rems-project/sail) | Formal ARMv8-A and RISC-V instruction models |

## Effects, ownership, reclamation, and compilation

| Source | Relevant topic |
| --- | --- |
| [Koka language book](https://koka-lang.github.io/koka/doc/book.html) | Algebraic effect rows, handlers, Perceus reference counting, `fip` |
| [Perceus: Garbage Free Reference Counting with Reuse](https://doi.org/10.1145/3453483.3454032) | Precise compiler-inserted RC, drop specialization, in-place reuse |
| [FP²: Fully in-Place Functional Programming](https://doi.org/10.1145/3607840) | Statically guaranteed zero-net-allocation functional updates |
| [Mimalloc: Free List Sharding in Action](https://www.microsoft.com/en-us/research/publication/mimalloc-free-list-sharding-in-action/) | Segment-aligned per-thread small-object allocation |
| [Zig 0.16 — I/O as an interface](https://ziglang.org/download/0.16.0/release-notes.html) | Explicit lexical capability passing for I/O and allocators |
| [Compiling with Continuations, Continued](https://www.microsoft.com/en-us/research/publication/compiling-with-continuations-continued/) / [Compiling without Continuations](https://www.microsoft.com/en-us/research/publication/compiling-without-continuations/) | Contification and join points |
| [Call-pattern specialisation for Haskell programs](https://www.microsoft.com/en-us/research/publication/call-pattern-specialisation-for-haskell-programs/) | `SpecConstr` for closures and records of functions |
| [GHC rewrite rules](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/rewrite_rules.html) & [Stream fusion](https://www.cs.tufts.edu/~nr/cs257/archive/duncan-coutts/stream-fusion.pdf) | Checked rewrites and producer-consumer fusion |
| [An LLVM Backend for GHC](https://www.microsoft.com/en-us/research/publication/an-llvm-backend-for-ghc/) | Tables next to code via LLVM prefix data |
| [LLVM Language Reference](https://llvm.org/docs/LangRef.html) | Vector IR, contract attributes, `tailcc`, `musttail`, data layout |

## Concurrency, FFI, and WebAssembly

| Source | Relevant topic |
| --- | --- |
| [Lamport — Safety, liveness, and fairness](https://lamport.org/tla/safety-liveness.pdf) | Temporal contracts and explicit fairness/scheduler assumptions |
| [TL2 — Transactional Locking II](https://doi.org/10.1007/11864219_14) | Version-clocked software transactional memory |
| [The Rustonomicon (FFI)](https://doc.rust-lang.org/nomicon/ffi.html) | Native ABI, callbacks, ownership, and unwinding boundaries |
| [WASI 0.3](https://wasi.dev/releases/wasi-p3) & [WIT reference](https://component-model.bytecodealliance.org/design/wit.html) | WebAssembly Component Model interfaces, async, and resources |
| [WebAssembly security](https://webassembly.org/docs/security/) | Sandboxed host and component isolation boundaries |
| [The Power of 10 (Holzmann, JPL)](https://spinroot.com/gerard/pdf/P10.pdf) | Safety-critical coding rules mapped to Cinch's `jpl` profile |
