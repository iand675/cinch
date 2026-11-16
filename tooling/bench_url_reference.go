// Measure independent public URL pipelines with matched live-input mutation and observations.
package main

import (
	"bytes"
	"fmt"
	"net/url"
	"os"
	"strconv"
	"time"
)

// fingerprint observes allocation length and three representative result positions.
func fingerprint(input []byte) uint64 {
	if len(input) == 0 {
		return 0
	}
	return uint64(len(input)) + uint64(input[0]) + uint64(input[len(input)/2]) + uint64(input[len(input)-1])
}

// operation includes the public byte/string conversions and immutable base reuse of its peer driver.
func operation(mode string, input []byte, base *url.URL) uint64 {
	s := string(input)
	switch mode {
	case "query-escape":
		return fingerprint([]byte(url.QueryEscape(s)))
	case "path-escape":
		return fingerprint([]byte(url.PathEscape(s)))
	case "query-unescape":
		out, err := url.QueryUnescape(s)
		if err != nil {
			return 0
		}
		return fingerprint([]byte(out))
	case "path-unescape":
		out, err := url.PathUnescape(s)
		if err != nil {
			return 0
		}
		return fingerprint([]byte(out))
	case "query-values":
		values, _ := url.ParseQuery(s)
		return fingerprint([]byte(values.Encode()))
	case "parse-render":
		value, err := url.Parse(s)
		if err != nil {
			return 0
		}
		return fingerprint([]byte(value.String()))
	case "resolve":
		value, err := base.Parse(s)
		if err != nil {
			return 0
		}
		return fingerprint([]byte(value.String()))
	case "join":
		return fingerprint([]byte(base.JoinPath(s).String()))
	case "escape-length":
		return uint64(len(url.QueryEscape(s)))
	case "bytes-copy":
		return fingerprint(bytes.Clone(input))
	default:
		panic("unknown URL workload")
	}
}

// main prepares deterministic complete inputs before the monotonic public-operation timer.
func main() {
	if len(os.Args) != 5 {
		panic("expected mode, size, iterations and pattern")
	}
	size, _ := strconv.Atoi(os.Args[2])
	iterations, _ := strconv.Atoi(os.Args[3])
	pattern, _ := strconv.Atoi(os.Args[4])
	alphabet := []byte("a")
	if pattern == 1 {
		alphabet = []byte("a/b?c&d=e+ f")
	} else if pattern == 2 {
		alphabet = []byte("a%20b%2Fc%FF")
	}
	input := make([]byte, size)
	for i := range input {
		input[i] = alphabet[i%len(alphabet)]
	}
	base, err := url.Parse("https://example.test/a/b?old#f")
	if err != nil {
		panic(err)
	}
	start := time.Now()
	var checksum uint64
	for i := 0; i < iterations; i++ {
		if len(input) > 0 {
			input[0] = byte(97 + i%2)
		}
		checksum = (checksum + operation(os.Args[1], input, base)) % 1000000007
	}
	fmt.Println(time.Since(start).Nanoseconds(), checksum)
}
