// Generate independent JSON byte grammar, formatting, number and stream observations.
package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"math"
	"math/big"
	"os"
	"reflect"
	"strconv"
	"strings"
)

// failure retains structured causes independently from human-readable diagnostic text.
type failure struct {
	Kind         string `json:"kind"`
	InvalidValue bool   `json:"invalid_value,omitempty"`
	Offset       int64  `json:"offset"`
	Value        string `json:"value,omitempty"`
	Type         string `json:"type,omitempty"`
	Struct       string `json:"struct,omitempty"`
	Field        string `json:"field,omitempty"`
	Text         string `json:"text,omitempty"`
	Cause        string `json:"cause,omitempty"`
}

// errorInfo extracts stable error metadata rather than converting every failure to a boolean.
func errorInfo(err error) failure {
	if err == nil {
		return failure{}
	}
	out := failure{Kind: "other", Text: err.Error()}
	var numeric *strconv.NumError
	var syntax *json.SyntaxError
	var mismatch *json.UnmarshalTypeError
	var custom *json.MarshalerError
	var unsupported *json.UnsupportedValueError
	var unsupportedType *json.UnsupportedTypeError
	var target *json.InvalidUnmarshalError
	switch {
	case errors.As(err, &custom):
		out.Kind, out.Type, out.Cause = "custom", custom.Type.String(), custom.Unwrap().Error()
	case errors.As(err, &numeric):
		out.Kind = "number"
		if errors.Is(numeric.Err, strconv.ErrRange) {
			out.Cause = "range"
		} else {
			out.Cause = "syntax"
		}
	case errors.As(err, &syntax):
		out.Kind, out.Offset = "syntax", syntax.Offset
	case errors.As(err, &mismatch):
		out.Kind, out.Offset = "type", mismatch.Offset
		out.Value, out.Type, out.Struct, out.Field = mismatch.Value, mismatch.Type.String(), mismatch.Struct, mismatch.Field
	case errors.As(err, &unsupported):
		out.Kind, out.Value = "value", unsupported.Str
		out.InvalidValue = !unsupported.Value.IsValid()
		if unsupported.Value.IsValid() {
			out.Type = unsupported.Value.Type().String()
		}
	case errors.As(err, &unsupportedType):
		out.Kind, out.Type = "unsupported-type", unsupportedType.Type.String()
	case errors.As(err, &target):
		out.Kind = "target"
		if target.Type != nil {
			out.Type = target.Type.String()
		}
	}
	return out
}

// wireHex prevents fixture serialization from repairing malformed UTF-8 bytes.
func wireHex(input []byte) string { return hex.EncodeToString(input) }

// syntaxCase records exact raw transformations and syntax offsets for one arbitrary byte sequence.
func syntaxCase(input []byte) map[string]any {
	var raw json.RawMessage
	err := json.Unmarshal(input, &raw)
	var compact, html bytes.Buffer
	compactErr := json.Compact(&compact, input)
	json.HTMLEscape(&html, input)
	indents := []map[string]any{}
	for _, settings := range [][2]string{{"", "  "}, {"P:", "\t"}, {"", ""}} {
		var output bytes.Buffer
		indentErr := json.Indent(&output, input, settings[0], settings[1])
		indents = append(indents, map[string]any{"prefix": settings[0], "step": settings[1], "output_hex": wireHex(output.Bytes()), "error": errorInfo(indentErr)})
	}
	return map[string]any{"input_hex": wireHex(input), "valid": json.Valid(input), "raw_hex": wireHex(raw), "error": errorInfo(err), "compact_hex": wireHex(compact.Bytes()), "compact_error": errorInfo(compactErr), "html_hex": wireHex(html.Bytes()), "indents": indents}
}

// quoteCase retains text replacement and both HTML policies over arbitrary string bytes.
func quoteCase(input []byte) map[string]any {
	results := []map[string]any{}
	for _, html := range []bool{false, true} {
		var output bytes.Buffer
		encoder := json.NewEncoder(&output)
		encoder.SetEscapeHTML(html)
		err := encoder.Encode(string(input))
		encoded := bytes.TrimSuffix(output.Bytes(), []byte{'\n'})
		var decoded string
		decodeErr := json.Unmarshal(encoded, &decoded)
		results = append(results, map[string]any{"html": html, "output_hex": wireHex(encoded), "error": errorInfo(err), "decoded_hex": wireHex([]byte(decoded)), "decode_error": errorInfo(decodeErr)})
	}
	return map[string]any{"input_hex": wireHex(input), "results": results}
}

// numberCase separates exact source spellings from integer/float conversion and range errors.
func numberCase(text string) map[string]any {
	number := json.Number(text)
	integer, integerErr := number.Int64()
	floating, floatErr := number.Float64()
	encoded, marshalErr := json.Marshal(number)
	return map[string]any{"text": text, "integer": strconv.FormatInt(integer, 10), "integer_error": errorInfo(integerErr), "float_bits": fmt.Sprintf("%016x", math.Float64bits(floating)), "float_error": errorInfo(floatErr), "marshal_hex": wireHex(encoded), "marshal_error": errorInfo(marshalErr)}
}

// chunkReader bounds each source read while retaining normal reader EOF semantics.
type chunkReader struct {
	input []byte
	chunk int
}

// Read copies one source fragment into the decoder's supplied buffer.
func (r *chunkReader) Read(out []byte) (int, error) {
	if len(r.input) == 0 {
		return 0, io.EOF
	}
	n := min(len(out), len(r.input), r.chunk)
	copy(out, r.input[:n])
	r.input = r.input[n:]
	return n, nil
}

// tokenValue converts one public token into stable data without losing float bit patterns.
func tokenValue(token json.Token) map[string]any {
	switch value := token.(type) {
	case nil:
		return map[string]any{"kind": "null"}
	case json.Delim:
		return map[string]any{"kind": "delimiter", "text": value.String()}
	case bool:
		return map[string]any{"kind": "bool", "value": value}
	case string:
		return map[string]any{"kind": "string", "utf8_hex": wireHex([]byte(value))}
	case json.Number:
		return map[string]any{"kind": "number", "text": value.String()}
	case float64:
		return map[string]any{"kind": "float", "bits": fmt.Sprintf("%016x", math.Float64bits(value))}
	default:
		panic(fmt.Sprintf("unhandled token %T", token))
	}
}

// streamCase observes every token, logical cursor and snapshot of already-buffered source bytes.
func streamCase(input []byte, chunk int, numbers bool) map[string]any {
	decoder := json.NewDecoder(&chunkReader{bytes.Clone(input), chunk})
	if numbers {
		decoder.UseNumber()
	}
	steps := []map[string]any{}
	for i := 0; i < 1000; i++ {
		more := decoder.More()
		token, err := decoder.Token()
		buffered, readErr := io.ReadAll(decoder.Buffered())
		if readErr != nil {
			panic(readErr)
		}
		step := map[string]any{"more": more, "offset": decoder.InputOffset(), "buffered_hex": wireHex(buffered)}
		if err == io.EOF {
			step["eof"] = true
		} else if err != nil {
			step["error"] = errorInfo(err)
		} else {
			step["token"] = tokenValue(token)
		}
		steps = append(steps, step)
		if err != nil {
			break
		}
	}
	return map[string]any{"input_hex": wireHex(input), "chunk": chunk, "numbers": numbers, "steps": steps}
}

// applicationRecord exercises exact-first field matching, quoted fields and omission metadata.
type applicationRecord struct {
	ID      int64  `json:"id"`
	Name    string `json:"name"`
	Enabled bool   `json:"enabled,omitempty"`
	Quoted  int64  `json:"quoted,string"`
	Zero    int64  `json:"zero,omitzero"`
}

// typedCase retains the partial result after unmarshal, independently from the returned error.
func typedCase(label string, input []byte, initial any) map[string]any {
	err := json.Unmarshal(input, initial)
	encoded, encodeErr := json.Marshal(initial)
	return map[string]any{"type": label, "input_hex": wireHex(input), "error": errorInfo(err), "output_hex": wireHex(encoded), "output_error": errorInfo(encodeErr)}
}

// typedCases covers null preservation, type errors, later updates, arrays, maps and binary values.
func typedCases() []map[string]any {
	rows := []map[string]any{}
	for _, text := range []string{"null", "true", "false", "0", "\"true\"", "[]"} {
		value := true
		rows = append(rows, typedCase("Bool", []byte(text), &value))
	}
	for _, text := range []string{"null", "\"x\"", "\"\\ud800\"", "42", "{}"} {
		value := "before"
		rows = append(rows, typedCase("String", []byte(text), &value))
	}
	for _, text := range []string{"null", "0", "-9223372036854775808", "9223372036854775807", "9223372036854775808", "1.0", "1e1", "\"1\"", "true"} {
		value := int64(7)
		rows = append(rows, typedCase("I64", []byte(text), &value))
	}
	for _, text := range []string{"null", "0", "18446744073709551615", "18446744073709551616", "-1", "1.0", "\"1\""} {
		value := uint64(7)
		rows = append(rows, typedCase("U64", []byte(text), &value))
	}
	for _, text := range []string{"null", "1.00e+09", "\"1.00e+09\"", "\"\\u0031.0\"", "\"01\"", "\"x\"", "true"} {
		value := json.Number("7")
		rows = append(rows, typedCase("JsonNumber", []byte(text), &value))
	}
	for _, text := range []string{"null", "[]", "[1]", "[1,2,3,4]", "[1,\"x\",3]", "[null,2]", "{}"} {
		value := []int64{7, 8, 9}
		rows = append(rows, typedCase("List I64", []byte(text), &value))
		array := [3]int64{7, 8, 9}
		rows = append(rows, typedCase("Array3 I64", []byte(text), &array))
	}
	for _, text := range []string{"null", "{}", "{\"a\":1}", "{\"a\":1,\"a\":2}", "{\"a\":\"x\",\"b\":3}", "{\"a\":null}", "[]"} {
		value := map[string]int64{"a": 7, "old": 8}
		rows = append(rows, typedCase("Map String I64", []byte(text), &value))
	}
	for _, text := range []string{"null", "\"\"", "\"AAECA/8=\"", "\"AB==\"", "\"AA\\nEC\"", "\"?\"", "[1,2,255]", "[256]", "[null,2]"} {
		value := []byte{7, 8}
		rows = append(rows, typedCase("Bytes", []byte(text), &value))
	}
	for _, text := range []string{"null", "{}", "{\"id\":2,\"name\":\"after\",\"quoted\":\"5\"}", "{\"ID\":2,\"NAME\":\"after\"}", "{\"id\":\"bad\",\"name\":\"after\",\"enabled\":true}", "{\"id\":9223372036854775808,\"name\":\"after\"}", "{\"unknown\":1,\"name\":\"after\"}", "{\"quoted\":5,\"name\":\"after\"}", "{\"quoted\":\"bad\",\"name\":\"after\"}", "[]"} {
		value := applicationRecord{ID: 7, Name: "before", Quoted: 9}
		rows = append(rows, typedCase("ApplicationRecord", []byte(text), &value))
	}
	for _, text := range []string{"null", " {\"a\": 1,\"a\":2} ", "\"\\ud800\"", "[]", ""} {
		value := json.RawMessage(`{"before":true}`)
		rows = append(rows, typedCase("JsonRawMessage", []byte(text), &value))
	}
	return rows
}

// byteIndentCases keeps arbitrary line-prefix bytes independent from scalar string fixtures.
func byteIndentCases() []map[string]any {
	rows := []map[string]any{}
	for _, input := range []string{`{"a":[1,2],"b":{}}`, "  [true,{}] \n", "  1.0 \t"} {
		for _, settings := range [][2]string{{string([]byte{255, 0}), "x\n"}, {"\"\\", string([]byte{192, 175})}, {"", ""}} {
			var out bytes.Buffer
			err := json.Indent(&out, []byte(input), settings[0], settings[1])
			rows = append(rows, map[string]any{"input_hex": wireHex([]byte(input)), "prefix_hex": wireHex([]byte(settings[0])), "step_hex": wireHex([]byte(settings[1])), "output_hex": wireHex(out.Bytes()), "error": errorInfo(err)})
		}
	}
	return rows
}

// floatCodecCases independently pins shortest JSON formatting and typed binary64 updates.
func floatCodecCases() map[string]any {
	encoding := []map[string]any{}
	patterns := []uint64{0, 1 << 63, 1, 0x000fffffffffffff, 0x0010000000000000, 0x3ff0000000000000, 0x3eb0c6f7a0b5ed8d, 0x3e7ad7f29abcaf48, 0x4415af1d78b58c40, 0x444b1ae4d6e2ef50, 0x7fefffffffffffff, 0xffefffffffffffff, 0x7ff0000000000000, 0xfff0000000000000, 0x7ff8000000000000}
	state := uint64(0x819c9e3779b97f4a)
	for index := 0; index < 128; index++ {
		state ^= state << 13
		state ^= state >> 7
		state ^= state << 17
		patterns = append(patterns, state)
	}
	for _, bits := range patterns {
		value := math.Float64frombits(bits)
		encoded, err := json.Marshal(value)
		encoding = append(encoding, map[string]any{"bits": fmt.Sprintf("%016x", bits), "output_hex": wireHex(encoded), "error": errorInfo(err)})
	}
	updates := []map[string]any{}
	for _, text := range []string{"null", "0", "-0", "1", "1.0", "1e-6", "1e-7", "1e20", "1e21", "9007199254740993", "1.7976931348623157e308", "1.7976931348623159e308", "5e-324", "1e-9999", "-1e-9999", "1e9999", "\"1\"", "true", "[]", "{}"} {
		value := float64(7)
		updates = append(updates, typedCase("F64", []byte(text), &value))
	}
	return map[string]any{"float_encoding": encoding, "typed": updates}
}

// float32CodecCases pins direct binary32 rounding at exact decimal midpoints and independent shortest output.
func float32CodecCases() map[string]any {
	encoding := []map[string]any{}
	patterns := []uint32{0, 0x80000000, 1, 2, 0x007fffff, 0x00800000, 0x3f7fffff, 0x3f800000, 0x3f800001, 0x358637bc, 0x358637bd, 0x358637be, 0x3490a9c0, 0x60ad78ec, 0x6258d727, 0x7f7fffff, 0xff7fffff, 0x7f800000, 0xff800000, 0x7fc00000}
	for exponent := uint32(1); exponent < 255; exponent++ {
		patterns = append(patterns, exponent<<23)
	}
	for fraction := uint32(0); fraction < 23; fraction++ {
		patterns = append(patterns, uint32(1)<<fraction)
	}

	state := uint32(0x819c9e37)
	for index := 0; index < 128; index++ {
		state ^= state << 13
		state ^= state >> 17
		state ^= state << 5
		patterns = append(patterns, state)
	}
	for _, bits := range patterns {
		encoded, err := json.Marshal(math.Float32frombits(bits))
		encoding = append(encoding, map[string]any{"bits": fmt.Sprintf("%08x", bits), "output_hex": wireHex(encoded), "error": errorInfo(err)})
	}
	texts := []string{"null", "0", "-0", "1", "1.0", "1e-6", "1e-7", "1e20", "1e21", "16777217", "3.4028234663852886e38", "3.4028235677973366e38", "1e-9999", "-1e-9999", "1e9999", "-1e9999", "\"1\"", "true", "[]", "{}"}
	unit := new(big.Rat).SetFrac(big.NewInt(1), new(big.Int).Exp(big.NewInt(10), big.NewInt(200), nil))
	for _, lower := range []uint32{0, 1, 2, 0x007ffffe, 0x007fffff, 0x00800000, 0x3f7fffff, 0x3f800000, 0x3f800001, 0x7f7ffffe, 0x7f7fffff} {
		a := new(big.Rat).SetFloat64(float64(math.Float32frombits(lower)))
		var b *big.Rat
		if lower == 0x7f7fffff {
			b = new(big.Rat).SetInt(new(big.Int).Lsh(big.NewInt(1), 128))
		} else {
			b = new(big.Rat).SetFloat64(float64(math.Float32frombits(lower + 1)))
		}
		midpoint := new(big.Rat).Quo(new(big.Rat).Add(a, b), big.NewRat(2, 1))
		for _, delta := range []*big.Rat{new(big.Rat).Neg(unit), new(big.Rat), unit} {
			exact := new(big.Rat).Add(midpoint, delta).FloatString(200)
			texts = append(texts, exact, "-"+exact)
		}
	}
	texts = append(texts, "1."+strings.Repeat("0", 2048)+"1", "0."+strings.Repeat("0", 2048)+"1e2049", "0e"+strings.Repeat("9", 2048), "1e-"+strings.Repeat("9", 2048))
	updates := []map[string]any{}
	conversions := []map[string]any{}
	for _, text := range texts {
		value := float32(7)
		updates = append(updates, typedCase("F32", []byte(text), &value))
		if json.Valid([]byte(text)) && text != "null" && text != "true" && text != "[]" && text != "{}" && text != "\"1\"" {
			parsed, err := strconv.ParseFloat(text, 32)
			conversions = append(conversions, map[string]any{"text": text, "bits": fmt.Sprintf("%08x", math.Float32bits(float32(parsed))), "error": errorInfo(err)})
		}
	}
	return map[string]any{"float32_encoding": encoding, "float32_conversion": conversions, "typed": updates}
}

// fieldCodecCases independently observes name matching across complete SIMD and Unicode boundaries.
func fieldCodecCases() map[string]any {
	rows := []map[string]any{}
	appendCase := func(name, key string) {
		tag := reflect.StructTag("json:" + strconv.Quote(name))
		typ := reflect.StructOf([]reflect.StructField{{Name: "Field", Type: reflect.TypeOf(int64(0)), Tag: tag}})
		value := reflect.New(typ)
		input, err := json.Marshal(map[string]int64{key: 19})
		if err != nil {
			panic(err)
		}
		err = json.Unmarshal(input, value.Interface())
		encoded, encodeErr := json.Marshal(value.Interface())
		rows = append(rows, map[string]any{"name": name, "input_hex": wireHex(input), "output_hex": wireHex(encoded), "value": value.Elem().Field(0).Int(), "error": errorInfo(err), "output_error": errorInfo(encodeErr)})
	}
	for _, n := range []int{1, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65} {
		name := strings.Repeat("A", n)
		appendCase(name, name)
		appendCase(name, strings.ToLower(name))
		for _, at := range []int{0, n / 2, n - 1} {
			key := []byte(strings.ToLower(name))
			key[at] = 'b'
			appendCase(name, string(key))
		}
	}
	name := strings.Repeat("A", 32)
	for at := 0; at < 32; at++ {
		key := []byte(strings.ToLower(name))
		key[at] = 'b'
		appendCase(name, string(key))
	}
	for _, names := range [][2]string{{"K", "K"}, {"K", "k"}, {"Σ", "ς"}, {"σ", "Σ"}, {"I", "ı"}, {"İ", "i"}, {"LongField_ABC-XYZ", "longfield_abc-xyz"}, {"LongField_ABC-XYZ", "longfieldabcxyz"}, {strings.Repeat("A", 15) + "K", "aaaaaaaaaaaaaaaK"}, {strings.Repeat("A", 16) + "Σ", "aaaaaaaaaaaaaaaaς"}, {strings.Repeat("A", 31) + "Σ", "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaσ"}, {"É漢😀", "é漢😀"}} {
		appendCase(names[0], names[1])
	}
	return map[string]any{"field_matching": rows}
}

// integerCodecCases preserves width-specific range errors independently from partially updated values.
func integerCodecCases() map[string]any {
	rows := []map[string]any{}
	inputs := []string{"null", "0", "-0", "1", "-1", "127", "128", "-128", "-129", "255", "256", "32767", "32768", "-32768", "-32769", "65535", "65536", "2147483647", "2147483648", "-2147483648", "-2147483649", "4294967295", "4294967296", "9223372036854775807", "9223372036854775808", "-9223372036854775808", "-9223372036854775809", "18446744073709551615", "18446744073709551616", "12345678", "1234567890123456", "12345678901234567890", "9999999999999999999999999999999999999999", "1.0", "1e1", "\"1\"", "true", "[]", "{}"}
	types := []struct {
		label string
		value any
	}{{"I8", int8(7)}, {"I16", int16(7)}, {"I32", int32(7)}, {"I64", int64(7)}, {"Isize", int(7)}, {"U8", uint8(7)}, {"U16", uint16(7)}, {"U32", uint32(7)}, {"U64", uint64(7)}, {"Usize", uint(7)}}
	for _, item := range types {
		for _, text := range inputs {
			value := reflect.New(reflect.TypeOf(item.value))
			value.Elem().Set(reflect.ValueOf(item.value))
			rows = append(rows, typedCase(item.label, []byte(text), value.Interface()))
		}
	}
	return map[string]any{"typed": rows}
}

// binaryCodecCases observes binary text, numeric-array updates and fixed byte-array limits.
func binaryCodecCases() map[string]any {
	rows := []map[string]any{}
	inputs := []string{"null", "[]", "[1]", "[1,2,255]", "[256]", "[1,256,3]", "[null,2]", "[1,2,3,4,\"bad\"]", "true", "42", "{}", "\"\"", "\"AAECA/8=\"", "\"AB==\"", "\"AA\\nEC\"", "\"?\"", "\"A\"", "\"AA\"", "\"AAA\"", "\"AA=\"", "\"AA===\"", "\"AAAA=\"", "\"AA==x\"", "\"AA\\tEC\"", "\"\\ud800\""}
	for _, n := range []int{0, 1, 2, 3, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65, 255, 256, 257} {
		raw := make([]byte, n)
		for i := range raw {
			raw[i] = byte(i*29 + 17)
		}
		encoded, err := json.Marshal(raw)
		if err != nil {
			panic(err)
		}
		inputs = append(inputs, string(encoded))
	}
	for _, text := range inputs {
		value := []byte{7, 8}
		rows = append(rows, typedCase("Bytes", []byte(text), &value))
		array := [3]byte{7, 8, 9}
		rows = append(rows, typedCase("ByteArray3", []byte(text), &array))
	}
	return map[string]any{"typed": rows}
}

// foldKey makes text-key method dispatch independent of built-in string-key formatting.
type foldKey struct{ Name string }

// MarshalText returns a visible canonical spelling for the map-name sorting observation.
func (key foldKey) MarshalText() ([]byte, error) {
	if key.Name == "reject" {
		return nil, errors.New("key rejected")
	}
	return []byte(strings.ToUpper(key.Name)), nil
}

// UnmarshalText records normalized text while preserving deliberate application refusals.
func (key *foldKey) UnmarshalText(input []byte) error {
	name := strings.ToLower(string(input))
	if name == "reject" {
		return errors.New("key rejected")
	}
	key.Name = name
	return nil
}

// mapCodecCases preserves key conversion, sorted names, unrelated members and first-error updates.
func mapCodecCases() map[string]any {
	rows := []map[string]any{}
	for _, text := range []string{"null", "{}", "{\"a\":1}", "{\"a\":1,\"a\":2}", "{\"a\":\"x\",\"b\":3}", "{\"a\":null}", "[]", "true", "42", "\"x\"", "{\"é\":2,\"漢\":3,\"A\":4}", "{\"\\u0061\":2,\"a\":3}"} {
		value := map[string]int64{"a": 7, "old": 8}
		rows = append(rows, typedCase("Map String I64", []byte(text), &value))
	}
	inputs := []string{"null", "{}", "{\"1\":\"one\"}", "{\"01\":\"first\",\"+1\":\"second\"}", "{\"-0\":\"zero\",\"0\":\"later\"}", "{\"9223372036854775807\":\"max\",\"-9223372036854775808\":\"min\"}", "{\"9223372036854775808\":\"bad\",\"2\":\"after\"}", "{\"18446744073709551615\":\"max\"}", "{\"18446744073709551616\":\"bad\",\"2\":\"after\"}", "{\"-1\":\"negative\",\"2\":\"after\"}", "{\"bad\":\"bad\",\"2\":\"after\"}", "{\" 1\":\"bad\",\"2\":\"after\"}", "{\"1_0\":\"bad\",\"2\":\"after\"}", "{\"1.0\":\"bad\",\"2\":\"after\"}", "{\"2\":null}", "{\"2\":1,\"3\":\"after\"}", "[]", "true"}
	for _, text := range inputs {
		signed := map[int64]string{1: "before", 7: "old"}
		rows = append(rows, typedCase("Map I64 String", []byte(text), &signed))
		unsigned := map[uint64]string{1: "before", 7: "old"}
		rows = append(rows, typedCase("Map U64 String", []byte(text), &unsigned))
	}
	for _, text := range []string{"null", "{}", "{\"true\":1}", "[]", "true"} {
		value := map[bool]int64{true: 7}
		rows = append(rows, typedCase("Map Bool I64", []byte(text), &value))
	}
	for _, text := range []string{"null", "{}", "{\"WORD\":1}", "{\"Word\":1,\"word\":2}", "{\"word\":\"bad\",\"next\":3}", "{\"reject\":1,\"next\":3}", "{\"\\u0057ORD\":4}", "[]"} {
		value := map[foldKey]int64{{Name: "before"}: 7, {Name: "old"}: 8}
		rows = append(rows, typedCase("Map TextKey I64", []byte(text), &value))
	}
	return map[string]any{"typed": rows}
}

// rawMapKey preserves arbitrary byte spelling in a comparable application key.
type rawMapKey struct{ Text string }

// MarshalText returns exact raw name bytes, including malformed UTF-8 and deliberate application errors.
func (key rawMapKey) MarshalText() ([]byte, error) {
	if key.Text == "reject" {
		return nil, errors.New("key rejected")
	}
	return []byte(key.Text), nil
}

// mapEncodingCases observes repaired UTF-8 name sorting, replacement-aware quoting and custom failure wrapping.
func mapEncodingCases() map[string]any {
	sets := [][]string{{"b", "a", "z"}, {"é", "漢", "😀", string([]byte{255}), string([]byte{192, 175}), string([]byte{237, 160, 128})}, {"z", string([]byte{128})}, {"é", "�", "漢"}, {"", "\"", "\\", "\n", "\x00", "<>&", "\u2028\u2029"}, {strings.Repeat("a", 15) + "z", strings.Repeat("a", 16) + "a", strings.Repeat("a", 17), strings.Repeat("a", 31), strings.Repeat("a", 32), strings.Repeat("a", 33)}, {"reject"}, {}}
	rows := []map[string]any{}
	for _, names := range sets {
		value := map[rawMapKey]int64{}
		entries := []map[string]any{}
		for at, name := range names {
			number := int64(at + 1)
			value[rawMapKey{Text: name}] = number
			entries = append(entries, map[string]any{"key_hex": wireHex([]byte(name)), "value": number})
		}
		encoded, err := json.Marshal(value)
		rows = append(rows, map[string]any{"null": false, "entries": entries, "output_hex": wireHex(encoded), "error": errorInfo(err)})
	}
	var absent map[rawMapKey]int64
	encoded, err := json.Marshal(absent)
	rows = append(rows, map[string]any{"null": true, "entries": []map[string]any{}, "output_hex": wireHex(encoded), "error": errorInfo(err)})
	return map[string]any{"map_encoding": rows}
}

// main spans malformed grammar, byte/escape boundaries and exact finite-number ranges.
func main() {
	if len(os.Args) == 2 && os.Args[1] == "--map-encoding-codecs" {
		encoder := json.NewEncoder(os.Stdout)
		encoder.SetIndent("", "  ")
		if err := encoder.Encode(mapEncodingCases()); err != nil {
			panic(err)
		}
		return
	}
	if len(os.Args) == 2 && os.Args[1] == "--map-codecs" {
		encoder := json.NewEncoder(os.Stdout)
		encoder.SetIndent("", "  ")
		if err := encoder.Encode(mapCodecCases()); err != nil {
			panic(err)
		}
		return
	}
	if len(os.Args) == 2 && os.Args[1] == "--binary-codecs" {
		encoder := json.NewEncoder(os.Stdout)
		encoder.SetIndent("", "  ")
		if err := encoder.Encode(binaryCodecCases()); err != nil {
			panic(err)
		}
		return
	}
	if len(os.Args) == 2 && os.Args[1] == "--integer-codecs" {
		encoder := json.NewEncoder(os.Stdout)
		encoder.SetIndent("", "  ")
		if err := encoder.Encode(integerCodecCases()); err != nil {
			panic(err)
		}
		return
	}
	if len(os.Args) == 2 && os.Args[1] == "--field-codecs" {
		encoder := json.NewEncoder(os.Stdout)
		encoder.SetIndent("", "  ")
		if err := encoder.Encode(fieldCodecCases()); err != nil {
			panic(err)
		}
		return
	}
	if len(os.Args) == 2 && os.Args[1] == "--float32-codecs" {
		encoder := json.NewEncoder(os.Stdout)
		encoder.SetIndent("", "  ")
		if err := encoder.Encode(float32CodecCases()); err != nil {
			panic(err)
		}
		return
	}
	if len(os.Args) == 2 && os.Args[1] == "--float-codecs" {
		encoder := json.NewEncoder(os.Stdout)
		encoder.SetIndent("", "  ")
		if err := encoder.Encode(floatCodecCases()); err != nil {
			panic(err)
		}
		return
	}
	inputs := [][]byte{}
	for _, text := range []string{"", " ", "null", " true \r\n", "false", "0", "-0", "0.00", "1e+09", "1e9999", "[1,2,3]", "{}", "[]", "{\"a\":1,\"a\":2}", "{\"x\":[true,null,{\"é\":\"漢😀\"}]}", "\"\\uD83D\\uDE00\"", "\"\\uD800\"", "\"\\uDC00\"", "\"\\uD800\\u0041\"", "\"\\uD800\\uD800\\uDC00\"", "01", "-", "1.", "1e", "1e+", "[1,]", "{\"x\":}", "{x:1}", "[}", "{]", "truex", "true false", "\"\\x00\"", "\"\\u12xz\"", "\"unterminated", "\"line\nfeed\"", "[1 2]", "{\"a\" 1}", "{\"a\":1 \"b\":2}"} {
		inputs = append(inputs, []byte(text))
	}
	inputs = append(inputs, []byte{34, 255, 34}, []byte{34, 237, 160, 128, 34}, []byte{34, 192, 175, 34}, []byte{239, 187, 191, 49}, []byte{0}, []byte{34, 0, 34})
	// Checkpoint offsets depend on token kind and the public operation that consumes it.
	for _, text := range []string{"  -", "[ -", "[12.", "{\"n\":12e+", "123.45e-", "n", "tru", "\"abc\\", "\"\\u12", "\"\\u12x", "\"\\u12x\"", "\"\\u12xZ\"", "1.x", "1ex", "-x", "[1. ]", "[\"\\x\"]"} {
		inputs = append(inputs, []byte(text))
	}
	for _, input := range [][]byte{{34, 226}, {34, 226, 130}, {34, 194}, {34, 240, 144}, {34, 255}, {226}, {226, 130}, {192}, {194, 65}, {194, 160}, {226, 128, 131}, {239, 191, 189}, {34, 92, 117, 49, 50, 255}, {34, 92, 117, 48, 48, 9, 34}, {34, 92, 117, 48, 48, 96, 34}} {
		inputs = append(inputs, input)
	}
	quotes := [][]byte{nil, []byte("plain"), []byte("\"\\\b\f\n\r\t\x00\x1f"), []byte("<>&\u2028\u2029é漢😀"), {255, 254, 192, 175}, {237, 160, 128}, {244, 144, 128, 128}, {239, 191, 189}}
	for _, n := range []int{0, 1, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65, 119, 120, 121, 127, 128, 129, 255, 256, 257} {
		body := bytes.Repeat([]byte{'a'}, n)
		wire := append(append([]byte{'"'}, body...), '"')
		inputs = append(inputs, wire)
		quotes = append(quotes, body, append(bytes.Clone(body), []byte("<\\\"\n😀")...))
	}
	syntax := []map[string]any{}
	for _, input := range inputs {
		syntax = append(syntax, syntaxCase(input))
	}
	quoteVectors := []map[string]any{}
	for _, input := range quotes {
		quoteVectors = append(quoteVectors, quoteCase(input))
	}
	numbers := []map[string]any{}
	for _, text := range []string{"0", "-0", "1", "-1", "1.0", "0.000001", "1e-7", "1e+21", "9007199254740993", "9223372036854775807", "-9223372036854775808", "9223372036854775808", "-9223372036854775809", "18446744073709551615", "1e9999", "-1e9999", "1e-9999", "1.7976931348623157e308", "1.7976931348623159e308", "5e-324", "", "01", "+1", "1x", " 1 "} {
		numbers = append(numbers, numberCase(text))
	}
	streams := []map[string]any{}
	for _, input := range inputs[:45] {
		for _, chunk := range []int{1, 7, 4096} {
			for _, numbers := range []bool{false, true} {
				streams = append(streams, streamCase(input, chunk, numbers))
			}
		}
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]any{"syntax": syntax, "quote": quoteVectors, "number": numbers, "stream": streams, "typed": typedCases(), "indent_bytes": byteIndentCases()}); err != nil {
		panic(err)
	}
}
