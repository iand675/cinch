// Measure complete mutable list pipelines with exact output observations.
package main

import (
	"container/list"
	"fmt"
	"os"
	"strconv"
	"time"
)

// mask keeps the output checksum printable as an exact nonnegative machine word.
const mask uint64 = (1 << 63) - 1

// mix consumes each complete output sequence in its actual order.
func mix(checksum, value uint64) uint64 { return (checksum*1099511628211 + value) & mask }

// construction includes list creation, node allocation, public pushes and the full public drain.
func construction(front bool, input []uint64, iteration int) uint64 {
	values := list.New()
	for index, value := range input {
		if index == 0 {
			value = uint64(iteration % 257)
		}
		if front {
			values.PushFront(value)
		} else {
			values.PushBack(value)
		}
	}
	checksum := uint64(1)
	for values.Len() > 0 {
		var element *list.Element
		if front {
			element = values.Back()
		} else {
			element = values.Front()
		}
		checksum = mix(checksum, values.Remove(element).(uint64))
	}
	return checksum
}

// snapshot materializes every output value before consuming the complete sequence.
func snapshot(values *list.List) uint64 {
	output := make([]uint64, 0, values.Len())
	for element := values.Front(); element != nil; element = element.Next() {
		output = append(output, element.Value.(uint64))
	}
	checksum := uint64(1)
	for _, value := range output {
		checksum = mix(checksum, value)
	}
	return checksum
}

// operation executes the public pipeline over the same changing prepared owners.
func operation(mode int, input []uint64, values *list.List, references []*list.Element, iteration int) uint64 {
	if mode < 2 {
		return construction(mode == 1, input, iteration)
	}
	if len(references) > 0 {
		references[0].Value = uint64(iteration % 257)
	}
	switch mode {
	case 2:
		for _, element := range references {
			values.MoveToFront(element)
		}
		return snapshot(values)
	case 3:
		for index, element := range references {
			references[index] = values.PushBack(values.Remove(element))
		}
		return snapshot(values)
	default:
		destination := list.New()
		destination.PushBackList(values)
		destination.PushBackList(destination)
		destination.PushFrontList(destination)
		return snapshot(destination)
	}
}

// main excludes startup and preparation while retaining all prepared owners beyond the clock.
func main() {
	mode, _ := strconv.Atoi(os.Args[1])
	size, _ := strconv.Atoi(os.Args[2])
	iterations, _ := strconv.Atoi(os.Args[3])
	input := make([]uint64, size)
	values := list.New()
	references := make([]*list.Element, 0, size)
	for index := range input {
		input[index] = uint64((index*37 + 11) % 65536)
		references = append(references, values.PushBack(input[index]))
	}
	start := time.Now()
	checksum := uint64(0)
	for iteration := 0; iteration < iterations; iteration++ {
		checksum = (checksum + operation(mode, input, values, references, iteration)) & mask
	}
	elapsed := time.Since(start).Nanoseconds()
	checksum = (checksum + uint64(values.Len()+len(references)+len(input))) & mask
	fmt.Printf("%d %d\n", elapsed, checksum)
}
