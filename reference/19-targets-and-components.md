# Target profiles, platform proofs, SIMD, and Wasm

A target profile states what an artifact can assume. Platform evidence is a validity condition, not an optimization wish or a guess based on the compiler host.

# Native and Wasm profiles

The Rust API exposes native and Wasm32 target profiles and explicit artifact profiles. The profile fixes pointer-sized widths, admitted IR families/features, runtime-evaluator admission, and optimization driving parameters. A profile can reject `eval` or a required feature. Target identity participates in artifacts and compatibility checks.

<span class="support-tag support-tag--planned">Coming soon</span> **command-line target selection.** The current executable accepts `check`, `emit-ir`, `build`, `run`, and `lsp`. It does not accept `--target wasm32`, `--component`, or a cross-target packaging command. The APIs and test fixtures below are current implementation surfaces; the complete end-user portable build workflow is planned.

# Feature evidence and SIMD

A target rewrite or primitive body can require a feature condition. Selection needs corresponding target evidence and must have an admitted baseline when the optional feature is absent. Requiring a feature that the artifact cannot assume is a compile-time refusal, not permission to emit instructions that might fault on a deployment machine.

SIMD kernels must preserve the scalar operation's meaning and the effects/resource constraints. Evidence can distinguish semantic requirements from representation-only choices. Selected rules and feature assumptions are recorded in the manifest, with compatibility checked before runtime reachability.

The shipped `Data.Target` demonstration currently lowers `simd-sum-wrap32` through its scalar loop, and `detect-vec-u32` conservatively selects the scalar path rather than performing runtime feature detection. <span class="support-tag support-tag--planned">Coming soon</span> feature-detecting dispatch and accelerated implementations for that demonstration must be distinguished from the existing proof/profile machinery.

The [platform specification](../specifications/platform-proofs-and-simd-specification.md) describes capability scopes, scalar fallbacks, width relationships, and target validity. Its full illustrative API is design material; use current Rust tests and the compiler vocabulary to distinguish the implemented subset.

# WIT and canonical ABI

The current WIT parser covers its admitted enum, record, variant, flags, union, alias, function, resource, world, list, option, result, tuple, ownership/borrow, stream, and future forms with named restrictions. Parsed syntax must also pass adapter/layout admission; parsing an item does not promise a complete implementation of the WIT ecosystem.

Canonical-ABI adapters record lowering/lifting and errors such as boundary/decode failures for the supported cases. The codec example has specific end-to-end assertions. Do not infer universal component-model interoperability from that one adapter path.

# External stream observations

`WasiStream a` and `WasiFuture a` are external handles. `wasiStreamRead`, `wasiStreamWrite`, and `wasiFutureWait` have foreign/divergence effects and boundary failure contracts. They do not have the productive `Stream a` codata guarantee. External scheduling, end-of-stream, and decoding remain observable outcomes.

The cooperative task profile preserves admitted memory/ownership semantics under its own execution restrictions. Native blocking-thread assumptions cannot simply be copied into a Wasm runtime.

# Additional backend families

`prim` permits per-family bodies. Current LLVM/native assembly paths are documented in [inline primitives](15-ffi-layout-and-attributes.md). **Coming soon — CLR/JVM execution:** these names are reserved and registered in the design, while executable backends are not implemented. Availability of a family name in syntax is not backend support.

Implementation: `core.rs` (`TargetProfile`/artifact profile), `ast.rs`/`parser.rs` (WIT), `interface.rs` (canonical adapters), `codegen.rs`; suites: `wasm.rs`, `optimizer.rs`, `prim.rs`.
