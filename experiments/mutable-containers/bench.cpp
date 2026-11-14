// The C++ side of Bench.cinch: the same phases and scrambled keys, against
// absl::flat_hash_map (SwissTable) and std::unordered_map, plus
// std::vector/std::sort and std::priority_queue. Build:
//   clang++ -O2 -std=c++17 bench.cpp -o bench -labsl_hash -labsl_raw_hash_set -labsl_city -labsl_low_level_hash
#include <absl/container/flat_hash_map.h>
#include <algorithm>
#include <chrono>
#include <cstdint>
#include <cstdio>
#include <functional>
#include <queue>
#include <unordered_map>
#include <vector>

static uint64_t key(uint64_t i) { return (i * 2654435761ull) % 4294967296ull; }
static double now() { return std::chrono::duration<double, std::nano>(std::chrono::steady_clock::now().time_since_epoch()).count(); }
static void span(const char* label, uint64_t n, double t0, double t1) {
  std::printf("%s ms=%.0f ns/op=%.0f\n", label, (t1 - t0) / 1e6, (t1 - t0) / n);
}

template <class Map> static uint64_t run(const char* name, uint64_t n) {
  char label[64];
  Map m;
  double c0 = now();
  for (uint64_t i = 0; i < n; i++) m[key(i)] = i;
  double c1 = now();
  uint64_t hits = 0;
  for (uint64_t i = 0; i < n; i++) { auto it = m.find(key(i)); hits += it == m.end() ? 0 : it->second; }
  double c2 = now();
  uint64_t misses = 0;
  for (uint64_t i = 0; i < n; i++) misses += m.count(key(i) + 4294967296ull);
  double c3 = now();
  for (uint64_t i = 0; i < n; i += 2) m.erase(key(i));
  double c4 = now();
  std::snprintf(label, sizeof label, "%s insert", name); span(label, n, c0, c1);
  std::snprintf(label, sizeof label, "%s lookup-hit", name); span(label, n, c1, c2);
  std::snprintf(label, sizeof label, "%s lookup-miss", name); span(label, n, c2, c3);
  std::snprintf(label, sizeof label, "%s delete-half", name); span(label, n / 2, c3, c4);
  return hits + misses + m.size();
}

int main() {
  const uint64_t n = 1000000;
  uint64_t sink = run<absl::flat_hash_map<uint64_t, uint64_t>>("absl", n);
  sink += run<std::unordered_map<uint64_t, uint64_t>>("unordered", n);
  std::vector<uint64_t> v;
  double v0 = now();
  for (uint64_t i = 0; i < n; i++) v.push_back(key(i));
  double v1 = now();
  std::sort(v.begin(), v.end());
  double v2 = now();
  span("vector push", n, v0, v1);
  span("vector sort", n, v1, v2);
  std::priority_queue<uint64_t, std::vector<uint64_t>, std::greater<uint64_t>> h;
  double h0 = now();
  for (uint64_t i = 0; i < n; i++) h.push(key(i));
  double h1 = now();
  while (!h.empty()) h.pop();
  double h2 = now();
  span("priority_queue push", n, h0, h1);
  span("priority_queue pop", n, h1, h2);
  return sink == 0;
}
