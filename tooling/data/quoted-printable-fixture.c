/** Independent fixture sink and foreign-object checks for the actual SIMD mask body. */
#include <stdint.h>
#include <stdio.h>
#include <string.h>

/** The linked object contains the selected, unmodified library LLVM primitive body. */
extern uint32_t qp_fixture_classify16(uint64_t, uint64_t, uint8_t);
/** Bounded process-local observable sink storage, independent of the language heap. */
static unsigned char sink_bytes[1024 * 1024];
/** Number of bytes actually accepted by the fixture sink. */
static int64_t sink_length;
/** Negative for unlimited writes, otherwise the exact accepted output budget. */
static int64_t sink_limit;

/** Reset one serialized I/O fixture without retaining any language-owned pointer. */
int64_t qp_fixture_reset(int64_t limit) { sink_length = 0; sink_limit = limit; return 0; }
/** Accept a prefix or report a stable failure on a later write; never retain the input span. */
int64_t qp_fixture_write(int64_t address, int64_t count) {
    if (count < 0 || count > (int64_t)sizeof sink_bytes - sink_length) return -2;
    int64_t accepted = count;
    if (sink_limit >= 0 && accepted > sink_limit - sink_length) accepted = sink_limit - sink_length;
    if (accepted < 0) accepted = 0;
    if (count > 0 && accepted == 0) return -1;
    if (accepted > 0) memcpy(sink_bytes + sink_length, (const void *)(uintptr_t)address, (size_t)accepted);
    sink_length += accepted;
    return accepted;
}
/** Observe the exact advanced sink length after either success or failure. */
int64_t qp_fixture_length(int64_t ignored) { (void)ignored; return sink_length; }
/** Observe one already accepted byte for independent prefix comparison. */
uint8_t qp_fixture_byte(int64_t index) { return index >= 0 && index < sink_length ? sink_bytes[index] : 0; }

/** Independent scalar classification used only by the foreign-object oracle. */
static int valid_byte(unsigned char byte, unsigned mode) {
    if (mode == 3) return byte != 10;
    if (mode == 2) return byte != 61 && ((byte >= 32 && byte != 127) || byte == 9 || byte == 13 || byte == 10);
    return (byte >= 33 && byte <= 126 && byte != 61) || byte == 32 || byte == 9 || (mode == 0 && (byte == 13 || byte == 10));
}
/** Check every byte at every lane in all four modes against the actual linked kernel object. */
int32_t qp_fixture_kernel_check(int64_t ignored) {
    (void)ignored;
    for (unsigned mode = 0; mode != 4; ++mode) {
        for (unsigned lane = 0; lane != 16; ++lane) {
            for (unsigned value = 0; value != 256; ++value) {
                unsigned char input[16]; memset(input, 'A', sizeof input); input[lane] = (unsigned char)value;
                uint64_t lo = 0, hi = 0; uint32_t expected = 0;
                for (unsigned i = 0; i != 16; ++i) {
                    if (!valid_byte(input[i], mode)) expected |= (uint32_t)1 << i;
                    if (i < 8) lo |= (uint64_t)input[i] << (i * 8);
                    else hi |= (uint64_t)input[i] << ((i - 8) * 8);
                }
                uint32_t actual = qp_fixture_classify16(lo, hi, (uint8_t)mode);
                if (actual != expected) {
                    fprintf(stderr, "quoted-printable kernel mismatch: mode=%u lane=%u byte=%u expected=%u actual=%u\n", mode, lane, value, expected, actual);
                    return 1;
                }
            }
        }
    }
    /* Simultaneous mixed lanes exercise mask composition as well as one-lane isolation. */
    uint32_t seed = 0x12345678u;
    for (unsigned sample = 0; sample != 4096; ++sample) {
        unsigned char input[16];
        uint64_t lo = 0, hi = 0;
        for (unsigned lane = 0; lane != 16; ++lane) {
            seed = seed * 1664525u + 1013904223u;
            input[lane] = (unsigned char)(seed >> 24);
            if (lane < 8) lo |= (uint64_t)input[lane] << (lane * 8);
            else hi |= (uint64_t)input[lane] << ((lane - 8) * 8);
        }
        for (unsigned mode = 0; mode != 4; ++mode) {
            uint32_t expected = 0;
            for (unsigned lane = 0; lane != 16; ++lane)
                if (!valid_byte(input[lane], mode)) expected |= (uint32_t)1 << lane;
            if (qp_fixture_classify16(lo, hi, (uint8_t)mode) != expected) return 2;
        }
    }
    return 0;
}
