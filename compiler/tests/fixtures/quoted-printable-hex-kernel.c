/* Exhaustive independent byte-table oracle for the actual unpacked classifier. */
#include <stdint.h>

/* The selected inline LLVM body is compiled as this real foreign object. */
extern uint32_t qp_hex_value(uint8_t value);

/* Search literal digit tables instead of repeating the primitive's arithmetic. */
static uint32_t reference(uint8_t value) {
    static const uint8_t decimal[] = "0123456789";
    static const uint8_t lower[] = "abcdef";
    static const uint8_t upper[] = "ABCDEF";
    for (uint32_t at = 0; at < 10; ++at)
        if (value == decimal[at]) return at;
    for (uint32_t at = 0; at < 6; ++at)
        if (value == lower[at] || value == upper[at]) return at + 10;
    return 16;
}

/* Observe every byte and every pair, including all non-ASCII invalid values. */
int main(void) {
    for (uint32_t high = 0; high < 256; ++high) {
        if (qp_hex_value((uint8_t)high) != reference((uint8_t)high)) return 1;
        for (uint32_t low = 0; low < 256; ++low) {
            uint32_t actual_high = qp_hex_value((uint8_t)high);
            uint32_t actual_low = qp_hex_value((uint8_t)low);
            uint32_t expected_high = reference((uint8_t)high);
            uint32_t expected_low = reference((uint8_t)low);
            uint32_t actual = actual_high < 16 && actual_low < 16 ? (actual_high << 4) | actual_low : 256;
            uint32_t expected = expected_high < 16 && expected_low < 16 ? expected_high * 16 + expected_low : 256;
            if (actual != expected) return 2;
        }
    }
    return 0;
}
