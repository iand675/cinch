// Measure complete public priority-queue pipelines with exact result observations.
package main

import (
	"container/heap"
	"fmt"
	"os"
	"strconv"
	"time"
)

// queue supplies mutable indexed sequence callbacks over natural-valued priorities.
type queue []uint64

// Len observes the current initialized sequence length.
func (q queue) Len() int { return len(q) }

// Less orders natural-valued priorities.
func (q queue) Less(i, j int) bool { return q[i] < q[j] }

// Swap exchanges initialized values while preserving the sequence length.
func (q queue) Swap(i, j int) { q[i], q[j] = q[j], q[i] }

// Push appends one initialized value.
func (q *queue) Push(value any) { *q = append(*q, value.(uint64)) }

// Pop removes exactly the final initialized value.
func (q *queue) Pop() any {
	n := len(*q) - 1
	value := (*q)[n]
	*q = (*q)[:n]
	return value
}

// mix consumes each value with the same bounded checksum as the Cinch driver.
func mix(checksum, value uint64) uint64 { return (checksum*33 + value) % 1000000007 }

// construction includes storage creation, public heap operations and the complete observed drain.
func construction(mode int, input []uint64, iteration int) uint64 {
	q := &queue{}
	if mode < 2 {
		*q = append(*q, input...)
		if len(*q) > 0 {
			(*q)[0] = uint64(iteration % 257)
		}
		heap.Init(q)
	} else {
		for index, value := range input {
			if index == 0 {
				value = uint64(iteration % 257)
			}
			heap.Push(q, value)
		}
	}
	checksum := uint64(1)
	for q.Len() > 0 {
		checksum = mix(checksum, heap.Pop(q).(uint64))
	}
	return checksum
}

// replace changes one root value and observes both the previous value and the repaired minimum.
func replace(q *queue, value uint64) uint64 {
	if q.Len() == 0 {
		return 0
	}
	previous := (*q)[0]
	(*q)[0] = value
	heap.Fix(q, 0)
	return mix(previous, (*q)[0])
}

// main prepares deterministic input before timing only the changing operation loop.
func main() {
	mode, _ := strconv.Atoi(os.Args[1])
	size, _ := strconv.Atoi(os.Args[2])
	iterations, _ := strconv.Atoi(os.Args[3])
	input := make([]uint64, size)
	for index := range input {
		input[index] = uint64((index*37 + 11) % 65536)
	}
	q := &queue{}
	*q = append(*q, input...)
	heap.Init(q)
	start := time.Now()
	checksum := uint64(0)
	for iteration := 0; iteration < iterations; iteration++ {
		var observed uint64
		if mode < 4 {
			observed = construction(mode, input, iteration)
		} else {
			observed = replace(q, uint64((iteration*43+17)%65536))
		}
		checksum = (checksum + observed) % 1000000007
	}
	elapsed := time.Since(start).Nanoseconds()
	checksum = (checksum + uint64(q.Len()+len(input)+q.Len())) % 1000000007
	fmt.Printf("%d %d\n", elapsed, checksum)
}
