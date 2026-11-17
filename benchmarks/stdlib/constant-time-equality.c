/* Public-pipeline timing support; actual equality remains in Cinch. */
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <time.h>

/* Keep each public loop token opaque without modifying prepared byte owners. */
__attribute__((noinline)) uint32_t constant_time_tick(uint32_t token) {
    __asm__ volatile ("" : "+r"(token) : : "memory");
    return token;
}

/* Read the monotonic clock outside the operation loop. */
int64_t constant_time_now(void) {
    struct timespec now;
    if (clock_gettime(CLOCK_MONOTONIC, &now)) return -1;
    return (int64_t)now.tv_sec * INT64_C(1000000000) + now.tv_nsec;
}

/* Preserve complete observed results alongside each raw interval. */
void constant_time_report(int64_t elapsed, uint64_t observed) {
    printf("%" PRId64 " %" PRIu64 "\n", elapsed, observed);
}

/* Preserve progress and a complete final-output fingerprint outside the timer. */
void constant_time_buffer_report(int64_t elapsed, uint64_t consumed, uint64_t hash) {
    printf("%" PRId64 " %" PRIu64 " %" PRIu64 "\n", elapsed, consumed, hash);
}
