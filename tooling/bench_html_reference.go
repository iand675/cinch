// Measure independent public HTML pipelines with the same input mutation and observable results.
package main

import (
	"bytes"
	"fmt"
	"html"
	"os"
	"strconv"
	"time"
)

// fingerprint observes the full length and three representative output positions.
func fingerprint(input []byte) uint64 {
	if len(input) == 0 {
		return 0
	}
	return uint64(len(input)) + uint64(input[0]) + uint64(input[len(input)/2]) + uint64(input[len(input)-1])
}

// operation includes the conversions needed by the corresponding public byte/text pipeline.
func operation(mode string, input []byte) uint64 {
	switch mode {
	case "escape-bytes", "escape-text":
		return fingerprint([]byte(html.EscapeString(string(input))))
	case "unescape-bytes", "unescape-text":
		return fingerprint([]byte(html.UnescapeString(string(input))))
	case "escape-length":
		return uint64(len(html.EscapeString(string(input))))
	case "bytes-copy":
		return fingerprint(bytes.Clone(input))
	default:
		panic("unknown HTML workload")
	}
}

// main prepares complete ASCII input before timing and retains every operation result.
func main() {
	if len(os.Args) != 4 && len(os.Args) != 5 {
		panic("expected mode, input size, iteration count and pattern")
	}
	size, _ := strconv.Atoi(os.Args[2])
	iterations, _ := strconv.Atoi(os.Args[3])
	pattern := 0
	if len(os.Args) == 5 {
		pattern, _ = strconv.Atoi(os.Args[4])
	}
	alphabet := []byte("a")
	if pattern == 1 {
		alphabet = []byte("<>&\"'")
	} else if pattern == 2 {
		alphabet = []byte("&amp;&#x1F600;&NotEqualTilde;not&a")
	}
	input := make([]byte, size)
	for i := range input {
		input[i] = alphabet[i%len(alphabet)]
	}
	start := time.Now()
	var checksum uint64
	for i := 0; i < iterations; i++ {
		if len(input) > 0 {
			input[0] = byte(97 + i%2)
		}
		checksum = (checksum + operation(os.Args[1], input)) % 1000000007
	}
	fmt.Println(time.Since(start).Nanoseconds(), checksum)
}
