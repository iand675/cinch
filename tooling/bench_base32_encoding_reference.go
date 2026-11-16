// Measure independent Base32 public pipelines with the same observable outputs.
package main

import (
	"bytes"
	"encoding/base32"
	"fmt"
	"io"
	"os"
	"strconv"
	"time"
)

// fingerprint consumes exactly the result length and boundary bytes.
func fingerprint(input []byte) uint64 {
	if len(input) == 0 {
		return 0
	}
	return uint64(len(input)) + uint64(input[0]) + uint64(input[len(input)-1])
}

// appendedFingerprint includes the first input-dependent byte after the fixed prefix.
func appendedFingerprint(input []byte) uint64 {
	value := fingerprint(input)
	if len(input) > 4 {
		value += uint64(input[4])
	}
	return value
}

// operation executes one complete byte, stream or preserved text pipeline.
func operation(mode int, encoding *base32.Encoding, input []byte) uint64 {
	switch mode {
	case 0, 2:
		out := make([]byte, encoding.EncodedLen(len(input)))
		encoding.Encode(out, input)
		return fingerprint(out)
	case 1, 3:
		out := make([]byte, encoding.DecodedLen(len(input)))
		n, err := encoding.Decode(out, input)
		if err != nil {
			return 1000000000
		}
		return fingerprint(out[:n])
	case 4:
		return fingerprint([]byte(encoding.EncodeToString(input)))
	case 5:
		out, err := encoding.DecodeString(string(input))
		if err != nil {
			return 1000000000
		}
		return fingerprint(out)
	case 6:
		var out bytes.Buffer
		writer := base32.NewEncoder(encoding, &out)
		for at := 0; at < len(input); at += 64 {
			end := at + 64
			if end > len(input) {
				end = len(input)
			}
			if _, err := writer.Write(input[at:end]); err != nil {
				return 1000000000
			}
		}
		if err := writer.Close(); err != nil {
			return 1000000000
		}
		return fingerprint(out.Bytes())
	case 7:
		out, err := io.ReadAll(base32.NewDecoder(encoding, bytes.NewReader(bytes.Clone(input))))
		if err != nil {
			return 1000000000
		}
		return fingerprint(out)
	case 8:
		return appendedFingerprint(encoding.AppendEncode([]byte{1, 2, 3, 4}, input))
	case 9:
		out, err := encoding.AppendDecode([]byte{1, 2, 3, 4}, input)
		if err != nil {
			return 1000000000
		}
		return appendedFingerprint(out)
	case 10:
		n := encoding.EncodedLen(len(input))
		out := make([]byte, n+2)
		encoding.Encode(out[2:], input)
		return fingerprint(out[2:])
	default:
		out := make([]byte, encoding.DecodedLen(len(input))+2)
		n, err := encoding.Decode(out[2:], input)
		if err != nil {
			return 1000000000
		}
		return fingerprint(out[2 : 2+n])
	}
}

// main prepares the mutable input before timing the exact public operation loop.
func main() {
	mode, _ := strconv.Atoi(os.Args[1])
	size, _ := strconv.Atoi(os.Args[2])
	iterations, _ := strconv.Atoi(os.Args[3])
	alphabet := "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"
	if mode == 2 || mode == 3 {
		alphabet = "765432ZYXWVUTSRQPONMLKJIHGFEDCBA"
	}
	encoding := base32.NewEncoding(alphabet)
	input := make([]byte, size)
	for i := range input {
		input[i] = byte(i*37 + 11)
	}
	decoding := mode == 1 || mode == 3 || mode == 5 || mode == 7 || mode == 9 || mode == 11
	if decoding {
		encoded := make([]byte, encoding.EncodedLen(len(input)))
		encoding.Encode(encoded, input)
		input = encoded
	}
	start := time.Now()
	var checksum uint64
	for i := 0; i < iterations; i++ {
		if len(input) > 0 {
			if decoding {
				input[0] = alphabet[i%32]
			} else {
				input[0] = byte(i % 256)
			}
		}
		checksum = (checksum + operation(mode, encoding, input)) % 1000000007
	}
	fmt.Printf("%d %d\n", time.Since(start).Nanoseconds(), checksum)
}
