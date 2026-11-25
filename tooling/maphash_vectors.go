// Command maphash_vectors verifies Go 1.24 hash/maphash contracts and emits a
// JSON fixture of edge-case inputs and contract groups for Cinch's Data.MapHash tests.
//
// Because hash/maphash uses per-process randomized seeds rather than fixed cross-process
// digests, this generator verifies every hash/maphash behavioral invariant directly
// against Go's standard library (determinism for equal seeds, independence across seeds,
// streaming chunk-split invariance across Write/WriteByte/WriteString, Sum64/Sum,
// Reset/SetSeed, Size/BlockSize, Bytes/String, and Comparable/WriteComparable including
// -0.0 == +0.0 normalization and composite boundary framing) and writes the verified
// test corpus to stdout:
//
//	GOCACHE=/tmp/go-cache GOTELEMETRY=off go run tooling/maphash_vectors.go > seed/tests/fixtures/maphash-vectors.json
package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"hash/maphash"
	"math"
	"os"
)

type constants struct {
	Size      int `json:"size"`
	BlockSize int `json:"block_size"`
}

type u128Seed struct {
	Hi uint64 `json:"hi"`
	Lo uint64 `json:"lo"`
}

type splitCase struct {
	Name     string   `json:"name"`
	InputHex string   `json:"input_hex"`
	Chunks   []string `json:"chunks"`
}

type stringPair struct {
	First  string `json:"first"`
	Second string `json:"second"`
}

type comparableCases struct {
	F64ZeroBits        []uint64     `json:"f64_zero_bits"`
	F32ZeroBits        []uint32     `json:"f32_zero_bits"`
	F64DistinctBits    []uint64     `json:"f64_distinct_bits"`
	F32DistinctBits    []uint32     `json:"f32_distinct_bits"`
	I64Distinct        []int64      `json:"i64_distinct"`
	U64Distinct        []uint64     `json:"u64_distinct"`
	StringPairDistinct []stringPair `json:"string_pair_distinct"`
}

type fixture struct {
	Constants          constants       `json:"constants"`
	GoChecksVerified   int             `json:"go_checks_verified"`
	U64Seeds           []uint64        `json:"u64_seeds"`
	U128Seeds          []u128Seed      `json:"u128_seeds"`
	ByteCasesHex       []string        `json:"byte_cases_hex"`
	AllZerosDistinct   []string        `json:"all_zeros_distinct"`
	TrailingZerosGroup []string        `json:"trailing_zeros_group"`
	SingleBitFlips     []string        `json:"single_bit_flips"`
	StringCases        []string        `json:"string_cases"`
	SplitCases         []splitCase     `json:"split_cases"`
	Comparable         comparableCases `json:"comparable"`
}

var checks int

func require(cond bool, msg string) {
	if !cond {
		panic("maphash contract violation: " + msg)
	}
	checks++
}

func makePattern(n int) []byte {
	out := make([]byte, n)
	for i := 0; i < n; i++ {
		out[i] = byte((i*131 + i/7*17 + 23) & 0xff)
	}
	return out
}

func verifyByteContracts(seed1, seed2 maphash.Seed, data []byte) {
	oneShot := maphash.Bytes(seed1, data)
	require(oneShot == maphash.Bytes(seed1, data), "determinism for same seed")

	// Streaming single Write.
	var h maphash.Hash
	h.SetSeed(seed1)
	require(h.Seed() == seed1, "Seed() matches SetSeed()")
	n, err := h.Write(data)
	require(err == nil && n == len(data), "Write length and nil error")
	require(h.Sum64() == oneShot, "Write + Sum64 equals Bytes")
	// Calling Sum64 twice does not alter state.
	require(h.Sum64() == oneShot, "Sum64 is non-destructive")

	// Sum appends 8 bytes without mutating prefix.
	prefix := []byte{0xde, 0xad, 0xbe, 0xef}
	summed := h.Sum(prefix)
	require(len(summed) == len(prefix)+8, "Sum appends 8 bytes")
	require(bytes.Equal(summed[:len(prefix)], prefix), "Sum preserves prefix")

	// Reset discards written data and preserves seed.
	h.Reset()
	require(h.Seed() == seed1, "Reset preserves seed")
	require(h.Sum64() == maphash.Bytes(seed1, nil), "Reset restores empty hash")

	// SetSeed discards written data and switches seed.
	_, _ = h.Write(data)
	h.SetSeed(seed2)
	require(h.Seed() == seed2, "SetSeed updates seed")
	require(h.Sum64() == maphash.Bytes(seed2, nil), "SetSeed discards buffered bytes")
	_, _ = h.Write(data)
	require(h.Sum64() == maphash.Bytes(seed2, data), "Write after SetSeed uses new seed")
	require(maphash.Bytes(seed1, data) != maphash.Bytes(seed2, data), "distinct seeds yield distinct hashes")

	// Byte-by-byte WriteByte invariance (for reasonable lengths).
	if len(data) <= 260 {
		var hb maphash.Hash
		hb.SetSeed(seed1)
		for _, b := range data {
			require(hb.WriteByte(b) == nil, "WriteByte nil error")
		}
		require(hb.Sum64() == oneShot, "WriteByte sequence equals Bytes")
	}

	// Exhaustive 2-way split invariance for lengths <= 130.
	if len(data) <= 130 {
		for split := 0; split <= len(data); split++ {
			var hs maphash.Hash
			hs.SetSeed(seed1)
			_, _ = hs.Write(data[:split])
			_, _ = hs.Write(data[split:])
			require(hs.Sum64() == oneShot, fmt.Sprintf("2-way split %d/%d", split, len(data)))
		}
	}
}

func verifyAllDistinctBytes(seed maphash.Seed, inputs [][]byte, label string) {
	seen := make(map[uint64]int, len(inputs))
	for i, b := range inputs {
		d := maphash.Bytes(seed, b)
		if prev, ok := seen[d]; ok {
			panic(fmt.Sprintf("unexpected collision in %s between %d and %d", label, prev, i))
		}
		seen[d] = i
		checks++
	}
}

func main() {
	seed1 := maphash.MakeSeed()
	seed2 := maphash.MakeSeed()
	for seed2 == seed1 {
		seed2 = maphash.MakeSeed()
	}

	var h0 maphash.Hash
	h0.SetSeed(seed1)
	require(h0.Size() == 8, "Size() == 8")
	require(h0.BlockSize() == 128, "BlockSize() == 128")

	// Byte lengths covering every boundary around 4, 8, 16, 32, 64, 128, 256, 512, 1024.
	lengths := []int{
		0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
		23, 24, 25, 31, 32, 33, 47, 48, 49, 63, 64, 65,
		126, 127, 128, 129, 130, 135, 136, 137,
		255, 256, 257, 512, 1024,
	}
	byteCasesHex := make([]string, 0, len(lengths))
	for _, n := range lengths {
		p := makePattern(n)
		verifyByteContracts(seed1, seed2, p)
		byteCasesHex = append(byteCasesHex, hex.EncodeToString(p))
	}

	// All-zero buffers of distinct lengths must all hash distinctly.
	zeroLens := []int{0, 1, 2, 3, 4, 7, 8, 9, 15, 16, 17, 32, 64, 128, 129}
	zeroBufs := make([][]byte, 0, len(zeroLens))
	zeroHex := make([]string, 0, len(zeroLens))
	for _, n := range zeroLens {
		b := make([]byte, n)
		verifyByteContracts(seed1, seed2, b)
		zeroBufs = append(zeroBufs, b)
		zeroHex = append(zeroHex, hex.EncodeToString(b))
	}
	verifyAllDistinctBytes(seed1, zeroBufs, "all_zeros")

	// Trailing-zero sensitivity.
	tzBufs := [][]byte{
		{1},
		{1, 0},
		{1, 0, 0},
		{1, 0, 0, 0},
		{1, 0, 0, 0, 0, 0, 0, 0},
		{1, 0, 0, 0, 0, 0, 0, 0, 0},
		{1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
	}
	tzHex := make([]string, 0, len(tzBufs))
	for _, b := range tzBufs {
		verifyByteContracts(seed1, seed2, b)
		tzHex = append(tzHex, hex.EncodeToString(b))
	}
	verifyAllDistinctBytes(seed1, tzBufs, "trailing_zeros")

	// Single-bit flips across a 32-byte message.
	base := makePattern(32)
	flipBufs := [][]byte{append([]byte(nil), base...)}
	flipPositions := [][2]int{
		{0, 0}, {0, 7}, {1, 3}, {3, 5}, {7, 7},
		{8, 0}, {8, 7}, {15, 4}, {16, 0}, {23, 6}, {24, 1}, {31, 0}, {31, 7},
	}
	for _, fp := range flipPositions {
		c := append([]byte(nil), base...)
		c[fp[0]] ^= 1 << fp[1]
		flipBufs = append(flipBufs, c)
	}
	verifyAllDistinctBytes(seed1, flipBufs, "single_bit_flips")
	flipHex := make([]string, 0, len(flipBufs))
	for _, b := range flipBufs {
		flipHex = append(flipHex, hex.EncodeToString(b))
	}

	// String cases including ASCII, UTF-8 multibyte, embedded NUL, and multi-block lengths.
	stringCases := []string{
		"",
		"a",
		"foo",
		"hello, world",
		"The quick brown fox jumps over the lazy dog",
		"héllo",
		"日本語テスト",
		"🦀🔥🚀",
		"\x00",
		"\x00\x00",
		"01234567",
		"012345678",
		string(bytes.Repeat([]byte("abcdefgh"), 16)),
		string(bytes.Repeat([]byte("abcdefgh"), 16)) + "!",
		string(bytes.Repeat([]byte("0123456789abcdef"), 16)) + "Z",
	}
	for _, s := range stringCases {
		hStr := maphash.String(seed1, s)
		require(hStr == maphash.Bytes(seed1, []byte(s)), "String equals Bytes")
		var hs maphash.Hash
		hs.SetSeed(seed1)
		n, err := hs.WriteString(s)
		require(err == nil && n == len(s), "WriteString length and nil error")
		require(hs.Sum64() == hStr, "WriteString + Sum64 equals String")
		require(maphash.String(seed1, s) != maphash.String(seed2, s), "String distinct across seeds")
	}

	// Multi-chunk split patterns.
	makeSplit := func(name string, total int, sizes []int) splitCase {
		full := makePattern(total)
		chunks := make([]string, 0, len(sizes)+1)
		pos := 0
		var hs maphash.Hash
		hs.SetSeed(seed1)
		for _, sz := range sizes {
			if pos >= len(full) && sz > 0 {
				break
			}
			end := pos + sz
			if end > len(full) {
				end = len(full)
			}
			chunk := full[pos:end]
			chunks = append(chunks, hex.EncodeToString(chunk))
			_, _ = hs.Write(chunk)
			pos = end
		}
		if pos < len(full) {
			chunk := full[pos:]
			chunks = append(chunks, hex.EncodeToString(chunk))
			_, _ = hs.Write(chunk)
		}
		require(hs.Sum64() == maphash.Bytes(seed1, full), "split case "+name)
		return splitCase{
			Name:     name,
			InputHex: hex.EncodeToString(full),
			Chunks:   chunks,
		}
	}

	splitCases := []splitCase{
		makeSplit("empty_chunks_around_word", 16, []int{0, 8, 0, 8, 0}),
		makeSplit("unaligned_primes_100", 100, []int{1, 2, 3, 5, 7, 11, 13, 17, 19, 22}),
		makeSplit("block_128_straddle", 258, []int{127, 1, 1, 127, 2}),
		makeSplit("seven_nine_alternating", 64, []int{7, 9, 7, 9, 7, 9, 7, 9}),
		makeSplit("single_bytes_20", 20, []int{1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1}),
		makeSplit("leading_three_then_bulk", 200, []int{3, 194, 3}),
		makeSplit("tail_boundaries_257", 257, []int{4, 5, 6, 7, 8, 9, 15, 16, 17, 31, 32, 33, 64, 10}),
	}

	// Comparable contracts in Go 1.24.
	f64PosZero := math.Float64frombits(0x0000000000000000)
	f64NegZero := math.Float64frombits(0x8000000000000000)
	require(maphash.Comparable(seed1, f64PosZero) == maphash.Comparable(seed1, f64NegZero), "Comparable F64 +0.0 == -0.0")
	f32PosZero := math.Float32frombits(0x00000000)
	f32NegZero := math.Float32frombits(0x80000000)
	require(maphash.Comparable(seed1, f32PosZero) == maphash.Comparable(seed1, f32NegZero), "Comparable F32 +0.0 == -0.0")

	f64DistinctBits := []uint64{
		0x0000000000000000, // +0.0
		0x3ff0000000000000, // 1.0
		0xbff0000000000000, // -1.0
		0x3fe0000000000000, // 0.5
		0xbfe0000000000000, // -0.5
		0x7ff0000000000000, // +Inf
		0xfff0000000000000, // -Inf
		0x0000000000000001, // smallest positive subnormal
		0x7fefffffffffffff, // max finite
	}
	{
		seen := make(map[uint64]int)
		for i, bits := range f64DistinctBits {
			v := math.Float64frombits(bits)
			d := maphash.Comparable(seed1, v)
			var hw maphash.Hash
			hw.SetSeed(seed1)
			maphash.WriteComparable(&hw, v)
			require(hw.Sum64() == d, "WriteComparable F64 matches Comparable")
			require(seen[d] == 0, "F64 distinct")
			seen[d] = i + 1
		}
	}

	f32DistinctBits := []uint32{
		0x00000000, // +0.0
		0x3f800000, // 1.0
		0xbf800000, // -1.0
		0x3f000000, // 0.5
		0xbf000000, // -0.5
		0x7f800000, // +Inf
		0xff800000, // -Inf
		0x00000001, // smallest positive subnormal
		0x7f7fffff, // max finite
	}
	{
		seen := make(map[uint64]int)
		for i, bits := range f32DistinctBits {
			v := math.Float32frombits(bits)
			d := maphash.Comparable(seed1, v)
			var hw maphash.Hash
			hw.SetSeed(seed1)
			maphash.WriteComparable(&hw, v)
			require(hw.Sum64() == d, "WriteComparable F32 matches Comparable")
			require(seen[d] == 0, "F32 distinct")
			seen[d] = i + 1
		}
	}

	i64Distinct := []int64{
		math.MinInt64, -1000000, -1, 0, 1, 2, 255, 256, 65535, 65536, 4294967295, 4294967296, math.MaxInt64,
	}
	{
		seen := make(map[uint64]int)
		for i, v := range i64Distinct {
			d := maphash.Comparable(seed1, v)
			require(seen[d] == 0, "I64 distinct")
			seen[d] = i + 1
		}
	}

	u64Distinct := []uint64{
		0, 1, 2, 255, 256, 65535, 65536, 4294967295, 4294967296, math.MaxInt64, 1 << 63, math.MaxUint64,
	}
	{
		seen := make(map[uint64]int)
		for i, v := range u64Distinct {
			d := maphash.Comparable(seed1, v)
			require(seen[d] == 0, "U64 distinct")
			seen[d] = i + 1
		}
	}

	pairDistinct := []stringPair{
		{"ab", "c"},
		{"a", "bc"},
		{"", "abc"},
		{"abc", ""},
		{"", ""},
		{"", " "},
		{" ", ""},
	}
	{
		seen := make(map[uint64]int)
		for i, p := range pairDistinct {
			d := maphash.Comparable(seed1, p)
			var hw maphash.Hash
			hw.SetSeed(seed1)
			maphash.WriteComparable(&hw, p)
			require(hw.Sum64() == d, "WriteComparable struct pair matches Comparable")
			require(seen[d] == 0, "stringPair distinct")
			seen[d] = i + 1
		}
	}

	// Verify WriteComparable after buffered Write does not commute.
	{
		var h1, h2 maphash.Hash
		h1.SetSeed(seed1)
		h2.SetSeed(seed1)
		_, _ = h1.WriteString("a")
		maphash.WriteComparable(&h1, "b")
		maphash.WriteComparable(&h2, "a")
		_, _ = h2.WriteString("b")
		require(h1.Sum64() != h2.Sum64(), "Write + WriteComparable non-commuting")
	}

	out := fixture{
		Constants:        constants{Size: h0.Size(), BlockSize: h0.BlockSize()},
		GoChecksVerified: checks,
		U64Seeds: []uint64{
			0,
			1,
			2,
			0x0123456789abcdef,
			0x0123456789abcdee,
			0xffffffffffffffff,
		},
		U128Seeds: []u128Seed{
			{Hi: 0, Lo: 0},
			{Hi: 0, Lo: 1},
			{Hi: 1, Lo: 0},
			{Hi: 0x0123456789abcdef, Lo: 0xfedcba9876543210},
			{Hi: 0x0123456789abcdef, Lo: 0xfedcba9876543211},
			{Hi: 0xffffffffffffffff, Lo: 0xffffffffffffffff},
		},
		ByteCasesHex:       byteCasesHex,
		AllZerosDistinct:   zeroHex,
		TrailingZerosGroup: tzHex,
		SingleBitFlips:     flipHex,
		StringCases:        stringCases,
		SplitCases:         splitCases,
		Comparable: comparableCases{
			F64ZeroBits:        []uint64{0x0000000000000000, 0x8000000000000000},
			F32ZeroBits:        []uint32{0x00000000, 0x80000000},
			F64DistinctBits:    f64DistinctBits,
			F32DistinctBits:    f32DistinctBits,
			I64Distinct:        i64Distinct,
			U64Distinct:        u64Distinct,
			StringPairDistinct: pairDistinct,
		},
	}

	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	if err := enc.Encode(out); err != nil {
		panic(err)
	}
}
