/*
 * Unified benchmark worker comparing Cinch (Multicore CPU & Apple Metal GPU)
 * against Futhark 0.25.37 (c, multicore, opencl) on identical inputs and
 * matched numerical contracts.
 *
 * Each invocation runs ONE (backend, workload, size) pair in a fresh process
 * so getrusage(RUSAGE_SELF).ru_maxrss reflects true unpolluted peak RSS.
 */

#include <dlfcn.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/resource.h>
#include <time.h>

#include "../../examples/gpu-pipeline/cinch_device.h"

/* Additional Cinch packed CPU array runtime exports. */
extern void cinch_drop(int64_t word);
extern int64_t cinch_array_len(int64_t a);
extern int64_t cinch_array_from_raw(const uint8_t *src, int64_t n, int64_t format);
extern int64_t cinch_array_copy_out_raw(int64_t a, uint8_t *dst, size_t max_bytes);
extern int64_t cinch_array_dot_f32(int64_t a, int64_t b, int64_t float_mode);
extern int64_t cinch_array_scan_i32(int64_t a, int64_t op, int64_t inclusive);
extern int64_t cinch_array_filter_i32_gt(int64_t a, int64_t threshold);
extern int64_t cinch_array_histogram_u8(int64_t data, int64_t num_bins);
extern int64_t cinch_array_stencil2d_f32(
    int64_t data, int64_t frames, int64_t h, int64_t w,
    int64_t weights, int64_t edge, int64_t steps
);
extern int64_t cinch_array_stencil2d_u8(
    int64_t data, int64_t frames, int64_t h, int64_t w,
    int64_t weights, int64_t divisor, int64_t edge, int64_t steps
);
extern int64_t cinch_array_histogram_batch_u8(
    int64_t data, int64_t frames, int64_t h, int64_t w, int64_t num_bins
);
extern int64_t cinch_array_spmv_csr_f32(
    int64_t vals, int64_t cols, int64_t row_ptr, int64_t x, int64_t float_mode
);

/* Packed format tags matching seed/runtime/src/arrays.rs */
#define FMT_U8  257
#define FMT_I32 388
#define FMT_U32 260
#define FMT_F32 260

static double now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (double)ts.tv_sec * 1000.0 + (double)ts.tv_nsec / 1e6;
}

static int cmp_double(const void *a, const void *b) {
    double da = *(const double *)a;
    double db = *(const double *)b;
    return (da > db) - (da < db);
}

static double median(double *vals, int n) {
    qsort(vals, (size_t)n, sizeof(double), cmp_double);
    if (n % 2 == 1) return vals[n / 2];
    return 0.5 * (vals[n / 2 - 1] + vals[n / 2]);
}

static double peak_rss_mb(void) {
    struct rusage ru;
    getrusage(RUSAGE_SELF, &ru);
    /* On macOS ru_maxrss is in bytes. */
    return (double)ru.ru_maxrss / (1024.0 * 1024.0);
}

static uint64_t hash_u32_slice(const uint32_t *p, size_t n) {
    uint64_t h = 1469598103934665603ULL;
    for (size_t i = 0; i < n; i++) {
        h ^= (uint64_t)p[i];
        h *= 1099511628211ULL;
    }
    return h;
}

static uint64_t hash_i32_slice(const int32_t *p, size_t n) {
    return hash_u32_slice((const uint32_t *)p, n);
}

static double sum_f64_of_f32(const float *p, size_t n) {
    double s = 0.0;
    for (size_t i = 0; i < n; i++) s += (double)p[i];
    return s;
}

/* Deterministic PRNG (xorshift64*) for reproducible inputs. */
static uint64_t rng_state = 0x9e3779b97f4a7c15ULL;
static uint64_t rng_next(void) {
    uint64_t x = rng_state;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    rng_state = x;
    return x * 2685821657736338717ULL;
}

static void gen_f32_pair(size_t n, float **a_out, float **b_out) {
    float *a = (float *)malloc(n * sizeof(float));
    float *b = (float *)malloc(n * sizeof(float));
    rng_state = 0x123456789abcdef0ULL ^ (uint64_t)n;
    for (size_t i = 0; i < n; i++) {
        int32_t va = (int32_t)(rng_next() & 15) - 7;
        int32_t vb = (int32_t)(rng_next() & 15) - 7;
        a[i] = (float)va * 0.125f;
        b[i] = (float)vb * 0.125f;
    }
    *a_out = a;
    *b_out = b;
}

static void gen_i32_vec(size_t n, int32_t **out) {
    int32_t *v = (int32_t *)malloc(n * sizeof(int32_t));
    rng_state = 0xfeedfacecafebeefULL ^ (uint64_t)n;
    for (size_t i = 0; i < n; i++) {
        v[i] = (int32_t)(rng_next() % 2001) - 1000;
    }
    *out = v;
}

static void gen_u8_vec(size_t n, int skewed, uint8_t **out) {
    uint8_t *v = (uint8_t *)malloc(n);
    rng_state = (skewed ? 0xabcdef0123456789ULL : 0x0123456789abcdefULL) ^ (uint64_t)n;
    for (size_t i = 0; i < n; i++) {
        uint64_t r = rng_next();
        if (skewed && (r % 10) < 8) {
            v[i] = 42; /* 80% hot-bin contention on bin 42 */
        } else {
            v[i] = (uint8_t)(r & 0xFFu);
        }
    }
    *out = v;
}

static void gen_f32_image(size_t h, size_t w, float **out) {
    size_t n = h * w;
    float *img = (float *)malloc(n * sizeof(float));
    for (size_t i = 0; i < n; i++) {
        img[i] = (float)((i * 31u + (i >> 5) * 17u) & 255u) / 255.0f;
    }
    *out = img;
}

static void gen_u8_batch(size_t f, size_t h, size_t w, uint8_t **out) {
    size_t n = f * h * w;
    uint8_t *imgs = (uint8_t *)malloc(n);
    for (size_t i = 0; i < n; i++) {
        imgs[i] = (uint8_t)((i * 37u + (i >> 7) * 13u) & 0xFFu);
    }
    *out = imgs;
}

typedef struct {
    size_t rows;
    size_t ncols;
    size_t nnz;
    float *vals;
    int32_t *cols;
    uint32_t *row_ptr;
    float *x;
} CsrMatrix;

static CsrMatrix gen_csr(size_t rows, size_t avg_nnz_per_row) {
    CsrMatrix m;
    m.rows = rows;
    m.ncols = rows;
    m.row_ptr = (uint32_t *)malloc((rows + 1) * sizeof(uint32_t));
    rng_state = 0x55aa55aa11223344ULL ^ (uint64_t)rows;
    uint32_t cur = 0;
    for (size_t r = 0; r < rows; r++) {
        m.row_ptr[r] = cur;
        if (r % 10 == 0) {
            /* 10% explicitly empty rows! */
            continue;
        }
        size_t cnt = (r % 5 == 0) ? (avg_nnz_per_row * 2) : avg_nnz_per_row;
        if (cnt > m.ncols) cnt = m.ncols;
        cur += (uint32_t)cnt;
    }
    m.row_ptr[rows] = cur;
    m.nnz = cur;
    m.vals = (float *)malloc(m.nnz * sizeof(float));
    m.cols = (int32_t *)malloc(m.nnz * sizeof(int32_t));
    m.x = (float *)malloc(m.ncols * sizeof(float));
    for (size_t i = 0; i < m.ncols; i++) {
        m.x[i] = (float)((int32_t)(i % 17) - 8) * 0.25f;
    }
    for (size_t r = 0; r < rows; r++) {
        uint32_t s = m.row_ptr[r];
        uint32_t e = m.row_ptr[r + 1];
        for (uint32_t j = s; j < e; j++) {
            uint64_t rv = rng_next();
            m.cols[j] = (int32_t)((r + (j - s) * 131u + (rv & 63u)) % m.ncols);
            m.vals[j] = (float)((int32_t)(rv & 15) - 7) * 0.125f;
        }
    }
    return m;
}

static void free_csr(CsrMatrix *m) {
    free(m->vals);
    free(m->cols);
    free(m->row_ptr);
    free(m->x);
}

/* Futhark dynamically-loaded API */
typedef struct {
    void *handle;
    void *(*cfg_new)(void);
    void (*cfg_free)(void *);
    void (*cfg_set_threads)(void *, int);
    void *(*ctx_new)(void *);
    void (*ctx_free)(void *);
    int (*ctx_sync)(void *);
    char *(*ctx_get_error)(void *);

    void *(*new_f32_1d)(void *, const float *, int64_t);
    int (*free_f32_1d)(void *, void *);
    int (*vals_f32_1d)(void *, void *, float *);
    const int64_t *(*shape_f32_1d)(void *, void *);

    void *(*new_f32_2d)(void *, const float *, int64_t, int64_t);
    int (*free_f32_2d)(void *, void *);
    int (*vals_f32_2d)(void *, void *, float *);

    void *(*new_i32_1d)(void *, const int32_t *, int64_t);
    int (*free_i32_1d)(void *, void *);
    int (*vals_i32_1d)(void *, void *, int32_t *);
    const int64_t *(*shape_i32_1d)(void *, void *);

    void *(*new_u32_1d)(void *, const uint32_t *, int64_t);
    int (*free_u32_1d)(void *, void *);
    int (*vals_u32_1d)(void *, void *, uint32_t *);

    int (*free_u32_2d)(void *, void *);
    int (*vals_u32_2d)(void *, void *, uint32_t *);

    void *(*new_u8_1d)(void *, const uint8_t *, int64_t);
    int (*free_u8_1d)(void *, void *);

    void *(*new_u8_3d)(void *, const uint8_t *, int64_t, int64_t, int64_t);
    int (*free_u8_3d)(void *, void *);

    int (*e_dot_f32)(void *, float *, const void *, const void *);
    int (*e_scan_i32)(void *, void **, const void *);
    int (*e_filter_i32)(void *, void **, const void *, int32_t);
    int (*e_hist_u8)(void *, void **, const void *);
    int (*e_stencil_f32)(void *, void **, const void *, const void *, int64_t);
    int (*e_stencil_hist)(void *, void **, const void *, const void *, int32_t, int64_t);
    int (*e_spmv_csr)(void *, void **, const void *, const void *, const void *, const void *);
} FutApi;

static int load_fut(const char *path, FutApi *f) {
    memset(f, 0, sizeof(*f));
    f->handle = dlopen(path, RTLD_NOW);
    if (!f->handle) {
        fprintf(stderr, "dlopen(%s) failed: %s\n", path, dlerror());
        return -1;
    }
#define SYM(field, name) do { f->field = dlsym(f->handle, name); } while (0)
    SYM(cfg_new, "futhark_context_config_new");
    SYM(cfg_free, "futhark_context_config_free");
    SYM(cfg_set_threads, "futhark_context_config_set_num_threads");
    SYM(ctx_new, "futhark_context_new");
    SYM(ctx_free, "futhark_context_free");
    SYM(ctx_sync, "futhark_context_sync");
    SYM(ctx_get_error, "futhark_context_get_error");

    SYM(new_f32_1d, "futhark_new_f32_1d");
    SYM(free_f32_1d, "futhark_free_f32_1d");
    SYM(vals_f32_1d, "futhark_values_f32_1d");
    SYM(shape_f32_1d, "futhark_shape_f32_1d");

    SYM(new_f32_2d, "futhark_new_f32_2d");
    SYM(free_f32_2d, "futhark_free_f32_2d");
    SYM(vals_f32_2d, "futhark_values_f32_2d");

    SYM(new_i32_1d, "futhark_new_i32_1d");
    SYM(free_i32_1d, "futhark_free_i32_1d");
    SYM(vals_i32_1d, "futhark_values_i32_1d");
    SYM(shape_i32_1d, "futhark_shape_i32_1d");

    SYM(new_u32_1d, "futhark_new_u32_1d");
    SYM(free_u32_1d, "futhark_free_u32_1d");
    SYM(vals_u32_1d, "futhark_values_u32_1d");

    SYM(free_u32_2d, "futhark_free_u32_2d");
    SYM(vals_u32_2d, "futhark_values_u32_2d");

    SYM(new_u8_1d, "futhark_new_u8_1d");
    SYM(free_u8_1d, "futhark_free_u8_1d");

    SYM(new_u8_3d, "futhark_new_u8_3d");
    SYM(free_u8_3d, "futhark_free_u8_3d");

    SYM(e_dot_f32, "futhark_entry_dot_f32");
    SYM(e_scan_i32, "futhark_entry_scan_i32_incl");
    SYM(e_filter_i32, "futhark_entry_filter_i32_gt");
    SYM(e_hist_u8, "futhark_entry_histogram_u8");
    SYM(e_stencil_f32, "futhark_entry_stencil2d_f32");
    SYM(e_stencil_hist, "futhark_entry_stencil_histogram_batch_u8");
    SYM(e_spmv_csr, "futhark_entry_spmv_csr_f32");
#undef SYM
    return 0;
}

static uint64_t parse_peak_dev_bytes(int64_t dev) {
    char buf[8192];
    if (cinch_device_report_cstr(dev, buf, sizeof(buf)) < 0) return 0;
    const char *p = strstr(buf, "\"peak_device_bytes\":");
    if (!p) return 0;
    p += strlen("\"peak_device_bytes\":");
    return strtoull(p, NULL, 10);
}

int main(int argc, char **argv) {
    if (argc < 4) {
        fprintf(stderr, "usage: %s <backend> <workload> <size_arg> [lib_dir]\n", argv[0]);
        return 2;
    }
    const char *backend = argv[1];
    const char *workload = argv[2];
    const char *size_arg = argv[3];
    const char *lib_dir = (argc >= 5) ? argv[4] : "benchmarks/futhark-compare/build";

    const int WARMUP = 3;
    const int WARM_ITERS = 15;
    const int XFER_ITERS = 7;
    double warm_t[32];
    double xfer_t[32];
    double result_val = 0.0;
    uint64_t result_hash = 0;
    int64_t result_len = 0;
    uint64_t peak_dev_bytes = 0;
    const char *contract = "exact";
    char extra_note[256] = "";

    int is_fut = (strncmp(backend, "futhark-", 8) == 0);
    int is_cinch_cpu = (strcmp(backend, "cinch-cpu") == 0 || strcmp(backend, "cinch-cpu-exact") == 0);
    int is_cinch_metal = (strcmp(backend, "cinch-metal") == 0 || strcmp(backend, "cinch-metal-exact") == 0);
    int exact_float = (strstr(backend, "-exact") != NULL);

    FutApi fut;
    void *fcfg = NULL;
    void *fctx = NULL;
    int64_t dev = -1;

    if (is_fut) {
        char path[512];
        if (strcmp(backend, "futhark-c") == 0) snprintf(path, sizeof(path), "%s/libfut_c.dylib", lib_dir);
        else if (strcmp(backend, "futhark-mc") == 0) snprintf(path, sizeof(path), "%s/libfut_mc.dylib", lib_dir);
        else if (strcmp(backend, "futhark-ocl") == 0) snprintf(path, sizeof(path), "%s/libfut_ocl.dylib", lib_dir);
        else { fprintf(stderr, "unknown futhark backend %s\n", backend); return 2; }
        if (load_fut(path, &fut) != 0) return 3;
        fcfg = fut.cfg_new();
        fctx = fut.ctx_new(fcfg);
        if (!fctx) {
            fprintf(stderr, "futhark_context_new failed for %s\n", backend);
            return 4;
        }
        char *err = fut.ctx_get_error(fctx);
        if (err) {
            fprintf(stderr, "futhark init error (%s): %s\n", backend, err);
            free(err);
            return 5;
        }
    } else if (is_cinch_metal) {
        dev = cinch_device_open(CINCH_BACKEND_METAL_REQUIRED);
        if (dev < 0) {
            fprintf(stderr, "Metal GPU unavailable: %lld\n", (long long)dev);
            return 6;
        }
    } else if (!is_cinch_cpu) {
        fprintf(stderr, "unknown backend %s\n", backend);
        return 2;
    }

    /* ---------------- 1. DOT PRODUCT F32 ---------------- */
    if (strcmp(workload, "dot_f32") == 0) {
        size_t n = (size_t)atoll(size_arg);
        float *a = NULL, *b = NULL;
        gen_f32_pair(n, &a, &b);
        contract = exact_float ? "FloatExactTree (deterministic binary tree)" : "FloatRelaxed (associative f32 sum)";
        int64_t mode = exact_float ? CINCH_FLOAT_EXACT_TREE : CINCH_FLOAT_RELAXED;

        if (is_fut) {
            void *fa = fut.new_f32_1d(fctx, a, (int64_t)n);
            void *fb = fut.new_f32_1d(fctx, b, (int64_t)n);
            fut.ctx_sync(fctx);
            float out = 0.0f;
            for (int i = 0; i < WARMUP; i++) { fut.e_dot_f32(fctx, &out, fa, fb); fut.ctx_sync(fctx); }
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                fut.e_dot_f32(fctx, &out, fa, fb);
                fut.ctx_sync(fctx);
                warm_t[i] = now_ms() - t0;
            }
            fut.free_f32_1d(fctx, fa);
            fut.free_f32_1d(fctx, fb);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                void *ta = fut.new_f32_1d(fctx, a, (int64_t)n);
                void *tb = fut.new_f32_1d(fctx, b, (int64_t)n);
                fut.e_dot_f32(fctx, &out, ta, tb);
                fut.ctx_sync(fctx);
                fut.free_f32_1d(fctx, ta);
                fut.free_f32_1d(fctx, tb);
                xfer_t[i] = now_ms() - t0;
            }
            result_val = (double)out;
        } else if (is_cinch_cpu) {
            int64_t ca = cinch_array_from_raw((const uint8_t *)a, (int64_t)n, FMT_F32);
            int64_t cb = cinch_array_from_raw((const uint8_t *)b, (int64_t)n, FMT_F32);
            float out = 0.0f;
            for (int i = 0; i < WARMUP; i++) {
                uint32_t bits = (uint32_t)cinch_array_dot_f32(ca, cb, mode);
                memcpy(&out, &bits, 4);
            }
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                uint32_t bits = (uint32_t)cinch_array_dot_f32(ca, cb, mode);
                memcpy(&out, &bits, 4);
                warm_t[i] = now_ms() - t0;
            }
            cinch_drop(ca);
            cinch_drop(cb);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                int64_t ta = cinch_array_from_raw((const uint8_t *)a, (int64_t)n, FMT_F32);
                int64_t tb = cinch_array_from_raw((const uint8_t *)b, (int64_t)n, FMT_F32);
                uint32_t bits = (uint32_t)cinch_array_dot_f32(ta, tb, mode);
                memcpy(&out, &bits, 4);
                cinch_drop(ta);
                cinch_drop(tb);
                xfer_t[i] = now_ms() - t0;
            }
            result_val = (double)out;
        } else if (is_cinch_metal) {
            int64_t da = cinch_device_upload_raw(dev, (const uint8_t *)a, n * 4);
            int64_t db = cinch_device_upload_raw(dev, (const uint8_t *)b, n * 4);
            float out = 0.0f;
            for (int i = 0; i < WARMUP; i++) cinch_device_dot_f32(dev, da, db, (int64_t)n, mode, &out);
            cinch_device_reset_telemetry(dev);
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                cinch_device_dot_f32(dev, da, db, (int64_t)n, mode, &out);
                warm_t[i] = now_ms() - t0;
            }
            peak_dev_bytes = parse_peak_dev_bytes(dev);
            cinch_device_buffer_free(da);
            cinch_device_buffer_free(db);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                int64_t ta = cinch_device_upload_raw(dev, (const uint8_t *)a, n * 4);
                int64_t tb = cinch_device_upload_raw(dev, (const uint8_t *)b, n * 4);
                cinch_device_dot_f32(dev, ta, tb, (int64_t)n, mode, &out);
                cinch_device_buffer_free(ta);
                cinch_device_buffer_free(tb);
                xfer_t[i] = now_ms() - t0;
            }
            result_val = (double)out;
        }
        result_len = 1;
        free(a); free(b);
    }

    /* ---------------- 2. PREFIX SCAN I32 ---------------- */
    else if (strcmp(workload, "scan_i32") == 0) {
        size_t n = (size_t)atoll(size_arg);
        int32_t *xs = NULL;
        gen_i32_vec(n, &xs);
        int32_t *out_host = (int32_t *)malloc(n * sizeof(int32_t));
        contract = "I32 wrapping-add inclusive prefix scan";

        if (is_fut) {
            void *fx = fut.new_i32_1d(fctx, xs, (int64_t)n);
            fut.ctx_sync(fctx);
            void *fo = NULL;
            for (int i = 0; i < WARMUP; i++) { fut.e_scan_i32(fctx, &fo, fx); fut.ctx_sync(fctx); fut.free_i32_1d(fctx, fo); }
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                fut.e_scan_i32(fctx, &fo, fx);
                fut.ctx_sync(fctx);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) fut.vals_i32_1d(fctx, fo, out_host);
                fut.free_i32_1d(fctx, fo);
            }
            fut.free_i32_1d(fctx, fx);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                void *tx = fut.new_i32_1d(fctx, xs, (int64_t)n);
                void *to = NULL;
                fut.e_scan_i32(fctx, &to, tx);
                fut.vals_i32_1d(fctx, to, out_host);
                fut.ctx_sync(fctx);
                fut.free_i32_1d(fctx, tx);
                fut.free_i32_1d(fctx, to);
                xfer_t[i] = now_ms() - t0;
            }
        } else if (is_cinch_cpu) {
            int64_t cx = cinch_array_from_raw((const uint8_t *)xs, (int64_t)n, FMT_I32);
            for (int i = 0; i < WARMUP; i++) { int64_t o = cinch_array_scan_i32(cx, 0, 1); cinch_drop(o); }
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                int64_t o = cinch_array_scan_i32(cx, 0, 1);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) cinch_array_copy_out_raw(o, (uint8_t *)out_host, n * 4);
                cinch_drop(o);
            }
            cinch_drop(cx);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                int64_t tx = cinch_array_from_raw((const uint8_t *)xs, (int64_t)n, FMT_I32);
                int64_t o = cinch_array_scan_i32(tx, 0, 1);
                cinch_array_copy_out_raw(o, (uint8_t *)out_host, n * 4);
                cinch_drop(tx);
                cinch_drop(o);
                xfer_t[i] = now_ms() - t0;
            }
        } else if (is_cinch_metal) {
            int64_t dx = cinch_device_upload_raw(dev, (const uint8_t *)xs, n * 4);
            for (int i = 0; i < WARMUP; i++) { int64_t o = cinch_device_scan_i32(dev, dx, (int64_t)n, 1); cinch_device_buffer_free(o); }
            cinch_device_reset_telemetry(dev);
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                int64_t o = cinch_device_scan_i32(dev, dx, (int64_t)n, 1);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) {
                    peak_dev_bytes = parse_peak_dev_bytes(dev);
                    cinch_device_download_raw(dev, o, (uint8_t *)out_host, n * 4);
                }
                cinch_device_buffer_free(o);
            }
            cinch_device_buffer_free(dx);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                int64_t tx = cinch_device_upload_raw(dev, (const uint8_t *)xs, n * 4);
                int64_t o = cinch_device_scan_i32(dev, tx, (int64_t)n, 1);
                cinch_device_download_raw(dev, o, (uint8_t *)out_host, n * 4);
                cinch_device_buffer_free(tx);
                cinch_device_buffer_free(o);
                xfer_t[i] = now_ms() - t0;
            }
        }
        result_hash = hash_i32_slice(out_host, n);
        result_val = (double)out_host[n - 1];
        result_len = (int64_t)n;
        free(xs); free(out_host);
    }

    /* ---------------- 3. STABLE FILTER / COMPACTION I32 ---------------- */
    else if (strcmp(workload, "filter_i32") == 0) {
        size_t n = (size_t)atoll(size_arg);
        int32_t *xs = NULL;
        gen_i32_vec(n, &xs);
        int32_t *out_host = (int32_t *)malloc(n * sizeof(int32_t));
        const int32_t thr = 250;
        contract = "I32 stable compaction (x > 250)";

        if (is_fut) {
            void *fx = fut.new_i32_1d(fctx, xs, (int64_t)n);
            fut.ctx_sync(fctx);
            void *fo = NULL;
            for (int i = 0; i < WARMUP; i++) { fut.e_filter_i32(fctx, &fo, fx, thr); fut.ctx_sync(fctx); fut.free_i32_1d(fctx, fo); }
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                fut.e_filter_i32(fctx, &fo, fx, thr);
                fut.ctx_sync(fctx);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) {
                    result_len = fut.shape_i32_1d(fctx, fo)[0];
                    fut.vals_i32_1d(fctx, fo, out_host);
                }
                fut.free_i32_1d(fctx, fo);
            }
            fut.free_i32_1d(fctx, fx);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                void *tx = fut.new_i32_1d(fctx, xs, (int64_t)n);
                void *to = NULL;
                fut.e_filter_i32(fctx, &to, tx, thr);
                fut.vals_i32_1d(fctx, to, out_host);
                fut.ctx_sync(fctx);
                fut.free_i32_1d(fctx, tx);
                fut.free_i32_1d(fctx, to);
                xfer_t[i] = now_ms() - t0;
            }
        } else if (is_cinch_cpu) {
            int64_t cx = cinch_array_from_raw((const uint8_t *)xs, (int64_t)n, FMT_I32);
            for (int i = 0; i < WARMUP; i++) { int64_t o = cinch_array_filter_i32_gt(cx, thr); cinch_drop(o); }
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                int64_t o = cinch_array_filter_i32_gt(cx, thr);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) {
                    result_len = cinch_array_len(o);
                    cinch_array_copy_out_raw(o, (uint8_t *)out_host, (size_t)result_len * 4);
                }
                cinch_drop(o);
            }
            cinch_drop(cx);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                int64_t tx = cinch_array_from_raw((const uint8_t *)xs, (int64_t)n, FMT_I32);
                int64_t o = cinch_array_filter_i32_gt(tx, thr);
                int64_t k = cinch_array_len(o);
                cinch_array_copy_out_raw(o, (uint8_t *)out_host, (size_t)k * 4);
                cinch_drop(tx);
                cinch_drop(o);
                xfer_t[i] = now_ms() - t0;
            }
        } else if (is_cinch_metal) {
            int64_t dx = cinch_device_upload_raw(dev, (const uint8_t *)xs, n * 4);
            int64_t k = 0;
            for (int i = 0; i < WARMUP; i++) { int64_t o = cinch_device_filter_i32_gt(dev, dx, (int64_t)n, thr, &k); cinch_device_buffer_free(o); }
            cinch_device_reset_telemetry(dev);
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                int64_t o = cinch_device_filter_i32_gt(dev, dx, (int64_t)n, thr, &k);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) {
                    result_len = k;
                    peak_dev_bytes = parse_peak_dev_bytes(dev);
                    cinch_device_download_raw(dev, o, (uint8_t *)out_host, (size_t)k * 4);
                }
                cinch_device_buffer_free(o);
            }
            cinch_device_buffer_free(dx);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                int64_t tx = cinch_device_upload_raw(dev, (const uint8_t *)xs, n * 4);
                int64_t o = cinch_device_filter_i32_gt(dev, tx, (int64_t)n, thr, &k);
                cinch_device_download_raw(dev, o, (uint8_t *)out_host, (size_t)k * 4);
                cinch_device_buffer_free(tx);
                cinch_device_buffer_free(o);
                xfer_t[i] = now_ms() - t0;
            }
        }
        result_hash = hash_i32_slice(out_host, (size_t)result_len);
        result_val = (double)result_len;
        free(xs); free(out_host);
    }

    /* ---------------- 4. HISTOGRAM U8 (UNIFORM OR SKEWED) ---------------- */
    else if (strcmp(workload, "hist_uniform") == 0 || strcmp(workload, "hist_skewed") == 0) {
        size_t n = (size_t)atoll(size_arg);
        int skewed = (strcmp(workload, "hist_skewed") == 0);
        uint8_t *xs = NULL;
        gen_u8_vec(n, skewed, &xs);
        uint32_t out_host[256];
        memset(out_host, 0, sizeof(out_host));
        const int32_t dummy_w[9] = {0,0,0,0,1,0,0,0,0};
        contract = skewed ? "U32 256-bin histogram (80% hot-bin contention)" : "U32 256-bin histogram (uniform)";

        if (is_fut) {
            void *fx = fut.new_u8_1d(fctx, xs, (int64_t)n);
            fut.ctx_sync(fctx);
            void *fo = NULL;
            for (int i = 0; i < WARMUP; i++) { fut.e_hist_u8(fctx, &fo, fx); fut.ctx_sync(fctx); fut.free_u32_1d(fctx, fo); }
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                fut.e_hist_u8(fctx, &fo, fx);
                fut.ctx_sync(fctx);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) fut.vals_u32_1d(fctx, fo, out_host);
                fut.free_u32_1d(fctx, fo);
            }
            fut.free_u8_1d(fctx, fx);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                void *tx = fut.new_u8_1d(fctx, xs, (int64_t)n);
                void *to = NULL;
                fut.e_hist_u8(fctx, &to, tx);
                fut.vals_u32_1d(fctx, to, out_host);
                fut.ctx_sync(fctx);
                fut.free_u8_1d(fctx, tx);
                fut.free_u32_1d(fctx, to);
                xfer_t[i] = now_ms() - t0;
            }
        } else if (is_cinch_cpu) {
            int64_t cx = cinch_array_from_raw(xs, (int64_t)n, FMT_U8);
            for (int i = 0; i < WARMUP; i++) { int64_t o = cinch_array_histogram_u8(cx, 256); cinch_drop(o); }
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                int64_t o = cinch_array_histogram_u8(cx, 256);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) cinch_array_copy_out_raw(o, (uint8_t *)out_host, 256 * 4);
                cinch_drop(o);
            }
            cinch_drop(cx);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                int64_t tx = cinch_array_from_raw(xs, (int64_t)n, FMT_U8);
                int64_t o = cinch_array_histogram_u8(tx, 256);
                cinch_array_copy_out_raw(o, (uint8_t *)out_host, 256 * 4);
                cinch_drop(tx);
                cinch_drop(o);
                xfer_t[i] = now_ms() - t0;
            }
        } else if (is_cinch_metal) {
            int64_t dx = cinch_device_upload_raw(dev, xs, n);
            for (int i = 0; i < WARMUP; i++) {
                int64_t o = cinch_device_stencil_histogram_u8(dev, dx, 1, 1, (int64_t)n, dummy_w, 1, 0, 0, 256);
                cinch_device_buffer_free(o);
            }
            cinch_device_reset_telemetry(dev);
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                int64_t o = cinch_device_stencil_histogram_u8(dev, dx, 1, 1, (int64_t)n, dummy_w, 1, 0, 0, 256);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) {
                    peak_dev_bytes = parse_peak_dev_bytes(dev);
                    cinch_device_download_raw(dev, o, (uint8_t *)out_host, 256 * 4);
                }
                cinch_device_buffer_free(o);
            }
            cinch_device_buffer_free(dx);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                int64_t tx = cinch_device_upload_raw(dev, xs, n);
                int64_t o = cinch_device_stencil_histogram_u8(dev, tx, 1, 1, (int64_t)n, dummy_w, 1, 0, 0, 256);
                cinch_device_download_raw(dev, o, (uint8_t *)out_host, 256 * 4);
                cinch_device_buffer_free(tx);
                cinch_device_buffer_free(o);
                xfer_t[i] = now_ms() - t0;
            }
        }
        result_hash = hash_u32_slice(out_host, 256);
        result_val = (double)out_host[42];
        result_len = 256;
        free(xs);
    }

    /* ---------------- 5. 2D 3x3 F32 STENCIL (3 STEPS) ---------------- */
    else if (strcmp(workload, "stencil_f32") == 0) {
        size_t side = (size_t)atoll(size_arg);
        size_t h = side, w = side, n = h * w;
        const int64_t steps = 3;
        const float ws[9] = {
            0.0625f, 0.125f, 0.0625f,
            0.125f,  0.25f,  0.125f,
            0.0625f, 0.125f, 0.0625f
        };
        float *img = NULL;
        gen_f32_image(h, w, &img);
        float *out_host = (float *)malloc(n * sizeof(float));
        contract = "F32 3x3 Gaussian stencil, EdgeClamp, 3 ping-pong steps";

        if (is_fut) {
            void *fi = fut.new_f32_2d(fctx, img, (int64_t)h, (int64_t)w);
            void *fw = fut.new_f32_1d(fctx, ws, 9);
            fut.ctx_sync(fctx);
            void *fo = NULL;
            for (int i = 0; i < WARMUP; i++) { fut.e_stencil_f32(fctx, &fo, fi, fw, steps); fut.ctx_sync(fctx); fut.free_f32_2d(fctx, fo); }
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                fut.e_stencil_f32(fctx, &fo, fi, fw, steps);
                fut.ctx_sync(fctx);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) fut.vals_f32_2d(fctx, fo, out_host);
                fut.free_f32_2d(fctx, fo);
            }
            fut.free_f32_2d(fctx, fi);
            fut.free_f32_1d(fctx, fw);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                void *ti = fut.new_f32_2d(fctx, img, (int64_t)h, (int64_t)w);
                void *tw = fut.new_f32_1d(fctx, ws, 9);
                void *to = NULL;
                fut.e_stencil_f32(fctx, &to, ti, tw, steps);
                fut.vals_f32_2d(fctx, to, out_host);
                fut.ctx_sync(fctx);
                fut.free_f32_2d(fctx, ti);
                fut.free_f32_1d(fctx, tw);
                fut.free_f32_2d(fctx, to);
                xfer_t[i] = now_ms() - t0;
            }
        } else if (is_cinch_cpu) {
            int64_t ci = cinch_array_from_raw((const uint8_t *)img, (int64_t)n, FMT_F32);
            int64_t cw = cinch_array_from_raw((const uint8_t *)ws, 9, FMT_F32);
            for (int i = 0; i < WARMUP; i++) { int64_t o = cinch_array_stencil2d_f32(ci, 1, (int64_t)h, (int64_t)w, cw, 0, steps); cinch_drop(o); }
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                int64_t o = cinch_array_stencil2d_f32(ci, 1, (int64_t)h, (int64_t)w, cw, 0, steps);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) cinch_array_copy_out_raw(o, (uint8_t *)out_host, n * 4);
                cinch_drop(o);
            }
            cinch_drop(ci); cinch_drop(cw);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                int64_t ti = cinch_array_from_raw((const uint8_t *)img, (int64_t)n, FMT_F32);
                int64_t tw = cinch_array_from_raw((const uint8_t *)ws, 9, FMT_F32);
                int64_t o = cinch_array_stencil2d_f32(ti, 1, (int64_t)h, (int64_t)w, tw, 0, steps);
                cinch_array_copy_out_raw(o, (uint8_t *)out_host, n * 4);
                cinch_drop(ti); cinch_drop(tw); cinch_drop(o);
                xfer_t[i] = now_ms() - t0;
            }
        } else if (is_cinch_metal) {
            int64_t di = cinch_device_upload_raw(dev, (const uint8_t *)img, n * 4);
            for (int i = 0; i < WARMUP; i++) { int64_t o = cinch_device_stencil2d_f32(dev, di, 1, (int64_t)h, (int64_t)w, ws, 0, steps); cinch_device_buffer_free(o); }
            cinch_device_reset_telemetry(dev);
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                int64_t o = cinch_device_stencil2d_f32(dev, di, 1, (int64_t)h, (int64_t)w, ws, 0, steps);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) {
                    peak_dev_bytes = parse_peak_dev_bytes(dev);
                    cinch_device_download_raw(dev, o, (uint8_t *)out_host, n * 4);
                }
                cinch_device_buffer_free(o);
            }
            cinch_device_buffer_free(di);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                int64_t ti = cinch_device_upload_raw(dev, (const uint8_t *)img, n * 4);
                int64_t o = cinch_device_stencil2d_f32(dev, ti, 1, (int64_t)h, (int64_t)w, ws, 0, steps);
                cinch_device_download_raw(dev, o, (uint8_t *)out_host, n * 4);
                cinch_device_buffer_free(ti); cinch_device_buffer_free(o);
                xfer_t[i] = now_ms() - t0;
            }
        }
        result_val = sum_f64_of_f32(out_host, n);
        result_len = (int64_t)n;
        free(img); free(out_host);
    }

    /* ---------------- 6. TARGET PIPELINE: BATCHED STENCIL -> HISTOGRAM ---------------- */
    else if (strcmp(workload, "stencil_hist_many") == 0 || strcmp(workload, "stencil_hist_few") == 0) {
        size_t frames, h, w;
        if (strcmp(workload, "stencil_hist_many") == 0) { frames = 1024; h = 64; w = 64; }
        else { frames = 2; h = 2048; w = 2048; }
        const int64_t steps = 3;
        const int32_t ws[9] = {1, 2, 1, 2, 4, 2, 1, 2, 1};
        const int32_t divisor = 16;
        size_t total = frames * h * w;
        size_t hist_n = frames * 256;
        uint8_t *imgs = NULL;
        gen_u8_batch(frames, h, w, &imgs);
        uint32_t *out_host = (uint32_t *)calloc(hist_n, sizeof(uint32_t));
        contract = "U8 3-step 3x3 EdgeClamp stencil -> per-frame 256-bin U32 histogram (1 upload, 1 download)";

        if (is_fut) {
            void *fi = fut.new_u8_3d(fctx, imgs, (int64_t)frames, (int64_t)h, (int64_t)w);
            void *fw = fut.new_i32_1d(fctx, ws, 9);
            fut.ctx_sync(fctx);
            void *fo = NULL;
            for (int i = 0; i < WARMUP; i++) { fut.e_stencil_hist(fctx, &fo, fi, fw, divisor, steps); fut.ctx_sync(fctx); fut.free_u32_2d(fctx, fo); }
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                fut.e_stencil_hist(fctx, &fo, fi, fw, divisor, steps);
                fut.ctx_sync(fctx);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) fut.vals_u32_2d(fctx, fo, out_host);
                fut.free_u32_2d(fctx, fo);
            }
            fut.free_u8_3d(fctx, fi);
            fut.free_i32_1d(fctx, fw);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                void *ti = fut.new_u8_3d(fctx, imgs, (int64_t)frames, (int64_t)h, (int64_t)w);
                void *tw = fut.new_i32_1d(fctx, ws, 9);
                void *to = NULL;
                fut.e_stencil_hist(fctx, &to, ti, tw, divisor, steps);
                fut.vals_u32_2d(fctx, to, out_host);
                fut.ctx_sync(fctx);
                fut.free_u8_3d(fctx, ti);
                fut.free_i32_1d(fctx, tw);
                fut.free_u32_2d(fctx, to);
                xfer_t[i] = now_ms() - t0;
            }
        } else if (is_cinch_cpu) {
            int64_t ci = cinch_array_from_raw(imgs, (int64_t)total, FMT_U8);
            int64_t cw = cinch_array_from_raw((const uint8_t *)ws, 9, FMT_I32);
            for (int i = 0; i < WARMUP; i++) {
                int64_t bl = cinch_array_stencil2d_u8(ci, (int64_t)frames, (int64_t)h, (int64_t)w, cw, divisor, 0, steps);
                int64_t hi = cinch_array_histogram_batch_u8(bl, (int64_t)frames, (int64_t)h, (int64_t)w, 256);
                cinch_drop(bl); cinch_drop(hi);
            }
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                int64_t bl = cinch_array_stencil2d_u8(ci, (int64_t)frames, (int64_t)h, (int64_t)w, cw, divisor, 0, steps);
                int64_t hi = cinch_array_histogram_batch_u8(bl, (int64_t)frames, (int64_t)h, (int64_t)w, 256);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) cinch_array_copy_out_raw(hi, (uint8_t *)out_host, hist_n * 4);
                cinch_drop(bl); cinch_drop(hi);
            }
            cinch_drop(ci); cinch_drop(cw);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                int64_t ti = cinch_array_from_raw(imgs, (int64_t)total, FMT_U8);
                int64_t tw = cinch_array_from_raw((const uint8_t *)ws, 9, FMT_I32);
                int64_t bl = cinch_array_stencil2d_u8(ti, (int64_t)frames, (int64_t)h, (int64_t)w, tw, divisor, 0, steps);
                int64_t hi = cinch_array_histogram_batch_u8(bl, (int64_t)frames, (int64_t)h, (int64_t)w, 256);
                cinch_array_copy_out_raw(hi, (uint8_t *)out_host, hist_n * 4);
                cinch_drop(ti); cinch_drop(tw); cinch_drop(bl); cinch_drop(hi);
                xfer_t[i] = now_ms() - t0;
            }
        } else if (is_cinch_metal) {
            int64_t di = cinch_device_upload_raw(dev, imgs, total);
            for (int i = 0; i < WARMUP; i++) {
                int64_t o = cinch_device_stencil_histogram_u8(dev, di, (int64_t)frames, (int64_t)h, (int64_t)w, ws, divisor, 0, steps, 256);
                cinch_device_buffer_free(o);
            }
            cinch_device_reset_telemetry(dev);
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                int64_t o = cinch_device_stencil_histogram_u8(dev, di, (int64_t)frames, (int64_t)h, (int64_t)w, ws, divisor, 0, steps, 256);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) {
                    peak_dev_bytes = parse_peak_dev_bytes(dev);
                    cinch_device_download_raw(dev, o, (uint8_t *)out_host, hist_n * 4);
                }
                cinch_device_buffer_free(o);
            }
            cinch_device_buffer_free(di);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                int64_t ti = cinch_device_upload_raw(dev, imgs, total);
                int64_t o = cinch_device_stencil_histogram_u8(dev, ti, (int64_t)frames, (int64_t)h, (int64_t)w, ws, divisor, 0, steps, 256);
                cinch_device_download_raw(dev, o, (uint8_t *)out_host, hist_n * 4);
                cinch_device_buffer_free(ti); cinch_device_buffer_free(o);
                xfer_t[i] = now_ms() - t0;
            }
        }
        result_hash = hash_u32_slice(out_host, hist_n);
        result_val = (double)out_host[0];
        result_len = (int64_t)hist_n;
        free(imgs); free(out_host);
    }

    /* ---------------- 7. CSR SPMV F32 (WITH EMPTY ROWS) ---------------- */
    else if (strcmp(workload, "spmv_csr") == 0) {
        size_t rows = (size_t)atoll(size_arg);
        CsrMatrix m = gen_csr(rows, 16);
        float *out_host = (float *)calloc(m.rows, sizeof(float));
        contract = exact_float ? "F32 CSR SpMV w/ 10% empty rows (FloatExactTree)" : "F32 CSR SpMV w/ 10% empty rows (FloatRelaxed)";
        int64_t mode = exact_float ? CINCH_FLOAT_EXACT_TREE : CINCH_FLOAT_RELAXED;
        snprintf(extra_note, sizeof(extra_note), "rows=%zu,nnz=%zu", m.rows, m.nnz);

        if (is_fut) {
            void *fv = fut.new_f32_1d(fctx, m.vals, (int64_t)m.nnz);
            void *fc = fut.new_i32_1d(fctx, m.cols, (int64_t)m.nnz);
            void *fp = fut.new_u32_1d(fctx, m.row_ptr, (int64_t)(m.rows + 1));
            void *fx = fut.new_f32_1d(fctx, m.x, (int64_t)m.ncols);
            fut.ctx_sync(fctx);
            void *fo = NULL;
            for (int i = 0; i < WARMUP; i++) { fut.e_spmv_csr(fctx, &fo, fv, fc, fp, fx); fut.ctx_sync(fctx); fut.free_f32_1d(fctx, fo); }
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                fut.e_spmv_csr(fctx, &fo, fv, fc, fp, fx);
                fut.ctx_sync(fctx);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) fut.vals_f32_1d(fctx, fo, out_host);
                fut.free_f32_1d(fctx, fo);
            }
            fut.free_f32_1d(fctx, fv); fut.free_i32_1d(fctx, fc); fut.free_u32_1d(fctx, fp); fut.free_f32_1d(fctx, fx);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                void *tv = fut.new_f32_1d(fctx, m.vals, (int64_t)m.nnz);
                void *tc = fut.new_i32_1d(fctx, m.cols, (int64_t)m.nnz);
                void *tp = fut.new_u32_1d(fctx, m.row_ptr, (int64_t)(m.rows + 1));
                void *tx = fut.new_f32_1d(fctx, m.x, (int64_t)m.ncols);
                void *to = NULL;
                fut.e_spmv_csr(fctx, &to, tv, tc, tp, tx);
                fut.vals_f32_1d(fctx, to, out_host);
                fut.ctx_sync(fctx);
                fut.free_f32_1d(fctx, tv); fut.free_i32_1d(fctx, tc); fut.free_u32_1d(fctx, tp); fut.free_f32_1d(fctx, tx); fut.free_f32_1d(fctx, to);
                xfer_t[i] = now_ms() - t0;
            }
        } else if (is_cinch_cpu) {
            int64_t cv = cinch_array_from_raw((const uint8_t *)m.vals, (int64_t)m.nnz, FMT_F32);
            int64_t cc = cinch_array_from_raw((const uint8_t *)m.cols, (int64_t)m.nnz, FMT_I32);
            int64_t cp = cinch_array_from_raw((const uint8_t *)m.row_ptr, (int64_t)(m.rows + 1), FMT_U32);
            int64_t cx = cinch_array_from_raw((const uint8_t *)m.x, (int64_t)m.ncols, FMT_F32);
            for (int i = 0; i < WARMUP; i++) { int64_t o = cinch_array_spmv_csr_f32(cv, cc, cp, cx, mode); cinch_drop(o); }
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                int64_t o = cinch_array_spmv_csr_f32(cv, cc, cp, cx, mode);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) cinch_array_copy_out_raw(o, (uint8_t *)out_host, m.rows * 4);
                cinch_drop(o);
            }
            cinch_drop(cv); cinch_drop(cc); cinch_drop(cp); cinch_drop(cx);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                int64_t tv = cinch_array_from_raw((const uint8_t *)m.vals, (int64_t)m.nnz, FMT_F32);
                int64_t tc = cinch_array_from_raw((const uint8_t *)m.cols, (int64_t)m.nnz, FMT_I32);
                int64_t tp = cinch_array_from_raw((const uint8_t *)m.row_ptr, (int64_t)(m.rows + 1), FMT_U32);
                int64_t tx = cinch_array_from_raw((const uint8_t *)m.x, (int64_t)m.ncols, FMT_F32);
                int64_t o = cinch_array_spmv_csr_f32(tv, tc, tp, tx, mode);
                cinch_array_copy_out_raw(o, (uint8_t *)out_host, m.rows * 4);
                cinch_drop(tv); cinch_drop(tc); cinch_drop(tp); cinch_drop(tx); cinch_drop(o);
                xfer_t[i] = now_ms() - t0;
            }
        } else if (is_cinch_metal) {
            int64_t dv = cinch_device_upload_raw(dev, (const uint8_t *)m.vals, m.nnz * 4);
            int64_t dc = cinch_device_upload_raw(dev, (const uint8_t *)m.cols, m.nnz * 4);
            int64_t dp = cinch_device_upload_raw(dev, (const uint8_t *)m.row_ptr, (m.rows + 1) * 4);
            int64_t dx = cinch_device_upload_raw(dev, (const uint8_t *)m.x, m.ncols * 4);
            for (int i = 0; i < WARMUP; i++) { int64_t o = cinch_device_spmv_csr_f32(dev, dv, dc, dp, dx, (int64_t)m.rows, (int64_t)m.nnz, mode); cinch_device_buffer_free(o); }
            cinch_device_reset_telemetry(dev);
            for (int i = 0; i < WARM_ITERS; i++) {
                double t0 = now_ms();
                int64_t o = cinch_device_spmv_csr_f32(dev, dv, dc, dp, dx, (int64_t)m.rows, (int64_t)m.nnz, mode);
                warm_t[i] = now_ms() - t0;
                if (i == WARM_ITERS - 1) {
                    peak_dev_bytes = parse_peak_dev_bytes(dev);
                    cinch_device_download_raw(dev, o, (uint8_t *)out_host, m.rows * 4);
                }
                cinch_device_buffer_free(o);
            }
            cinch_device_buffer_free(dv); cinch_device_buffer_free(dc); cinch_device_buffer_free(dp); cinch_device_buffer_free(dx);
            for (int i = 0; i < XFER_ITERS; i++) {
                double t0 = now_ms();
                int64_t tv = cinch_device_upload_raw(dev, (const uint8_t *)m.vals, m.nnz * 4);
                int64_t tc = cinch_device_upload_raw(dev, (const uint8_t *)m.cols, m.nnz * 4);
                int64_t tp = cinch_device_upload_raw(dev, (const uint8_t *)m.row_ptr, (m.rows + 1) * 4);
                int64_t tx = cinch_device_upload_raw(dev, (const uint8_t *)m.x, m.ncols * 4);
                int64_t o = cinch_device_spmv_csr_f32(dev, tv, tc, tp, tx, (int64_t)m.rows, (int64_t)m.nnz, mode);
                cinch_device_download_raw(dev, o, (uint8_t *)out_host, m.rows * 4);
                cinch_device_buffer_free(tv); cinch_device_buffer_free(tc); cinch_device_buffer_free(tp); cinch_device_buffer_free(tx); cinch_device_buffer_free(o);
                xfer_t[i] = now_ms() - t0;
            }
        }
        /* Verify empty rows produced exact 0.0f. */
        for (size_t r = 0; r < m.rows; r += 10) {
            if (out_host[r] != 0.0f) {
                fprintf(stderr, "empty row %zu non-zero: %g\n", r, out_host[r]);
                return 9;
            }
        }
        result_val = sum_f64_of_f32(out_host, m.rows);
        result_len = (int64_t)m.rows;
        free_csr(&m);
        free(out_host);
    } else {
        fprintf(stderr, "unknown workload %s\n", workload);
        return 2;
    }

    double warm_ms = median(warm_t, WARM_ITERS);
    double xfer_ms = median(xfer_t, XFER_ITERS);
    double rss_mb = peak_rss_mb();

    if (is_fut) {
        fut.ctx_free(fctx);
        fut.cfg_free(fcfg);
        dlclose(fut.handle);
    } else if (is_cinch_metal) {
        cinch_device_close(dev);
    }

    printf("{\"backend\":\"%s\",\"workload\":\"%s\",\"size\":\"%s\",\"contract\":\"%s\",\"note\":\"%s\","
           "\"warm_ms\":%.4f,\"xfer_ms\":%.4f,\"peak_rss_mb\":%.2f,\"peak_dev_mb\":%.2f,"
           "\"result_val\":%.6f,\"result_hash\":\"%016llx\",\"result_len\":%lld}\n",
           backend, workload, size_arg, contract, extra_note,
           warm_ms, xfer_ms, rss_mb, (double)peak_dev_bytes / (1024.0 * 1024.0),
           result_val, (unsigned long long)result_hash, (long long)result_len);
    return 0;
}
