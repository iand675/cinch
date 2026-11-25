//go:build ignore

// Generate independent reference test vectors from Go's standard library
// strconv package for Cinch's Text.Strconv differential test suite.
//
// Usage:
//
//	GOCACHE=/tmp/go-cache GOTELEMETRY=off go run tooling/strconv_vectors.go > seed/tests/fixtures/strconv-vectors.json
package main

import (
	"encoding/hex"
	"encoding/json"
	"errors"
	"math"
	"os"
	"strconv"
	"strings"
)

type ParseBoolVec struct {
	Input string `json:"input"`
	Value bool   `json:"value"`
	Err   string `json:"err"`
}

type FormatBoolVec struct {
	Input bool   `json:"input"`
	Out   string `json:"out"`
}

type ParseIntVec struct {
	Input   string `json:"input"`
	Base    int    `json:"base"`
	BitSize int    `json:"bit_size"`
	Value   int64  `json:"value"`
	Err     string `json:"err"`
}

type ParseUintVec struct {
	Input   string `json:"input"`
	Base    int    `json:"base"`
	BitSize int    `json:"bit_size"`
	Value   uint64 `json:"value"`
	Err     string `json:"err"`
}

type AtoiVec struct {
	Input string `json:"input"`
	Value int64  `json:"value"`
	Err   string `json:"err"`
}

type FormatIntVec struct {
	Value int64  `json:"value"`
	Base  int    `json:"base"`
	Out   string `json:"out"`
}

type FormatUintVec struct {
	Value uint64 `json:"value"`
	Base  int    `json:"base"`
	Out   string `json:"out"`
}

type ParseFloatVec struct {
	Input   string `json:"input"`
	BitSize int    `json:"bit_size"`
	Bits    uint64 `json:"bits"`
	IsNaN   bool   `json:"is_nan"`
	Err     string `json:"err"`
}

type FormatFloatVec struct {
	Bits    uint64 `json:"bits"`
	Fmt     string `json:"fmt"`
	Prec    int    `json:"prec"`
	BitSize int    `json:"bit_size"`
	Out     string `json:"out"`
}

type QuoteVec struct {
	InputHex  string `json:"input_hex"`
	ValidUTF8 bool   `json:"valid_utf8"`
	Quote     string `json:"quote"`
	ASCII     string `json:"ascii"`
	Graphic   string `json:"graphic"`
}

type QuoteRuneVec struct {
	Code    int32  `json:"code"`
	Quote   string `json:"quote"`
	ASCII   string `json:"ascii"`
	Graphic string `json:"graphic"`
}

type UnquoteVec struct {
	Input     string `json:"input"`
	OutHex    string `json:"out_hex"`
	ValidUTF8 bool   `json:"valid_utf8"`
	Err       string `json:"err"`
}

type QuotedPrefixVec struct {
	Input string `json:"input"`
	Out   string `json:"out"`
	Err   string `json:"err"`
}

type UnquoteCharVec struct {
	Input     string `json:"input"`
	Quote     int32  `json:"quote"`
	Value     int32  `json:"value"`
	Multibyte bool   `json:"multibyte"`
	Tail      string `json:"tail"`
	Err       string `json:"err"`
}

type CanBackquoteVec struct {
	Input string `json:"input"`
	Out   bool   `json:"out"`
}

type RuneClassVec struct {
	Code      int32 `json:"code"`
	IsPrint   bool  `json:"is_print"`
	IsGraphic bool  `json:"is_graphic"`
}

type ParseComplexVec struct {
	Input     string `json:"input"`
	BitSize   int    `json:"bit_size"`
	RealBits  uint64 `json:"real_bits"`
	RealIsNaN bool   `json:"real_is_nan"`
	ImagBits  uint64 `json:"imag_bits"`
	ImagIsNaN bool   `json:"imag_is_nan"`
	Err       string `json:"err"`
}

type FormatComplexVec struct {
	RealBits uint64 `json:"real_bits"`
	ImagBits uint64 `json:"imag_bits"`
	Fmt      string `json:"fmt"`
	Prec     int    `json:"prec"`
	BitSize  int    `json:"bit_size"`
	Out      string `json:"out"`
}

type Vectors struct {
	ParseBool     []ParseBoolVec     `json:"parse_bool"`
	FormatBool    []FormatBoolVec    `json:"format_bool"`
	ParseInt      []ParseIntVec      `json:"parse_int"`
	ParseUint     []ParseUintVec     `json:"parse_uint"`
	Atoi          []AtoiVec          `json:"atoi"`
	FormatInt     []FormatIntVec     `json:"format_int"`
	FormatUint    []FormatUintVec    `json:"format_uint"`
	ParseFloat    []ParseFloatVec    `json:"parse_float"`
	FormatFloat   []FormatFloatVec   `json:"format_float"`
	Quote         []QuoteVec         `json:"quote"`
	QuoteRune     []QuoteRuneVec     `json:"quote_rune"`
	Unquote       []UnquoteVec       `json:"unquote"`
	QuotedPrefix  []QuotedPrefixVec  `json:"quoted_prefix"`
	UnquoteChar   []UnquoteCharVec   `json:"unquote_char"`
	CanBackquote  []CanBackquoteVec  `json:"can_backquote"`
	RuneClass     []RuneClassVec     `json:"rune_class"`
	ParseComplex  []ParseComplexVec  `json:"parse_complex"`
	FormatComplex []FormatComplexVec `json:"format_complex"`
}

func classifyErr(err error) string {
	if err == nil {
		return ""
	}
	if errors.Is(err, strconv.ErrSyntax) {
		return "syntax"
	}
	if errors.Is(err, strconv.ErrRange) {
		return "range"
	}
	msg := err.Error()
	if strings.Contains(msg, "invalid base") {
		return "bad_base"
	}
	if strings.Contains(msg, "invalid bit size") {
		return "bad_bit_size"
	}
	return "syntax"
}

func isValidUTF8(b []byte) bool {
	s := string(b)
	for i := 0; i < len(s); {
		if s[i] < 0x80 {
			i++
			continue
		}
		// Check via Go's utf8 rules by round-tripping through range.
		break
	}
	// Simpler: compare bytes against string reconstructed from runes without U+FFFD mismatches.
	var rebuilt []byte
	for _, r := range string(b) {
		rebuilt = append(rebuilt, string(r)...)
	}
	return string(rebuilt) == string(b)
}

func main() {
	var v Vectors

	// 1. ParseBool / FormatBool
	boolInputs := []string{
		"1", "t", "T", "TRUE", "true", "True",
		"0", "f", "F", "FALSE", "false", "False",
		"", "2", "-1", "yes", "no", " true", "true ", "tRuE", "fAlSe", "00", "11", "on", "off",
	}
	for _, s := range boolInputs {
		val, err := strconv.ParseBool(s)
		v.ParseBool = append(v.ParseBool, ParseBoolVec{Input: s, Value: val, Err: classifyErr(err)})
	}
	for _, b := range []bool{false, true} {
		v.FormatBool = append(v.FormatBool, FormatBoolVec{Input: b, Out: strconv.FormatBool(b)})
	}

	// 2. ParseInt
	intCases := []struct {
		s       string
		base    int
		bitSize int
	}{
		// Basic decimal
		{"0", 10, 64}, {"-0", 10, 64}, {"+0", 10, 64},
		{"42", 10, 64}, {"-42", 10, 64}, {"+42", 10, 64},
		{"1234567890", 10, 0}, {"-1234567890", 10, 0},
		// Bit sizes & clamping
		{"127", 10, 8}, {"-128", 10, 8}, {"128", 10, 8}, {"-129", 10, 8}, {"300", 10, 8}, {"-300", 10, 8},
		{"32767", 10, 16}, {"-32768", 10, 16}, {"32768", 10, 16}, {"-32769", 10, 16},
		{"2147483647", 10, 32}, {"-2147483648", 10, 32}, {"2147483648", 10, 32}, {"-2147483649", 10, 32},
		{"9223372036854775807", 10, 64}, {"-9223372036854775808", 10, 64},
		{"9223372036854775808", 10, 64}, {"-9223372036854775809", 10, 64},
		{"18446744073709551615", 10, 64}, {"-18446744073709551616", 10, 64},
		{"999999999999999999999999999999", 10, 64}, {"-999999999999999999999999999999", 10, 64},
		{"999999999999999999999999999999", 10, 8}, {"-999999999999999999999999999999", 10, 8},
		{"0", 10, 1}, {"-1", 10, 1}, {"1", 10, 1}, {"-2", 10, 1},
		{"7", 10, 4}, {"-8", 10, 4}, {"8", 10, 4}, {"-9", 10, 4},
		// Explicit bases
		{"101010", 2, 64}, {"-11111111", 2, 16}, {"102", 2, 64},
		{"755", 8, 64}, {"-777", 8, 64}, {"789", 8, 64},
		{"deadbeef", 16, 64}, {"-DEADBEEF", 16, 64}, {"7fffffffffffffff", 16, 64}, {"-8000000000000000", 16, 64},
		{"8000000000000000", 16, 64}, {"-8000000000000001", 16, 64},
		{"z", 36, 64}, {"Z", 36, 64}, {"1y2p0ij32e8e7", 36, 64}, {"-1y2p0ij32e8e8", 36, 64}, {"1y2p0ij32e8e8", 36, 64},
		{"10", 3, 64}, {"22", 3, 64}, {"23", 3, 64},
		// Base 0 prefixes
		{"0", 0, 64}, {"-0", 0, 64}, {"+0", 0, 64},
		{"0b1010", 0, 64}, {"-0B1111", 0, 64}, {"+0b0", 0, 64}, {"0b", 0, 64}, {"0b2", 0, 64},
		{"0o755", 0, 64}, {"-0O10", 0, 64}, {"0o", 0, 64}, {"0o8", 0, 64},
		{"0755", 0, 64}, {"-0777", 0, 64}, {"00", 0, 64}, {"08", 0, 64}, {"09", 0, 64},
		{"0x1f", 0, 64}, {"-0XFF", 0, 64}, {"+0x0", 0, 64}, {"0x", 0, 64}, {"0xg", 0, 64},
		// Base 0 underscores
		{"1_000_000", 0, 64}, {"-1_2_3", 0, 64}, {"+0_1_0", 0, 64},
		{"0x_dead_beef", 0, 64}, {"-0X_1_A", 0, 64},
		{"0b_1010_0101", 0, 64}, {"0o_7_5_5", 0, 64}, {"0_7_5_5", 0, 64}, {"0_0", 0, 64},
		// Invalid underscores in base 0
		{"_1", 0, 64}, {"1_", 0, 64}, {"1__0", 0, 64}, {"0x1_", 0, 64}, {"0x__1", 0, 64},
		{"0_x1", 0, 64}, {"0b_", 0, 64}, {"0o_", 0, 64}, {"0_", 0, 64},
		{"-_1", 0, 64}, {"+_1", 0, 64}, {"_", 0, 64}, {"-0_", 0, 64}, {"-0x_", 0, 64},
		// Underscores rejected in explicit base
		{"1_000", 10, 64}, {"1_a", 16, 64}, {"1_0", 2, 64},
		// Syntax & parameter errors
		{"", 10, 64}, {"+", 10, 64}, {"-", 10, 64}, {" 1", 10, 64}, {"1 ", 10, 64},
		{"12a", 10, 64}, {"--1", 10, 64}, {"++1", 10, 64}, {"+-1", 10, 64},
		{"-", 99, 64}, {"", 99, 64},
		{"10", 1, 64}, {"10", -1, 64}, {"10", 37, 64},
		{"10", 10, -1}, {"10", 10, 65}, {"10", 10, 128},
		{"10", 99, 99},
	}
	for _, tc := range intCases {
		val, err := strconv.ParseInt(tc.s, tc.base, tc.bitSize)
		v.ParseInt = append(v.ParseInt, ParseIntVec{
			Input: tc.s, Base: tc.base, BitSize: tc.bitSize, Value: val, Err: classifyErr(err),
		})
	}

	// 3. ParseUint
	uintCases := []struct {
		s       string
		base    int
		bitSize int
	}{
		{"0", 10, 64}, {"42", 10, 64}, {"+0", 10, 64}, {"-0", 10, 64}, {"+1", 10, 64}, {"-1", 10, 64},
		{"255", 10, 8}, {"256", 10, 8}, {"999999999999999999999", 10, 8},
		{"65535", 10, 16}, {"65536", 10, 16},
		{"4294967295", 10, 32}, {"4294967296", 10, 32},
		{"18446744073709551615", 10, 64}, {"18446744073709551616", 10, 64},
		{"18446744073709551615", 10, 0}, {"18446744073709551616", 10, 0},
		{"999999999999999999999999999999", 10, 64},
		{"0", 10, 1}, {"1", 10, 1}, {"2", 10, 1},
		{"0xffffffffffffffff", 0, 64}, {"0x10000000000000000", 0, 64},
		{"0x_ff_ff", 0, 16}, {"0b_1111_1111", 0, 8}, {"0o_377", 0, 8}, {"0377", 0, 8},
		{"0_377", 0, 8}, {"0_0", 0, 8}, {"0_", 0, 8}, {"1_000", 0, 64}, {"1_000", 10, 64},
		{"3w5e11264sgsf", 36, 64}, {"3w5e11264sgsg", 36, 64},
		{"", 10, 64}, {"10", 1, 64}, {"10", 37, 64}, {"10", 10, -1}, {"10", 10, 65},
	}
	for _, tc := range uintCases {
		val, err := strconv.ParseUint(tc.s, tc.base, tc.bitSize)
		v.ParseUint = append(v.ParseUint, ParseUintVec{
			Input: tc.s, Base: tc.base, BitSize: tc.bitSize, Value: val, Err: classifyErr(err),
		})
	}

	// 4. Atoi
	atoiCases := []string{
		"0", "-0", "+0", "1", "-1", "+1", "42", "-42",
		"9223372036854775807", "-9223372036854775808",
		"9223372036854775808", "-9223372036854775809",
		"", "+", "-", "1_000", "0x10", " 1", "1 ", "abc", "123abc",
	}
	for _, s := range atoiCases {
		val, err := strconv.Atoi(s)
		v.Atoi = append(v.Atoi, AtoiVec{Input: s, Value: int64(val), Err: classifyErr(err)})
	}

	// 5. FormatInt / FormatUint
	intVals := []int64{0, 1, -1, 9, 10, -10, 35, 36, 99, 100, -100, 42, -42, 255, -256, 65535, -32768, math.MaxInt64, math.MinInt64}
	bases := []int{2, 3, 8, 10, 16, 36}
	for _, val := range intVals {
		for _, base := range bases {
			v.FormatInt = append(v.FormatInt, FormatIntVec{Value: val, Base: base, Out: strconv.FormatInt(val, base)})
		}
	}
	uintVals := []uint64{0, 1, 9, 10, 35, 36, 99, 100, 255, 256, 65535, 1 << 32, math.MaxInt64, 1 << 63, math.MaxUint64}
	for _, val := range uintVals {
		for _, base := range bases {
			v.FormatUint = append(v.FormatUint, FormatUintVec{Value: val, Base: base, Out: strconv.FormatUint(val, base)})
		}
	}

	// 6. ParseFloat (both 64 and 32)
	floatInputs := []string{
		// Specials
		"NaN", "nan", "nAn", "Inf", "+Inf", "-Inf", "inf", "+inf", "-inf",
		"Infinity", "+Infinity", "-Infinity", "INFINITY", "-iNfInItY",
		"+NaN", "-NaN", "Infin", "Infinityx", "nan1", "inf ", " inf",
		// Zeros
		"0", "+0", "-0", "0.0", "-0.0", ".0", "0.", "0e0", "-0e0", "0e99999", "-0e99999",
		"0x0p0", "-0x0p0", "0x0.0p+1000", "-0x0.0p-1000",
		// Normal decimals
		"1", "-1", "+1", "0.5", "-0.5", "0.1", "0.2", "0.3", ".5", "5.", "+.5", "-5.",
		"3.141592653589793", "2.718281828459045", "1e10", "1E-10", "1e+20", "123.456e-7",
		"1e22", "1e23", "1e-22", "1e-23", "12345678901234567890",
		// F64 boundaries & exact midpoints
		"1.7976931348623157e+308",
		"1.7976931348623158e+308",
		"1.79769313486231580793728971405302307e+308",
		"1.79769313486231580793728971405302308e+308",
		"1e309", "-1e309", "1e999999", "-1e999999",
		"5e-324", "4.9406564584124654e-324",
		"2.4703282292062327e-324", "2.4703282292062328e-324",
		"2.2250738585072014e-308", "2.225073858507201e-308",
		"1e-500", "-1e-500", "1e-10000",
		"9007199254740992.5", "9007199254740993.5", "9007199254740992.5000000000000000000001",
		// F32 boundaries & double-rounding traps
		"16777216.5", "16777217.5",
		"1.000000059604644775390625",
		"1.000000059604644775390625000000001",
		"1.000000178813934326171875",
		"7.038531e-26",
		"1.401298464324817e-45",
		"7.006492321624085e-46",
		"7.006492321624086e-46",
		"1.17549435e-38", "1.1754942e-38",
		"3.4028234663852886e+38",
		"3.4028235677973366e+38",
		"3.4028235677973367e+38",
		"1e39", "-1e39", "1e-50",
		// Hex floats
		"0x1p0", "0x1p1", "0x1p-1", "0x1.fp+3", "-0X1.8P-2", "0x.8p0", "0x1.p0", "+0x1.ap+4",
		"0x1.fffffffffffffp+1023", "0x1.fffffffffffff8p+1023", "-0x1.fffffffffffff8p+1023",
		"0x1p-1022", "0x1p-1074", "0x1p-1075", "0x1.0000000000001p-1075",
		"0x1.00000000000008p0", "0x1.00000000000018p0", "0x1.000000000000080000001p0",
		"0x1.fffffep+127", "0x1.ffffffp+127", "0x1p-149", "0x1p-150", "0x1.000001p-150",
		"0x1p2000", "0x1p-2000", "0x1p200", "0x1p-200",
		// Underscores
		"1_000.5_0", "1_2e3_4", "0x_1_a.b_cp+1_0", "0x1_0p-2_0", "1.0_0e+0_1",
		"_1.0", "1_.0", "1._0", "1.0_", "1_e2", "1e_2", "1e+_2", "1e2_", "1__0.0",
		"0x1_.2p3", "0x1._2p3", "0x1.2_p3", "0x1.2p_3", "0x1.2p3_", "0x__1p0",
		// Syntax errors
		"", ".", "+", "-", "1e", "1e+", "1e-", "0x1", "0x.p0", "0x1p", "0x1p+",
		"1.2.3", " 1.0", "1.0 ", "1.0a", "0x1.2p3a",
	}
	for _, bits := range []int{64, 32} {
		for _, s := range floatInputs {
			val, err := strconv.ParseFloat(s, bits)
			var raw uint64
			var isNaN bool
			if bits == 64 {
				isNaN = math.IsNaN(val)
				raw = math.Float64bits(val)
			} else {
				f32 := float32(val)
				isNaN = math.IsNaN(float64(f32))
				raw = uint64(math.Float32bits(f32))
			}
			v.ParseFloat = append(v.ParseFloat, ParseFloatVec{
				Input: s, BitSize: bits, Bits: raw, IsNaN: isNaN, Err: classifyErr(err),
			})
		}
	}

	// 7. FormatFloat (64 and 32)
	negZero64 := math.Copysign(0, -1)
	f64Vals := []float64{
		0.0, negZero64, 1.0, -1.0, 0.5, -0.5, 0.1, 0.2, 0.3, 0.1 + 0.2,
		2.5, 3.5, 0.05, 0.15, 0.25, 1.25, 1.35, 9.999, 99.995,
		3.141592653589793, 123456.789,
		1e-4, 1e-5, 0.000099999, 1e5, 1e6, 1e20, 1e-20, 1e23,
		math.MaxFloat64, -math.MaxFloat64,
		math.SmallestNonzeroFloat64, -math.SmallestNonzeroFloat64,
		math.Float64frombits(0x0010000000000000), // min normal
		math.Float64frombits(0x000fffffffffffff), // max subnormal
		math.Inf(1), math.Inf(-1), math.NaN(),
	}
	fmts := []byte{'f', 'e', 'E', 'g', 'G', 'b', 'x', 'X'}
	precs := []int{-1, 0, 1, 2, 4, 6, 10}
	for _, f := range f64Vals {
		bits := math.Float64bits(f)
		for _, fm := range fmts {
			ps := precs
			if fm == 'b' {
				ps = []int{-1, 0}
			} else if fm == 'f' && (math.Abs(f) > 1e30 || (f != 0 && math.Abs(f) < 1e-30 && !math.IsInf(f, 0))) {
				// Keep vector JSON compact for extreme 'f' exponents, still test prec -1 and 2 on moderate values.
				if math.Abs(f) > 1e100 || (f != 0 && math.Abs(f) < 1e-100) {
					ps = []int{0, 2}
				}
			}
			for _, p := range ps {
				v.FormatFloat = append(v.FormatFloat, FormatFloatVec{
					Bits: bits, Fmt: string(fm), Prec: p, BitSize: 64,
					Out: strconv.FormatFloat(f, fm, p, 64),
				})
			}
		}
	}
	// Also test prec=20 on a few representative f64 values (tests trailing zeros / deep rounding!)
	for _, f := range []float64{0.1, 0.5, 3.141592653589793, 1e-10, 1e10} {
		for _, fm := range []byte{'f', 'e', 'g', 'x'} {
			v.FormatFloat = append(v.FormatFloat, FormatFloatVec{
				Bits: math.Float64bits(f), Fmt: string(fm), Prec: 20, BitSize: 64,
				Out: strconv.FormatFloat(f, fm, 20, 64),
			})
		}
	}

	negZero32 := float32(math.Copysign(0, -1))
	f32Vals := []float32{
		0.0, negZero32, 1.0, -1.0, 0.5, 0.1, 0.3, 2.5, 3.5, 9.999,
		3.1415927, 1e-4, 1e-5, 1e5, 1e6, 1e20, 1e-20,
		math.MaxFloat32, -math.MaxFloat32,
		math.SmallestNonzeroFloat32, -math.SmallestNonzeroFloat32,
		math.Float32frombits(0x00800000), // min normal f32
		math.Float32frombits(0x007fffff), // max subnormal f32
		float32(math.Inf(1)), float32(math.Inf(-1)), float32(math.NaN()),
	}
	for _, f := range f32Vals {
		bits := uint64(math.Float32bits(f))
		for _, fm := range fmts {
			ps := []int{-1, 0, 2, 6}
			if fm == 'b' {
				ps = []int{-1}
			} else if fm == 'f' && (math.Abs(float64(f)) > 1e30 || (f != 0 && math.Abs(float64(f)) < 1e-30 && !math.IsInf(float64(f), 0))) {
				ps = []int{0, 2}
			}
			for _, p := range ps {
				v.FormatFloat = append(v.FormatFloat, FormatFloatVec{
					Bits: bits, Fmt: string(fm), Prec: p, BitSize: 32,
					Out: strconv.FormatFloat(float64(f), fm, p, 32),
				})
			}
		}
	}

	// 8. Quote / QuoteToASCII / QuoteToGraphic (valid UTF-8 + raw non-UTF-8 bytes)
	quoteInputs := [][]byte{
		[]byte(""),
		[]byte("hello world"),
		[]byte(`"quoted" and 'single' and \backslash`),
		[]byte("\a\b\f\n\r\t\v"),
		[]byte{0x00, 0x01, 0x1f, 0x20, 0x7e, 0x7f},
		[]byte("Hello, 世界!"),
		[]byte("\u0080\u009f\u00a0\u00a1\u00ad\u00ff"),
		[]byte("e\u0301 \u2000\u2002\u200b\u3000\ufeff\ufffd"),
		[]byte("emoji: \U0001f600 and nonprint: \U000e0000"),
		// Invalid UTF-8 byte sequences (for quote-bytes*)
		{0x80, 0xff, 0xfe},
		{0x61, 0xc0, 0xaf, 0x62},
		{0xed, 0xa0, 0x80}, // UTF-8 surrogate
	}
	for _, b := range quoteInputs {
		s := string(b)
		v.Quote = append(v.Quote, QuoteVec{
			InputHex:  hex.EncodeToString(b),
			ValidUTF8: isValidUTF8(b),
			Quote:     strconv.Quote(s),
			ASCII:     strconv.QuoteToASCII(s),
			Graphic:   strconv.QuoteToGraphic(s),
		})
	}

	// 9. QuoteRune / QuoteRuneToASCII / QuoteRuneToGraphic
	runes := []rune{
		'a', 'Z', '0', ' ', '~', '\'', '"', '\\',
		'\a', '\b', '\f', '\n', '\r', '\t', '\v',
		0x00, 0x01, 0x1f, 0x7f,
		0x80, 0x9f, 0xa0, 0xa1, 0xad, 0xff,
		0x0300, '世', 0x1680, 0x2000, 0x200b, 0x2028, 0x3000, 0xfeff, 0xfffd,
		0x1f600, 0xe0000, 0x10ffff,
	}
	for _, r := range runes {
		v.QuoteRune = append(v.QuoteRune, QuoteRuneVec{
			Code:    r,
			Quote:   strconv.QuoteRune(r),
			ASCII:   strconv.QuoteRuneToASCII(r),
			Graphic: strconv.QuoteRuneToGraphic(r),
		})
	}

	// 10. Unquote & QuotedPrefix
	unquoteInputs := []string{
		`""`, `"hello"`, `"hello\nworld"`, `"\a\b\f\n\r\t\v\\\""`,
		`"\x00\x41\x7f"`, `"\x80\xff"`, `"\000\101\377"`,
		`"\u0041\u4e16"`, `"\U0001f600"`, `"Hello, 世界"`,
		`''`, `'a'`, `'\''`, `'"'`, `'\\'`, `'\n'`, `'\x41'`, `'\x80'`, `'\101'`, `'\u4e16'`, `'\U0001f600'`, `'世'`,
		"``", "`raw string`", "`line1\nline2`", "`a\rb\r\nc`", "`\\n\\t\"'`",
		// Invalid / trailing
		``, `"`, `'`, "`", `"unterminated`, "'unterminated", "`unterminated",
		`"line` + "\n" + `break"`, `'ab'`, `'\""'`, `"\'"`, `'\"'`,
		`"\x1"`, `"\xGG"`, `"\u123"`, `"\ud800"`, `"\udfff"`, `"\U00110000"`, `"\U0000d800"`,
		`"\400"`, `"\08"`, `"\0"`, `"\q"`, `"\`,
		`"abc"def`, `'a'bc`, "`abc`def", `"a\"b"rest`, "`a\rb`rest",
	}
	for _, s := range unquoteInputs {
		out, err := strconv.Unquote(s)
		b := []byte(out)
		v.Unquote = append(v.Unquote, UnquoteVec{
			Input:     s,
			OutHex:    hex.EncodeToString(b),
			ValidUTF8: isValidUTF8(b),
			Err:       classifyErr(err),
		})
		pfx, perr := strconv.QuotedPrefix(s)
		v.QuotedPrefix = append(v.QuotedPrefix, QuotedPrefixVec{
			Input: s, Out: pfx, Err: classifyErr(perr),
		})
	}

	// 11. UnquoteChar
	unquoteCharCases := []struct {
		s     string
		quote byte
	}{
		{"a", '"'}, {"abc", '"'}, {`\"rest`, '"'}, {`\'rest`, '"'}, {`"rest`, '"'}, {`'rest`, '"'},
		{`\'rest`, '\''}, {`\"rest`, '\''}, {`'rest`, '\''}, {`"rest`, '\''},
		{`"rest`, 0}, {`'rest`, 0}, {`\"rest`, 0}, {`\'rest`, 0},
		{`\a1`, '"'}, {`\b2`, '"'}, {`\f3`, '"'}, {`\n4`, '"'}, {`\r5`, '"'}, {`\t6`, '"'}, {`\v7`, '"'}, {`\\8`, '"'},
		{`\x41z`, '"'}, {`\xffz`, '"'}, {`\000z`, '"'}, {`\101z`, '"'}, {`\377z`, '"'},
		{`\u0041z`, '"'}, {`\u4e16z`, '"'}, {`\U0001f600z`, '"'},
		{"世界", '"'}, {"😀!", '"'},
		{"", '"'}, {`\`, '"'}, {`\x1`, '"'}, {`\xzz`, '"'}, {`\u123`, '"'}, {`\ud800`, '"'},
		{`\U00110000`, '"'}, {`\400`, '"'}, {`\08`, '"'}, {`\q`, '"'},
	}
	for _, tc := range unquoteCharCases {
		val, mb, tail, err := strconv.UnquoteChar(tc.s, tc.quote)
		v.UnquoteChar = append(v.UnquoteChar, UnquoteCharVec{
			Input: tc.s, Quote: int32(tc.quote), Value: val, Multibyte: mb, Tail: tail, Err: classifyErr(err),
		})
	}

	// 12. CanBackquote
	backquoteInputs := []string{
		"", "abc", "hello\tworld", "hello\nworld", "hello\rworld", "has`backtick",
		"\x00", "\x1f", "\x7f", "\u00a0", "\u00ad", "\ufeff", "a\ufeffb", "Hello, 世界! 😀",
	}
	for _, s := range backquoteInputs {
		v.CanBackquote = append(v.CanBackquote, CanBackquoteVec{Input: s, Out: strconv.CanBackquote(s)})
	}

	// 13. IsPrint / IsGraphic
	classRunes := []rune{
		0x00, 0x09, 0x0a, 0x1f, 0x20, 0x21, 0x41, 0x7e, 0x7f,
		0x80, 0x9f, 0xa0, 0xa1, 0xac, 0xad, 0xae, 0xff,
		0x0100, 0x0300, 0x0378, 0x03b1, 0x0600, 0x1680,
		0x2000, 0x200a, 0x200b, 0x2028, 0x2029, 0x202f, 0x205f,
		0x3000, 0x4e16, 0xe000, 0xfeff, 0xfffd, 0xffff,
		0x1f600, 0xe0000, 0x10ffff,
	}
	for _, r := range classRunes {
		v.RuneClass = append(v.RuneClass, RuneClassVec{
			Code: r, IsPrint: strconv.IsPrint(r), IsGraphic: strconv.IsGraphic(r),
		})
	}

	// 14. ParseComplex / FormatComplex
	complexInputs := []string{
		"0", "1", "-1", "0i", "1i", "-1i", "+1i",
		"1+2i", "1-2i", "-1+2i", "-1-2i", "(1+2i)", "(-1.5-2.5i)", "(3i)", "(5)",
		"1+NaNi", "NaN+NaNi", "NaN+1i", "+Inf-Infi", "-Inf+Infi", "1e2+3e-4i", "0x1p2+0x1.8p1i",
		"1e500+1i", "1+1e500i", "1e50+1i",
		"", "()", "(1+2i", "1+2i)", "1-NaNi", "1++2i", "1+-2i", "1+2j", "1 + 2i", "i", "+i", "-i",
	}
	for _, bits := range []int{128, 64} {
		for _, s := range complexInputs {
			c, err := strconv.ParseComplex(s, bits)
			var rb, ib uint64
			var rnan, inan bool
			if bits == 128 {
				rnan = math.IsNaN(real(c))
				inan = math.IsNaN(imag(c))
				rb = math.Float64bits(real(c))
				ib = math.Float64bits(imag(c))
			} else {
				re := float32(real(c))
				im := float32(imag(c))
				rnan = math.IsNaN(float64(re))
				inan = math.IsNaN(float64(im))
				rb = uint64(math.Float32bits(re))
				ib = uint64(math.Float32bits(im))
			}
			v.ParseComplex = append(v.ParseComplex, ParseComplexVec{
				Input: s, BitSize: bits, RealBits: rb, RealIsNaN: rnan, ImagBits: ib, ImagIsNaN: inan, Err: classifyErr(err),
			})
		}
	}
	complexFormatVals := []complex128{
		complex(0, 0), complex(1, 2), complex(1, -2), complex(-1.5, 2.5),
		complex(math.Inf(1), math.Inf(-1)), complex(math.NaN(), 1), complex(1, math.NaN()),
	}
	for _, c := range complexFormatVals {
		for _, fm := range []byte{'g', 'f', 'e'} {
			v.FormatComplex = append(v.FormatComplex, FormatComplexVec{
				RealBits: math.Float64bits(real(c)),
				ImagBits: math.Float64bits(imag(c)),
				Fmt:      string(fm),
				Prec:     -1,
				BitSize:  128,
				Out:      strconv.FormatComplex(c, fm, -1, 128),
			})
			v.FormatComplex = append(v.FormatComplex, FormatComplexVec{
				RealBits: uint64(math.Float32bits(float32(real(c)))),
				ImagBits: uint64(math.Float32bits(float32(imag(c)))),
				Fmt:      string(fm),
				Prec:     2,
				BitSize:  64,
				Out:      strconv.FormatComplex(c, fm, 2, 64),
			})
		}
	}

	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	enc.SetEscapeHTML(false)
	if err := enc.Encode(v); err != nil {
		panic(err)
	}
}
