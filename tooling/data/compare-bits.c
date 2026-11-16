/** Deterministic foreign scalar fixtures preserve IEEE payload bits exactly. */
#include <stdint.h>
#include <string.h>
/** Reinterpret one binary32 bit pattern without a numeric conversion. */
float compare_from_f32(uint32_t bits) { float value; memcpy(&value, &bits, sizeof value); return value; }
/** Observe one binary32 result's exact payload and zero sign. */
uint32_t compare_bits_f32(float value) { uint32_t bits; memcpy(&bits, &value, sizeof bits); return bits; }
/** Reinterpret one binary64 bit pattern without a numeric conversion. */
double compare_from_f64(uint64_t bits) { double value; memcpy(&value, &bits, sizeof value); return value; }
/** Observe one binary64 result's exact payload and zero sign. */
uint64_t compare_bits_f64(double value) { uint64_t bits; memcpy(&bits, &value, sizeof bits); return bits; }
