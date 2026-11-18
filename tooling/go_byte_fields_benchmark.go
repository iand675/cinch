// Independent pinned-SDK public pipelines with matching observations and retention boundaries.
package main

import (
	"bytes"
	"crypto/sha256"
	"fmt"
	"os"
	"runtime"
	"strconv"
	"time"
	"unicode"
)

// inputByte reproduces raw input bytes without reading any Cinch implementation.
func inputByte(pattern string, at int) byte {
	switch pattern {
	case "space":
		return 32
	case "text":
		return 122
	case "dense":
		if at%2 == 0 {
			return 122
		}
		return 32
	case "fringe":
		if at%33 == 32 {
			return 32
		}
		return 122
	default:
		return [...]byte{97, 194, 160, 98, 227, 128, 128, 99, 32, 255, 128, 122}[at%12]
	}
}

// hashField makes every ordered nonempty extent and both raw endpoints observable.
func hashField(total uint64, field []byte) uint64 {
	if len(field) == 0 {
		panic("empty field")
	}
	return total*65599 + uint64(len(field))*257 + uint64(field[0])*17 + uint64(field[len(field)-1])*5 + 1
}

// operation consumes complete eager or lazy public outputs before returning the checksum.
func operation(kind string, input []byte) uint64 {
	var total, calls uint64
	switch kind {
	case "producer", "visit":
		bytes.FieldsSeq(input)(func(field []byte) bool {
			total = hashField(total, field)
			return true
		})
	case "predicate", "effect":
		predicate := unicode.IsSpace
		if kind == "effect" {
			predicate = func(value rune) bool {
				calls++
				return unicode.IsSpace(value)
			}
		}
		for _, field := range bytes.FieldsFunc(input, predicate) {
			total = hashField(total, field)
		}
	default:
		for _, field := range bytes.Fields(input) {
			total = hashField(total, field)
		}
	}
	return total + calls*104729
}

// main excludes owner preparation/hashing and keeps all input references alive through the end clock.
func main() {
	if len(os.Args) != 5 {
		os.Exit(2)
	}
	kind, pattern := os.Args[1], os.Args[2]
	size, err := strconv.Atoi(os.Args[3])
	if err != nil || size < 0 {
		os.Exit(2)
	}
	iterations, err := strconv.Atoi(os.Args[4])
	if err != nil || iterations < 0 {
		os.Exit(2)
	}
	owner := make([]byte, size+2)
	owner[0], owner[len(owner)-1] = 255, 255
	for at := 0; at < size; at++ {
		owner[at+1] = inputByte(pattern, at)
	}
	input := owner[1 : size+1]
	original := sha256.Sum256(owner)
	var checksum uint64
	begin := time.Now()
	for index := 0; index < iterations; index++ {
		owner[1] = byte(97 + index%2)
		checksum += operation(kind, input)
	}
	elapsed := time.Since(begin).Nanoseconds()
	retained := sha256.Sum256(owner)
	runtime.KeepAlive(input)
	runtime.KeepAlive(owner)
	fmt.Printf("%d %d %x %x %d\n", elapsed, checksum, original, retained, len(input))
}
