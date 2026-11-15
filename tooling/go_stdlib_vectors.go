// Regenerate with: go run tooling/go_stdlib_vectors.go > compiler/tests/fixtures/go-stdlib-vectors.json
// Go is a fixture-generation tool only; Cinch's test suite consumes the JSON.
package main

import (
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
)

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
		f32, f32a, f64, f64a := fnv.New32(), fnv.New32a(), fnv.New64(), fnv.New64a()
		f32.Write(b)
		f32a.Write(b)
		f64.Write(b)
		f64a.Write(b)
		checks = append(checks, map[string]string{
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
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]interface{}{"buffers": checks, "words": words, "signed": signed}); err != nil {
		panic(err)
	}
}
