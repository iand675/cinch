/** Public-pipeline benchmark endpoints; input construction and retention stay in the driver. */
#include <stdint.h>
#include <string.h>

/** Borrowed immutable input, held live by the language driver until after the end clock. */
static const uint8_t *input_bytes;
/** Exact input span length. */
static uint64_t input_length;
/** Explicit sequential position of the current benchmark source. */
static uint64_t input_position;
/** Bounded scalar digest of all bytes accepted by the current sink. */
static uint64_t output_digest;

/** Restart one borrowed source without copying or allocating its retained input. */
int64_t multipart_bench_input(int64_t address, int64_t length) {
    if (length < 0) return -1;
    input_bytes = (const uint8_t *)(uintptr_t)address;
    input_length = (uint64_t)length;
    input_position = 0;
    return 0;
}

/** Copy one actual requested prefix into the caller's fresh checked output buffer. */
int64_t multipart_bench_read(int64_t address, int64_t capacity) {
    if (capacity < 0 || input_position > input_length) return -1;
    uint64_t count = (uint64_t)capacity;
    if (count > input_length - input_position) count = input_length - input_position;
    if (count) memcpy((void *)(uintptr_t)address, input_bytes + input_position, (size_t)count);
    input_position += count;
    return (int64_t)count;
}

/** Begin one new complete writer observation without touching its caller input. */
int64_t multipart_bench_reset(int64_t ignored) {
    (void)ignored;
    output_digest = 0;
    return 0;
}

/** Accept one complete writer offer and independently consume every emitted byte. */
int64_t multipart_bench_write(int64_t address, int64_t length) {
    if (length < 0) return -1;
    const uint8_t *bytes = (const uint8_t *)(uintptr_t)address;
    for (int64_t at = 0; at < length; ++at)
        output_digest = (output_digest * UINT64_C(131) + bytes[at]) & UINT64_C(2147483647);
    return length;
}

/** Observe the complete output digest after public writer finalization. */
int64_t multipart_bench_digest(int64_t ignored) {
    (void)ignored;
    return (int64_t)output_digest;
}
