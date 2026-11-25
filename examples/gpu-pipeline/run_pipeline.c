#include "cinch_device.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

static double now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (double)ts.tv_sec * 1000.0 + (double)ts.tv_nsec / 1e6;
}

static int clamp_idx(int x, int n) {
    if (x < 0) return 0;
    if (x >= n) return n - 1;
    return x;
}

/* Scalar C reference for exact verification of the GPU pipeline. */
static void cpu_ref_stencil_histogram(
    const uint8_t *in_frames,
    size_t frames,
    size_t h,
    size_t w,
    const int32_t weights[9],
    int32_t divisor,
    size_t steps,
    size_t bins,
    uint32_t *out_hist
) {
    size_t fp = h * w;
    uint8_t *buf_a = (uint8_t *)malloc(fp);
    uint8_t *buf_b = (uint8_t *)malloc(fp);
    memset(out_hist, 0, frames * bins * sizeof(uint32_t));
    for (size_t f = 0; f < frames; f++) {
        memcpy(buf_a, in_frames + f * fp, fp);
        uint8_t *src = buf_a;
        uint8_t *dst = buf_b;
        for (size_t s = 0; s < steps; s++) {
            for (size_t y = 0; y < h; y++) {
                for (size_t x = 0; x < w; x++) {
                    int32_t acc = 0;
                    for (int dy = -1; dy <= 1; dy++) {
                        int yy = clamp_idx((int)y + dy, (int)h);
                        for (int dx = -1; dx <= 1; dx++) {
                            int xx = clamp_idx((int)x + dx, (int)w);
                            acc += (int32_t)src[(size_t)yy * w + (size_t)xx]
                                 * weights[(dy + 1) * 3 + (dx + 1)];
                        }
                    }
                    int32_t v = acc / divisor;
                    if (v < 0) v = 0;
                    if (v > 255) v = 255;
                    dst[y * w + x] = (uint8_t)v;
                }
            }
            uint8_t *tmp = src;
            src = dst;
            dst = tmp;
        }
        uint32_t *fh = out_hist + f * bins;
        for (size_t i = 0; i < fp; i++) {
            if (src[i] < bins) fh[src[i]]++;
        }
    }
    free(buf_a);
    free(buf_b);
}

static int run_shape(
    int64_t dev,
    const char *label,
    size_t frames,
    size_t h,
    size_t w,
    size_t steps,
    const char *expected_schedule,
    int verify_full
) {
    const int32_t weights[9] = {1, 2, 1, 2, 4, 2, 1, 2, 1};
    const int32_t divisor = 16;
    const size_t bins = 256;
    const size_t total_pixels = frames * h * w;
    const size_t hist_elems = frames * bins;

    uint8_t *imgs = (uint8_t *)malloc(total_pixels);
    uint32_t *gpu_hist = (uint32_t *)calloc(hist_elems, sizeof(uint32_t));
    if (!imgs || !gpu_hist) {
        fprintf(stderr, "host allocation failed\n");
        return 1;
    }
    for (size_t i = 0; i < total_pixels; i++) {
        imgs[i] = (uint8_t)((i * 37u + (i >> 7) * 13u) & 0xFFu);
    }

    cinch_device_reset_telemetry(dev);

    double t0 = now_ms();
    int64_t d_in = cinch_device_upload_raw(dev, imgs, total_pixels);
    if (d_in < 0) {
        fprintf(stderr, "[%s] upload failed: %lld\n", label, (long long)d_in);
        return 2;
    }
    double t_up = now_ms();

    /* Warmup + timed resident execution (single command buffer, zero host copies). */
    int64_t d_hist = cinch_device_stencil_histogram_u8(
        dev, d_in, (int64_t)frames, (int64_t)h, (int64_t)w,
        weights, divisor, CINCH_EDGE_CLAMP, (int64_t)steps, (int64_t)bins
    );
    if (d_hist < 0) {
        fprintf(stderr, "[%s] GPU pipeline failed: %lld\n", label, (long long)d_hist);
        return 3;
    }
    double t_kern = now_ms();

    int64_t rc = cinch_device_download_raw(dev, d_hist, (uint8_t *)gpu_hist, hist_elems * sizeof(uint32_t));
    if (rc != CINCH_DEV_OK) {
        fprintf(stderr, "[%s] download failed: %lld\n", label, (long long)rc);
        return 4;
    }
    double t_done = now_ms();

    char report[8192];
    cinch_device_report_cstr(dev, report, sizeof(report));

    cinch_device_buffer_free(d_in);
    cinch_device_buffer_free(d_hist);

    printf("=== %s (%zux%zux%zu, %zu stencil steps -> %zux%zu histogram) ===\n",
           label, frames, h, w, steps, frames, bins);
    printf("  upload=%.3f ms  resident_gpu=%.3f ms  download=%.3f ms  total=%.3f ms\n",
           t_up - t0, t_kern - t_up, t_done - t_kern, t_done - t0);
    printf("  telemetry: %s\n", report);

    if (!strstr(report, "\"uploads\":1") || !strstr(report, "\"downloads\":1")) {
        fprintf(stderr, "[%s] expected exactly 1 upload and 1 download!\n", label);
        return 5;
    }
    if (!strstr(report, expected_schedule)) {
        fprintf(stderr, "[%s] expected launch plan '%s'!\n", label, expected_schedule);
        return 6;
    }

    /* Verify total pixel count per frame and exact bin counts against C reference. */
    for (size_t f = 0; f < frames; f++) {
        uint64_t sum = 0;
        for (size_t b = 0; b < bins; b++) sum += gpu_hist[f * bins + b];
        if (sum != h * w) {
            fprintf(stderr, "[%s] frame %zu pixel sum mismatch: %llu != %zu\n",
                    label, f, (unsigned long long)sum, h * w);
            return 7;
        }
    }
    if (verify_full) {
        uint32_t *ref_hist = (uint32_t *)calloc(hist_elems, sizeof(uint32_t));
        cpu_ref_stencil_histogram(imgs, frames, h, w, weights, divisor, steps, bins, ref_hist);
        for (size_t i = 0; i < hist_elems; i++) {
            if (gpu_hist[i] != ref_hist[i]) {
                fprintf(stderr, "[%s] bit-exact mismatch at index %zu: gpu=%u ref=%u\n",
                        label, i, gpu_hist[i], ref_hist[i]);
                free(ref_hist);
                return 8;
            }
        }
        printf("  verification: BIT-EXACT match across all %zu histogram bins!\n", hist_elems);
        free(ref_hist);
    }

    free(imgs);
    free(gpu_hist);
    return 0;
}

int main(void) {
    int64_t dev = cinch_device_open(CINCH_BACKEND_METAL_REQUIRED);
    if (dev < 0) {
        fprintf(stderr, "failed to open required Metal GPU device: %lld\n", (long long)dev);
        return 1;
    }

    /* Shape 1: Many small frames (1024 x 64 x 64) -> one threadgroup per frame. */
    if (run_shape(dev, "many-small-frames", 1024, 64, 64, 3, "many-small-frames", 1) != 0) {
        cinch_device_close(dev);
        return 10;
    }

    /* Shape 2: Few large frames (2 x 2048 x 2048) -> multi-threadgroup per frame. */
    if (run_shape(dev, "few-large-frames", 2, 2048, 2048, 3, "few-large-frames", 1) != 0) {
        cinch_device_close(dev);
        return 11;
    }

    cinch_device_close(dev);
    printf("ALL C METAL GPU PIPELINE CHECKS PASSED.\n");
    return 0;
}
