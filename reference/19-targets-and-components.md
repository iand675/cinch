# Target profiles, platform proofs, SIMD, and Wasm

A target profile states what an artifact may assume: the triple, the word width, the endianness, the vector shape, the capability set, and whether runtime code generation is admitted. The profile is an input to checking, not a hint to the backend, so a fact that depends on the target is checked against the target. Every artifact records the profile it was built under.

# Profiles and artifact shapes

| Field | Meaning |
| --- | --- |
| `triple` | The target triple. `Usize` and `Isize` widths and `when arch <name>` predicates key on it. |
| `word_size` | Pointer and word width in bits. |
| `endian` | `little` or `big`. |
| `vector_shape` | The admitted SIMD shape. |
| `features`, `required` | Capabilities the profile supplies, and capabilities the deployment contract must supply. |
| `allow_eval` | Whether `eval` of a typed quote is admitted. |
| `artifact` | The artifact shape: `native`, `core-wasm`, `browser`, `wasi-component`, `wasi-0.2-compat`. |
| `fuel`, `measure` | Driving bounds for specialization. |

The artifact shape is visible in every build. A native build of a trivial program records:

```json
{
  "target": {
    "admission": "the deployment contract must satisfy the recorded feature assumptions",
    "conditions": [],
    "endian": "little",
    "features": [ "u32.wrappingAdd" ],
    "profile": "native64",
    "vector_shape": "fixed(8)",
    "word_size": 64
  },
  "wasm": {
    "adapters": [],
    "artifact": "native",
    "available": [ "u32.wrappingAdd" ],
    "canonical_abi": "none",
    "lowerings": [ "tasks:os-thread" ],
    "required": [],
    "selected": { "features": [ "u32.wrappingAdd" ], "profile": "native64" }
  }
}
```

`"artifact": "native"` and `"canonical_abi": "none"` are the honest summary of a host build. A Wasm build changes both.

The program those records come from is the smallest complete program, and building it is how you inspect your own host's profile:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main = 0
```

# The CLI builds for the host only

There is no target flag and no environment variable that selects a profile. `cinch check`, `emit-ir`, `emit-object`, `build`, and `run` all use the native target profile. Two behaviors follow, and both surprise people:

```
$ cinch build --target wasm32 Main.cinch
cinch: --target: No such file or directory (os error 2)
```

A word before the entry file is taken as the entry file. A word after it is not a flag, so it is absorbed into the program argument list and silently ignored:

```
$ cinch build Main.cinch -o prog --target wasm32
$ file prog
prog: Mach-O 64-bit executable arm64
```

The build succeeds and the output is a host executable. Do not read a zero exit status as evidence of cross-compilation. Cross-target artifacts come from the Rust API: `cinch::codegen::emit_for(&TargetProfile::wasm32(ArtifactProfile::CoreWasm), &program)` emits Wasm IR, and the `wasm` suite asserts that the character of that IR includes `target triple = "wasm32-unknown-unknown"`. `TargetProfile::wasm32` also narrows `Usize` to 32 bits, so a literal that fits the host is refused for the Wasm profile with `out of range for Usize`.

# Platform proof obligations

A capability the artifact requires and the profile cannot supply is refused at artifact admission:

```
cinch: missing required capability: <name>
```

The refusal happens when the artifact is produced, not when the capability is first used, so a build cannot silently emit instructions whose precondition the deployment does not meet. Capability requirements come from a profile's `required` list, from a selected `target rewrite`, and from component adapters (`wasm.component-model` is added for a WIT adapter on a Wasm artifact).

A `prim` body can be conditioned on a capability, and the condition names a `signature`, not an arbitrary string:

```
cinch: 6:12: unknown capability `U32`: a `when` condition names a `signature` of the target profile's capability evidence (platform proofs and SIMD specification section 3)
```

The trust boundary is recorded per artifact. The manifest separates the phases whose results were checked from the phases that are trusted:

```json
{
  "checked": [
    "core structure and scoping",
    "usage of %1/%aff parameters",
    "memo and demand sharing",
    "decision-point weights",
    "contract identity"
  ],
  "trusted": [ "LLVM code generation", "the runtime and its instruction model" ]
}
```

# The SIMD surface

`Data.Target` is the current SIMD surface. The vector shape, the capability, its evidence, and the dispatched intrinsic are ordinary declarations:

```text
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

These are fragments of a module you have to import; they are not a program.

Two honesty points about the current lowering. `simd-sum-wrap32` calls the equivalent scalar loop, so the intrinsic computes the right sum with one lane at a time; the SIMD instruction selection is not implemented. `detect-vec-u32` returns `None` unconditionally, so `case` dispatch conservatively takes the scalar path and no runtime feature detection happens. Both facts are deliberate and recorded: the manifest names the selected lowering (`"lowerings": ["tasks:os-thread"]` on a host build), the optimizer is specified never to fold the dispatch boundary, and the platform specification describes the missing pieces as proof obligations ([platform proofs and SIMD specification](../specifications/platform-proofs-and-simd-specification.md) sections 3 to 8). A scalar fallback is not a shortcut that hides an absent proof; it is the case the proof must cover.

<span class="support-tag support-tag--planned">Coming soon</span> **vector code generation.** The scalar/SIMD pair is stated and its instruction law is a named obligation, but no backend emits vector instructions for it, and no dispatch selects a vector path.

# Object artifacts

`cinch emit-object` compiles a library and writes one object file. It needs a library program, meaning a program with no entry point:

```
$ cinch emit-object Lib.cinch -o Lib.o
$ file Lib.o
Lib.o: Mach-O 64-bit object arm64
$ nm -g Lib.o | grep 'Lib\.double'
00000000001a04c0 T _Lib.double
00000000001a0518 T _Lib.double.app
00000000001a0768 T _Lib.double.object-closure
```

Every unit of the program is exported under its qualified name, with its apply stub and its closure, which is why the symbols repeat. `file` and `nm` are the verification commands: the object is a real host object, and the units named in the interface are the symbols it defines.

A program with a process entry is refused:

```
cinch: object emission requires a library program without a process entry
```

`--object-roots <roots.json>` supplies the concrete library variants a consumer demands, and the flag is refused in any other mode with `explicit object roots require emit-object mode`.

# WIT input and component artifacts

A `.wit` file is the input for a component boundary. The toolchain discovers `*.wit` files **in the directory of the entry file**, and only when the entry is given with a directory component:

```
$ cinch check Main.cinch     # no .wit discovery: the parent of a bare name is empty
$ cinch check ./Main.cinch   # discovers codec.wit beside the entry
```

An invalid file is then reported with its own path and position, for example `cinch: ./broken.wit: 1:17: WIT: unexpected character '!'`. Pass `./` (or an absolute path) if you want the adapters installed.

The adapter generates the canonical table, records it in the manifest, and installs one raw binding per WIT function under the `Raw` module. For an interface with one record method and two stream methods:

```json
{
  "adapters": [
    { "core_params": [ "i32", "i32", "i32", "i32" ], "core_results": [],
      "export": false, "module": "Raw", "name": "decode",
      "package": "example:feed@0.1.0",
      "ret_area": { "align": 4, "cases": 2, "size": 16 },
      "symbol": "example:feed@0.1.0/feeder#decode" },
    { "core_params": [], "core_results": [ "i32" ], "export": false,
      "module": "Raw", "name": "feed",
      "package": "example:feed@0.1.0",
      "symbol": "example:feed@0.1.0/feeder#feed" }
  ],
  "canonical_abi": "wasi-0.3"
}
```

The manifest also sets `"canonical_abi": "wasi-0.3"` once an adapter is present. A `list<u8>` parameter flattens to `(i32, i32)`, and a two-case `result` crosses through the return area, which is why `decode` takes four core parameters and returns nothing.

Emitting that glue requires a Wasm artifact profile. The host profile refuses, and the refusal names the target the glue is written for:

```
cinch: the canonical ABI glue for example:feed@0.1.0/feeder#decode is defined for the wasm32 target (WebAssembly and WASI specification section 4); emit under a Wasm artifact profile
```

Since the CLI cannot select a Wasm profile, a program with a `.wit` file beside it checks and does not build. Parsing WIT and building an adapter is also not a promise of component-model interoperability: an adapter records a lowering for the types it admits, and a shape it cannot marshal is reported as a named gap (`more than 16 flattened parameters (the parameter area crossing is not marshalled yet)` is the current example).

WASI streams map to external handles, not to the language's codata `Stream`. The generated vocabulary declares `WasiStream a`, `WasiFuture a`, and `StreamError`, and the three observations are foreign imports with divergence:

| Binding | Type | Row |
| --- | --- | --- |
| `wasiStreamRead` | `WasiStream a -> Result StreamError (Option a)` | `Foreign Component, Diverge` |
| `wasiStreamWrite` | `WasiStream a -> a -> Result StreamError Unit` | `Foreign Component, Diverge` |
| `wasiFutureWait` | `WasiFuture a -> a` | `Foreign Component, Diverge` |

They carry no productive `Stream a` guarantee: end of stream, scheduling, and decode failure are observable outcomes you handle.

<span class="support-tag support-tag--planned">Coming soon</span> **component emission and execution.** There is no component artifact on disk, no Wasm linker step, and no CLI path that produces either. The WIT parser, the canonical-ABI planner, and the adapter vocabulary exist and are exercised; the packaging and instantiation workflow is specified in the [WebAssembly and WASI specification](../specifications/webassembly-and-wasi-specification.md) sections 2 and 8 to 10, and its illustrative API is design material.

# Additional backend families

`prim` bodies are per family. `llvm` and admitted `asm` lower today; the `clr` and `jvm` families are reserved and refused by name when used, and the `wasm` family is available to a Wasm profile. A family name in syntax is not backend support. See [inline primitives](15-ffi-layout-and-attributes.md).

Implementation: `core.rs` (`TargetProfile`, `ArtifactProfile`), `codegen.rs` (`emit_for`, reserved symbols), `driver.rs` (manifest `target`/`wasm` sections, artifact admission, `.wit` discovery), `interface.rs` (canonical tables, vocabulary), `arrays.rs` and `arrays_codegen.rs` (the `Vec` surface SIMD owns); suites `wasm.rs`, `optimizer.rs`, `prim.rs`, `arrays.rs`. Prelude: `Data.Target`.
