// Reference text-codec public-API pipelines for tooling/bench_stdlib.py.
// Inputs, mutations and observable checksums match the Cinch workload driver.
// Allocation strategies and immutable string guarantees remain implementation-specific.
package main

import (
	"bytes"
	"encoding/ascii85"
	"encoding/binary"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
	"time"
	"unicode/utf16"
)

// encoded allocates and returns an unframed complete ASCII85 encoding.
func encoded(input []byte) []byte {
	out := make([]byte, ascii85.MaxEncodedLen(len(input)))
	return out[:ascii85.Encode(out, input)]
}

// unitOrder selects the explicit UTF-16 wire byte order.
func unitOrder(mode string) binary.ByteOrder {
	if strings.HasPrefix(mode, "utf16be-") {
		return binary.BigEndian
	}
	return binary.LittleEndian
}

// encodedUnits converts valid UTF-8 text to UTF-16 through the reference scalar API.
func encodedUnits(mode string, input []byte) []byte {
	units := utf16.Encode([]rune(string(input)))
	out := make([]byte, len(units)*2)
	order := unitOrder(mode)
	for i, unit := range units {
		order.PutUint16(out[i*2:], unit)
	}
	return out
}

// fingerprint consumes the same length and boundary bytes as the Cinch driver.
func fingerprint(input []byte) uint64 {
	if len(input) == 0 {
		return 0
	}
	return uint64(len(input)) + uint64(input[0]) + uint64(input[len(input)-1])
}

// operation exercises a public buffer or stream pipeline and consumes its output.
func operation(mode string, input []byte) uint64 {
	switch mode {
	case "ascii85-encode":
		return fingerprint([]byte(string(encoded(input))))
	case "ascii85-decode":
		out := make([]byte, len(input)*4+4)
		n, _, err := ascii85.Decode(out, []byte(string(input)), true)
		if err != nil {
			return 1000000000
		}
		return fingerprint(out[:n])
	case "ascii85-stream-encode":
		var sink bytes.Buffer
		writer := ascii85.NewEncoder(&sink)
		if _, err := writer.Write(input); err != nil {
			return 1000000000
		}
		if err := writer.Close(); err != nil {
			return 1000000000
		}
		return fingerprint(sink.Bytes())
	case "ascii85-stream-decode":
		// The source snapshot matches reader-from-bytes's aliasing contract.
		out, err := io.ReadAll(ascii85.NewDecoder(bytes.NewReader(bytes.Clone(input))))
		if err != nil {
			return 1000000000
		}
		return fingerprint(out)
	case "utf16le-encode", "utf16be-encode":
		return fingerprint(encodedUnits(mode, input))
	case "utf16le-decode", "utf16be-decode":
		units := make([]uint16, len(input)/2)
		order := unitOrder(mode)
		for i := range units {
			units[i] = order.Uint16(input[i*2:])
		}
		return fingerprint([]byte(string(utf16.Decode(units))))
	default:
		panic("unsupported reference workload")
	}
}

// main prepares outside timing and prints elapsed nanoseconds plus the rolling checksum.
func main() {
	if len(os.Args) != 4 {
		panic("expected workload, byte count and iteration count")
	}
	mode := os.Args[1]
	size, sizeErr := strconv.Atoi(os.Args[2])
	iterations, countErr := strconv.Atoi(os.Args[3])
	if sizeErr != nil || countErr != nil || size < 0 || iterations <= 0 {
		panic("invalid benchmark arguments")
	}
	input := make([]byte, size)
	for i := range input {
		input[i] = byte(i*37 + 11)
		if strings.HasPrefix(mode, "utf16") {
			input[i] %= 128
		}
	}
	decoding := strings.HasSuffix(mode, "decode")
	if decoding {
		if strings.HasPrefix(mode, "utf16") {
			input = encodedUnits(mode, input)
		} else {
			input = encoded(input)
		}
	}
	var checksum uint64
	start := time.Now()
	for i := 0; i < iterations; i++ {
		if len(input) > 0 {
			if strings.HasPrefix(mode, "utf16") {
				index := 0
				if mode == "utf16be-decode" {
					index = 1
				}
				input[index] = byte(i % 128)
			} else if decoding {
				input[0] = byte(33 + i%85)
			} else {
				input[0] = byte(i % 256)
			}
		}
		checksum = (checksum + operation(mode, input)) % 1000000007
	}
	fmt.Printf("%d %d\n", time.Since(start).Nanoseconds(), checksum)
}
