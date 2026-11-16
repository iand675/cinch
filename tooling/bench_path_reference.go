// Measure public slash-path pipelines and preserve observable results.
package main

import (
	"fmt"
	"os"
	"path"
	"strconv"
	"time"
)

// fingerprint observes exact output boundaries and length without timing a second full scan.
func fingerprint(text string) uint64 {
	if len(text) == 0 {
		return 0
	}
	return uint64(len(text)) + uint64(text[0]) + uint64(text[len(text)-1])
}

// operation exercises the same public lexical or pattern pipeline as the source driver.
func operation(mode string, input []byte, pattern string) uint64 {
	text := string(input)
	switch mode {
	case "clean-component":
		return fingerprint(path.Clean(text + "/../z"))
	case "clean-segments":
		return fingerprint(path.Clean(text))
	case "base":
		return fingerprint(path.Base("root/" + text))
	case "ext":
		return fingerprint(path.Ext(text))
	case "compiled-match", "match":
		matched, err := path.Match(pattern, text)
		if err != nil {
			return 1000000000
		}
		if matched {
			return 1
		}
		return 0
	case "join":
		return fingerprint(path.Join(text, "..", "z"))
	case "absolute-control":
		if path.IsAbs(text) {
			return 1
		}
		return 0
	default:
		panic("unknown path workload")
	}
}

// main prepares input before timing and changes one source byte on every iteration.
func main() {
	if len(os.Args) != 4 {
		panic("expected workload, size and iterations")
	}
	mode := os.Args[1]
	n, err := strconv.Atoi(os.Args[2])
	if err != nil || n < 0 {
		panic("invalid size")
	}
	iterations, err := strconv.Atoi(os.Args[3])
	if err != nil || iterations < 1 {
		panic("invalid iterations")
	}
	input := make([]byte, n)
	for i := range input {
		if mode == "clean-segments" {
			input[i] = "a/./b/../"[i%9]
		} else {
			input[i] = 'x'
		}
	}
	pattern := string(input)
	var checksum uint64
	start := time.Now()
	for i := 0; i < iterations; i++ {
		if len(input) != 0 {
			input[len(input)-1] = byte(120 + i%2)
		}
		checksum = (checksum + operation(mode, input, pattern)) % 1000000007
	}
	elapsed := time.Since(start).Nanoseconds()
	fmt.Printf("%d %d\n", elapsed, checksum)
}
