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
	"os"
	"strconv"
	"unicode/utf16"
)

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
		f32.Write(b)
		f32a.Write(b)
		f64.Write(b)
		f64a.Write(b)
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
	if err := encoder.Encode(map[string]interface{}{"buffers": checks, "words": words, "signed": signed, "ascii85_decodes": asciiDecodes, "utf16": utf16Vectors()}); err != nil {
		panic(err)
	}
}
