# Mutable containers benchmark

`Bench.cinch` times `Data.Mutable.HashTable`, the persistent `Data.HashMap`,
`Data.Mutable.Vec` and `Data.Mutable.Heap`; `bench.cpp` times the same phases
against `absl::flat_hash_map`, `std::unordered_map`, `std::vector`/`std::sort`
and `std::priority_queue`. Both use the same scrambled keys
(`i * 2654435761 mod 2^32`), so neither side gets a sequential memory walk out
of an identity hash, which is what a plain `0 .. n` key range gives
libstdc++'s `std::hash<uint64_t>`.

## Running

```
# the Cinch side (the non-embedded closure is copied beside the entry)
mkdir /tmp/mutbench && cd /tmp/mutbench
for m in Vec HashTable Heap; do
  cp ~/cinch/compiler/prelude/Data/Mutable/$m.cinch Data.Mutable.$m.cinch; done
cp ~/cinch/experiments/mutable-containers/Bench.cinch Main.cinch
(cd ~/cinch/compiler/runtime && cargo build --release)   # an optimized runtime
~/cinch/compiler/target/release/cinch build Main.cinch -o bench --opt-level 2
./bench

# the C++ side (Ubuntu: apt install libabsl-dev)
clang++ -O2 -std=c++17 ~/cinch/experiments/mutable-containers/bench.cpp -o cpp \
  -labsl_hash -labsl_raw_hash_set -labsl_city -labsl_low_level_hash
./cpp
```

`cinch run` links the debug runtime unless a fresh release archive exists,
which is why the runtime is built in release first.

## Recorded

See `design/mutable-containers.md` section 7 for the numbers and the machine
they were measured on.
