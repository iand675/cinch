// Measure complete circular-sequence pipelines with independent ordered observations.
package main

import (
	"container/ring"
	"fmt"
	"os"
	"runtime"
	"strconv"
	"time"
)

// mask preserves printable exact nonnegative checksums after wrapping arithmetic.
const mask uint64 = (1 << 63) - 1

// mix consumes one value in its actual observation order.
func mix(hash, value uint64) uint64 { return (hash*1099511628211 + value) & mask }

// cycleHash observes every value from the selected cursor without materializing an output slice.
func cycleHash(first *ring.Ring) uint64 {
	hash := uint64(1)
	cursor := first
	count := first.Len()
	for index := 0; index < count; index++ {
		hash = mix(hash, cursor.Value.(uint64))
		cursor = cursor.Next()
	}
	return hash
}

// initialize constructs one dense logical cycle with a changing first value.
func initialize(input []uint64, iteration int) *ring.Ring {
	first := ring.New(len(input))
	cursor := first
	for index, value := range input {
		if index == 0 {
			value = uint64(iteration % 257)
		}
		cursor.Value = value
		cursor = cursor.Next()
	}
	return first
}

// growing includes fresh singleton construction, incremental joins and complete traversal.
func growing(input []uint64, iteration int) uint64 {
	var first, last *ring.Ring
	for index, value := range input {
		if index == 0 {
			value = uint64(iteration % 257)
		}
		node := ring.New(1)
		node.Value = value
		if first == nil {
			first = node
		} else {
			last.Link(node)
		}
		last = node
	}
	return cycleHash(first)
}

// balanced constructs separate singleton owners and consumes every original alias after balanced joins.
func balanced(input []uint64, iteration int) uint64 {
	references := make([]*ring.Ring, len(input))
	for index, value := range input {
		if index == 0 {
			value = uint64(iteration % 257)
		}
		references[index] = ring.New(1)
		references[index].Value = value
	}
	for stride := 1; stride < len(references); stride *= 2 {
		for index := 0; index+stride < len(references); index += stride * 2 {
			references[index].Prev().Link(references[index+stride])
		}
	}
	var first *ring.Ring
	if len(references) > 0 {
		first = references[0]
	}
	hash := cycleHash(first)
	for index, cursor := range references {
		hash = mix(hash, cursor.Value.(uint64))
		next := index + 1
		if next == len(references) {
			next = 0
		}
		if cursor.Next() == references[next] {
			hash = mix(hash, 1)
		} else {
			hash = mix(hash, 0)
		}
	}
	return hash
}

// shortSplices observes64 changing short split/rejoin operations and their complete final cycle.
func shortSplices(first *ring.Ring, iteration int) uint64 {
	hash := uint64(1)
	for round := 0; round < 64; round++ {
		removed := first.Unlink(1)
		previous := removed.Value.(uint64)
		removed.Value = uint64((iteration + round) % 257)
		first.Link(removed)
		hash = mix(hash, previous)
	}
	return mix(hash, cycleHash(first))
}

// clone allocates independent nodes while preserving the source component's current values.
func clone(first *ring.Ring) *ring.Ring {
	count := first.Len()
	output := ring.New(count)
	source, destination := first, output
	for index := 0; index < count; index++ {
		destination.Value = source.Value
		source, destination = source.Next(), destination.Next()
	}
	return output
}

// detachedCopy copies one changing detached component, restores the source and observes known fresh retention sizes.
func detachedCopy(first *ring.Ring, iteration int) uint64 {
	count := first.Len()
	if count == 1 {
		return mix(mix(cycleHash(clone(first)), 1), 1)
	}
	removed := first.Unlink(count / 2)
	removed.Value = uint64(iteration % 257)
	copied := clone(removed)
	hash := mix(cycleHash(copied), uint64(count/2))
	first.Link(removed)
	return mix(hash, uint64(count))
}

// operation selects the same complete public pipeline over changing inputs and prepared owners.
func operation(mode int, input []uint64, prepared *ring.Ring, iteration int) uint64 {
	switch mode {
	case 0:
		return cycleHash(initialize(input, iteration))
	case 1:
		return growing(input, iteration)
	case 2:
		return balanced(input, iteration)
	}
	if prepared == nil {
		return 1
	}
	prepared.Value = uint64(iteration % 257)
	if mode == 3 {
		return shortSplices(prepared, iteration)
	}
	return detachedCopy(prepared, iteration)
}

// main excludes preparation and keeps all prepared input and alias owners alive beyond the last clock.
func main() {
	mode, _ := strconv.Atoi(os.Args[1])
	size, _ := strconv.Atoi(os.Args[2])
	iterations, _ := strconv.Atoi(os.Args[3])
	input := make([]uint64, size)
	for index := range input {
		input[index] = uint64((index*37 + 11) % 65536)
	}
	prepared := ring.New(size)
	references := make([]*ring.Ring, size)
	cursor := prepared
	for index, value := range input {
		cursor.Value = value
		references[index] = cursor
		cursor = cursor.Next()
	}
	start := time.Now()
	hash := uint64(0)
	for iteration := 0; iteration < iterations; iteration++ {
		hash = (hash + operation(mode, input, prepared, iteration)) & mask
	}
	elapsed := time.Since(start).Nanoseconds()
	hash = (hash + uint64(prepared.Len()+len(references)+len(input))) & mask
	runtime.KeepAlive(prepared)
	runtime.KeepAlive(references)
	runtime.KeepAlive(input)
	fmt.Printf("%d %d\n", elapsed, hash)
}
