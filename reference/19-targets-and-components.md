# Target profiles, platform proofs, SIMD, and Wasm

A target profile declares what a compiled artifact may assume about its execution environment: target triple, word width, byte order, vector shape, supplied and required capabilities, artifact format, and whether runtime `eval` is admitted. The profile is an input to typechecking—not merely a backend flag—so width bounds, conditional `prim` bodies, and capability obligations are verified against the target before any IR is emitted. Every build records its target profile in `build/<Entry>.manifest.json`.

# Target profiles and artifact shapes

| Field | Role |
| --- | --- |
| `triple` | Target triple governing `Usize`/`Isize` widths, `prim` `when` conditions, and package manifest conditions. |
| `word_size` | Pointer and machine-word width in bits (`32` or `64`). |
| `endian` | Byte order (`little` or `big`). |
| `vector_shape` | Admitted SIMD vector shape (for example `fixed(8)`). |
| `features`, `required` | Capabilities supplied by the target profile, and capabilities the deployment contract must satisfy. |
| `allow_eval` | Whether runtime `eval` of a typed quotation is permitted. |
| `artifact` | Output shape: `native`, `core-wasm`, `browser`, `wasi-component`, or `wasi-0.2-compat`. |
| `fuel`, `measure` | Specialization driving bounds. |

Every build records both its `target` profile and its `wasm` lowering summary in `build/<Entry>.manifest.json`. A host native build records:

```json
{
  "target": {
    "admission": "the deployment contract must satisfy the recorded feature assumptions",
    "conditions": [],
    "endian": "little",
    "features": ["u32.wrappingAdd"],
    "profile": "native64",
    "vector_shape": "fixed(8)",
    "word_size": 64
  },
  "wasm": {
    "adapters": [],
    "artifact": "native",
    "available": ["u32.wrappingAdd"],
    "canonical_abi": "none",
    "lowerings": ["tasks:os-thread"],
    "required": [],
    "selected": {"features": ["u32.wrappingAdd"], "profile": "native64"}
  }
}
```

Compiling a minimal program lets you inspect the manifest for any target profile:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main = 0
```

# Target triples and cross-compilation

Pass `--target <triple>` to `check`, `emit-ir`, `emit-object`, `build`, or `run` to select one of the five supported triples:

| Triple | `arch` | `os` | `libc` | Link pipeline |
| --- | --- | --- | --- | --- |
| `aarch64-apple-darwin` | `aarch64` | `darwin` | `libSystem` | `clang` driver |
| `x86_64-apple-darwin` | `x86_64` | `darwin` | `libSystem` | `clang` driver |
| `x86_64-unknown-linux-gnu` | `x86_64` | `linux` | `glibc` | `clang` driver |
| `aarch64-unknown-linux-gnu` | `aarch64` | `linux` | `glibc` | `clang` driver |
| `wasm32-unknown-unknown` | `wasm32` | `unknown` | `none` | `clang -c` then `wasm-ld` |

`check`, `emit-ir`, and `emit-object` support all five triples from any host. Type widths, `when` branch selection, and the emitted LLVM `target triple` and `target datalayout` are derived exclusively from `--target`.

Because the target profile governs typechecking, width-dependent errors fire before code generation. Under `--target wasm32-unknown-unknown`, `Usize` is 32 bits wide, so a 64-bit literal is rejected during `check`:

```text
$ cinch check --target wasm32-unknown-unknown Main.cinch
cinch: 3:7: integer literal 4294967296 is out of range for Usize (in Main.cinch)
```

## Linking cross-compiled binaries and Wasm modules

- **Foreign native targets:** `cinch build` requires a prebuilt `libcinchruntime.a` for the target architecture, passed via `--runtime-archive <path>`. The driver verifies the archive's validity and architecture before linking.
- **Freestanding WebAssembly (`wasm32-unknown-unknown`):** The runtime crate compiles for `wasm32-unknown-unknown` directly with Cargo (no C cross-toolchain required once the Rust `wasm32-unknown-unknown` target is installed), so `--runtime-archive` is not needed. Because freestanding Wasm has no `crt1.o` or `-lc`, the driver assembles the LLVM module with `clang -c` and links it against the runtime archive with `wasm-ld --no-entry --export-all`.

Building `wasm32-unknown-unknown` requires two tools on the host:

1. **`clang` 19 or newer** (overridable via `CINCH_CLANG`). Emitted IR uses `range(...)` return attributes (rejected by LLVM 18 and older) and `nocapture` on borrowed pointers (accepted by LLVM 19 and automatically upgraded by LLVM 21+).
2. **`wasm-ld`** (overridable via `CINCH_WASM_LD`). The driver resolves `wasm-ld` first from `CINCH_WASM_LD`, then via `clang -print-prog-name=wasm-ld`, and finally on `PATH`. On Nix systems where `wasm-ld` is not on the default `PATH`, locate it from `llvmPackages.bintools`:

```sh
CINCH_WASM_LD="$(nix-shell -p llvmPackages.bintools --run 'which wasm-ld')" \
  cinch build --target wasm32-unknown-unknown Main.cinch -o prog.wasm
```

## Running target artifacts

`cinch run` executes targets the host can run:

- **Host native target:** executes the linked binary directly.
- **`wasm32-unknown-unknown`:** invokes Node.js (overridable via `CINCH_NODE`) to instantiate the module, call the exported `main`, and return its exit code. Because a freestanding Wasm module has no command line, passing program arguments to `cinch run --target wasm32-unknown-unknown` is refused rather than silently dropped.
- **Other foreign native targets:** refused, because the host cannot execute the resulting binary.

# Platform proof obligations and trust boundary

A capability required by an artifact that the selected target profile does not supply is refused at artifact admission, before any output is emitted:

```text
cinch: missing required capability: <name>
```

Capability requirements originate from the profile's `required` list, selected `target rewrite` rules, and component adapters (for example `wasm.component-model` for WIT adapters on Wasm artifacts).

A `prim` body conditioned on a capability via `when Available <Cap>` must name a declared capability `signature`, not an arbitrary type or string:

```text
cinch: 6:12: unknown capability `U32`: a `when` condition names a `signature` of the target profile's capability evidence (platform proofs and SIMD specification section 3)
```

Every build manifest also records the exact boundary between passes whose invariants were mechanically checked and components that belong to the trusted computing base:

```json
{
  "checked": [
    "core structure and scoping",
    "usage of %1/%aff parameters",
    "memo and demand sharing",
    "decision-point weights",
    "contract identity"
  ],
  "trusted": ["LLVM code generation", "the runtime and its instruction model"]
}
```

# SIMD surface

`Data.Target` defines the capability-gated SIMD surface as ordinary declarations:

```cinch
data AVX2_256 where
  AVX2_256Tag :: AVX2_256

signature VecU32Target t w where
  positive-width :: Unit -> Nat

data CapAvailable t w where
  Available :: VecU32Target t w -> CapAvailable t w

simd-sum-wrap32 :: forall t w n region. (vectors :: VecU32Target t w) -> Ref region (Vec n U32) -> U32

detect-vec-u32 :: Unit -> Option (VecU32Target AVX2_256 EightLanes)

signature SumProofs where
  scalar-to-simd :: forall t w n region. (vectors :: VecU32Target t w)
                 -> (xs :: Ref region (Vec n U32))
                 -> Eq (scalar-sum-wrap32 xs) (simd-sum-wrap32 vectors xs)
```

Under the current backend, `simd-sum-wrap32` lowers to the equivalent scalar loop and `detect-vec-u32` returns `None`, so runtime `case` dispatch always takes the scalar fallback. The optimizer preserves the dispatch boundary rather than folding it away, keeping the scalar fallback and instruction-equivalence proof obligation (`SumProofs.scalar-to-simd`) explicit in the checked program and manifest.

<span class="support-tag support-tag--planned">Coming soon</span> **Vector instruction selection and runtime feature dispatch.** The capability evidence types and equivalence obligations are checked today; hardware vector instruction emission and CPUID/feature probing are planned.

# Library object artifacts

`cinch emit-object` compiles a library module (one without `main`) into a relocatable host or cross-target object file alongside its `.ll` IR and `.abi.json` manifest:

```sh
$ cinch emit-object Lib.cinch -o Lib.o
$ file Lib.o
Lib.o: Mach-O 64-bit object arm64
$ nm -g Lib.o | grep 'Lib\.double'
00000000001a04c0 T _Lib.double
00000000001a0518 T _Lib.double.app
00000000001a0768 T _Lib.double.object-closure
```

Each exported unit defines its qualified worker symbol, its curried apply stub (`.app`), and its closure descriptor (`.object-closure`).

- Passing a program with a `main` entry point to `emit-object` is refused:
  `cinch: object emission requires a library program without a process entry`
- Passing a non-empty `--object-roots <roots.json>` outside `emit-object` is refused:
  `cinch: explicit object roots require emit-object mode`

See [Chapter 18](18-artifacts-and-incremental.md) for the roots schema and ABI manifest fields.

# WIT interfaces and WebAssembly components

A `.wit` file defines a WebAssembly Component Model boundary. The driver discovers `*.wit` files in the directory of the entry file **only when the entry path includes a directory component**:

```sh
cinch check Main.cinch     # bare filename: no .wit discovery
cinch check ./Main.cinch   # discovers sibling *.wit files beside Main.cinch
```

Syntax errors in discovered `.wit` files report their own path and position (for example `cinch: ./broken.wit: 1:17: WIT: unexpected character '!'`).

## Canonical ABI lowering and `Raw` bindings

For each discovered WIT interface, the compiler computes the canonical-ABI lowering table, records it in the build manifest with `"canonical_abi": "wasi-0.3"`, and installs one qualified binding per WIT function under the `Raw` module:

```json
{
  "adapters": [
    {
      "core_params": ["i32", "i32", "i32", "i32"],
      "core_results": [],
      "export": false,
      "module": "Raw",
      "name": "decode",
      "package": "example:feed@0.1.0",
      "ret_area": {"align": 4, "cases": 2, "size": 16},
      "symbol": "example:feed@0.1.0/feeder#decode"
    },
    {
      "core_params": [],
      "core_results": ["i32"],
      "export": false,
      "module": "Raw",
      "name": "feed",
      "package": "example:feed@0.1.0",
      "symbol": "example:feed@0.1.0/feeder#feed"
    }
  ],
  "canonical_abi": "wasi-0.3"
}
```

In the example above, `list<u8>` flattens to `(i32, i32)` and a two-case `result` crosses through the return area (`ret_area`), giving `decode` four core parameters and no direct core result. Unsupported canonical shapes (such as more than 16 flattened parameters requiring parameter-area marshalling) are rejected with a specific gap diagnostic.

Emitting canonical-ABI adapter glue requires a Wasm target profile; emitting under a native host profile is refused:

```text
cinch: the canonical ABI glue for example:feed@0.1.0/feeder#decode is defined for the wasm32 target (WebAssembly and WASI specification section 4); emit under a Wasm artifact profile
```

With `--target wasm32-unknown-unknown`, the adapter links into the resulting freestanding core `.wasm` module.

## WASI streams and futures

WASI streams and futures are modeled as external handles—not productive codata `Stream` values. When a WIT adapter uses streams or futures, the generated vocabulary (`Prelude.Generated`) declares `WasiStream a`, `WasiFuture a`, `StreamError`, and `BoundaryError`, and installs three foreign operations with effect row `{Foreign Component, Diverge}`:

| Binding | Type | Effect row |
| --- | --- | --- |
| `wasiStreamRead` | `WasiStream a -> Result StreamError (Option a)` | `{Foreign Component, Diverge}` |
| `wasiStreamWrite` | `WasiStream a -> a -> Result StreamError Unit` | `{Foreign Component, Diverge}` |
| `wasiFutureWait` | `WasiFuture a -> a` | `{Foreign Component, Diverge}` |

End-of-stream, scheduling, and decode errors are therefore explicit values the caller handles rather than hidden runtime faults.

<span class="support-tag support-tag--planned">Coming soon</span> **Component-model wrapping and execution.** The WIT parser, canonical-ABI planner, and core-module adapter emission work under `--target wasm32-unknown-unknown`, producing a core `.wasm` module linked by `wasm-ld`. Component packaging (`.wasm` component binary emission and component instantiation) is planned.

# Target-IR backend families

Inline `prim` declarations dispatch by target-IR family name:

- `llvm` and admitted `asm` lower on native targets.
- `wasm` is available under Wasm target profiles.
- `clr` and `jvm` are reserved family names and are rejected if selected during lowering.

See [Chapter 15](15-ffi-layout-and-attributes.md) for `prim` body syntax, `when` predicates, and `unsafe` trust boundaries.
