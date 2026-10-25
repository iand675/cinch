# References and related work

Source: <https://chatgpt.com/space/page_e5b11110b6408191b7b5389ab932a93f>  
Exported: 2026-10-22T18:59:36Z

---

[Documentation overview](../overview.md)

These primary sources informed the design discussion. They are foundations and comparisons to study, not evidence that the proposed combination has already been implemented or proved sound. The language specifications define their own proposed behavior.

| Source | Relevant topic |
| --- | --- |
| [F star proof oriented programming](https://fstar-lang.org/tutorial/book/intro.html) | Dependent and refinement types, effects, proof automation |
| [Flux liquid types for Rust](https://doi.org/10.1145/3591283) | Refinements working with ownership |
| [Verus modes](https://verus-lang.github.io/verus/guide/modes.html) | Distinguishing specification, proof, and executable code |
| [Koka language documentation](https://koka-lang.github.io/koka/doc/book.html) | Effect inference and effect-polymorphic programming |
| [Sound gradual verification with symbolic execution](https://arxiv.org/abs/2311.07559) | Sound interaction of partial specifications and runtime checks |
| [Idris 2 quantitative type theory in practice](https://drops.dagstuhl.de/storage/00lipics/lipics-vol194-ecoop2021/LIPIcs.ECOOP.2021.9/LIPIcs.ECOOP.2021.9.pdf) | Dependent types, erasure, and usage information |
| [GHC rewrite rules](https://ghc.gitlab.haskell.org/ghc/doc/users_guide/exts/rewrite_rules.html) | User rewrite interfaces, matching, phases, and trust |
| [Stream fusion from lists to streams to nothing at all](https://www.cs.tufts.edu/~nr/cs257/archive/duncan-coutts/stream-fusion.pdf) | Producer state machines and elimination of intermediates |
| [Formalization of stream fusion](https://isa-afp.org/entries/Stream-Fusion.html) | Formal stream equivalence and fusion reasoning |
| [Agda coinduction](https://agda.readthedocs.io/en/stable/language/coinduction.html) | Guarded corecursion and stream bisimulation |
| [Lamport on safety liveness and fairness](https://lamport.org/tla/safety-liveness.pdf) | Infinite behaviors and temporal properties |
| [PlusCal tutorial on liveness](https://lamport.azurewebsites.net/tla/tutorial/session9.html) | Progress and fairness assumptions |
| [LLVM language reference](https://llvm.org/docs/LangRef.html) | Vector IR, data layout, masked memory operations, and backend semantics |
| [Alive2 bounded translation validation](https://users.cs.utah.edu/~regehr/alive2-pldi21.pdf) | Checking compiler transformations and limits of bounded validation |

# FFI, WebAssembly and concurrency sources

Standards context checked 22 October 2026. Runtime support must still be checked for the selected deployment host; a standards release does not imply every host implements it.

| Source | Relevant topic |
| --- | --- |
| [Rustonomicon FFI](https://doc.rust-lang.org/nomicon/ffi.html) | Native ABI, callbacks, ownership and unwinding concerns |
| [WASI 0.3 release](https://wasi.dev/releases/wasi-p3) | Released 11 June 2026; async functions, futures and streams |
| [WIT reference](https://component-model.bytecodealliance.org/design/wit.html) | Worlds, interface types and owned/borrowed resources |
| [Component model concurrency design](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Concurrency.md) | Tasks, async calls and component concurrency semantics |
| [WebAssembly security](https://webassembly.org/docs/security/) | Sandbox and host security boundaries |

The proposed structured-concurrency, STM and foreign-proof policies in this draft are language-design choices. These references do not establish their soundness.

# Further research required

The integration of dependent staging, typed module identity, dynamic software updating, and ownership-preserving migration needs a dedicated literature review before the core is finalized. This draft intentionally treats its mechanisms for those features as design proposals. It does not claim novelty or a proven advantage over every existing system.

# Runtime and effect inspirations

- [Zig 0.16 release notes: I/O as an interface](https://ziglang.org/download/0.16.0/release-notes.html) — explicit implementation passing for I/O.

- [Koka language book](https://koka-lang.github.io/koka/doc/book.html) — effect polymorphism and handler-based control abstractions.

The lexical capability and sequencing module combination in this draft is our proposal; these sources do not prove its integration sound.