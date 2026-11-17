/** Public machine-word ABI checks against one explicitly linked runtime archive. */
#include <assert.h>
#include <stdint.h>
#include <stdio.h>

/** Return fresh owned random bytes, or a negative structured error code. */
extern int64_t cinch_random_bytes(int64_t length);
/** Return the length of a live Bytes word. */
extern int64_t cinch_bytes_len(int64_t word);
/** Read one in-range byte from a live Bytes word. */
extern int64_t cinch_bytes_get(int64_t word, int64_t index);
/** Write one in-range byte to owned mutable Bytes storage. */
extern int64_t cinch_bytes_set(int64_t word, int64_t index, int64_t value);
/** Release a live runtime box and its owned byte allocation. */
extern void cinch_free(int64_t word);
/** Release one owned runtime reference through ordinary reference counting. */
extern void cinch_drop(int64_t word);
/** Check runtime-owned headers for leaks when the diagnostic is enabled. */
extern int64_t cinch_exit_check(int64_t code);

/** Exercise actual secure draws, exact lengths, independent storage and release. */
static void check_length(int64_t length) {
    int64_t first = cinch_random_bytes(length);
    int64_t second = cinch_random_bytes(length);
    assert(first > 0 && second > 0 && first != second);
    assert(cinch_bytes_len(first) == length);
    assert(cinch_bytes_len(second) == length);
    if (length >= 16) {
        int differs = 0;
        for (int64_t index = 0; index < 16; ++index) {
            differs |= cinch_bytes_get(first, index) != cinch_bytes_get(second, index);
        }
        assert(differs);
    }
    if (length > 0) {
        int64_t before = cinch_bytes_get(second, 0);
        int64_t marker = (before + 1) & 255;
        assert(cinch_bytes_set(first, 0, marker) == 0);
        assert(cinch_bytes_get(first, 0) == marker);
        assert(cinch_bytes_get(second, 0) == before);
        int64_t last = cinch_bytes_get(first, length - 1);
        assert(last >= 0 && last <= 255);
    }
    cinch_free(first);
    cinch_drop(second);
    printf("c-abi length=%lld draws=2 status=ok\n", (long long)length);
}

/** Run the public ABI matrix without replacing the actual entropy provider. */
int main(void) {
    _Static_assert(sizeof(int64_t) == 8, "Cinch ABI requires 64-bit words");
    assert(cinch_random_bytes(-1) == -6);
    assert(cinch_random_bytes(INT64_MIN) == -6);
    const int64_t lengths[] = {0, 1, 16, 255, 256, 257, 4096, 1048576};
    for (unsigned index = 0; index < sizeof(lengths) / sizeof(lengths[0]); ++index) {
        check_length(lengths[index]);
    }
    return (int)cinch_exit_check(0);
}
