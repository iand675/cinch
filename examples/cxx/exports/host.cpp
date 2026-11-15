#include "HostApi.hpp"
#include <type_traits>
#include <cmath>
static_assert(!std::is_copy_constructible_v<demo::Counter>);
static_assert(!std::is_move_constructible_v<demo::Counter>);
int main() {
    if (heapLength(1000) != 1000) return 5;
    if (add(20, 22) != 42 || destroyed() != 0) return 1;
    {
        demo::Counter first;
        demo::Counter second;
        if (first.get_ratio() != 1e100 || !std::signbit(first.get_negativeZero())) return 6;
        if (first.add(41) != 42 || second.value() != 1) return 2;
        if (second.add(5) != 6 || first.value() != 42) return 3;
    }
    return destroyed() == 48 ? 0 : 4;
}
