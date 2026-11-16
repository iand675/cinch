// Measure complete public MIME pipelines with independently observable output checksums.
package main

import (
	"fmt"
	"mime"
	"os"
	"strconv"
	"strings"
	"time"
)

// checksum traverses every result byte using the same explicitly bounded recurrence.
func checksum(input string, value uint64) uint64 {
	for i := 0; i < len(input); i++ {
		value = (value*131 + uint64(input[i])) & 2147483647
	}
	return value
}

// main prepares inputs and contexts before timing complete operations and result traversal.
func main() {
	name := os.Args[1]
	size, _ := strconv.Atoi(os.Args[2])
	count, _ := strconv.Atoi(os.Args[3])
	raw := make([]byte, size)
	for i := range raw {
		if name == "parse" || name == "registry" {
			raw[i] = byte(65 + i%26)
		} else {
			raw[i] = byte((i*37 + 11) % 256)
		}
	}
	input := string(raw)
	if name == "parse" {
		input = "TEXT/PLAIN; Name=\"" + input + "\""
	} else if name == "header" || name == "header-q" {
		encoder := mime.BEncoding
		if name == "header-q" {
			encoder = mime.QEncoding
		}
		input = encoder.Encode("UTF-8", input)
	} else if name == "registry" {
		input = ".cinch-" + strings.ToLower(input)
	}
	params := map[string]string{"name": string(raw)}
	if name == "registry" {
		if err := mime.AddExtensionType(".cinch-"+string(raw), "application/x-cinch-benchmark"); err != nil {
			panic(err)
		}
	}
	decoder := mime.WordDecoder{}
	var value uint64
	start := time.Now()
	for i := 0; i < count; i++ {
		var output string
		switch name {
		case "parse":
			base, parsed, err := mime.ParseMediaType(input)
			if err != nil {
				panic(err)
			}
			output = mime.FormatMediaType(base, parsed)
		case "format":
			output = mime.FormatMediaType("application/x-cinch", params)
		case "b-encode":
			output = mime.BEncoding.Encode("UTF-8", input)
		case "q-encode":
			output = mime.QEncoding.Encode("UTF-8", input)
		case "header", "header-q":
			var err error
			output, err = decoder.DecodeHeader(input)
			if err != nil {
				panic(err)
			}
		case "registry":
			output = mime.TypeByExtension(input)
		default:
			panic("unknown mode")
		}
		value = checksum(output, value)
	}
	elapsed := time.Since(start).Nanoseconds()
	retained := checksum(input, 0)
	retained = checksum(params["name"], retained)
	retained = checksum(mime.TypeByExtension(input), retained)
	fmt.Println(elapsed, value, retained)
}
