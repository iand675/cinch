// Generate independent fixed-field, aggregate and variable-integer binary fixtures.
package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"math"
	"os"
	"reflect"
)

// pattern selects exact bit values without floating-point arithmetic.
func pattern(kind string, i int) uint64 {
	if kind == "f32" || kind == "c64" {
		values := []uint64{0, 0x80000000, 1, 0x80000001, 0x3f800000, 0x7f7fffff, 0x7f800000, 0xff800000, 0x7fc00001, 0xffc12345, 0x7f800001}
		if kind == "c64" {
			values = values[:10]
		}
		return values[i%len(values)]
	}
	if kind == "f64" || kind == "c128" {
		values := []uint64{0, 0x8000000000000000, 1, 0x8000000000000001, 0x3ff0000000000000, 0x7fefffffffffffff, 0x7ff0000000000000, 0xfff0000000000000, 0x7ff8000000000001, 0xfff8123456789abc, 0x7ff0000000000001}
		if kind == "c128" {
			values = values[:10]
		}
		return values[i%len(values)]
	}
	return []uint64{0, 1, 127, 128, 255, 256, 32767, 32768, 65535, 0x7fffffff, 0x80000000, 0xffffffff, 0x7fffffffffffffff, 0x8000000000000000, 0xffffffffffffffff}[i%15]
}

// scalar constructs the concrete public arithmetic carrier for one requested bit pattern.
func scalar(kind string, bits, imaginary uint64) any {
	switch kind {
	case "bool":
		return bits != 0
	case "u8":
		return uint8(bits)
	case "i8":
		return int8(bits)
	case "u16":
		return uint16(bits)
	case "i16":
		return int16(bits)
	case "u32":
		return uint32(bits)
	case "i32":
		return int32(bits)
	case "u64":
		return bits
	case "i64":
		return int64(bits)
	case "f32":
		return math.Float32frombits(uint32(bits))
	case "f64":
		return math.Float64frombits(bits)
	case "c64":
		return complex(math.Float32frombits(uint32(bits)), math.Float32frombits(uint32(imaginary)))
	case "c128":
		return complex(math.Float64frombits(bits), math.Float64frombits(imaginary))
	default:
		panic("unknown scalar")
	}
}

// encoded checks size, buffer encoding, copied-prefix append and stream writing against each other.
func encoded(order binary.ByteOrder, value any) []byte {
	n := binary.Size(value)
	if n < 0 {
		panic("unsupported fixture value")
	}
	out := make([]byte, n)
	used, err := binary.Encode(out, order, value)
	if err != nil || used != n {
		panic(fmt.Sprint(used, err))
	}
	appended, err := binary.Append([]byte{0xaa, 0xbb}, order, value)
	if err != nil || !bytes.Equal(appended, append([]byte{0xaa, 0xbb}, out...)) {
		panic("append disagrees")
	}
	var stream bytes.Buffer
	if err := binary.Write(&stream, order, value); err != nil || !bytes.Equal(stream.Bytes(), out) {
		panic("write disagrees")
	}
	return out
}

// wireRecord exercises nested fixed arithmetic fields, explicit blank padding and an array.
type wireRecord struct {
	Active  bool
	Code    uint16
	Count   int32
	Single  float32
	Double  float64
	Complex complex128
	_       [3]byte
	Samples [3]uint16
}

// failure distinguishes empty EOF, partial EOF and arithmetic overflow without matching error text.
func failure(err error) string {
	switch err {
	case nil:
		return ""
	case io.EOF:
		return "eof"
	case io.ErrUnexpectedEOF:
		return "truncated"
	default:
		return "overflow"
	}
}

// main emits ordinary values, exact float payloads, array block boundaries and precise varint progress.
func main() {
	orders := []struct {
		name  string
		value binary.ByteOrder
	}{{"little", binary.LittleEndian}, {"big", binary.BigEndian}, {"native", binary.NativeEndian}}
	kinds := []string{"bool", "u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "f32", "f64", "c64", "c128"}
	scalars := []map[string]any{}
	arrays := []map[string]any{}
	records := []map[string]any{}
	for _, order := range orders {
		for _, kind := range kinds {
			count := 15
			if kind == "f32" || kind == "f64" {
				count = 11
			} else if kind == "c64" || kind == "c128" {
				count = 10
			}
			for i := 0; i < count; i++ {
				bits, imaginary := pattern(kind, i), pattern(kind, i+3)
				out := encoded(order.value, scalar(kind, bits, imaginary))
				scalars = append(scalars, map[string]any{"kind": kind, "order": order.name, "bits": bits, "imaginary": imaginary, "hex": hex.EncodeToString(out), "size": len(out)})
			}
			if kind == "c64" || kind == "c128" {
				continue
			}
			for _, n := range []int{0, 1, 2, 3, 4, 5, 7, 8, 9, 17, 65, 1024} {
				typeOf := reflect.TypeOf(scalar(kind, 0, 0))
				value := reflect.MakeSlice(reflect.SliceOf(typeOf), n, n)
				for i := 0; i < n; i++ {
					value.Index(i).Set(reflect.ValueOf(scalar(kind, pattern(kind, i), 0)))
				}
				out := encoded(order.value, value.Interface())
				digest := sha256.Sum256(out)
				full := ""
				if n <= 65 {
					full = hex.EncodeToString(out)
				}
				arrays = append(arrays, map[string]any{"kind": kind, "order": order.name, "count": n, "hex": full, "sha256": hex.EncodeToString(digest[:]), "size": len(out)})
			}
		}
		for i := 0; i < 8; i++ {
			// Quiet NaNs remain exact across the reference's reflective float32 promotion.
			single := uint32(pattern("f32", i%10))
			double := pattern("f64", i%10)
			real, imaginary := pattern("f64", (i+2)%10), pattern("f64", (i+4)%10)
			value := wireRecord{Active: i%2 != 0, Code: uint16(i*8191 + 3), Count: int32(i)*-100003 - 1, Single: math.Float32frombits(single), Double: math.Float64frombits(double), Complex: complex(math.Float64frombits(real), math.Float64frombits(imaginary)), Samples: [3]uint16{uint16(i), 65535, uint16(i * 257)}}
			out := encoded(order.value, value)
			records = append(records, map[string]any{"order": order.name, "active": value.Active, "code": value.Code, "count_bits": uint32(value.Count), "single_bits": single, "double_bits": double, "real_bits": real, "imaginary_bits": imaginary, "samples": value.Samples, "hex": hex.EncodeToString(out), "size": len(out)})
		}
	}
	integers := []uint64{0, 1, 2, 127, 128, 255, 256, 16383, 16384, 65535, 0xffffffff, 0x100000000, 0x7fffffffffffffff, 0x8000000000000000, 0xffffffffffffffff}
	varints := []map[string]any{}
	for _, bits := range integers {
		var unsigned, signed [10]byte
		u := binary.PutUvarint(unsigned[:], bits)
		v := binary.PutVarint(signed[:], int64(bits))
		varints = append(varints, map[string]any{"bits": bits, "unsigned": hex.EncodeToString(unsigned[:u]), "signed": hex.EncodeToString(signed[:v]), "unsigned_count": u, "signed_count": v})
	}
	inputs := [][]byte{{}, {0}, {1}, {127}, {128, 0}, {255, 0}, {128, 1}}
	for n := 1; n <= 12; n++ {
		inputs = append(inputs, bytes.Repeat([]byte{128}, n), bytes.Repeat([]byte{255}, n))
		if n <= 10 {
			inputs = append(inputs, append(bytes.Repeat([]byte{128}, n-1), 0), append(bytes.Repeat([]byte{255}, n-1), 1))
		}
	}
	inputs = append(inputs, append(bytes.Repeat([]byte{128}, 9), 2), append(bytes.Repeat([]byte{128}, 10), 1), append(bytes.Repeat([]byte{255}, 9), 2))
	prefixes := []map[string]any{}
	for _, input := range inputs {
		value, n := binary.Uvarint(input)
		kind, count := "", n
		if n == 0 {
			kind, count = "incomplete", len(input)
		} else if n < 0 {
			kind, count = "overflow", -n
		}
		reader := bytes.NewReader(input)
		streamValue, err := binary.ReadUvarint(reader)
		used := len(input) - reader.Len()
		prefixes = append(prefixes, map[string]any{"input": hex.EncodeToString(input), "kind": kind, "count": count, "value": value, "stream_kind": failure(err), "stream_value": streamValue, "stream_count": used, "remaining": hex.EncodeToString(input[used:])})
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]any{"native_little": binary.NativeEndian.Uint16([]byte{1, 0}) == 1, "scalars": scalars, "arrays": arrays, "records": records, "varints": varints, "prefixes": prefixes}); err != nil {
		panic(err)
	}
}
