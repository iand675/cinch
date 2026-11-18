/* Explicit foreign input, monotonic timing, and checksum observation for the scalar recurrence. */
#include <errno.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

/* Admit a bounded runtime input so the recurrence cannot become a compile-time constant. */
uint64_t scalar_iterations(void) {
    const char *text = getenv("CINCH_BENCH_ITERATIONS");
    if (!text) return UINT64_C(1000000);
    char *end = NULL;
    errno = 0;
    unsigned long long count = strtoull(text, &end, 10);
    if (errno || end == text || *end || count > UINT64_C(100000000)) exit(70);
    return (uint64_t)count;
}

/* Return nanoseconds without adding implicit clock effects to the language intrinsic worker. */
uint64_t scalar_clock(void) {
    struct timespec now;
    if (clock_gettime(CLOCK_MONOTONIC, &now)) exit(71);
    return (uint64_t)now.tv_sec * UINT64_C(1000000000) + (uint64_t)now.tv_nsec;
}

/* Print the independently checked checksum only after the measured interval ends. */
int64_t scalar_report(uint64_t start, uint64_t end, uint64_t checksum) {
    printf("{\"elapsed_ns\":%" PRIu64 ",\"checksum\":%" PRIu64 "}\n", end - start, checksum);
    return end >= start ? 0 : 1;
}
