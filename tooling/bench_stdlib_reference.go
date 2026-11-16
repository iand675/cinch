// Reference standard-library public-API pipelines for tooling/bench_stdlib.py.
// Inputs, mutations and observable checksums match the Cinch workload driver.
// Allocation strategies and immutable string guarantees remain implementation-specific.
package main

import (
	"bytes"
	"encoding/ascii85"
	"encoding/binary"
	"encoding/hex"
	"fmt"
	"hash/fnv"
	"io"
	"math/bits"
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
func unitOrder(mode int) binary.ByteOrder {
	if mode == 23 || mode == 24 {
		return binary.BigEndian
	}
	return binary.LittleEndian
}

// encodedUnits converts valid UTF-8 text to UTF-16 through the reference scalar API.
func encodedUnits(mode int, input []byte) []byte {
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

// sampleByte observes an existing output byte and uses zero for absent intervals.
func sampleByte(input []byte, offset int) uint64 {
	if offset < len(input) {
		return uint64(input[offset])
	}
	return 0
}

// dumpFingerprint includes input-dependent columns after the fixed address prefix.
func dumpFingerprint(input []byte) uint64 {
	return fingerprint(input) + sampleByte(input, 10) + sampleByte(input, 11) + sampleByte(input, 61)
}

// wordFingerprint consumes both halves while matching the bounded Cinch checksum.
func wordFingerprint(value uint64) uint64 { return uint64(uint32(value) ^ uint32(value>>32)) }

// inputWord reads one field, matching the short-buffer zero fallback.
func inputWord(input []byte) uint64 {
	if len(input) < 8 {
		return 0
	}
	return binary.LittleEndian.Uint64(input)
}

// operation exercises a public buffer or stream pipeline and consumes its output.
func operation(mode int, input []byte) uint64 {
	switch mode {
	case 0:
		return fingerprint([]byte(hex.EncodeToString(input)))
	case 1:
		out, err := hex.DecodeString(string(input))
		if err != nil {
			return 1000000000
		}
		return fingerprint(out)
	case 33:
		out := hex.AppendEncode([]byte{1, 2, 3, 4}, input)
		return fingerprint(out) + sampleByte(out, 4) + sampleByte(out, 5)
	case 34:
		out, err := hex.AppendDecode([]byte{1, 2, 3, 4}, input)
		if err != nil {
			return 1000000000
		}
		return fingerprint(out) + sampleByte(out, 4) + sampleByte(out, 5)
	case 35:
		out := make([]byte, hex.EncodedLen(len(input))+2)
		n := hex.Encode(out[1:], input)
		return uint64(n) + fingerprint(out) + sampleByte(out, 1) + sampleByte(out, 2)
	case 36:
		out := make([]byte, hex.DecodedLen(len(input))+2)
		n, err := hex.Decode(out[1:], input)
		if err != nil {
			return 1000000000
		}
		return uint64(n) + fingerprint(out) + sampleByte(out, 1) + sampleByte(out, 2)
	case 37:
		return dumpFingerprint([]byte(hex.Dump(input)))
	case 38:
		var sink bytes.Buffer
		_, err := hex.NewEncoder(&sink).Write(input)
		if err != nil {
			return 1000000000
		}
		return fingerprint(sink.Bytes())
	case 39:
		out, err := io.ReadAll(hex.NewDecoder(bytes.NewReader(bytes.Clone(input))))
		if err != nil {
			return 1000000000
		}
		return fingerprint(out)
	case 40:
		var sink bytes.Buffer
		writer := hex.Dumper(&sink)
		if _, err := writer.Write(input); err != nil {
			return 1000000000
		}
		if err := writer.Close(); err != nil {
			return 1000000000
		}
		return dumpFingerprint(sink.Bytes())
	case 41:
		out := make([]byte, hex.EncodedLen(len(input)))
		hex.Encode(out, input)
		return fingerprint(out)
	case 42:
		out := make([]byte, hex.DecodedLen(len(input)))
		n, err := hex.Decode(out, input)
		if err != nil {
			return 1000000000
		}
		return fingerprint(out[:n])
	case 43:
		return fingerprint(encoded(input))
	case 44:
		out := make([]byte, ascii85.MaxEncodedLen(len(input))+2)
		n := ascii85.Encode(out[1:], input)
		return uint64(n) + fingerprint(out) + sampleByte(out, 1) + sampleByte(out, 2)
	case 45:
		out := make([]byte, len(input)*4+2)
		n, used, err := ascii85.Decode(out[1:], input, true)
		if err != nil {
			return 1000000000
		}
		return uint64(n+used) + fingerprint(out) + sampleByte(out, 1) + sampleByte(out, 2)
	case 46, 48:
		out := make([]byte, len(input)*4)
		n, used, err := ascii85.Decode(out, input, true)
		if err != nil {
			return 1000000000
		}
		return fingerprint(out[:n]) + uint64(used)
	case 49:
		out := make([]byte, 4)
		at := 0
		var sum uint64
		for at < len(input) {
			n, used, err := ascii85.Decode(out, input[at:], true)
			if err != nil || used == 0 {
				return 1000000000
			}
			sum += fingerprint(out[:n])
			at += used
		}
		return sum + uint64(at)
	case 17:
		return fingerprint([]byte(string(encoded(input))))
	case 18, 47:
		out := make([]byte, len(input)*4+4)
		n, _, err := ascii85.Decode(out, []byte(string(input)), true)
		if err != nil {
			return 1000000000
		}
		return fingerprint(out[:n])
	case 19:
		var sink bytes.Buffer
		writer := ascii85.NewEncoder(&sink)
		if _, err := writer.Write(input); err != nil {
			return 1000000000
		}
		if err := writer.Close(); err != nil {
			return 1000000000
		}
		return fingerprint(sink.Bytes())
	case 20:
		// The source snapshot matches reader-from-bytes's aliasing contract.
		out, err := io.ReadAll(ascii85.NewDecoder(bytes.NewReader(bytes.Clone(input))))
		if err != nil {
			return 1000000000
		}
		return fingerprint(out)
	case 21, 23:
		return fingerprint(encodedUnits(mode, input))
	case 22, 24:
		units := make([]uint16, len(input)/2)
		order := unitOrder(mode)
		for i := range units {
			units[i] = order.Uint16(input[i*2:])
		}
		return fingerprint([]byte(string(utf16.Decode(units))))
	case 25:
		value := inputWord(input)
		sum, carry := bits.Add64(value, 0x9e3779b97f4a7c15, value&1)
		return wordFingerprint(sum) + carry
	case 26:
		high, low := bits.Mul64(inputWord(input), 0x9e3779b97f4a7c15)
		return wordFingerprint(high ^ low)
	case 27, 30:
		value := inputWord(input)
		q, r := bits.Div64(value&65535, value, 0x100000001)
		return wordFingerprint(q ^ r)
	case 28:
		value := inputWord(input)
		return wordFingerprint(bits.Reverse64(value) ^ bits.RotateLeft64(bits.ReverseBytes64(value), -13))
	case 29:
		return wordFingerprint(inputWord(input))
	case 31, 32:
		hash := fnv.New128()
		if mode == 32 {
			hash = fnv.New128a()
		}
		hash.Write(input)
		sum := hash.Sum(nil)
		return wordFingerprint(binary.BigEndian.Uint64(sum[:8]) ^ binary.BigEndian.Uint64(sum[8:]))
	default:
		panic("unsupported reference workload")
	}
}

// workloadCode resolves command-line text outside the measured interval.
func workloadCode(name string) int {
	switch name {
	case "hex-encode":
		return 0
	case "hex-decode":
		return 1
	case "hex-append-encode":
		return 33
	case "hex-append-decode":
		return 34
	case "hex-into-encode":
		return 35
	case "hex-into-decode":
		return 36
	case "hex-dump":
		return 37
	case "hex-stream-encode":
		return 38
	case "hex-stream-decode":
		return 39
	case "hex-stream-dump":
		return 40
	case "hex-bytes-encode":
		return 41
	case "hex-prefix-decode":
		return 42
	case "ascii85-bytes-encode":
		return 43
	case "ascii85-into-encode":
		return 44
	case "ascii85-into-decode":
		return 45
	case "ascii85-prefix-decode":
		return 46
	case "ascii85-controls-decode":
		return 47
	case "ascii85-controls-prefix-decode":
		return 48
	case "ascii85-offset-decode":
		return 49
	case "ascii85-encode":
		return 17
	case "ascii85-decode":
		return 18
	case "ascii85-stream-encode":
		return 19
	case "ascii85-stream-decode":
		return 20
	case "utf16le-encode":
		return 21
	case "utf16le-decode":
		return 22
	case "utf16be-encode":
		return 23
	case "utf16be-decode":
		return 24
	case "word-add64":
		return 25
	case "word-mul64":
		return 26
	case "word-div64":
		return 27
	case "word-bits64":
		return 28
	case "word-load":
		return 29
	case "word-div64-proven":
		return 30
	case "fnv128":
		return 31
	case "fnv128a":
		return 32
	default:
		panic("unsupported reference workload")
	}
}

// main prepares outside timing and prints elapsed nanoseconds plus the rolling checksum.
func main() {
	if len(os.Args) != 4 {
		panic("expected workload, byte count and iteration count")
	}
	name := os.Args[1]
	mode := workloadCode(name)
	isUTF16 := mode >= 21 && mode <= 24
	size, sizeErr := strconv.Atoi(os.Args[2])
	iterations, countErr := strconv.Atoi(os.Args[3])
	if sizeErr != nil || countErr != nil || size < 0 || iterations <= 0 {
		panic("invalid benchmark arguments")
	}
	input := make([]byte, size)
	for i := range input {
		input[i] = byte(i*37 + 11)
		if isUTF16 {
			input[i] %= 128
		}
	}
	decoding := strings.HasSuffix(name, "decode")
	if decoding {
		if isUTF16 {
			input = encodedUnits(mode, input)
		} else if strings.HasPrefix(name, "hex-") {
			input = []byte(hex.EncodeToString(input))
		} else {
			input = encoded(input)
		}
	}
	if mode == 47 || mode == 48 {
		input = append(make([]byte, 4096), input...)
	}
	var checksum uint64
	start := time.Now()
	for i := 0; i < iterations; i++ {
		if mode == 47 || mode == 48 {
			if len(input) > 4096 {
				input[4096] = byte(33 + i%85)
			}
		} else if len(input) > 0 {
			if isUTF16 {
				index := 0
				if mode == 24 {
					index = 1
				}
				input[index] = byte(i % 128)
			} else if decoding && strings.HasPrefix(name, "hex-") {
				input[0] = byte(48 + i%10)
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
