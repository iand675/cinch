/** Count real checked bulk copies without changing runtime allocation or source semantics. */
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>

/** Aggregate positive byte-copy requests across the complete independent consumer. */
static uint64_t copied_bytes;
/** Count positive bulk-copy calls independently of source-level offer telemetry. */
static uint64_t copy_calls;

/** Preserve the existing checked runtime ABI exactly; no new runtime entry point is introduced. */
extern int64_t __real_cinch_bytes_copy(int64_t, int64_t, int64_t, int64_t, int64_t);

/** Observe actual requested bytes and delegate every operation to the unchanged matched runtime. */
int64_t __wrap_cinch_bytes_copy(int64_t dst, int64_t src, int64_t di, int64_t si, int64_t n) {
    if (n > 0) {
        copied_bytes += (uint64_t)n;
        copy_calls += 1;
    }
    return __real_cinch_bytes_copy(dst, src, di, si, n);
}

/** Report actual copy work after ordinary runtime leak accounting has completed. */
__attribute__((destructor)) static void report_copy_work(void) {
    fprintf(stderr, "stream-copy: bytes=%" PRIu64 " calls=%" PRIu64 "\n", copied_bytes, copy_calls);
}
