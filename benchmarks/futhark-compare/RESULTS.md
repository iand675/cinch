# Cinch vs. Futhark 0.25.37 — Independent Benchmark Comparison

## Hardware & Pinned Toolchain

- **Machine**: Apple M5 Max (18 CPU cores, 128 GB unified RAM)
- **OS**: macOS 27.2.1 (arm64)
- **Futhark**: Futhark 0.25.37. / Compiled with GHC 9.10.3.
- **Host C Compiler**: Apple clang version 21.0.0 (clang-2100.4.1.1)
- **Cinch Revision**: `kqwzlulupztmmorryzxwrmqwuszrkwsp 93ce04478668dad13faa1e0115023dde230cad0c`
- **Timing Methodology**: Warm resident time = median of 15 timed iterations after 3 warmups (with explicit GPU/context synchronization per iteration). Transfer-inclusive time = median of 7 full upload → compute → sync → download cycles. Each `(backend, workload, size)` runs in a fresh isolated OS process so `ru_maxrss` reports true unpolluted peak host RSS.

## Cinch Typed Array IR Compiler Report (`workloads.cinch`)

```text
{"target":"cpu","kernel_language":"LLVM/C","nodes":[{"unit":"Data.Array.Compute.spmv-csr-composed-f32","op":"arraySpmvCsrF32","elem_type":"F32","shape":"CSR(rows, nnz)","contract":"f32-dynamic-contract","buffer_use":"fused-gather-mul-segreduce"},{"unit":"Data.Array.Compute.spmv-csr-composed-f32","op":"arraySpmvCsrF32","elem_type":"F32","shape":"CSR(rows, nnz)","contract":"f32-dynamic-contract","buffer_use":"fused-gather-mul-segreduce"},{"unit":"Workloads.dot-f32-composed","op":"arrayDotF32","elem_type":"F32","shape":"1D(n)","contract":"f32-dynamic-contract","buffer_use":"fused-zero-intermediate"},{"unit":"Workloads.dot-f32-composed","op":"arrayDotF32","elem_type":"F32","shape":"1D(n)","contract":"f32-dynamic-contract","buffer_use":"fused-zero-intermediate"},{"unit":"Workloads.spmv-csr-f32","op":"arraySpmvCsrF32","elem_type":"F32","shape":"CSR(rows, nnz)","contract":"f32-dynamic-contract","buffer_use":"fused-gather-mul-segreduce"},{"unit":"Workloads.spmv-csr-f32","op":"arraySpmvCsrF32","elem_type":"F32","shape":"CSR(rows, nnz)","contract":"f32-dynamic-contract","buffer_use":"fused-gather-mul-segreduce"},{"unit":"Main.main","op":"arrayDotF32","elem_type":"F32","shape":"1D(n)","contract":"f32-add-exact-tree","buffer_use":"fused-zero-intermediate"},{"unit":"Main.main","op":"arrayDotF32","elem_type":"F32","shape":"1D(n)","contract":"f32-dynamic-contract","buffer_use":"fused-zero-intermediate"},{"unit":"Main.main","op":"arraySpmvCsrF32","elem_type":"F32","shape":"CSR(rows, nnz)","contract":"f32-add-exact-tree","buffer_use":"fused-gather-mul-segreduce"},{"unit":"Main.main","op":"arraySpmvCsrF32","elem_type":"F32","shape":"CSR(rows, nnz)","contract":"f32-dynamic-contract","buffer_use":"fused-gather-mul-segreduce"}],"fusions":[{"unit":"Data.Array.Compute.spmv-csr-composed-f32","producer":"Data.Array.Compute.array-gather-f32 + Data.Array.Compute.array-zip-binop-f32","consumer":"Data.Array.Compute.array-segmented-reduce-f32","fused_op":"arraySpmvCsrF32","eliminated_allocation":"2 intermediate Array F32 buffers (8 * nnz bytes)","numerical_mode":"f32-dynamic-contract"},{"unit":"Workloads.dot-f32-composed","producer":"Data.Array.Compute.array-zip-binop-f32","consumer":"Data.Array.Compute.array-reduce-f32","fused_op":"arrayDotF32","eliminated_allocation":"intermediate product Array F32 (4 * n bytes)","numerical_mode":"f32-dynamic-contract"},{"unit":"Workloads.spmv-csr-f32","producer":"Data.Array.Compute.array-gather-f32 + Data.Array.Compute.array-zip-binop-f32","consumer":"Data.Array.Compute.array-segmented-reduce-f32","fused_op":"arraySpmvCsrF32","eliminated_allocation":"2 intermediate Array F32 buffers (8 * nnz bytes)","numerical_mode":"f32-dynamic-contract"},{"unit":"Main.main","producer":"Data.Array.Compute.array-zip-binop-f32","consumer":"Data.Array.Compute.array-reduce-f32","fused_op":"arrayDotF32","eliminated_allocation":"intermediate product Array F32 (4 * n bytes)","numerical_mode":"f32-add-exact-tree"},{"unit":"Main.main","producer":"Data.Array.Compute.array-gather-f32 + Data.Array.Compute.array-zip-binop-f32","consumer":"Data.Array.Compute.array-segmented-reduce-f32","fused_op":"arraySpmvCsrF32","eliminated_allocation":"2 intermediate Array F32 buffers (8 * nnz bytes)","numerical_mode":"f32-add-exact-tree"}],"schedules":[{"unit":"Data.Array.Compute.spmv-csr-composed-f32","op":"arraySpmvCsrF32","policy":"adaptive-csr(scalar-row if avg_nnz<16 else simd-group-row; empty-row=0.0)","numerical_mode":"f32-dynamic-contract"},{"unit":"Data.Array.Compute.spmv-csr-composed-f32","op":"arraySpmvCsrF32","policy":"adaptive-csr(scalar-row if avg_nnz<16 else simd-group-row; empty-row=0.0)","numerical_mode":"f32-dynamic-contract"},{"unit":"Workloads.spmv-csr-f32","op":"arraySpmvCsrF32","policy":"adaptive-csr(scalar-row if avg_nnz<16 else simd-group-row; empty-row=0.0)","numerical_mode":"f32-dynamic-contract"},{"unit":"Workloads.spmv-csr-f32","op":"arraySpmvCsrF32","policy":"adaptive-csr(scalar-row if avg_nnz<16 else simd-group-row; empty-row=0.0)","numerical_mode":"f32-dynamic-contract"},{"unit":"Main.main","op":"arraySpmvCsrF32","policy":"adaptive-csr(scalar-row if avg_nnz<16 else simd-group-row; empty-row=0.0)","numerical_mode":"f32-add-exact-tree"},{"unit":"Main.main","op":"arraySpmvCsrF32","policy":"adaptive-csr(scalar-row if avg_nnz<16 else simd-group-row; empty-row=0.0)","numerical_mode":"f32-dynamic-contract"}],"storage":[],"gpu_diagnostics":[],"msl_kernel_count":0,"generated_kernels":[]}
```

## Summary Tables

### Dot F32 (100k)

- **Verification**: MATCH (val=489.0781, max_rel_err=0.00e+00, FloatExactTree CPU==OpenCL==Metal: BIT-EXACT)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | FloatRelaxed (associative f32 sum) | 0.106 | 0.146 | 3.53 | — | `489.0781` |
| `cinch-seq` | FloatRelaxed (associative f32 sum) | 0.057 | 0.076 | 4.11 | — | `489.0781` |
| `futhark-mc` | FloatRelaxed (associative f32 sum) | 0.027 | 0.081 | 4.05 | — | `489.0781` |
| `cinch-cpu` | FloatRelaxed (associative f32 sum) | 0.103 | 0.104 | 5.41 | — | `489.0781` |
| `cinch-cpu-exact` | FloatExactTree (deterministic binary tree) | 0.127 | 0.163 | 4.55 | — | `489.0781` |
| `cinch-ispc` | FloatRelaxed (associative f32 sum) | 0.218 | 0.231 | 4.67 | — | `489.0781` |
| `futhark-ocl` | FloatRelaxed (associative f32 sum) | 0.226 | 0.245 | 33.39 | — | `489.0781` |
| `cinch-opencl` | FloatRelaxed (associative f32 sum) | 0.197 | 0.653 | 22.06 | 0.76 | `489.0781` |
| `cinch-opencl-exact` | FloatExactTree (deterministic binary tree) | 0.673 | 1.235 | 22.06 | 0.76 | `489.0781` |
| `cinch-metal` | FloatRelaxed (associative f32 sum) | 0.195 | 0.299 | 20.52 | 0.76 | `489.0781` |
| `cinch-metal-exact` | FloatExactTree (deterministic binary tree) | 0.646 | 0.756 | 20.47 | 0.76 | `489.0781` |

### Dot F32 (1M)

- **Verification**: MATCH (val=3257.9375, max_rel_err=0.00e+00, FloatExactTree CPU==OpenCL==Metal: BIT-EXACT)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | FloatRelaxed (associative f32 sum) | 0.641 | 0.740 | 17.31 | — | `3257.9375` |
| `cinch-seq` | FloatRelaxed (associative f32 sum) | 0.830 | 0.970 | 17.89 | — | `3257.9375` |
| `futhark-mc` | FloatRelaxed (associative f32 sum) | 0.177 | 0.366 | 17.84 | — | `3257.9375` |
| `cinch-cpu` | FloatRelaxed (associative f32 sum) | 0.090 | 0.663 | 18.38 | — | `3257.9375` |
| `cinch-cpu-exact` | FloatExactTree (deterministic binary tree) | 0.222 | 0.745 | 18.39 | — | `3257.9375` |
| `cinch-ispc` | FloatRelaxed (associative f32 sum) | 0.251 | 0.482 | 18.39 | — | `3257.9375` |
| `futhark-ocl` | FloatRelaxed (associative f32 sum) | 0.253 | 0.436 | 40.42 | — | `3257.9375` |
| `cinch-opencl` | FloatRelaxed (associative f32 sum) | 0.234 | 0.953 | 28.98 | 7.63 | `3257.9375` |
| `cinch-opencl-exact` | FloatExactTree (deterministic binary tree) | 0.646 | 1.198 | 28.97 | 7.63 | `3257.9375` |
| `cinch-metal` | FloatRelaxed (associative f32 sum) | 0.210 | 0.912 | 41.11 | 7.63 | `3257.9375` |
| `cinch-metal-exact` | FloatExactTree (deterministic binary tree) | 0.611 | 1.025 | 41.17 | 7.63 | `3257.9375` |

### Dot F32 (10M)

- **Verification**: MATCH (val=39210.4062, max_rel_err=0.00e+00, FloatExactTree CPU==OpenCL==Metal: BIT-EXACT)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | FloatRelaxed (associative f32 sum) | 4.979 | 6.421 | 154.62 | — | `39210.4062` |
| `cinch-seq` | FloatRelaxed (associative f32 sum) | 4.920 | 6.351 | 155.20 | — | `39210.4062` |
| `futhark-mc` | FloatRelaxed (associative f32 sum) | 0.419 | 1.859 | 155.14 | — | `39210.4062` |
| `cinch-cpu` | FloatRelaxed (associative f32 sum) | 0.396 | 1.736 | 155.66 | — | `39210.4062` |
| `cinch-cpu-exact` | FloatExactTree (deterministic binary tree) | 1.793 | 3.137 | 155.73 | — | `39210.4062` |
| `cinch-ispc` | FloatRelaxed (associative f32 sum) | 0.481 | 1.994 | 155.97 | — | `39210.4062` |
| `futhark-ocl` | FloatRelaxed (associative f32 sum) | 0.515 | 1.852 | 109.42 | — | `39210.4062` |
| `cinch-opencl` | FloatRelaxed (associative f32 sum) | 0.509 | 4.750 | 97.75 | 76.30 | `39210.4062` |
| `cinch-opencl-exact` | FloatExactTree (deterministic binary tree) | 0.779 | 4.603 | 97.62 | 76.33 | `39210.4062` |
| `cinch-metal` | FloatRelaxed (associative f32 sum) | 0.424 | 7.094 | 247.11 | 76.30 | `39210.4062` |
| `cinch-metal-exact` | FloatExactTree (deterministic binary tree) | 0.770 | 7.137 | 247.16 | 76.33 | `39210.4062` |

### Inclusive Scan I32 (100k)

- **Verification**: BIT-EXACT (ee56c044e16a, len=100000)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | I32 wrapping-add inclusive prefix scan | 0.052 | 0.074 | 3.53 | — | `ee56c044e1` |
| `cinch-seq` | I32 wrapping-add inclusive prefix scan | 0.109 | 0.127 | 5.69 | — | `ee56c044e1` |
| `futhark-mc` | I32 wrapping-add inclusive prefix scan | 0.043 | 0.062 | 6.02 | — | `ee56c044e1` |
| `cinch-cpu` | I32 wrapping-add inclusive prefix scan | 0.107 | 0.161 | 5.41 | — | `ee56c044e1` |
| `cinch-ispc` | I32 wrapping-add inclusive prefix scan | 0.588 | 0.473 | 6.22 | — | `ee56c044e1` |
| `futhark-ocl` | I32 wrapping-add inclusive prefix scan | 0.255 | 0.315 | 30.39 | — | `ee56c044e1` |
| `cinch-opencl` | I32 wrapping-add inclusive prefix scan | 0.611 | 0.919 | 22.22 | 0.76 | `ee56c044e1` |
| `cinch-metal` | I32 wrapping-add inclusive prefix scan | 0.521 | 0.517 | 20.11 | 0.77 | `ee56c044e1` |

### Inclusive Scan I32 (1M)

- **Verification**: BIT-EXACT (b02ff1534b64, len=1000000)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | I32 wrapping-add inclusive prefix scan | 0.240 | 0.351 | 17.31 | — | `b02ff1534b` |
| `cinch-seq` | I32 wrapping-add inclusive prefix scan | 0.560 | 0.751 | 21.77 | — | `b02ff1534b` |
| `futhark-mc` | I32 wrapping-add inclusive prefix scan | 0.106 | 0.325 | 18.98 | — | `b02ff1534b` |
| `cinch-cpu` | I32 wrapping-add inclusive prefix scan | 0.304 | 0.856 | 18.41 | — | `b02ff1534b` |
| `cinch-ispc` | I32 wrapping-add inclusive prefix scan | 0.681 | 0.900 | 22.47 | — | `b02ff1534b` |
| `futhark-ocl` | I32 wrapping-add inclusive prefix scan | 0.612 | 0.812 | 40.80 | — | `b02ff1534b` |
| `cinch-opencl` | I32 wrapping-add inclusive prefix scan | 0.809 | 1.549 | 29.20 | 7.64 | `b02ff1534b` |
| `cinch-metal` | I32 wrapping-add inclusive prefix scan | 0.683 | 1.216 | 37.39 | 7.66 | `b02ff1534b` |

### Inclusive Scan I32 (5M)

- **Verification**: BIT-EXACT (274ac995124e, len=5000000)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | I32 wrapping-add inclusive prefix scan | 1.492 | 2.195 | 78.31 | — | `274ac99512` |
| `cinch-seq` | I32 wrapping-add inclusive prefix scan | 2.274 | 2.844 | 98.02 | — | `274ac99512` |
| `futhark-mc` | I32 wrapping-add inclusive prefix scan | 0.228 | 0.793 | 81.02 | — | `274ac99512` |
| `cinch-cpu` | I32 wrapping-add inclusive prefix scan | 0.636 | 1.157 | 79.41 | — | `274ac99512` |
| `cinch-ispc` | I32 wrapping-add inclusive prefix scan | 1.118 | 1.954 | 98.89 | — | `274ac99512` |
| `futhark-ocl` | I32 wrapping-add inclusive prefix scan | 0.761 | 1.338 | 70.86 | — | `274ac99512` |
| `cinch-opencl` | I32 wrapping-add inclusive prefix scan | 1.624 | 3.192 | 61.22 | 38.22 | `274ac99512` |
| `cinch-metal` | I32 wrapping-add inclusive prefix scan | 1.557 | 4.204 | 113.72 | 38.30 | `274ac99512` |

### Stable Compaction I32 (100k)

- **Verification**: BIT-EXACT (9794c0adcc3c, len=37486)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | I32 stable compaction (x > 250) | 0.477 | 0.437 | 3.06 | — | `9794c0adcc` |
| `cinch-seq` | I32 stable compaction (x > 250) | 0.352 | 0.331 | 4.73 | — | `9794c0adcc` |
| `futhark-mc` | I32 stable compaction (x > 250) | 0.090 | 0.138 | 6.12 | — | `9794c0adcc` |
| `cinch-cpu` | I32 stable compaction (x > 250) | 0.169 | 0.236 | 5.06 | — | `9794c0adcc` |
| `cinch-ispc` | I32 stable compaction (x > 250) | 0.431 | 0.493 | 5.11 | — | `9794c0adcc` |
| `futhark-ocl` | I32 stable compaction (x > 250) | 0.284 | 0.465 | 31.73 | — | `9794c0adcc` |
| `cinch-opencl` | I32 stable compaction (x > 250) | 1.136 | 1.497 | 21.98 | 1.29 | `9794c0adcc` |
| `cinch-metal` | I32 stable compaction (x > 250) | 0.832 | 0.927 | 19.75 | 1.29 | `9794c0adcc` |

### Stable Compaction I32 (1M)

- **Verification**: BIT-EXACT (6639d9b383d7, len=375080)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | I32 stable compaction (x > 250) | 2.935 | 2.569 | 14.92 | — | `6639d9b383` |
| `cinch-seq` | I32 stable compaction (x > 250) | 2.659 | 2.393 | 14.78 | — | `6639d9b383` |
| `futhark-mc` | I32 stable compaction (x > 250) | 0.333 | 0.430 | 18.48 | — | `6639d9b383` |
| `cinch-cpu` | I32 stable compaction (x > 250) | 0.506 | 1.067 | 13.61 | — | `6639d9b383` |
| `cinch-ispc` | I32 stable compaction (x > 250) | 0.718 | 0.851 | 16.88 | — | `6639d9b383` |
| `futhark-ocl` | I32 stable compaction (x > 250) | 0.720 | 0.991 | 38.08 | — | `6639d9b383` |
| `cinch-opencl` | I32 stable compaction (x > 250) | 1.566 | 1.954 | 26.92 | 12.87 | `6639d9b383` |
| `cinch-metal` | I32 stable compaction (x > 250) | 1.284 | 1.619 | 32.73 | 12.87 | `6639d9b383` |

### Stable Compaction I32 (5M)

- **Verification**: BIT-EXACT (248fddf67b21, len=1875670)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | I32 stable compaction (x > 250) | 11.220 | 11.462 | 66.39 | — | `248fddf67b` |
| `cinch-seq` | I32 stable compaction (x > 250) | 10.695 | 10.733 | 230.27 | — | `248fddf67b` |
| `futhark-mc` | I32 stable compaction (x > 250) | 1.059 | 1.441 | 69.98 | — | `248fddf67b` |
| `cinch-cpu` | I32 stable compaction (x > 250) | 1.541 | 2.139 | 55.59 | — | `248fddf67b` |
| `cinch-ispc` | I32 stable compaction (x > 250) | 1.882 | 2.366 | 63.03 | — | `248fddf67b` |
| `futhark-ocl` | I32 stable compaction (x > 250) | 0.905 | 1.375 | 59.41 | — | `248fddf67b` |
| `cinch-opencl` | I32 stable compaction (x > 250) | 3.049 | 4.125 | 49.48 | 64.38 | `248fddf67b` |
| `cinch-metal` | I32 stable compaction (x > 250) | 3.266 | 5.603 | 90.16 | 64.38 | `248fddf67b` |

### Histogram U8 Uniform (256k)

- **Verification**: BIT-EXACT (2b62e045fe5e, len=256)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | U32 256-bin histogram (uniform) | 0.050 | 0.053 | 2.52 | — | `2b62e045fe` |
| `cinch-seq` | U32 256-bin histogram (uniform) | 0.183 | 0.227 | 3.36 | — | `2b62e045fe` |
| `futhark-mc` | U32 256-bin histogram (uniform) | 0.052 | 0.099 | 3.05 | — | `2b62e045fe` |
| `cinch-cpu` | U32 256-bin histogram (uniform) | 0.077 | 0.074 | 3.89 | — | `2b62e045fe` |
| `cinch-ispc` | U32 256-bin histogram (uniform) | 0.356 | 0.304 | 4.50 | — | `2b62e045fe` |
| `futhark-ocl` | U32 256-bin histogram (uniform) | 0.294 | 0.366 | 30.70 | — | `2b62e045fe` |
| `cinch-opencl` | U32 256-bin histogram (uniform) | 0.236 | 0.591 | 21.56 | 0.25 | `2b62e045fe` |
| `cinch-metal` | U32 256-bin histogram (uniform) | 0.273 | 0.357 | 18.98 | 0.25 | `2b62e045fe` |

### Histogram U8 Uniform (4M)

- **Verification**: BIT-EXACT (ba51d34c6b6a, len=256)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | U32 256-bin histogram (uniform) | 1.000 | 1.000 | 9.98 | — | `ba51d34c6b` |
| `cinch-seq` | U32 256-bin histogram (uniform) | 2.034 | 1.854 | 18.67 | — | `ba51d34c6b` |
| `futhark-mc` | U32 256-bin histogram (uniform) | 0.151 | 0.257 | 10.58 | — | `ba51d34c6b` |
| `cinch-cpu` | U32 256-bin histogram (uniform) | 0.281 | 0.538 | 11.14 | — | `ba51d34c6b` |
| `cinch-ispc` | U32 256-bin histogram (uniform) | 0.432 | 0.535 | 19.52 | — | `ba51d34c6b` |
| `futhark-ocl` | U32 256-bin histogram (uniform) | 0.505 | 0.621 | 37.16 | — | `ba51d34c6b` |
| `cinch-opencl` | U32 256-bin histogram (uniform) | 0.262 | 0.864 | 25.39 | 4.00 | `ba51d34c6b` |
| `cinch-metal` | U32 256-bin histogram (uniform) | 0.277 | 0.609 | 30.33 | 4.00 | `ba51d34c6b` |

### Histogram U8 Uniform (16M)

- **Verification**: BIT-EXACT (5e9d2f0e6927, len=256)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | U32 256-bin histogram (uniform) | 3.373 | 3.444 | 33.98 | — | `5e9d2f0e69` |
| `cinch-seq` | U32 256-bin histogram (uniform) | 5.521 | 5.908 | 66.67 | — | `5e9d2f0e69` |
| `futhark-mc` | U32 256-bin histogram (uniform) | 0.553 | 0.876 | 34.56 | — | `5e9d2f0e69` |
| `cinch-cpu` | U32 256-bin histogram (uniform) | 0.757 | 0.861 | 35.14 | — | `5e9d2f0e69` |
| `cinch-ispc` | U32 256-bin histogram (uniform) | 1.135 | 1.506 | 67.52 | — | `5e9d2f0e69` |
| `futhark-ocl` | U32 256-bin histogram (uniform) | 0.798 | 0.966 | 49.12 | — | `5e9d2f0e69` |
| `cinch-opencl` | U32 256-bin histogram (uniform) | 0.416 | 1.493 | 37.34 | 16.00 | `5e9d2f0e69` |
| `cinch-metal` | U32 256-bin histogram (uniform) | 0.426 | 1.761 | 66.30 | 16.00 | `5e9d2f0e69` |

### Histogram U8 80%-Hot-Bin (256k)

- **Verification**: BIT-EXACT (7d9c0e44cb46, len=256)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | U32 256-bin histogram (80% hot-bin contention) | 0.793 | 0.662 | 2.50 | — | `7d9c0e44cb` |
| `cinch-seq` | U32 256-bin histogram (80% hot-bin contention) | 0.422 | 0.431 | 3.89 | — | `7d9c0e44cb` |
| `futhark-mc` | U32 256-bin histogram (80% hot-bin contention) | 0.135 | 0.135 | 3.31 | — | `7d9c0e44cb` |
| `cinch-cpu` | U32 256-bin histogram (80% hot-bin contention) | 0.159 | 0.244 | 3.86 | — | `7d9c0e44cb` |
| `cinch-ispc` | U32 256-bin histogram (80% hot-bin contention) | 0.234 | 0.220 | 4.59 | — | `7d9c0e44cb` |
| `futhark-ocl` | U32 256-bin histogram (80% hot-bin contention) | 0.296 | 0.372 | 31.66 | — | `7d9c0e44cb` |
| `cinch-opencl` | U32 256-bin histogram (80% hot-bin contention) | 0.230 | 0.685 | 21.41 | 0.25 | `7d9c0e44cb` |
| `cinch-metal` | U32 256-bin histogram (80% hot-bin contention) | 0.284 | 0.314 | 19.05 | 0.25 | `7d9c0e44cb` |

### Histogram U8 80%-Hot-Bin (4M)

- **Verification**: BIT-EXACT (d61efeb126d9, len=256)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | U32 256-bin histogram (80% hot-bin contention) | 6.818 | 6.651 | 9.98 | — | `d61efeb126` |
| `cinch-seq` | U32 256-bin histogram (80% hot-bin contention) | 6.730 | 6.442 | 18.66 | — | `d61efeb126` |
| `futhark-mc` | U32 256-bin histogram (80% hot-bin contention) | 0.615 | 0.931 | 10.59 | — | `d61efeb126` |
| `cinch-cpu` | U32 256-bin histogram (80% hot-bin contention) | 0.832 | 1.082 | 11.16 | — | `d61efeb126` |
| `cinch-ispc` | U32 256-bin histogram (80% hot-bin contention) | 0.992 | 1.021 | 19.52 | — | `d61efeb126` |
| `futhark-ocl` | U32 256-bin histogram (80% hot-bin contention) | 0.529 | 0.537 | 37.17 | — | `d61efeb126` |
| `cinch-opencl` | U32 256-bin histogram (80% hot-bin contention) | 0.383 | 0.955 | 25.31 | 4.00 | `d61efeb126` |
| `cinch-metal` | U32 256-bin histogram (80% hot-bin contention) | 0.422 | 0.767 | 30.30 | 4.00 | `d61efeb126` |

### Histogram U8 80%-Hot-Bin (16M)

- **Verification**: BIT-EXACT (8cfc8296fd03, len=256)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | U32 256-bin histogram (80% hot-bin contention) | 25.801 | 26.818 | 34.00 | — | `8cfc8296fd` |
| `cinch-seq` | U32 256-bin histogram (80% hot-bin contention) | 25.418 | 25.675 | 66.69 | — | `8cfc8296fd` |
| `futhark-mc` | U32 256-bin histogram (80% hot-bin contention) | 1.799 | 2.504 | 34.58 | — | `8cfc8296fd` |
| `cinch-cpu` | U32 256-bin histogram (80% hot-bin contention) | 2.155 | 2.124 | 35.16 | — | `8cfc8296fd` |
| `cinch-ispc` | U32 256-bin histogram (80% hot-bin contention) | 2.512 | 2.928 | 67.47 | — | `8cfc8296fd` |
| `futhark-ocl` | U32 256-bin histogram (80% hot-bin contention) | 0.794 | 1.040 | 49.17 | — | `8cfc8296fd` |
| `cinch-opencl` | U32 256-bin histogram (80% hot-bin contention) | 0.943 | 1.890 | 37.34 | 16.00 | `8cfc8296fd` |
| `cinch-metal` | U32 256-bin histogram (80% hot-bin contention) | 0.555 | 1.977 | 66.28 | 16.00 | `8cfc8296fd` |

### 2D 3x3 Stencil F32 3-step (512x512)

- **Verification**: MATCH (val=131072.0045, max_rel_err=0.00e+00)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 1.318 | 1.209 | 7.03 | — | `131072.0045` |
| `cinch-seq` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 2.563 | 2.181 | 8.61 | — | `131072.0045` |
| `futhark-mc` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.506 | 0.463 | 8.52 | — | `131072.0045` |
| `cinch-cpu` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.185 | 0.221 | 11.19 | — | `131072.0045` |
| `cinch-ispc` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.602 | 0.605 | 13.16 | — | `131072.0045` |
| `futhark-ocl` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.465 | 0.549 | 34.97 | — | `131072.0045` |
| `cinch-opencl` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.351 | 0.760 | 23.33 | 3.00 | `131072.0045` |
| `cinch-metal` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.330 | 0.539 | 23.23 | 3.00 | `131072.0045` |

### 2D 3x3 Stencil F32 3-step (1024x1024)

- **Verification**: MATCH (val=524288.0197, max_rel_err=0.00e+00)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 3.768 | 3.267 | 22.03 | — | `524288.0197` |
| `cinch-seq` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 7.395 | 7.305 | 26.69 | — | `524288.0197` |
| `futhark-mc` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.799 | 0.855 | 26.58 | — | `524288.0197` |
| `cinch-cpu` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.365 | 0.568 | 23.22 | — | `524288.0197` |
| `cinch-ispc` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 1.101 | 1.251 | 27.48 | — | `524288.0197` |
| `futhark-ocl` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.765 | 0.724 | 40.92 | — | `524288.0197` |
| `cinch-opencl` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.659 | 1.374 | 29.34 | 12.00 | `524288.0197` |
| `cinch-metal` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.665 | 1.298 | 38.25 | 12.00 | `524288.0197` |

### 2D 3x3 Stencil F32 3-step (2048x2048)

- **Verification**: MATCH (val=2097152.0669, max_rel_err=0.00e+00)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 12.910 | 13.226 | 82.06 | — | `2097152.0669` |
| `cinch-seq` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 28.851 | 29.292 | 98.67 | — | `2097152.0669` |
| `futhark-mc` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 2.181 | 2.562 | 98.58 | — | `2097152.0669` |
| `cinch-cpu` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 0.858 | 1.490 | 83.17 | — | `2097152.0669` |
| `cinch-ispc` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 1.824 | 2.420 | 99.53 | — | `2097152.0669` |
| `futhark-ocl` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 1.051 | 1.508 | 65.38 | — | `2097152.0669` |
| `cinch-opencl` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 1.736 | 2.937 | 53.34 | 48.00 | `2097152.0669` |
| `cinch-metal` | F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps | 1.865 | 4.009 | 98.25 | 48.00 | `2097152.0669` |

### Target Pipeline: Many Small Frames (1024x64x64, 3 steps)

- **Verification**: BIT-EXACT (33d8aff5e547, len=262144)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 25.749 | 25.651 | 20.06 | — | `33d8aff5e5` |
| `cinch-seq` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 37.545 | 37.490 | 13.72 | — | `33d8aff5e5` |
| `futhark-mc` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 5.112 | 4.751 | 24.66 | — | `33d8aff5e5` |
| `cinch-cpu` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 1.819 | 1.948 | 21.20 | — | `33d8aff5e5` |
| `cinch-ispc` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 4.210 | 4.021 | 14.64 | — | `33d8aff5e5` |
| `futhark-ocl` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 1.216 | 1.301 | 38.36 | — | `33d8aff5e5` |
| `cinch-opencl` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 1.216 | 1.637 | 26.20 | 13.00 | `33d8aff5e5` |
| `cinch-metal` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 1.699 | 1.638 | 32.31 | 13.00 | `33d8aff5e5` |

### Target Pipeline: Few Large Frames (2x2048x2048, 3 steps)

- **Verification**: BIT-EXACT (eeffc9880009, len=512)

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 51.491 | 51.137 | 34.11 | — | `eeffc98800` |
| `cinch-seq` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 72.418 | 71.785 | 26.70 | — | `eeffc98800` |
| `futhark-mc` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 9.253 | 9.255 | 50.73 | — | `eeffc98800` |
| `cinch-cpu` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 2.069 | 2.104 | 35.23 | — | `eeffc98800` |
| `cinch-ispc` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 6.466 | 7.375 | 35.59 | — | `eeffc98800` |
| `futhark-ocl` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 2.047 | 2.211 | 40.91 | — | `eeffc98800` |
| `cinch-opencl` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 1.762 | 2.494 | 29.47 | 24.00 | `eeffc98800` |
| `cinch-metal` | U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download) | 1.882 | 2.406 | 42.28 | 24.00 | `eeffc98800` |

### CSR SpMV F32 w/ 10% Empty Rows (4096 rows, ~65k nnz)

- **Verification**: MATCH (val=272.6875, max_rel_err=0.00e+00, FloatExactTree CPU==OpenCL==Metal: BIT-EXACT)
- **Dimensions**: `rows=4096,nnz=65536`

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.060 | 0.083 | 3.62 | — | `272.6875` |
| `cinch-seq` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.076 | 0.094 | 3.69 | — | `272.6875` |
| `futhark-mc` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.052 | 0.053 | 3.62 | — | `272.6875` |
| `cinch-cpu` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.072 | 0.071 | 4.22 | — | `272.6875` |
| `cinch-cpu-exact` | F32 CSR SpMV w/ 10% empty rows (FloatExactTree) | 0.088 | 0.112 | 4.20 | — | `272.6875` |
| `cinch-ispc` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.367 | 0.372 | 4.66 | — | `272.6875` |
| `futhark-ocl` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.225 | 0.297 | 32.05 | — | `272.6875` |
| `cinch-opencl` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.178 | 0.966 | 22.19 | 0.55 | `272.6875` |
| `cinch-opencl-exact` | F32 CSR SpMV w/ 10% empty rows (FloatExactTree) | 0.210 | 1.436 | 22.14 | 0.55 | `272.6875` |
| `cinch-metal` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.216 | 0.438 | 19.91 | 0.55 | `272.6875` |
| `cinch-metal-exact` | F32 CSR SpMV w/ 10% empty rows (FloatExactTree) | 0.274 | 0.335 | 19.88 | 0.55 | `272.6875` |

### CSR SpMV F32 w/ 10% Empty Rows (32768 rows, ~524k nnz)

- **Verification**: MATCH (val=419.5938, max_rel_err=0.00e+00, FloatExactTree CPU==OpenCL==Metal: BIT-EXACT)
- **Dimensions**: `rows=32768,nnz=524288`

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.395 | 0.518 | 10.78 | — | `419.5938` |
| `cinch-seq` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.247 | 0.348 | 19.53 | — | `419.5938` |
| `futhark-mc` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.114 | 0.176 | 11.69 | — | `419.5938` |
| `cinch-cpu` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.085 | 0.183 | 12.00 | — | `419.5938` |
| `cinch-cpu-exact` | F32 CSR SpMV w/ 10% empty rows (FloatExactTree) | 0.139 | 0.238 | 12.41 | — | `419.5938` |
| `cinch-ispc` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.353 | 0.470 | 13.56 | — | `419.5938` |
| `futhark-ocl` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.239 | 0.418 | 35.75 | — | `419.5938` |
| `cinch-opencl` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.250 | 1.296 | 25.58 | 4.38 | `419.5938` |
| `cinch-opencl-exact` | F32 CSR SpMV w/ 10% empty rows (FloatExactTree) | 0.277 | 1.484 | 25.61 | 4.38 | `419.5938` |
| `cinch-metal` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.227 | 0.680 | 31.25 | 4.38 | `419.5938` |
| `cinch-metal-exact` | F32 CSR SpMV w/ 10% empty rows (FloatExactTree) | 0.278 | 0.820 | 31.31 | 4.38 | `419.5938` |

### CSR SpMV F32 w/ 10% Empty Rows (131072 rows, ~2.1M nnz)

- **Verification**: MATCH (val=950.4375, max_rel_err=0.00e+00, FloatExactTree CPU==OpenCL==Metal: BIT-EXACT)
- **Dimensions**: `rows=131072,nnz=2097136`

| Backend | Numerical Contract | Warm Resident (ms) | Transfer-Inclusive (ms) | Peak Host RSS (MB) | Peak Device Mem (MB) | Result |
| :--- | :--- | ---: | ---: | ---: | ---: | ---: |
| `futhark-c` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 1.268 | 1.424 | 37.08 | — | `950.4375` |
| `cinch-seq` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 1.120 | 1.344 | 38.19 | — | `950.4375` |
| `futhark-mc` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.175 | 0.548 | 37.58 | — | `950.4375` |
| `cinch-cpu` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.245 | 0.791 | 38.22 | — | `950.4375` |
| `cinch-cpu-exact` | F32 CSR SpMV w/ 10% empty rows (FloatExactTree) | 0.546 | 0.995 | 38.17 | — | `950.4375` |
| `cinch-ispc` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.399 | 0.844 | 38.86 | — | `950.4375` |
| `futhark-ocl` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.386 | 0.862 | 50.83 | — | `950.4375` |
| `cinch-opencl` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.357 | 1.901 | 38.95 | 17.50 | `950.4375` |
| `cinch-opencl-exact` | F32 CSR SpMV w/ 10% empty rows (FloatExactTree) | 0.388 | 2.520 | 38.94 | 17.50 | `950.4375` |
| `cinch-metal` | F32 CSR SpMV w/ 10% empty rows (FloatRelaxed) | 0.346 | 1.901 | 70.28 | 17.50 | `950.4375` |
| `cinch-metal-exact` | F32 CSR SpMV w/ 10% empty rows (FloatExactTree) | 0.437 | 2.084 | 70.30 | 17.50 | `950.4375` |

