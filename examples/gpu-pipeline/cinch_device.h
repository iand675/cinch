#ifndef CINCH_DEVICE_H
#define CINCH_DEVICE_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Device error codes (non-negative returns are handles or byte counts). */
#define CINCH_DEV_OK                  0
#define CINCH_DEV_ERR_NO_DEVICE      -1
#define CINCH_DEV_ERR_UNAVAILABLE    -1
#define CINCH_DEV_ERR_INVALID_HANDLE -2
#define CINCH_DEV_ERR_OUT_OF_MEMORY  -3
#define CINCH_DEV_ERR_OOM            -3
#define CINCH_DEV_ERR_COMPILE_FAILED -4
#define CINCH_DEV_ERR_EXEC_FAILED    -5
#define CINCH_DEV_ERR_INVALID_SHAPE  -6

/* Backend selection for cinch_device_open (matches seed/runtime/src/metal.rs). */
#define CINCH_BACKEND_CPU             0   /* Multicore CPU (futhark multicore) */
#define CINCH_BACKEND_METAL           1   /* Apple Metal GPU with CPU fallback */
#define CINCH_BACKEND_METAL_REQUIRED  2   /* Apple Metal GPU required */
#define CINCH_BACKEND_OPENCL_REQUIRED 3   /* OpenCL GPU required (futhark opencl) */
#define CINCH_BACKEND_CUDA_REQUIRED   4   /* NVIDIA CUDA required (futhark cuda) */
#define CINCH_BACKEND_HIP_REQUIRED    5   /* AMD ROCm HIP required (futhark hip) */
#define CINCH_BACKEND_CPU_SEQ         6   /* Sequential single-thread CPU (futhark c) */
#define CINCH_BACKEND_ISPC_SIMD       7   /* SIMD-vectorized multicore CPU (futhark ispc) */
#define CINCH_BACKEND_AUTO            8   /* Auto-select best available accelerator */
#define CINCH_BACKEND_OPENCL          9   /* OpenCL with CPU fallback */
#define CINCH_BACKEND_CUDA           10   /* CUDA with CPU fallback */
#define CINCH_BACKEND_HIP            11   /* HIP with CPU fallback */

/* Stencil edge policies. */
#define CINCH_EDGE_CLAMP  0
#define CINCH_EDGE_ZERO   1
#define CINCH_EDGE_MIRROR 2
#define CINCH_EDGE_WRAP   3

/* Float reduction contracts. */
#define CINCH_FLOAT_EXACT_TREE 10
#define CINCH_FLOAT_RELAXED    11

int64_t cinch_device_open(int64_t backend_pref);
int64_t cinch_device_close(int64_t ctx_id);
int64_t cinch_device_reset_telemetry(int64_t ctx_id);

int64_t cinch_device_upload_raw(int64_t ctx_id, const uint8_t *src, size_t bytes);
int64_t cinch_device_download_raw(int64_t ctx_id, int64_t buf_id, uint8_t *dst, size_t bytes);
int64_t cinch_device_buffer_free(int64_t buf_id);
int64_t cinch_device_buffer_bytes(int64_t buf_id);

/* Telemetry & backend introspection. */
int64_t cinch_device_report_cstr(int64_t ctx_id, char *out, size_t out_cap);
int64_t cinch_device_available_backends_cstr(char *out, size_t out_cap);
int64_t cinch_device_backend_source_cstr(int64_t kind, char *out, size_t out_cap);

/* Resident batched 3x3 stencil (ping-pong) -> per-frame histogram. */
int64_t cinch_device_stencil_histogram_u8(
    int64_t ctx_id,
    int64_t in_buf_id,
    int64_t frames,
    int64_t height,
    int64_t width,
    const int32_t *weights9,
    int64_t divisor,
    int64_t edge_policy,
    int64_t steps,
    int64_t num_bins
);

/* Additional resident compute kernels (all supported on Metal, OpenCL, CUDA, HIP, ISPC, CPU, Seq). */
int64_t cinch_device_dot_f32(int64_t ctx_id, int64_t a_buf, int64_t b_buf, int64_t n, int64_t mode, float *out);
int64_t cinch_device_scan_i32(int64_t ctx_id, int64_t in_buf, int64_t n, int64_t inclusive);
int64_t cinch_device_filter_i32_gt(int64_t ctx_id, int64_t in_buf, int64_t n, int64_t threshold, int64_t *out_count);
int64_t cinch_device_stencil2d_f32(
    int64_t ctx_id, int64_t in_buf, int64_t frames, int64_t height, int64_t width,
    const float *weights9, int64_t edge_policy, int64_t steps
);
int64_t cinch_device_spmv_csr_f32(
    int64_t ctx_id, int64_t vals_buf, int64_t cols_buf, int64_t row_ptr_buf,
    int64_t x_buf, int64_t num_rows, int64_t nnz, int64_t mode
);
int64_t cinch_device_row_reduce_f32(int64_t ctx_id, int64_t mat_buf, int64_t rows, int64_t cols);

/* Generic 1D kernel runners for compiler-emitted MSL / OpenCL C kernels. */
int64_t cinch_device_run_msl_1d(
    int64_t ctx_id, const char *msl_src, const char *kernel_name,
    const int64_t *buf_ids, size_t nbufs,
    const uint8_t *uniform_ptr, size_t uniform_len, size_t n_threads
);
int64_t cinch_device_run_opencl_1d(
    int64_t ctx_id, const char *ocl_src, const char *kernel_name,
    const int64_t *buf_ids, size_t nbufs, size_t n_threads
);

#ifdef __cplusplus
}
#endif

#endif /* CINCH_DEVICE_H */
