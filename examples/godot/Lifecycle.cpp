#include <cstdint>
static std::int64_t created = 0;
static std::int64_t destroyed = 0;
extern "C" void example_godot_created() noexcept { ++created; }
extern "C" void example_godot_destroyed() noexcept { ++destroyed; }
extern "C" std::int64_t example_godot_destructions() noexcept { return destroyed; }
