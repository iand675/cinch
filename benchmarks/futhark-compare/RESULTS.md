# Cinch vs. Futhark 0.25.37 — Independent Benchmark Comparison

## Hardware & Pinned Toolchain

- **Machine**: Apple M5 Max (18 CPU cores, 128 GB unified RAM)
- **OS**: macOS 27.2.1 (arm64)
- **Futhark**: Futhark 0.25.37. / Compiled with GHC 9.10.3.
- **Host C Compiler**: Apple clang version 21.0.0 (clang-2100.4.1.1)
- **Cinch Revision**: `xklqorkmlxouknyovqspnnvvtvxxtsmq 2b245027bd4a2510f0effcabbf84a3d2f56d5217`
- **Timing Methodology**: Warm resident time = median of 15 timed iterations after 3 warmups (with explicit GPU/context synchronization per iteration). Transfer-inclusive time = median of 7 full upload → compute → sync → download cycles. Each `(backend, workload, size)` runs in a fresh isolated OS process so `ru_maxrss` reports true unpolluted peak host RSS.

## Cinch Typed Array IR Compiler Report (`workloads.cinch`)

```text
{"target":"cpu","nodes":[{"unit":"Data.Array.Compute.spmv-csr-composed-f32","op":"arraySpmvCsrF32","elem_type":"F32","shape":"CSR(rows, nnz)","contract":"f32-dynamic-contract","buffer_use":"fused-gather-mul-segreduce"},{"unit":"Data.Array.Compute.spmv-csr-composed-f32","op":"arraySpmvCsrF32","elem_type":"F32","shape":"CSR(rows, nnz)","contract":"f32-dynamic-contract","buffer_use":"fused-gather-mul-segreduce"},{"unit":"Workloads.dot-f32-composed","op":"arrayDotF32","elem_type":"F32","shape":"1D(n)","contract":"f32-dynamic-contract","buffer_use":"fused-zero-intermediate"},{"unit":"Workloads.dot-f32-composed","op":"arrayDotF32","elem_type":"F32","shape":"1D(n)","contract":"f32-dynamic-contract","buffer_use":"fused-zero-intermediate"},{"unit":"Workloads.spmv-csr-f32","op":"arraySpmvCsrF32","elem_type":"F32","shape":"CSR(rows, nnz)","contract":"f32-dynamic-contract","buffer_use":"fused-gather-mul-segreduce"},{"unit":"Workloads.spmv-csr-f32","op":"arraySpmvCsrF32","elem_type":"F32","shape":"CSR(rows, nnz)","contract":"f32-dynamic-contract","buffer_use":"fused-gather-mul-segreduce"},{"unit":"Main.main","op":"arrayDotF32","elem_type":"F32","shape":"1D(n)","contract":"f32-add-exact-tree","buffer_use":"fused-zero-intermediate"},{"unit":"Main.main","op":"arrayDotF32","elem_type":"F32","shape":"1D(n)","contract":"f32-dynamic-contract","buffer_use":"fused-zero-intermediate"},{"unit":"Main.main","op":"arraySpmvCsrF32","elem_type":"F32","shape":"CSR(rows, nnz)","contract":"f32-add-exact-tree","buffer_use":"fused-gather-mul-segreduce"},{"unit":"Main.main","op":"arraySpmvCsrF32","elem_type":"F32","shape":"CSR(rows, nnz)","contract":"f32-dynamic-contract","buffer_use":"fused-gather-mul-segreduce"}],"fusions":[{"unit":"Data.Array.Compute.spmv-csr-composed-f32","producer":"Data.Array.Compute.array-gather-f32 + Data.Array.Compute.array-zip-binop-f32","consumer":"Data.Array.Compute.array-segmented-reduce-f32","fused_op":"arraySpmvCsrF32","eliminated_allocation":"2 intermediate Array F32 buffers (8 * nnz bytes)","numerical_mode":"f32-dynamic-contract"},{"unit":"Workloads.dot-f32-composed","producer":"Data.Array.Compute.array-zip-binop-f32","consumer":"Data.Array.Compute.array-reduce-f32","fused_op":"arrayDotF32","eliminated_allocation":"intermediate product Array F32 (4 * n bytes)","numerical_mode":"f32-dynamic-contract"},{"unit":"Workloads.spmv-csr-f32","producer":"Data.Array.Compute.array-gather-f32 + Data.Array.Compute.array-zip-binop-f32","consumer":"Data.Array.Compute.array-segmented-reduce-f32","fused_op":"arraySpmvCsrF32","eliminated_allocation":"2 intermediate Array F32 buffers (8 * nnz bytes)","numerical_mode":"f32-dynamic-contract"},{"unit":"Main.main","producer":"Data.Array.Compute.array-zip-binop-f32","consumer":"Data.Array.Compute.array-reduce-f32","fused_op":"arrayDotF32","eliminated_allocation":"intermediate product Array F32 (4 * n bytes)","numerical_mode":"f32-add-exact-tree"},{"unit":"Main.main","producer":"Data.Array.Compute.array-gather-f32 + Data.Array.Compute.array-zip-binop-f32","consumer":"Data.Array.Compute.array-segmented-reduce-f32","fused_op":"arraySpmvCsrF32","eliminated_allocation":"2 intermediate Array F32 buffers (8 * nnz bytes)","numerical_mode":"f32-add-exact-tree"}],"schedules":[{"unit":"Data.Array.Compute.spmv-csr-composed-f32","op":"arraySpmvCsrF32","policy":"adaptive-csr(scalar-row if avg_nnz<16 else simd-group-row; empty-row=0.0)","numerical_mode":"f32-dynamic-contract"},{"unit":"Data.Array.Compute.spmv-csr-composed-f32","op":"arraySpmvCsrF32","policy":"adaptive-csr(scalar-row if avg_nnz<16 else simd-group-row; empty-row=0.0)","numerical_mode":"f32-dynamic-contract"},{"unit":"Workloads.spmv-csr-f32","op":"arraySpmvCsrF32","policy":"adaptive-csr(scalar-row if avg_nnz<16 else simd-group-row; empty-row=0.0)","numerical_mode":"f32-dynamic-contract"},{"unit":"Workloads.spmv-csr-f32","op":"arraySpmvCsrF32","policy":"adaptive-csr(scalar-row if avg_nnz<16 else simd-group-row; empty-row=0.0)","numerical_mode":"f32-dynamic-contract"},{"unit":"Main.main","op":"arraySpmvCsrF32","policy":"adaptive-csr(scalar-row if avg_nnz<16 else simd-group-row; empty-row=0.0)","numerical_mode":"f32-add-exact-tree"},{"unit":"Main.main","op":"arraySpmvCsrF32","policy":"adaptive-csr(scalar-row if avg_nnz<16 else simd-group-row; empty-row=0.0)","numerical_mode":"f32-dynamic-contract"}],"storage":[],"gpu_diagnostics":[],"msl_kernel_count":0}
```

## Summary Tables

### Dot F32 (100k)

- **Verification**: MATCH (val=489.0781, max_rel_err=0.00e+00, FloatExactTree CPU==Metal: BIT-EXACT)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | FloatRelaxed (associative f32 sum) | 0.106 | 0.131 | 3.58 | — | `489.0781` |
| `futhark-mc` | FloatRelaxed (associative f32 sum) | 0.069 | 0.060 | 4.08 | — | `489.0781` |
| `futhark-ocl` | FloatRelaxed (associative f32 sum) | 0.223 | 0.293 | 31.36 | — | `489.0781` |
| `cinch-cpu` | FloatRelaxed (associative f32 sum) | 0.110 | 0.126 | 4.52 | — | `489.0781` |
| `cinch-cpu-exact` | FloatExactTree (deterministic binary tree) | 0.155 | 0.161 | 4.52 | — | `489.0781` |
| `cinch-metal` | FloatRelaxed (associative f32 sum) | 0.191 | 0.573 | 20.19 | 0.76 | `489.0781` |
| `cinch-metal-exact` | FloatExactTree (deterministic binary tree) | 0.663 | 0.781 | 20.23 | 0.76 | `489.0781` |

### Dot F32 (1M)

- **Verification**: MATCH (val=3257.9375, max_rel_err=0.00e+00, FloatExactTree CPU==Metal: BIT-EXACT)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | FloatRelaxed (associative f32 sum) | 0.842 | 0.927 | 17.36 | — | `3257.9375` |
| `futhark-mc` | FloatRelaxed (associative f32 sum) | 0.188 | 0.379 | 17.89 | — | `3257.9375` |
| `futhark-ocl` | FloatRelaxed (associative f32 sum) | 0.246 | 0.433 | 40.05 | — | `3257.9375` |
| `cinch-cpu` | FloatRelaxed (associative f32 sum) | 0.092 | 0.615 | 18.30 | — | `3257.9375` |
| `cinch-cpu-exact` | FloatExactTree (deterministic binary tree) | 0.155 | 0.956 | 18.33 | — | `3257.9375` |
| `cinch-metal` | FloatRelaxed (associative f32 sum) | 0.247 | 1.096 | 56.12 | 7.63 | `3257.9375` |
| `cinch-metal-exact` | FloatExactTree (deterministic binary tree) | 0.709 | 1.397 | 40.88 | 7.63 | `3257.9375` |

### Dot F32 (10M)

- **Verification**: MATCH (val=39210.4062, max_rel_err=0.00e+00, FloatExactTree CPU==Metal: BIT-EXACT)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | FloatRelaxed (associative f32 sum) | 5.151 | 5.990 | 154.67 | — | `39210.4062` |
| `futhark-mc` | FloatRelaxed (associative f32 sum) | 0.477 | 1.798 | 155.19 | — | `39210.4062` |
| `futhark-ocl` | FloatRelaxed (associative f32 sum) | 0.568 | 2.426 | 109.08 | — | `39210.4062` |
| `cinch-cpu` | FloatRelaxed (associative f32 sum) | 0.384 | 1.672 | 155.62 | — | `39210.4062` |
| `cinch-cpu-exact` | FloatExactTree (deterministic binary tree) | 1.050 | 2.443 | 155.69 | — | `39210.4062` |
| `cinch-metal` | FloatRelaxed (associative f32 sum) | 0.434 | 7.098 | 246.81 | 76.30 | `39210.4062` |
| `cinch-metal-exact` | FloatExactTree (deterministic binary tree) | 0.879 | 6.976 | 246.86 | 76.33 | `39210.4062` |

### Inclusive Scan I32 (100k)

- **Verification**: BIT-EXACT (ee56c044e16a, len=100000)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | I32 wrapping-add inclusive prefix scan | 0.045 | 0.064 | 3.58 | — | `ee56c044e1` |
| `futhark-mc` | I32 wrapping-add inclusive prefix scan | 0.034 | 0.059 | 4.67 | — | `ee56c044e1` |
| `futhark-ocl` | I32 wrapping-add inclusive prefix scan | 0.249 | 0.306 | 29.97 | — | `ee56c044e1` |
| `cinch-cpu` | I32 wrapping-add inclusive prefix scan | 0.229 | 0.238 | 4.52 | — | `ee56c044e1` |
| `cinch-metal` | I32 wrapping-add inclusive prefix scan | 0.468 | 0.512 | 19.94 | 0.77 | `ee56c044e1` |

### Inclusive Scan I32 (1M)

- **Verification**: BIT-EXACT (b02ff1534b64, len=1000000)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | I32 wrapping-add inclusive prefix scan | 0.451 | 0.584 | 17.36 | — | `b02ff1534b` |
| `futhark-mc` | I32 wrapping-add inclusive prefix scan | 0.125 | 0.376 | 19.11 | — | `b02ff1534b` |
| `futhark-ocl` | I32 wrapping-add inclusive prefix scan | 0.481 | 0.597 | 40.53 | — | `b02ff1534b` |
| `cinch-cpu` | I32 wrapping-add inclusive prefix scan | 0.261 | 0.897 | 18.31 | — | `b02ff1534b` |
| `cinch-metal` | I32 wrapping-add inclusive prefix scan | 0.681 | 1.202 | 37.11 | 7.66 | `b02ff1534b` |

### Inclusive Scan I32 (5M)

- **Verification**: BIT-EXACT (274ac995124e, len=5000000)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | I32 wrapping-add inclusive prefix scan | 1.517 | 2.051 | 78.36 | — | `274ac99512` |
| `futhark-mc` | I32 wrapping-add inclusive prefix scan | 0.240 | 1.156 | 80.12 | — | `274ac99512` |
| `futhark-ocl` | I32 wrapping-add inclusive prefix scan | 0.830 | 1.484 | 72.95 | — | `274ac99512` |
| `cinch-cpu` | I32 wrapping-add inclusive prefix scan | 0.399 | 1.135 | 79.31 | — | `274ac99512` |
| `cinch-metal` | I32 wrapping-add inclusive prefix scan | 1.384 | 4.030 | 113.39 | 38.30 | `274ac99512` |

### Stable Compaction I32 (100k)

- **Verification**: BIT-EXACT (9794c0adcc3c, len=37486)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | I32 stable compaction (x > 250) | 0.362 | 0.369 | 3.11 | — | `9794c0adcc` |
| `futhark-mc` | I32 stable compaction (x > 250) | 0.118 | 0.118 | 5.95 | — | `9794c0adcc` |
| `futhark-ocl` | I32 stable compaction (x > 250) | 0.313 | 0.506 | 31.47 | — | `9794c0adcc` |
| `cinch-cpu` | I32 stable compaction (x > 250) | 0.168 | 0.171 | 4.00 | — | `9794c0adcc` |
| `cinch-metal` | I32 stable compaction (x > 250) | 0.889 | 0.932 | 19.55 | 1.29 | `9794c0adcc` |

### Stable Compaction I32 (1M)

- **Verification**: BIT-EXACT (6639d9b383d7, len=375080)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | I32 stable compaction (x > 250) | 2.899 | 2.783 | 14.97 | — | `6639d9b383` |
| `futhark-mc` | I32 stable compaction (x > 250) | 0.326 | 0.395 | 18.52 | — | `6639d9b383` |
| `futhark-ocl` | I32 stable compaction (x > 250) | 0.739 | 0.676 | 38.12 | — | `6639d9b383` |
| `cinch-cpu` | I32 stable compaction (x > 250) | 0.389 | 0.940 | 14.97 | — | `6639d9b383` |
| `cinch-metal` | I32 stable compaction (x > 250) | 1.230 | 1.479 | 32.42 | 12.87 | `6639d9b383` |

### Stable Compaction I32 (5M)

- **Verification**: BIT-EXACT (248fddf67b21, len=1875670)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | I32 stable compaction (x > 250) | 11.157 | 11.558 | 66.44 | — | `248fddf67b` |
| `futhark-mc` | I32 stable compaction (x > 250) | 0.993 | 1.476 | 69.95 | — | `248fddf67b` |
| `futhark-ocl` | I32 stable compaction (x > 250) | 1.050 | 1.385 | 59.06 | — | `248fddf67b` |
| `cinch-cpu` | I32 stable compaction (x > 250) | 1.513 | 1.546 | 55.47 | — | `248fddf67b` |
| `cinch-metal` | I32 stable compaction (x > 250) | 2.931 | 5.515 | 89.59 | 64.38 | `248fddf67b` |

### Histogram U8 Uniform (256k)

- **Verification**: BIT-EXACT (2b62e045fe5e, len=256)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | U32 256-bin histogram (uniform) | 0.108 | 0.102 | 2.53 | — | `2b62e045fe` |
| `futhark-mc` | U32 256-bin histogram (uniform) | 0.085 | 0.108 | 3.09 | — | `2b62e045fe` |
| `futhark-ocl` | U32 256-bin histogram (uniform) | 0.322 | 0.347 | 30.95 | — | `2b62e045fe` |
| `cinch-cpu` | U32 256-bin histogram (uniform) | 0.089 | 0.087 | 3.50 | — | `2b62e045fe` |
| `cinch-metal` | U32 256-bin histogram (uniform) | 0.681 | 0.304 | 18.69 | 0.25 | `2b62e045fe` |

### Histogram U8 Uniform (4M)

- **Verification**: BIT-EXACT (ba51d34c6b6a, len=256)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | U32 256-bin histogram (uniform) | 1.203 | 1.159 | 10.05 | — | `ba51d34c6b` |
| `futhark-mc` | U32 256-bin histogram (uniform) | 0.183 | 0.249 | 10.59 | — | `ba51d34c6b` |
| `futhark-ocl` | U32 256-bin histogram (uniform) | 0.490 | 0.609 | 36.81 | — | `ba51d34c6b` |
| `cinch-cpu` | U32 256-bin histogram (uniform) | 0.184 | 0.509 | 11.06 | — | `ba51d34c6b` |
| `cinch-metal` | U32 256-bin histogram (uniform) | 0.322 | 0.669 | 30.03 | 4.00 | `ba51d34c6b` |

### Histogram U8 Uniform (16M)

- **Verification**: BIT-EXACT (5e9d2f0e6927, len=256)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | U32 256-bin histogram (uniform) | 3.382 | 3.297 | 34.03 | — | `5e9d2f0e69` |
| `futhark-mc` | U32 256-bin histogram (uniform) | 0.331 | 0.596 | 34.62 | — | `5e9d2f0e69` |
| `futhark-ocl` | U32 256-bin histogram (uniform) | 0.830 | 1.013 | 49.36 | — | `5e9d2f0e69` |
| `cinch-cpu` | U32 256-bin histogram (uniform) | 0.625 | 0.891 | 35.06 | — | `5e9d2f0e69` |
| `cinch-metal` | U32 256-bin histogram (uniform) | 0.899 | 1.808 | 65.95 | 16.00 | `5e9d2f0e69` |

### Histogram U8 80%-Hot-Bin (256k)

- **Verification**: BIT-EXACT (7d9c0e44cb46, len=256)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | U32 256-bin histogram (80% hot-bin contention) | 0.709 | 0.695 | 2.53 | — | `7d9c0e44cb` |
| `futhark-mc` | U32 256-bin histogram (80% hot-bin contention) | 0.170 | 0.162 | 3.36 | — | `7d9c0e44cb` |
| `futhark-ocl` | U32 256-bin histogram (80% hot-bin contention) | 0.325 | 0.377 | 29.48 | — | `7d9c0e44cb` |
| `cinch-cpu` | U32 256-bin histogram (80% hot-bin contention) | 0.182 | 0.177 | 3.52 | — | `7d9c0e44cb` |
| `cinch-metal` | U32 256-bin histogram (80% hot-bin contention) | 0.306 | 0.370 | 18.73 | 0.25 | `7d9c0e44cb` |

### Histogram U8 80%-Hot-Bin (4M)

- **Verification**: BIT-EXACT (d61efeb126d9, len=256)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | U32 256-bin histogram (80% hot-bin contention) | 6.714 | 6.505 | 10.03 | — | `d61efeb126` |
| `futhark-mc` | U32 256-bin histogram (80% hot-bin contention) | 0.613 | 0.645 | 10.64 | — | `d61efeb126` |
| `futhark-ocl` | U32 256-bin histogram (80% hot-bin contention) | 0.549 | 0.674 | 36.39 | — | `d61efeb126` |
| `cinch-cpu` | U32 256-bin histogram (80% hot-bin contention) | 0.869 | 1.109 | 11.03 | — | `d61efeb126` |
| `cinch-metal` | U32 256-bin histogram (80% hot-bin contention) | 0.455 | 0.825 | 29.98 | 4.00 | `d61efeb126` |

### Histogram U8 80%-Hot-Bin (16M)

- **Verification**: BIT-EXACT (8cfc8296fd03, len=256)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | U32 256-bin histogram (80% hot-bin contention) | 25.919 | 26.746 | 34.03 | — | `8cfc8296fd` |
| `futhark-mc` | U32 256-bin histogram (80% hot-bin contention) | 1.794 | 1.872 | 34.64 | — | `8cfc8296fd` |
| `futhark-ocl` | U32 256-bin histogram (80% hot-bin contention) | 0.844 | 1.036 | 48.41 | — | `8cfc8296fd` |
| `cinch-cpu` | U32 256-bin histogram (80% hot-bin contention) | 1.707 | 1.861 | 35.05 | — | `8cfc8296fd` |
| `cinch-metal` | U32 256-bin histogram (80% hot-bin contention) | 1.053 | 1.810 | 65.95 | 16.00 | `8cfc8296fd` |

### 2D 3x3 Stencil F32 3-step (512x512)

- **Verification**: MATCH (val=131072.0045, max_rel_err=0.00e+00)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.908 | 1.223 | 7.03 | — | `131072.0045` |
| `futhark-mc` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.655 | 0.531 | 9.56 | — | `131072.0045` |
| `futhark-ocl` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.477 | 0.540 | 34.95 | — | `131072.0045` |
| `cinch-cpu` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.219 | 0.223 | 8.03 | — | `131072.0045` |
| `cinch-metal` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.394 | 0.540 | 23.94 | 3.00 | `131072.0045` |

### 2D 3x3 Stencil F32 3-step (1024x1024)

- **Verification**: MATCH (val=524288.0197, max_rel_err=0.00e+00)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 3.321 | 3.462 | 22.06 | — | `524288.0197` |
| `futhark-mc` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.924 | 1.068 | 26.59 | — | `524288.0197` |
| `futhark-ocl` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 1.040 | 0.976 | 40.92 | — | `524288.0197` |
| `cinch-cpu` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.418 | 0.611 | 23.11 | — | `524288.0197` |
| `cinch-metal` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.980 | 1.203 | 37.89 | 12.00 | `524288.0197` |

### 2D 3x3 Stencil F32 3-step (2048x2048)

- **Verification**: MATCH (val=2097152.0669, max_rel_err=0.00e+00)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 13.305 | 13.118 | 82.06 | — | `2097152.0669` |
| `futhark-mc` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 2.511 | 2.538 | 98.61 | — | `2097152.0669` |
| `futhark-ocl` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.986 | 1.498 | 64.53 | — | `2097152.0669` |
| `cinch-cpu` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.736 | 1.338 | 83.12 | — | `2097152.0669` |
| `cinch-metal` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 1.965 | 4.057 | 97.91 | 48.00 | `2097152.0669` |

### Target Pipeline: Many Small Frames (1024x64x64, 3 steps)

- **Verification**: BIT-EXACT (33d8aff5e547, len=262144)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 24.960 | 25.035 | 20.09 | — | `33d8aff5e5` |
| `futhark-mc` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 4.287 | 4.029 | 24.67 | — | `33d8aff5e5` |
| `futhark-ocl` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 1.245 | 1.315 | 37.94 | — | `33d8aff5e5` |
| `cinch-cpu` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 2.206 | 1.776 | 21.19 | — | `33d8aff5e5` |
| `cinch-metal` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 1.531 | 1.924 | 31.98 | 13.00 | `33d8aff5e5` |

### Target Pipeline: Few Large Frames (2x2048x2048, 3 steps)

- **Verification**: BIT-EXACT (eeffc9880009, len=512)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 49.923 | 50.141 | 34.09 | — | `eeffc98800` |
| `futhark-mc` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 9.110 | 8.782 | 50.84 | — | `eeffc98800` |
| `futhark-ocl` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 2.004 | 2.219 | 40.55 | — | `eeffc98800` |
| `cinch-cpu` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 1.774 | 1.605 | 35.19 | — | `eeffc98800` |
| `cinch-metal` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 2.079 | 2.352 | 42.02 | 24.00 | `eeffc98800` |

### CSR SpMV F32 w/ 10% Empty Rows (4096 rows, ~65k nnz)

- **Verification**: MATCH (val=272.6875, max_rel_err=0.00e+00, FloatExactTree CPU==Metal: BIT-EXACT)
- **Dimensions**: `rows=4096,nnz=65536`

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.059 | 0.074 | 3.12 | — | `272.6875` |
| `futhark-mc` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.059 | 0.089 | 4.17 | — | `272.6875` |
| `futhark-ocl` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.243 | 0.292 | 31.89 | — | `272.6875` |
| `cinch-cpu` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.095 | 0.098 | 4.11 | — | `272.6875` |
| `cinch-cpu-exact` | F32 CSR SpMV w/ 10% empty rows (FloatExactTree) | 0.111 | 0.132 | 4.09 | — | `272.6875` |
| `cinch-metal` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.244 | 0.305 | 19.56 | 0.55 | `272.6875` |
| `cinch-metal-exact` | F32 CSR SpMV w/ 10% empty rows (FloatExactTree) | 0.260 | 0.369 | 19.61 | 0.55 | `272.6875` |

### CSR SpMV F32 w/ 10% Empty Rows (32768 rows, ~524k nnz)

- **Verification**: MATCH (val=419.5938, max_rel_err=0.00e+00, FloatExactTree CPU==Metal: BIT-EXACT)
- **Dimensions**: `rows=32768,nnz=524288`

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.397 | 0.533 | 10.88 | — | `419.5938` |
| `futhark-mc` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.085 | 0.172 | 11.38 | — | `419.5938` |
| `futhark-ocl` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.294 | 0.419 | 36.77 | — | `419.5938` |
| `cinch-cpu` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.142 | 0.279 | 11.84 | — | `419.5938` |
| `cinch-cpu-exact` | F32 CSR SpMV w/ 10% empty rows (FloatExactTree) | 0.214 | 0.352 | 11.89 | — | `419.5938` |
| `cinch-metal` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.249 | 0.662 | 31.00 | 4.38 | `419.5938` |
| `cinch-metal-exact` | F32 CSR SpMV w/ 10% empty rows (FloatExactTree) | 0.692 | 0.679 | 30.88 | 4.38 | `419.5938` |

### CSR SpMV F32 w/ 10% Empty Rows (131072 rows, ~2.1M nnz)

- **Verification**: MATCH (val=950.4375, max_rel_err=0.00e+00, FloatExactTree CPU==Metal: BIT-EXACT)
- **Dimensions**: `rows=131072,nnz=2097136`

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 1.208 | 1.381 | 37.12 | — | `950.4375` |
| `futhark-mc` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.213 | 0.613 | 37.64 | — | `950.4375` |
| `futhark-ocl` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.406 | 0.829 | 50.48 | — | `950.4375` |
| `cinch-cpu` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.236 | 0.615 | 38.14 | — | `950.4375` |
| `cinch-cpu-exact` | F32 CSR SpMV w/ 10% empty rows (FloatExactTree) | 0.353 | 0.665 | 38.12 | — | `950.4375` |
| `cinch-metal` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.414 | 2.117 | 70.03 | 17.50 | `950.4375` |
| `cinch-metal-exact` | F32 CSR SpMV w/ 10% empty rows (FloatExactTree) | 0.497 | 1.968 | 69.95 | 17.50 | `950.4375` |

