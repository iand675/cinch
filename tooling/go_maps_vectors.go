// Produce operation-level reference observations for the pinned maps package.
// Iteration observations are commutative; no unspecified map order is frozen.
package main

import (
	"encoding/json"
	"fmt"
	"iter"
	"maps"
	"math"
	"os"
	"runtime"
	"slices"
)

// Pair preserves duplicate-bearing input sequences independently of map order.
type Pair struct {
	Key   uint64 `json:"key"`
	Value uint64 `json:"value"`
}

// BoolPair supplies heterogeneous values to the value-comparison callback.
type BoolPair struct {
	Key   uint64 `json:"key"`
	Value bool   `json:"value"`
}

// Step records inputs, complete post-state and one operation observation.
type Step struct {
	Operation string     `json:"operation"`
	Key       uint64     `json:"key"`
	Value     uint64     `json:"value"`
	Pairs     []Pair     `json:"pairs"`
	Expected  []Pair     `json:"expected"`
	Auxiliary []Pair     `json:"auxiliary"`
	Other     []BoolPair `json:"other"`
	Observed  uint64     `json:"observed"`
}

// Trace keeps each seed in its own bounded memory/ownership process.
type Trace struct {
	Seed  uint64 `json:"seed"`
	Steps []Step `json:"steps"`
}

// FloatCase uses complete bit representations so JSON preserves NaNs and signed zeros.
type FloatCase struct {
	Width     uint64 `json:"width"`
	Operation string `json:"operation"`
	Expected  []Pair `json:"expected"`
	Observed  uint64 `json:"observed"`
}

// FloatSnapshot observes key multiplicity through iteration, never through NaN lookup.
func floatSnapshot[T ~float32 | ~float64](m map[T]uint64, bits func(T) uint64) []Pair {
	out := make([]Pair, 0, len(m))
	for key, value := range maps.All(m) {
		out = append(out, Pair{bits(key), value})
	}
	slices.SortFunc(out, func(a, b Pair) int {
		if a.Key < b.Key || a.Key == b.Key && a.Value < b.Value {
			return -1
		}
		if a.Key == b.Key && a.Value == b.Value {
			return 0
		}
		return 1
	})
	return out
}

// FloatVectors pins non-reflexive key operations without assuming equivalence laws.
func floatVectors[T ~float32 | ~float64](width uint64, from func(uint64) T, bits func(T) uint64, nan, other, negative, negativeZero uint64) []FloatCase {
	m := make(map[T]uint64)
	rows := make([]FloatCase, 0)
	record := func(name string, observed uint64) {
		rows = append(rows, FloatCase{width, name, floatSnapshot(m, bits), observed})
	}
	m[from(nan)] = 1
	m[from(nan)] = 2
	record("repeat-nan", 0)
	m[from(other)] = 3
	m[from(negative)] = 4
	record("distinct-nans", 0)
	m[from(0)] = 5
	m[from(negativeZero)] = 6
	record("replace-signed-zero", 0)
	var observed uint64
	if maps.Equal(m, m) {
		observed = 1
	}
	record("equal-self", observed)
	copy := maps.Clone(m)
	rows = append(rows, FloatCase{width, "clone", floatSnapshot(copy, bits), 0})
	fresh := make(map[T]uint64)
	maps.Copy(fresh, m)
	rows = append(rows, FloatCase{width, "copy-fresh", floatSnapshot(fresh, bits), 0})
	maps.DeleteFunc(m, func(key T, value uint64) bool { return true })
	record("delete-all-by-key", 0)
	return rows
}

// Panics observes assignment refusals without ending the reference generator.
func panics(action func()) (result bool) {
	defer func() {
		if recover() != nil {
			result = true
		}
	}()
	action()
	return false
}

// NilVectors retains observable absent-versus-empty behavior separately from maps.
func nilVectors() map[string]bool {
	var absent map[uint64]uint64
	empty := make(map[uint64]uint64)
	return map[string]bool{
		"clone-absent":           maps.Clone(absent) == nil,
		"equal-empty":            maps.Equal(absent, empty),
		"copy-empty-succeeds":    !panics(func() { maps.Copy(absent, empty) }),
		"copy-binding-refused":   panics(func() { maps.Copy(absent, map[uint64]uint64{1: 2}) }),
		"insert-empty-succeeds":  !panics(func() { maps.Insert(absent, source(nil)) }),
		"insert-binding-refused": panics(func() { maps.Insert(absent, source([]Pair{{1, 2}})) }),
		"collect-empty-present":  maps.Collect(source(nil)) != nil,
	}
}

// Source supplies pairs in their specified order and obeys immediate stop.
func source(pairs []Pair) iter.Seq2[uint64, uint64] {
	return func(yield func(uint64, uint64) bool) {
		for _, pair := range pairs {
			if !yield(pair.Key, pair.Value) {
				return
			}
		}
	}
}

// Snapshot sorts only the observations; the operations themselves use maps.
func snapshot(m map[uint64]uint64) []Pair {
	keys := slices.Sorted(maps.Keys(m))
	out := make([]Pair, 0, len(keys))
	for _, key := range keys {
		out = append(out, Pair{key, m[key]})
	}
	return out
}

// Advance supplies deterministic, full-width inputs without a random runtime.
func advance(state *uint64) uint64 {
	x := *state
	x ^= x << 13
	x ^= x >> 7
	x ^= x << 17
	*state = x
	return x
}

// Build observes every actual package operation several times per trace.
func build(seed uint64) Trace {
	state := seed
	m := make(map[uint64]uint64)
	trace := Trace{Seed: seed}
	operations := []string{"Collect", "Insert", "Copy", "DeleteFunc", "Clone", "Equal", "EqualFunc", "All", "Keys", "Values"}
	for i := 0; i < 100; i++ {
		step := Step{Operation: operations[i%len(operations)], Key: advance(&state) % 32, Value: advance(&state) % 4096}
		step.Pairs = make([]Pair, 0)
		for n := int(advance(&state) % 9); n > 0; n-- {
			step.Pairs = append(step.Pairs, Pair{advance(&state) % 32, advance(&state) % 4096})
		}
		switch step.Operation {
		case "Collect":
			m = maps.Collect(source(step.Pairs))
		case "Insert":
			maps.Insert(m, source(step.Pairs))
		case "Copy":
			maps.Copy(m, maps.Collect(source(step.Pairs)))
		case "DeleteFunc":
			maps.DeleteFunc(m, func(key, value uint64) bool { return key%4 == step.Key%4 })
		case "Clone":
			step.Key = 10000 + uint64(i)
			copy := maps.Clone(m)
			maps.Insert(copy, source([]Pair{{step.Key, step.Value}}))
			step.Auxiliary = snapshot(copy)
			if _, present := m[step.Key]; !present {
				step.Observed = 1
			}
		case "Equal":
			other := maps.Clone(m)
			if step.Value%2 != 0 {
				other[10000+uint64(i)] = step.Value
			}
			step.Auxiliary = snapshot(other)
			if maps.Equal(m, other) {
				step.Observed = 1
			}
		case "EqualFunc":
			other := make(map[uint64]bool)
			for key, value := range maps.All(m) {
				other[key] = value%2 == 0
			}
			if step.Value%2 != 0 {
				other[10000+uint64(i)] = true
			}
			for _, key := range slices.Sorted(maps.Keys(other)) {
				step.Other = append(step.Other, BoolPair{key, other[key]})
			}
			if maps.EqualFunc(m, other, func(value uint64, flag bool) bool { return (value%2 == 0) == flag }) {
				step.Observed = 1
			}
		case "All":
			for key, value := range maps.All(m) {
				step.Observed += 17*key + value
			}
		case "Keys":
			for key := range maps.Keys(m) {
				step.Observed += key
			}
		case "Values":
			for value := range maps.Values(m) {
				step.Observed += value
			}
		}
		step.Expected = snapshot(m)
		trace.Steps = append(trace.Steps, step)
	}
	return trace
}

// Main writes exact observations to stdout for a separately implemented consumer.
func main() {
	traces := make([]Trace, 0, 8)
	for _, seed := range []uint64{1, 7, 42, 31337, 65537, 0x8000000000000001, 0xdeadbeef12345678, 0xffffffffffffffff} {
		traces = append(traces, build(seed))
	}
	floats := floatVectors[float64](64, math.Float64frombits, math.Float64bits, 0x7ff8000000000001, 0x7ff0000000000001, 0xfff8000000000017, 0x8000000000000000)
	floats = append(floats, floatVectors[float32](32, func(bits uint64) float32 { return math.Float32frombits(uint32(bits)) }, func(value float32) uint64 { return uint64(math.Float32bits(value)) }, 0x7fc00001, 0x7f800001, 0xffc00017, 0x80000000)...)
	report := struct {
		Format int             `json:"format"`
		SDK    string          `json:"sdk"`
		Traces []Trace         `json:"traces"`
		Floats []FloatCase     `json:"floats"`
		Nil    map[string]bool `json:"nil"`
	}{1, runtime.Version(), traces, floats, nilVectors()}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(report); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
