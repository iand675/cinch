// Regenerate with: go run tooling/go_stdlib_vectors.go > compiler/tests/fixtures/go-stdlib-vectors.json
// Go is a fixture-generation tool only; Cinch's test suite consumes the JSON.
package main

import (
	"encoding/ascii85"
	"encoding/base32"
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"hash/adler32"
	"hash/crc32"
	"hash/crc64"
	"hash/fnv"
	"math/bits"
	"os"
	"strconv"
	"unicode/utf16"
)

// checkedWordDivision records the reference API's fault as well as its successful result.
func checkedWordDivision(width int, high, low, divisor uint64) (quotient, remainder uint64, failed bool) {
	defer func() {
		if recover() != nil {
			failed = true
		}
	}()
	if width == 32 {
		q, r := bits.Div32(uint32(high), uint32(low), uint32(divisor))
		return uint64(q), uint64(r), false
	}
	q, r := bits.Div64(high, low, divisor)
	return q, r, false
}

// wordVectors exercises exact arithmetic, width boundaries, signed rotates and division faults.
func wordVectors() map[string]interface{} {
	values := []uint64{0, 1, 2, 127, 128, 255, 256, 65535, 65536, 0x7fffffff, 0x80000000, 0xffffffff, 1 << 32, 1 << 63, ^uint64(0)}
	scalars := []map[string]interface{}{}
	for _, width := range []int{8, 16, 32, 64} {
		for _, input := range values {
			value := input
			if width < 64 {
				value &= (uint64(1) << width) - 1
			}
			var reversed, swapped uint64
			var length, leading, trailing, ones int
			rotates := []map[string]interface{}{}
			for _, count := range []int{-129, -65, -64, -33, -1, 0, 1, 32, 63, 64, 129} {
				var rotated uint64
				switch width {
				case 8:
					rotated = uint64(bits.RotateLeft8(uint8(value), count))
				case 16:
					rotated = uint64(bits.RotateLeft16(uint16(value), count))
				case 32:
					rotated = uint64(bits.RotateLeft32(uint32(value), count))
				case 64:
					rotated = bits.RotateLeft64(value, count)
				}
				rotates = append(rotates, map[string]interface{}{"count": count, "value": rotated})
			}
			switch width {
			case 8:
				x := uint8(value)
				reversed, swapped = uint64(bits.Reverse8(x)), value
				length, leading, trailing, ones = bits.Len8(x), bits.LeadingZeros8(x), bits.TrailingZeros8(x), bits.OnesCount8(x)
			case 16:
				x := uint16(value)
				reversed, swapped = uint64(bits.Reverse16(x)), uint64(bits.ReverseBytes16(x))
				length, leading, trailing, ones = bits.Len16(x), bits.LeadingZeros16(x), bits.TrailingZeros16(x), bits.OnesCount16(x)
			case 32:
				x := uint32(value)
				reversed, swapped = uint64(bits.Reverse32(x)), uint64(bits.ReverseBytes32(x))
				length, leading, trailing, ones = bits.Len32(x), bits.LeadingZeros32(x), bits.TrailingZeros32(x), bits.OnesCount32(x)
			case 64:
				reversed, swapped = bits.Reverse64(value), bits.ReverseBytes64(value)
				length, leading, trailing, ones = bits.Len64(value), bits.LeadingZeros64(value), bits.TrailingZeros64(value), bits.OnesCount64(value)
			}
			scalars = append(scalars, map[string]interface{}{"width": width, "value": value, "reverse": reversed, "swap": swapped, "length": length, "leading": leading, "trailing": trailing, "ones": ones, "rotates": rotates})
		}
	}
	arithmetic, divisions := []map[string]interface{}{}, []map[string]interface{}{}
	for _, width := range []int{32, 64} {
		mask := ^uint64(0)
		if width == 32 {
			mask = 0xffffffff
		}
		pairs := [][2]uint64{{0, 0}, {0, mask}, {mask, 0}, {mask, mask}, {1, mask}, {mask, 1}, {1, 1}, {mask >> 1, mask >> 1}, {mask >> 1, mask>>1 + 1}}
		state := uint64(42)
		for i := 0; i < 16; i++ {
			state = state*6364136223846793005 + 1
			a := state & mask
			state = state*6364136223846793005 + 1
			pairs = append(pairs, [2]uint64{a, state & mask})
		}
		for _, pair := range pairs {
			a, b := pair[0], pair[1]
			for _, carry := range []uint64{0, 1} {
				var sum, cout, difference, borrow, high, low uint64
				if width == 32 {
					x, c := bits.Add32(uint32(a), uint32(b), uint32(carry))
					y, d := bits.Sub32(uint32(a), uint32(b), uint32(carry))
					h, l := bits.Mul32(uint32(a), uint32(b))
					sum, cout, difference, borrow, high, low = uint64(x), uint64(c), uint64(y), uint64(d), uint64(h), uint64(l)
				} else {
					sum, cout = bits.Add64(a, b, carry)
					difference, borrow = bits.Sub64(a, b, carry)
					high, low = bits.Mul64(a, b)
				}
				arithmetic = append(arithmetic, map[string]interface{}{"width": width, "a": a, "b": b, "carry": carry == 1, "sum": sum, "out_carry": cout == 1, "difference": difference, "borrow": borrow == 1, "high": high, "low": low})
			}
		}
		triples := [][3]uint64{{0, 0, 0}, {1, mask, 0}, {0, mask, 1}, {1, 0, 1}, {1, mask, 2}, {mask, mask, mask}, {mask - 1, mask, mask}, {0, 0, mask}, {mask >> 1, mask, mask}}
		for _, pair := range pairs {
			divisor := pair[1]
			if divisor == 0 {
				divisor = 1
			}
			triples = append(triples, [3]uint64{pair[0] % divisor, pair[1], divisor})
		}
		for _, triple := range triples {
			high, low, divisor := triple[0], triple[1], triple[2]
			q, r, failed := checkedWordDivision(width, high, low, divisor)
			var remainder uint64
			if divisor != 0 {
				if width == 32 {
					remainder = uint64(bits.Rem32(uint32(high), uint32(low), uint32(divisor)))
				} else {
					remainder = bits.Rem64(high, low, divisor)
				}
			}
			divisions = append(divisions, map[string]interface{}{"width": width, "high": high, "low": low, "divisor": divisor, "quotient": q, "remainder": r, "failed": failed, "modulus": remainder})
		}
	}
	reverseBytes := make([]byte, 256)
	for i := range reverseBytes {
		reverseBytes[i] = bits.Reverse8(uint8(i))
	}
	return map[string]interface{}{"scalars": scalars, "arithmetic": arithmetic, "divisions": divisions, "reverse8_hex": hex.EncodeToString(reverseBytes), "native_bits": bits.UintSize}
}

// utf16Vectors records scalar boundaries, surrogate errors and byte-order-independent units.
func utf16Vectors() map[string]interface{} {
	scalars := []map[string]interface{}{}
	values := []rune{-2147483648, -1, 0, 127, 128, 0xd7ff, 0xd800, 0xdbff, 0xdc00, 0xdfff, 0xe000, 0xffff, 0x10000, 0x1f600, 0x10ffff, 0x110000, 2147483647}
	for _, r := range values {
		high, low := utf16.EncodeRune(r)
		scalars = append(scalars, map[string]interface{}{"value": int64(r), "length": utf16.RuneLen(r), "surrogate": utf16.IsSurrogate(r), "pair": []uint16{uint16(high), uint16(low)}, "encoded": utf16.Encode([]rune{r})})
	}
	sequences := []map[string]interface{}{}
	for _, units := range [][]uint16{{}, {0}, {65, 66, 67, 68, 69}, {0xfeff, 65, 0xfffe}, {0xd800}, {0xdc00}, {0xd800, 65}, {0xd800, 0xd800, 0xdc00}, {0xdc00, 0xdc00}, {0xdbff, 0xdfff}, {0xd7ff, 0xd800, 0xdc00, 0xe000}, {65, 66, 67, 0xd83d, 0xde00, 68, 69, 70, 71}} {
		decoded := utf16.Decode(units)
		sequences = append(sequences, map[string]interface{}{"units": units, "decoded_hex": hex.EncodeToString([]byte(string(decoded))), "normalized_units": utf16.Encode(decoded)})
	}
	pairs := []map[string]interface{}{}
	for _, high := range values {
		for _, low := range values {
			pairs = append(pairs, map[string]interface{}{"high": int64(high), "low": int64(low), "decoded": int64(utf16.DecodeRune(high, low))})
		}
	}
	return map[string]interface{}{"scalars": scalars, "sequences": sequences, "pairs": pairs}
}

func main() {
	inputs := [][]byte{[]byte(""), []byte("f"), []byte("fo"), []byte("foo"), []byte("foob"), []byte("fooba"), []byte("foobar"), []byte("123456789"), []byte("abc")}
	all := make([]byte, 256)
	for i := range all {
		all[i] = byte(i)
	}
	inputs = append(inputs, all)
	random := make([]byte, 257)
	state := uint32(42)
	for i := range random {
		state = state*1664525 + 1013904223
		random[i] = byte(state >> 24)
	}
	inputs = append(inputs, random)
	// Exercise both sides of the SIMD, SWAR and scalar-tail boundaries.
	for _, n := range []int{7, 8, 15, 16, 17, 31, 32, 33, 255} {
		inputs = append(inputs, random[:n])
	}
	checks := []map[string]string{}
	for _, b := range inputs {
		ascii := make([]byte, ascii85.MaxEncodedLen(len(b)))
		ascii = ascii[:ascii85.Encode(ascii, b)]
		f32, f32a, f64, f64a := fnv.New32(), fnv.New32a(), fnv.New64(), fnv.New64a()
		f128, f128a := fnv.New128(), fnv.New128a()
		f32.Write(b)
		f32a.Write(b)
		f64.Write(b)
		f64a.Write(b)
		f128.Write(b)
		f128a.Write(b)
		checks = append(checks, map[string]string{
			"ascii85":          string(ascii),
			"hex":              hex.EncodeToString(b),
			"base32":           base32.StdEncoding.EncodeToString(b),
			"base32_hex":       base32.HexEncoding.EncodeToString(b),
			"adler32":          strconv.FormatUint(uint64(adler32.Checksum(b)), 10),
			"crc32_ieee":       strconv.FormatUint(uint64(crc32.ChecksumIEEE(b)), 10),
			"crc32_castagnoli": strconv.FormatUint(uint64(crc32.Checksum(b, crc32.MakeTable(crc32.Castagnoli))), 10),
			"crc32_koopman":    strconv.FormatUint(uint64(crc32.Checksum(b, crc32.MakeTable(crc32.Koopman))), 10),
			"crc64_iso":        strconv.FormatUint(crc64.Checksum(b, crc64.MakeTable(crc64.ISO)), 10),
			"crc64_ecma":       strconv.FormatUint(crc64.Checksum(b, crc64.MakeTable(crc64.ECMA)), 10),
			"fnv32":            strconv.FormatUint(uint64(f32.Sum32()), 10),
			"fnv32a":           strconv.FormatUint(uint64(f32a.Sum32()), 10),
			"fnv64":            strconv.FormatUint(f64.Sum64(), 10),
			"fnv64a":           strconv.FormatUint(f64a.Sum64(), 10),
			"fnv128_hex":       hex.EncodeToString(f128.Sum(nil)),
			"fnv128a_hex":      hex.EncodeToString(f128a.Sum(nil)),
		})
	}
	words := []map[string]string{}
	for _, x := range []uint64{0, 1, 127, 128, 255, 256, 300, 0x1234, 0x12345678, 1 << 32, 1 << 63, ^uint64(0)} {
		le16, be16, le32, be32, le64, be64, variable := make([]byte, 2), make([]byte, 2), make([]byte, 4), make([]byte, 4), make([]byte, 8), make([]byte, 8), make([]byte, 10)
		binary.LittleEndian.PutUint16(le16, uint16(x))
		binary.BigEndian.PutUint16(be16, uint16(x))
		binary.LittleEndian.PutUint32(le32, uint32(x))
		binary.BigEndian.PutUint32(be32, uint32(x))
		binary.LittleEndian.PutUint64(le64, x)
		binary.BigEndian.PutUint64(be64, x)
		n := binary.PutUvarint(variable, x)
		words = append(words, map[string]string{"value": strconv.FormatUint(x, 10), "le16": hex.EncodeToString(le16), "be16": hex.EncodeToString(be16), "le32": hex.EncodeToString(le32), "be32": hex.EncodeToString(be32), "le64": hex.EncodeToString(le64), "be64": hex.EncodeToString(be64), "uvarint": hex.EncodeToString(variable[:n])})
	}
	signed := []map[string]string{}
	for _, x := range []int64{0, -1, 1, -64, 64, -65, 65, -1 << 63, 1<<63 - 1} {
		b := make([]byte, 10)
		n := binary.PutVarint(b, x)
		signed = append(signed, map[string]string{"value": strconv.FormatInt(x, 10), "varint": hex.EncodeToString(b[:n])})
	}
	// Cover compression, ignored controls, noncanonical groups and malformed tails.
	asciiDecodes := []map[string]string{}
	for _, text := range []string{"", "z", "zz z", "!!!!!", "uuuuu", "s8W-!", "!!", "!!!", "!!!!", "!\x00!\t!\r!\n!", "!", "z!", "!z", "!!!!z", "!!!!!~", "v", "\xff", "\xc3\xa9", "<~!!!!!~>", "z~>"} {
		out := make([]byte, len(text)*4+4)
		n, _, err := ascii85.Decode(out, []byte(text), true)
		offset := ""
		if err != nil {
			offset = strconv.FormatInt(int64(err.(ascii85.CorruptInputError)), 10)
		}
		asciiDecodes = append(asciiDecodes, map[string]string{"encoded_hex": hex.EncodeToString([]byte(text)), "decoded_hex": hex.EncodeToString(out[:n]), "error_offset": offset})
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]interface{}{"buffers": checks, "words": words, "signed": signed, "ascii85_decodes": asciiDecodes, "utf16": utf16Vectors(), "word_ops": wordVectors()}); err != nil {
		panic(err)
	}
}
