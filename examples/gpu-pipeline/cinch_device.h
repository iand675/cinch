#ifndef CINCH_DEVICE_H
#define CINCH_DEVICE_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Device error codes (non-negative returns are handles or byte counts). */
#define CINCH_DEV_OK                 0
#define CINCH_DEV_ERR_UNAVAILABLE   -1
#define CINCH_DEV_ERR_INVALID_HANDLE -2
#define CINCH_DEV_ERR_INVALID_SHAPE -3
#define CINCH_DEV_ERR_COMPILE_FAILED -4
#define CINCH_DEV_ERR_OOM           -5
#define CINCH_DEV_ERR_EXEC_FAILED   -6

/* Backend selection for cinch_device_open. */
#define CINCH_BACKEND_AUTO           0
#define CINCH_BACKEND_METAL_REQUIRED 1
#define CINCH_BACKEND_CPU_ONLY       2

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

/* Write JSON telemetry (uploads, downloads, sync points, launch plan, peak device bytes). */
int64_t cinch_device_report_cstr(int64_t ctx_id, char *out, size_t out_cap);

/* Resident GPU batched 3x3 stencil (ping-pong) -> per-frame histogram in ONE command buffer. */
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

/* Additional resident GPU compute kernels. */
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
int64_t cinch_device_row_reduce_f32(int64_t ctx_id, int64_t mat_buf, int64_t m, int64_t n, int64_t mode);

#ifdef __cplusplus
}
#endif

#endif /* CINCH_DEVICE_H */
