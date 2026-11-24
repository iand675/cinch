// Generates reference Go encoding/gob wire vectors for Cinch Text.Gob and Effect.Gob tests.
// Run: GOCACHE=/tmp/go-cache GOTELEMETRY=off go run tooling/gob_vectors.go > seed/tests/fixtures/gob-vectors.json
package main

import (
	"bytes"
	"encoding/gob"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"math"
	"os"
	"strings"
)

type Empty struct{}

type Point struct {
	X int64
	Y int64
}

type Inner struct {
	Code  int64
	Label string
}

type Outer struct {
	Title string
	Inner Inner
	Count int64
	Extra Inner
}

type Person struct {
	Name   string
	Age    int64
	Active bool
	Score  float64
	Tag    []byte
	Scores []int64
	Meta   map[string]string
}

type WithArray struct {
	Id    int64
	Fixed [3]int64
	Note  string
}

type WithComplex struct {
	C     complex128
	ZeroC complex128
	U     uint64
}

type Vector struct {
	Name     string `json:"name"`
	Category string `json:"category"`
	Hex      string `json:"hex"`
	Expected any    `json:"expected"`
}

type InvalidVector struct {
	Name   string `json:"name"`
	Hex    string `json:"hex"`
	Reason string `json:"reason"`
}

type Fixture struct {
	Vectors []Vector        `json:"vectors"`
	Invalid []InvalidVector `json:"invalid"`
}

func encodeOne(v any) string {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	if err := enc.Encode(v); err != nil {
		panic(fmt.Sprintf("encode failed: %v", err))
	}
	return hex.EncodeToString(buf.Bytes())
}

func encodeStream(vals ...any) string {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	for i, v := range vals {
		if err := enc.Encode(v); err != nil {
			panic(fmt.Sprintf("encode stream item %d failed: %v", i, err))
		}
	}
	return hex.EncodeToString(buf.Bytes())
}

func floatBitsHex(f float64) string {
	return fmt.Sprintf("%016x", math.Float64bits(f))
}

func main() {
	longStr := strings.Repeat("gob-wire-test-", 12) // 168 bytes (> 127)
	longBytes := make([]byte, 200)
	for i := range longBytes {
		longBytes[i] = byte((i*37 + 13) & 0xff)
	}

	vectors := []Vector{
		// Booleans
		{Name: "bool_false", Category: "bool", Hex: encodeOne(false), Expected: false},
		{Name: "bool_true", Category: "bool", Hex: encodeOne(true), Expected: true},

		// Signed integers
		{Name: "int_0", Category: "int", Hex: encodeOne(int64(0)), Expected: "0"},
		{Name: "int_1", Category: "int", Hex: encodeOne(int64(1)), Expected: "1"},
		{Name: "int_neg1", Category: "int", Hex: encodeOne(int64(-1)), Expected: "-1"},
		{Name: "int_63", Category: "int", Hex: encodeOne(int64(63)), Expected: "63"},
		{Name: "int_64", Category: "int", Hex: encodeOne(int64(64)), Expected: "64"},
		{Name: "int_neg64", Category: "int", Hex: encodeOne(int64(-64)), Expected: "-64"},
		{Name: "int_neg65", Category: "int", Hex: encodeOne(int64(-65)), Expected: "-65"},
		{Name: "int_127", Category: "int", Hex: encodeOne(int64(127)), Expected: "127"},
		{Name: "int_128", Category: "int", Hex: encodeOne(int64(128)), Expected: "128"},
		{Name: "int_neg129", Category: "int", Hex: encodeOne(int64(-129)), Expected: "-129"},
		{Name: "int_32767", Category: "int", Hex: encodeOne(int64(32767)), Expected: "32767"},
		{Name: "int_neg32768", Category: "int", Hex: encodeOne(int64(-32768)), Expected: "-32768"},
		{Name: "int_million", Category: "int", Hex: encodeOne(int64(1000000)), Expected: "1000000"},
		{Name: "int_neg_million", Category: "int", Hex: encodeOne(int64(-1000000)), Expected: "-1000000"},
		{Name: "int_max64", Category: "int", Hex: encodeOne(int64(math.MaxInt64)), Expected: "9223372036854775807"},
		{Name: "int_min64", Category: "int", Hex: encodeOne(int64(math.MinInt64)), Expected: "-9223372036854775808"},

		// Unsigned integers
		{Name: "uint_0", Category: "uint", Hex: encodeOne(uint64(0)), Expected: "0"},
		{Name: "uint_1", Category: "uint", Hex: encodeOne(uint64(1)), Expected: "1"},
		{Name: "uint_127", Category: "uint", Hex: encodeOne(uint64(127)), Expected: "127"},
		{Name: "uint_128", Category: "uint", Hex: encodeOne(uint64(128)), Expected: "128"},
		{Name: "uint_255", Category: "uint", Hex: encodeOne(uint64(255)), Expected: "255"},
		{Name: "uint_256", Category: "uint", Hex: encodeOne(uint64(256)), Expected: "256"},
		{Name: "uint_65535", Category: "uint", Hex: encodeOne(uint64(65535)), Expected: "65535"},
		{Name: "uint_65536", Category: "uint", Hex: encodeOne(uint64(65536)), Expected: "65536"},
		{Name: "uint_u32max", Category: "uint", Hex: encodeOne(uint64(math.MaxUint32)), Expected: "4294967295"},
		{Name: "uint_max64", Category: "uint", Hex: encodeOne(uint64(math.MaxUint64)), Expected: "18446744073709551615"},

		// Float64 (exact IEEE-754 bit hex for bit-for-bit verification)
		{Name: "float_pos0", Category: "float", Hex: encodeOne(float64(0.0)), Expected: floatBitsHex(0.0)},
		{Name: "float_neg0", Category: "float", Hex: encodeOne(math.Copysign(0.0, -1)), Expected: floatBitsHex(math.Copysign(0.0, -1))},
		{Name: "float_1", Category: "float", Hex: encodeOne(float64(1.0)), Expected: floatBitsHex(1.0)},
		{Name: "float_neg1", Category: "float", Hex: encodeOne(float64(-1.0)), Expected: floatBitsHex(-1.0)},
		{Name: "float_17", Category: "float", Hex: encodeOne(float64(17.0)), Expected: floatBitsHex(17.0)},
		{Name: "float_neg3_5", Category: "float", Hex: encodeOne(float64(-3.5)), Expected: floatBitsHex(-3.5)},
		{Name: "float_pi", Category: "float", Hex: encodeOne(math.Pi), Expected: floatBitsHex(math.Pi)},
		{Name: "float_max", Category: "float", Hex: encodeOne(math.MaxFloat64), Expected: floatBitsHex(math.MaxFloat64)},
		{Name: "float_subnormal", Category: "float", Hex: encodeOne(math.SmallestNonzeroFloat64), Expected: floatBitsHex(math.SmallestNonzeroFloat64)},
		{Name: "float_pos_inf", Category: "float", Hex: encodeOne(math.Inf(1)), Expected: floatBitsHex(math.Inf(1))},
		{Name: "float_neg_inf", Category: "float", Hex: encodeOne(math.Inf(-1)), Expected: floatBitsHex(math.Inf(-1))},
		{Name: "float_nan", Category: "float", Hex: encodeOne(math.NaN()), Expected: floatBitsHex(math.NaN())},

		// Strings
		{Name: "string_empty", Category: "string", Hex: encodeOne(""), Expected: ""},
		{Name: "string_ascii", Category: "string", Hex: encodeOne("hello, gob!"), Expected: "hello, gob!"},
		{Name: "string_utf8", Category: "string", Hex: encodeOne("日本語🚀é"), Expected: "日本語🚀é"},
		{Name: "string_long", Category: "string", Hex: encodeOne(longStr), Expected: longStr},

		// Bytes
		{Name: "bytes_empty", Category: "bytes", Hex: encodeOne([]byte{}), Expected: ""},
		{Name: "bytes_single_zero", Category: "bytes", Hex: encodeOne([]byte{0}), Expected: "00"},
		{Name: "bytes_mixed", Category: "bytes", Hex: encodeOne([]byte{0, 1, 127, 128, 255}), Expected: "00017f80ff"},
		{Name: "bytes_long", Category: "bytes", Hex: encodeOne(longBytes), Expected: hex.EncodeToString(longBytes)},

		// Complex128
		{Name: "complex_zero", Category: "complex", Hex: encodeOne(complex(0, 0)), Expected: []string{floatBitsHex(0), floatBitsHex(0)}},
		{Name: "complex_1_2", Category: "complex", Hex: encodeOne(complex(1, 2)), Expected: []string{floatBitsHex(1), floatBitsHex(2)}},
		{Name: "complex_neg", Category: "complex", Hex: encodeOne(complex(-3.5, 4.25)), Expected: []string{floatBitsHex(-3.5), floatBitsHex(4.25)}},
		{Name: "complex_imag_only", Category: "complex", Hex: encodeOne(complex(0, -1.5)), Expected: []string{floatBitsHex(0), floatBitsHex(-1.5)}},

		// Slices
		{Name: "slice_int_empty", Category: "slice_int", Hex: encodeOne([]int64{}), Expected: []string{}},
		{Name: "slice_int_vals", Category: "slice_int", Hex: encodeOne([]int64{0, 1, -1, 127, -129, 999999}), Expected: []string{"0", "1", "-1", "127", "-129", "999999"}},
		{Name: "slice_str", Category: "slice_str", Hex: encodeOne([]string{"", "alpha", "beta", ""}), Expected: []string{"", "alpha", "beta", ""}},
		{Name: "slice_nested", Category: "slice_nested", Hex: encodeOne([][]int64{{}, {1, 2}, {0, -3}}), Expected: [][]string{{}, {"1", "2"}, {"0", "-3"}}},

		// Fixed arrays
		{Name: "array_int4", Category: "array_int", Hex: encodeOne([4]int64{0, 10, -20, 30}), Expected: []string{"0", "10", "-20", "30"}},
		{Name: "array_byte4", Category: "array_uint", Hex: encodeOne([4]byte{0, 1, 128, 255}), Expected: []string{"0", "1", "128", "255"}},
		{Name: "array_int0", Category: "array_int", Hex: encodeOne([0]int64{}), Expected: []string{}},

		// Maps
		{Name: "map_str_int_empty", Category: "map_str_int", Hex: encodeOne(map[string]int64{}), Expected: map[string]string{}},
		{Name: "map_str_int_one", Category: "map_str_int", Hex: encodeOne(map[string]int64{"only": 42}), Expected: map[string]string{"only": "42"}},
		{Name: "map_str_int_zero_val", Category: "map_str_int", Hex: encodeOne(map[string]int64{"zero": 0}), Expected: map[string]string{"zero": "0"}},
		{Name: "map_str_int_multi", Category: "map_str_int", Hex: encodeOne(map[string]int64{"a": 1, "b": -2, "c": 0}), Expected: map[string]string{"a": "1", "b": "-2", "c": "0"}},
		{Name: "map_int_str", Category: "map_int_str", Hex: encodeOne(map[int64]string{-5: "neg", 0: "zero", 10: "ten"}), Expected: map[string]string{"-5": "neg", "0": "zero", "10": "ten"}},

		// Structs
		{Name: "struct_empty", Category: "struct_empty", Hex: encodeOne(Empty{}), Expected: map[string]any{}},
		{Name: "struct_point_zero", Category: "struct_point", Hex: encodeOne(Point{X: 0, Y: 0}), Expected: map[string]string{"X": "0", "Y": "0"}},
		{Name: "struct_point_both", Category: "struct_point", Hex: encodeOne(Point{X: 10, Y: -20}), Expected: map[string]string{"X": "10", "Y": "-20"}},
		{Name: "struct_point_omitted_first", Category: "struct_point", Hex: encodeOne(Point{X: 0, Y: 99}), Expected: map[string]string{"X": "0", "Y": "99"}},
		{
			Name:     "struct_outer_nested",
			Category: "struct_outer",
			Hex:      encodeOne(Outer{Title: "root", Inner: Inner{Code: 0, Label: ""}, Count: 5, Extra: Inner{Code: 7, Label: "ok"}}),
			Expected: map[string]any{
				"Title": "root",
				"Inner": map[string]any{"Code": "0", "Label": ""},
				"Count": "5",
				"Extra": map[string]any{"Code": "7", "Label": "ok"},
			},
		},
		{
			Name:     "struct_person_full",
			Category: "struct_person",
			Hex: encodeOne(Person{
				Name:   "Ada",
				Age:    36,
				Active: true,
				Score:  98.5,
				Tag:    []byte{0xDE, 0xAD},
				Scores: []int64{10, 0, 20},
				Meta:   map[string]string{"role": "eng"},
			}),
			Expected: map[string]any{
				"Name":   "Ada",
				"Age":    "36",
				"Active": true,
				"Score":  floatBitsHex(98.5),
				"Tag":    "dead",
				"Scores": []string{"10", "0", "20"},
				"Meta":   map[string]string{"role": "eng"},
			},
		},
		{
			Name:     "struct_person_zero",
			Category: "struct_person",
			Hex:      encodeOne(Person{}),
			Expected: map[string]any{
				"Name":   "",
				"Age":    "0",
				"Active": false,
				"Score":  floatBitsHex(0.0),
				"Tag":    "",
				"Scores": []string{},
				"Meta":   map[string]string{},
			},
		},
		{
			Name:     "struct_with_array",
			Category: "struct_with_array",
			Hex:      encodeOne(WithArray{Id: 1, Fixed: [3]int64{0, 0, 0}, Note: "after"}),
			Expected: map[string]any{
				"Id":    "1",
				"Fixed": []string{"0", "0", "0"},
				"Note":  "after",
			},
		},
		{
			Name:     "slice_of_structs",
			Category: "slice_points",
			Hex:      encodeOne([]Point{{X: 1, Y: 2}, {X: 0, Y: 0}, {X: -3, Y: 4}}),
			Expected: []map[string]string{
				{"X": "1", "Y": "2"},
				{"X": "0", "Y": "0"},
				{"X": "-3", "Y": "4"},
			},
		},
		{
			Name:     "struct_with_complex",
			Category: "struct_with_complex",
			Hex:      encodeOne(WithComplex{C: complex(1.5, -2.5), ZeroC: complex(0, 0), U: 42}),
			Expected: map[string]any{
				"C":     []string{floatBitsHex(1.5), floatBitsHex(-2.5)},
				"ZeroC": []string{floatBitsHex(0.0), floatBitsHex(0.0)},
				"U":     "42",
			},
		},

		// Multi-message streams reusing type descriptors
		{
			Name:     "stream_points_reuse",
			Category: "stream_points",
			Hex: encodeStream(
				Point{X: 1, Y: 2},
				Point{X: 0, Y: -5},
				Point{X: 100, Y: 0},
			),
			Expected: []map[string]string{
				{"X": "1", "Y": "2"},
				{"X": "0", "Y": "-5"},
				{"X": "100", "Y": "0"},
			},
		},
		{
			Name:     "stream_mixed_reuse",
			Category: "stream_mixed",
			Hex: encodeStream(
				int64(-42),
				"hello",
				Point{X: 3, Y: 4},
				true,
				Point{X: 5, Y: 6},
				[]int64{7, 8},
				[]int64{9},
			),
			Expected: []any{"-42", "hello", "3,4", true, "5,6", "7,8", "9"},
		},
	}

	// Build a valid Point type definition message so we can craft targeted invalid streams too.
	pointWire := encodeOne(Point{X: 1, Y: 2})
	pointBytes, _ := hex.DecodeString(pointWire)
	typeDefLen := int(pointBytes[0]) + 1
	typeDefOnly := hex.EncodeToString(pointBytes[:typeDefLen])
	dupTypeDef := typeDefOnly + typeDefOnly + hex.EncodeToString(pointBytes[typeDefLen:])
	// Extract the positive typeId varint bytes from the Point value message (starts at typeDefLen+1).
	valStart := typeDefLen + 1
	var idBytes []byte
	if pointBytes[valStart] <= 0x7f {
		idBytes = pointBytes[valStart : valStart+1]
	} else {
		n := -int(int8(pointBytes[valStart]))
		idBytes = pointBytes[valStart : valStart+1+n]
	}
	// Out-of-bounds field delta (5 -> field index 4 on 2-field Point)
	oobPayload := append(append([]byte{}, idBytes...), 0x05, 0x02, 0x00)
	oobMsg := append([]byte{byte(len(oobPayload))}, oobPayload...)
	// Truncated struct missing 0x00 terminator (field 0 = 1, then EOF)
	noTermPayload := append(append([]byte{}, idBytes...), 0x01, 0x02)
	noTermMsg := append([]byte{byte(len(noTermPayload))}, noTermPayload...)

	invalid := []InvalidVector{
		{Name: "empty_input", Hex: "", Reason: "truncated"},
		{Name: "truncated_len_header", Hex: "fe01", Reason: "truncated"},
		{Name: "overflow_uint_header", Hex: "f0010203040506070809", Reason: "overflow"},
		{Name: "truncated_payload", Hex: "0a040002", Reason: "truncated"},
		{Name: "zero_length_message", Hex: "00", Reason: "malformed"},
		{Name: "unknown_type_id_64", Hex: "03ff8000", Reason: "unknown_type"},
		{Name: "unknown_type_id_0", Hex: "020000", Reason: "unknown_type"},
		{Name: "reserved_type_def_below_64", Hex: "0b2703010101580128000000", Reason: "malformed"},
		{Name: "type_def_without_value", Hex: typeDefOnly, Reason: "truncated"},
		{Name: "duplicate_type_def", Hex: dupTypeDef, Reason: "malformed"},
		{Name: "nonzero_singleton_delimiter", Hex: "03040102", Reason: "malformed"},
		{Name: "trailing_bytes_in_frame", Hex: "0404000299", Reason: "malformed"},
		{Name: "invalid_utf8_string", Hex: "050c0002fffe", Reason: "malformed"},
		{Name: "truncated_string_body", Hex: "040c000561", Reason: "truncated"},
		// Type 65 = Slice of Type 65 (cyclic type definition!), followed by value of Type 65:
		{Name: "cyclic_wire_type", Hex: "0dff81020102ff820001ff82000004ff820000", Reason: "depth_limit"},
		// Struct field number out of bounds on Point:
		{Name: "struct_field_oob", Hex: typeDefOnly + hex.EncodeToString(oobMsg), Reason: "malformed"},
		// Struct missing terminating 0x00 byte:
		{Name: "struct_missing_terminator", Hex: typeDefOnly + hex.EncodeToString(noTermMsg), Reason: "truncated"},
		// Array length mismatch: define [3]int as type 65, send array with length 2:
		{Name: "array_length_mismatch", Hex: "0eff81010102ff820001040106000006ff8200020000", Reason: "malformed"},
	}

	fixture := Fixture{Vectors: vectors, Invalid: invalid}
	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	if err := enc.Encode(fixture); err != nil {
		panic(err)
	}
}
