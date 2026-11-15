// An ordinary C++ library: all exported functions retain C++ linkage.
namespace direct {
static long long count = 0;
class Handle {
public:
    Handle() noexcept { ++count; }
    virtual ~Handle() noexcept;
    long long read() const noexcept;
};
Handle::~Handle() noexcept { --count; }
long long Handle::read() const noexcept { return 40; }
Handle* open() noexcept { return new Handle; }
long long live() noexcept { return count; }
long long add(long long a, long long b) noexcept { return a + b; }
double add(double a, double b) noexcept { return a - b; }
}
