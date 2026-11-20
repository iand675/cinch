/** Exercise the represented base object's consuming Nat comparison entry. */
#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

_Static_assert(sizeof(uintptr_t) == 8, "this fixture requires the admitted 64-bit provider ABI");

/** Allocate a mathematical natural using the matched runtime's checked parser. */
extern int64_t cinch_nat_from_decimal(const unsigned char *, size_t);
/** Enable the same reclamation policy declared by the provider artifact. */
extern void cinch_reclaim_enable(void);
/** Give a second owned reference to an explicitly shared runtime value. */
extern void cinch_dup(int64_t);
/** Report live blocks and preserve the raw process status. */
extern int64_t cinch_exit_check(int64_t);
/** Bind the genuine metadata-declared worker, not a consumer-local comparison. */
extern int64_t represented_nat_cmp(uint64_t, uint64_t) __asm__("Data.Nat.nat-cmp");

/** Encode only values within the runtime's 63-bit immediate Nat range. */
static uint64_t immediate(uint64_t value) {
    if (value > UINT64_C(9223372036854775807)) abort();
    return (value << 1) | UINT64_C(1);
}

/** Require a newly allocated boxed value, keeping its sole owned reference. */
static uint64_t fresh_box(const char *decimal) {
    uint64_t value = (uint64_t)cinch_nat_from_decimal((const unsigned char *)decimal, strlen(decimal));
    if (value == 0 || (value & UINT64_C(1)) != 0) abort();
    return value;
}

/** Pass both owned operands to the actual consuming provider worker exactly once. */
static int compare_owned(uint64_t left, uint64_t right, int64_t expected) {
    return represented_nat_cmp(left, right) == expected ? 0 : 1;
}

/** Select independent immediate, fresh-boxed and explicitly shared operand observations. */
static int observation(unsigned int number) {
    const char *a = "9223372036854775808";
    const char *b = "18446744073709551617";
    switch (number) {
    case 0: return compare_owned(immediate(0), immediate(1), -1);
    case 1: return compare_owned(fresh_box(a), immediate(UINT64_C(9223372036854775807)), 1);
    case 2: return compare_owned(immediate(0), fresh_box(a), -1);
    case 3: return compare_owned(fresh_box(a), fresh_box(a), 0);
    case 4: return compare_owned(fresh_box(b), fresh_box(a), 1);
    case 5: return compare_owned(fresh_box(a), fresh_box(b), -1);
    case 6: {
        uint64_t shared = fresh_box(b);
        cinch_dup((int64_t)shared);
        return compare_owned(shared, shared, 0);
    }
    case 7: return compare_owned(fresh_box("170141183460469231731687303715884105728"), fresh_box(b), 1);
    default: return 2;
    }
}

/** Run one bounded observation with the matched runtime's final leak accounting. */
int main(int argc, char **argv) {
    if (argc != 2) return 2;
    char *end = NULL;
    unsigned long number = strtoul(argv[1], &end, 10);
    if (*argv[1] == '\0' || *end != '\0' || number > 7) return 2;
    cinch_reclaim_enable();
    return (int)cinch_exit_check(observation((unsigned int)number));
}
