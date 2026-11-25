# Data-Parallel Arrays, Typed Array IR, and Metal GPU Execution

Status: implemented across runtime (`seed/runtime/src/par_arrays.rs`, `seed/runtime/src/metal.rs`), Rust seed compiler (`seed/src/arrays.rs`, `seed/src/array_ir.rs`, `seed/src/arrays_codegen.rs`, `seed/src/interface.rs`), standard library modules (`Data.Array.Compute`, `Data.Device`), and self-hosted compiler (`compiler/Bootstrap.Builtin.cinch`, `compiler/Bootstrap.RuntimeIr.cinch`).

## 1. Shape-Indexed Views over Existing Buffers

`Tensor a` remains the dynamic entry point (`Data.Tensor`). A successful shape check (`view1-of`, `view2-of`, `view2-from-tensor`, `view3-of`, `view3-from-tensor`) produces a dimension-indexed view (`View1 n a`, `View2 m n a`, `View3 f h w a`) over an existing immutable `Array a` buffer without copying:

- `dot-view1-f32 :: forall (n :: Nat). View1 n F32 -> View1 n F32 -> FloatMode -> F32` requires two vectors with the same static length witness `n`.
- `row-reduce-view2-f32 :: forall (m :: Nat) (n :: Nat). View2 m n F32 -> FloatMode -> Option (View1 m F32)` takes `m × n` to `m`.
- `filter-view1-i32-gt :: forall (n :: Nat). View1 n I32 -> I32 -> Option (Sigma (k :: Nat). (View1 k I32, k <= n))` returns the compacted view with its length `k` and evidence `k <= n`.
- `view1-get-fin :: forall (n :: Nat) a. View1 n a -> Fin n -> Option a` indexes using Cinch's `Fin n` witness.
- `view1-slice` and `view2-transpose` produce strided views with zero data copy.
- `view2-reshape-shared` shares backing storage only when `view2-is-contiguous` holds; non-contiguous views require explicit `view2-pack`.

## 2. Parallel Array Primitives & Contracts

| Primitive | Shape | Ordering / Empty Input | Work / Temp Storage |
| --- | --- | --- | --- |
| `array-tabulate-*` | `n -> [n]` | Independent per index; `n=0 -> []` | `O(n)` work, `0` list/temp bytes (direct packed allocation) |
| `array-map-*` | `[n] -> [n]` | Elementwise independent; `[] -> []` | `O(n)` work, `0` temp bytes |
| `array-zip-with-*` | `[n] x [n] -> Option [n]` | Equal length checked (`None` on mismatch) | `O(n)` work, `0` temp bytes |
| `array-foldl-*` | `acc -> [n] -> acc` | Strict sequential left-to-right; `[] -> init` | `O(n)` work, `0` temp bytes (rejected on required GPU target) |
| `array-reduce-*` | `[n] -> a` | Explicit `IntReducer` / `FloatMode`; `[] -> identity` | `O(n)` work, `O(threads)` temp bytes |
| `array-row-reduce-*` | `[m x n] -> Option [m]` | `m=0 -> []`; `n=0 -> replicate m identity` | `O(m*n)` work, dynamic outer/inner schedule |
| `array-scan-*` | `[n] -> [n]` | Inclusive or exclusive prefix; `[] -> []` | `O(n)` work, `O(threads)` / `O(n/256)` block sums |
| `array-filter-*` | `[n] -> [k] (k <= n)` | Stable order preserved; `[] -> []` | `O(n)` work, `O(n)` flags/counts |
| `array-gather-*` | `[n] x [m] -> [m]` | Explicit `EdgePolicy` (`Clamp`/`Zero`/`Mirror`/`Wrap`) | `O(m)` work, `0` temp bytes |
| `array-scatter-unique-*` | `[n] x [m] x [m] -> [n]` | Unique destinations; OOB ignored; in-place if `RC == 1` | `O(m)` work, `0` temp bytes if unique |
| `array-scatter-combine-*` | `[n] x [m] x [m] -> [n]` | Explicit associative combine for collisions; in-place if `RC == 1` | `O(m)` work, thread-privatized when parallel |
| `array-histogram-*` | `[n] -> [bins]` | Thread/TG-privatized bins; collision-safe on hot bins | `O(n + threads*bins)` work |
| `array-stencil2d-*` | `[f x h x w] -> Option [f x h x w]` | Explicit `EdgePolicy`; `steps` ping-pong reuse | `O(steps*f*h*w)` work, at most 2 ping-pong buffers |
| `array-segmented-reduce-f32` | `vals x offsets[r+1] -> [r]` | Empty rows (`off[i]==off[i+1]`) yield identity | `O(nnz + r)` work, `0` temp bytes |

## 3. Explicit Numerical Semantics

- Ordered left fold (`array-foldl-*`) is strictly sequential and never reassociated.
- Integer reductions use explicit wrapping/bitwise/min/max contracts (`IntReducer`).
- Floating-point reductions require an explicit `FloatMode`:
  - `FloatExactTree`: specified left-complete power-of-2 binary tree over 1024-element blocks. Bit-for-bit identical across scalar reference evaluator, multicore CPU (any thread count), and Apple Metal GPU.
  - `FloatRelaxed`: explicit reassociative SIMD / `simd_sum` mode for peak throughput.

## 4. Kernels, Callback Forwarding, and `.cinchif` Metadata

- Builtin evidence class `Kernel (a -> b)` allows reusable parallel library functions to forward callback parameters safely across module boundaries (`using (k :: Kernel (a -> b)) => ...`), solving Section 12 of `design/parallelism-as-a-value.md`.
- Compiled `.cinchif` interfaces record optional `"kernel": {"cpu_parallel": bool, "gpu_metal": bool, "blocker": ...}` on exported array/kernel functions.
- `--array-target=metal-required` enforces GPU eligibility at compile time and names the specific effect, representation, capture, or recursion preventing device execution.

## 5. Typed Array IR Transformations & Compiler Report

Pass `seed/src/array_ir.rs` runs after specialization/simplification and before RC insertion and LLVM codegen:

1. **Producer-consumer fusion**: fuses `zip-mul -> reduce` (`DotF32`/`DotI32`), `map -> reduce` (`FusedMapReduceF32`), `gather + zip-mul -> segmented-reduce` (`SpmvCsrF32`), and consecutive `stencil2d` calls into one ping-pong loop — including through imported helper functions.
2. **Nested parallelism schedule selection**: selects `outer-parallel` / `many-small-frames` vs `inner-parallel` / `few-large-frames` statically or via runtime dispatch.
3. **Storage planning**: plans 2-buffer ping-pong reuse, in-place scatter updates, and single-command-buffer resident GPU pipelines (1 upload, 0 intermediate copies, 1 download).

Inspect via `--array-report`, `--array-report-json`, or `CINCH_ARRAY_REPORT=1`.

## 6. Apple Metal GPU Backend & External C/Python Boundary

`seed/runtime/src/metal.rs` loads `Metal.framework` and `Foundation.framework` dynamically via `crate::sys`, providing:
- Resident `MTLBuffer` management with in-flight command-buffer retention until completion.
- Defined `DeviceError` reporting (`DeviceNoHardware`, `DeviceOutOfMemory`, `DeviceCompileFailed`, `DeviceExecutionFailed`, `DeviceInvalidShape`).
- Single-command-buffer `stencil -> histogram` pipeline with adaptive `many-small-frames` (1 threadgroup/frame, `threadgroup atomic_uint local_bins[256]`) vs `few-large-frames` (tiled grid-stride privatization) schedules.
- C ABI exports consumable directly from C and Python `ctypes` (`examples/gpu-pipeline/`).
