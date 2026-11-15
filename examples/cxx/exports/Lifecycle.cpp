#include <cstdint>
static std::int64_t destroyed = 0;
extern "C" void example_note_destroyed(std::int64_t count) noexcept { destroyed += count; }
extern "C" std::int64_t example_destroyed() noexcept { return destroyed; }
