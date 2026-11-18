/** Guard-page observations of the actual emitted private comparison workers. */
#define _GNU_SOURCE
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/mman.h>
#include <unistd.h>

/** Accumulate complete blocks using the actual source-emitted machine ABI. */
extern uint64_t blocks(int64_t, int64_t, int64_t, int64_t, uint64_t)
    __asm__("Crypto.ConstantTime.difference-blocks");
/** Accumulate the exact fringe using the actual source-emitted machine ABI. */
extern uint64_t tail(int64_t, int64_t, int64_t, int64_t, uint64_t)
    __asm__("Crypto.ConstantTime.difference-tail");

/** Reserve inaccessible pages on both sides of one writable page. */
static uint8_t *guarded(size_t page) {
    uint8_t *mapping = mmap(NULL, page * 3, PROT_NONE,
                           MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (mapping == MAP_FAILED ||
        mprotect(mapping + page, page, PROT_READ | PROT_WRITE)) {
        perror("guard pages");
        exit(2);
    }
    return mapping;
}

/** Compose precisely the two workers used by the public equality operation. */
static uint64_t difference(uint8_t *left, uint8_t *right, size_t size) {
    int64_t complete = (int64_t)(size & ~(size_t)15);
    uint64_t result = blocks((int64_t)(intptr_t)left,
                             (int64_t)(intptr_t)right, 0, complete, 0);
    return tail((int64_t)(intptr_t)left, (int64_t)(intptr_t)right,
                complete, (int64_t)size, result);
}

/** Replace only the executable entry during linking; retain unmodified worker IR. */
int __wrap_main(void) {
    size_t page = (size_t)sysconf(_SC_PAGESIZE), observations = 0;
    if (page < 512) return 2;
    uint8_t *first = guarded(page), *second = guarded(page);
    for (size_t size = 0; size <= 257; ++size) {
        for (unsigned left_end = 0; left_end < 2; ++left_end) {
            for (unsigned right_end = 0; right_end < 2; ++right_end) {
                uint8_t *left = first + page + (left_end ? page - size : 0);
                uint8_t *right = second + page + (right_end ? page - size : 0);
                for (size_t i = 0; i < size; ++i)
                    left[i] = right[i] = (uint8_t)(i * 73 + size);
                if (difference(left, right, size)) return 3;
                ++observations;
                for (size_t i = 0; i < size; ++i) {
                    right[i] ^= 128;
                    if (!difference(left, right, size)) return 4;
                    right[i] ^= 128;
                    ++observations;
                }
            }
        }
    }
    if (munmap(first, page * 3) || munmap(second, page * 3)) return 5;
    printf("%zu guard observations passed\n", observations);
    return 0;
}
