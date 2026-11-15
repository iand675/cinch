#pragma once
#include <cstdint>
#include <stdexcept>
namespace demo {
inline std::int64_t live = 0;
class Counter {
    std::int64_t value;
public:
    explicit Counter(std::int64_t initial) : value(initial) { ++live; }
    ~Counter() { --live; }
    std::int64_t add(std::int64_t n) { return value += n; }
    std::int64_t get() const { return value; }
};
inline std::int64_t live_count() { return live; }
inline std::int64_t throwing() { throw std::runtime_error("native failure"); }
}
