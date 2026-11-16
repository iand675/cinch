// Measure independent public CSV pipelines with observable output and matching prepared inputs.
package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/csv"
	"fmt"
	"os"
	"strconv"
	"time"
)

// sample observes bounded output positions while preserving the exact byte length.
func sample(input []byte) uint64 {
	n := len(input)
	result := uint64(n)
	for _, at := range []int{0, 1, 2, 3, n - 1, n / 2} {
		if at >= 0 && at < n {
			result += uint64(input[at])
		}
	}
	return result
}

// code maps host-validated command names outside the measured operation loop.
func code(name string) int {
	for i, mode := range []string{"read-plain", "read-quoted", "write-plain", "write-quoted", "stream-read", "stream-write", "copy", "sha256-runtime"} {
		if name == mode {
			return i
		}
	}
	panic("unknown workload")
}

// prepare builds the same quoted/unquoted wire prefix or mutable writer field before timing.
func prepare(mode, size int) ([]byte, int) {
	body := bytes.Repeat([]byte{'a'}, size)
	if (mode == 1 || mode == 3 || mode == 4 || mode == 5) && size > 0 {
		body[0] = ' '
	}
	if mode == 0 {
		return append(body, []byte(",tail\n")...), 0
	}
	if mode == 1 || mode == 4 {
		return append(append([]byte{'"'}, body...), []byte("\",tail\n")...), 1
	}
	return body, 0
}

// operation consumes one whole public parse/render/copy operation, retaining its observable output.
func operation(mode int, input []byte) uint64 {
	if mode == 0 || mode == 1 || mode == 4 {
		source := input
		if mode == 4 {
			source = bytes.Clone(input)
		}
		r := csv.NewReader(bytes.NewReader(source))
		fields, err := r.Read()
		if err != nil {
			return 1000000000
		}
		var result uint64
		for _, field := range fields {
			result += 1 + sample([]byte(field))
		}
		return result
	}
	if mode == 2 || mode == 3 || mode == 5 {
		var output bytes.Buffer
		w := csv.NewWriter(&output)
		if err := w.Write([]string{string(input), "tail"}); err != nil {
			return 1000000000
		}
		w.Flush()
		if w.Error() != nil {
			return 1000000000
		}
		return sample(output.Bytes())
	}
	if mode == 6 {
		return sample(bytes.Clone(input))
	}
	value := sha256.Sum256(input)
	return sample(value[:])
}

// main mutates prepared data between timed operations and prints nanoseconds plus a checksum.
func main() {
	if len(os.Args) != 4 {
		panic("expected mode, size and iteration count")
	}
	mode := code(os.Args[1])
	size, err := strconv.Atoi(os.Args[2])
	if err != nil {
		panic(err)
	}
	iterations, err := strconv.Atoi(os.Args[3])
	if err != nil {
		panic(err)
	}
	input, offset := prepare(mode, size)
	change := offset
	if mode == 1 || mode == 3 || mode == 4 || mode == 5 {
		if size > 1 {
			change++
		}
	}
	var checksum uint64
	start := time.Now()
	for i := 0; i < iterations; i++ {
		if size > 0 {
			input[change] = byte(65 + i%26)
		}
		checksum = (checksum + operation(mode, input)) % 1000000007
	}
	fmt.Printf("%d %d\n", time.Since(start).Nanoseconds(), checksum)
}
