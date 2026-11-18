// Frozen scalar reference fixtures and an independent public comparison benchmark.
package main

import (
	"cmp"
	"encoding/json"
	"fmt"
	"math"
	"os"
	"runtime"
	"strconv"
	"time"
)

// FloatCase keeps payload-bearing operands and the reference comparison results.
type FloatCase struct {
	X       uint64 `json:"x"`
	Y       uint64 `json:"y"`
	Compare int    `json:"compare"`
	Less    bool   `json:"less"`
}

// IntegerCase retains decimal boundaries without JSON number precision loss.
type IntegerCase struct {
	Type    string `json:"type"`
	X       string `json:"x"`
	Y       string `json:"y"`
	Compare int    `json:"compare"`
	Less    bool   `json:"less"`
}

// StringCase exercises byte lexicography including embedded NUL and non-ASCII text.
type StringCase struct {
	X       string `json:"x"`
	Y       string `json:"y"`
	Compare int    `json:"compare"`
	Less    bool   `json:"less"`
}

// Fixtures records every scalar fixture independently of the implementation language.
type Fixtures struct {
	WordBits     int           `json:"native_word_bits"`
	Architecture string        `json:"architecture"`
	F32          []FloatCase   `json:"f32"`
	F64          []FloatCase   `json:"f64"`
	Integers     []IntegerCase `json:"integers"`
	Strings      []StringCase  `json:"strings"`
}

// integerCases exercises each declared width at its signedness and overflow boundaries.
func integerCases[T cmp.Ordered](kind string, values []T) []IntegerCase {
	out := []IntegerCase{}
	for _, x := range values {
		for _, y := range values {
			out = append(out, IntegerCase{kind, fmt.Sprint(x), fmt.Sprint(y), cmp.Compare(x, y), cmp.Less(x, y)})
		}
	}
	return out
}

// fixtures constructs NaNs of both signs and kinds, subnormals, zeros, infinities and boundaries.
func fixtures() Fixtures {
	f := Fixtures{WordBits: strconv.IntSize, Architecture: runtime.GOARCH}
	p32 := []uint32{0, 0x80000000, 1, 0x80000001, 0x007fffff, 0x807fffff, 0x00800000, 0x80800000, 0x3f800000, 0xbf800000, 0x7f7fffff, 0xff7fffff, 0x7f800000, 0xff800000, 0x7fc00000, 0x7fc01234, 0xffc01234, 0x7f800001, 0xff800001}
	p64 := []uint64{0, 0x8000000000000000, 1, 0x8000000000000001, 0xfffffffffffff, 0x800fffffffffffff, 0x10000000000000, 0x8010000000000000, 0x3ff0000000000000, 0xbff0000000000000, 0x7fefffffffffffff, 0xffefffffffffffff, 0x7ff0000000000000, 0xfff0000000000000, 0x7ff8000000000000, 0x7ff8123412345678, 0xfff8123412345678, 0x7ff0000000000001, 0xfff0000000000001}
	for _, x := range p32 {
		for _, y := range p32 {
			a, b := math.Float32frombits(x), math.Float32frombits(y)
			f.F32 = append(f.F32, FloatCase{uint64(x), uint64(y), cmp.Compare(a, b), cmp.Less(a, b)})
		}
	}
	for _, x := range p64 {
		for _, y := range p64 {
			a, b := math.Float64frombits(x), math.Float64frombits(y)
			f.F64 = append(f.F64, FloatCase{x, y, cmp.Compare(a, b), cmp.Less(a, b)})
		}
	}
	f.Integers = append(f.Integers, integerCases("I8", []int8{-128, -1, 0, 1, 127})...)
	f.Integers = append(f.Integers, integerCases("I16", []int16{-32768, -1, 0, 1, 32767})...)
	f.Integers = append(f.Integers, integerCases("I32", []int32{-2147483648, -1, 0, 1, 2147483647})...)
	f.Integers = append(f.Integers, integerCases("I64", []int64{math.MinInt64, -1, 0, 1, math.MaxInt64})...)
	f.Integers = append(f.Integers, integerCases("Isize", []int{math.MinInt64, -1, 0, 1, math.MaxInt64})...)
	f.Integers = append(f.Integers, integerCases("U8", []uint8{0, 1, 127, 128, 255})...)
	f.Integers = append(f.Integers, integerCases("U16", []uint16{0, 1, 32767, 32768, 65535})...)
	f.Integers = append(f.Integers, integerCases("U32", []uint32{0, 1, 2147483647, 2147483648, 4294967295})...)
	f.Integers = append(f.Integers, integerCases("U64", []uint64{0, 1, math.MaxInt64, 1 << 63, math.MaxUint64})...)
	f.Integers = append(f.Integers, integerCases("Usize", []uint{0, 1, math.MaxInt64, 1 << 63, math.MaxUint64})...)
	f.Integers = append(f.Integers, integerCases("Int", []int64{math.MinInt64, -1, 0, 1, math.MaxInt64})...)
	f.Integers = append(f.Integers, integerCases("Nat", []uint64{0, 1, math.MaxInt64, 1 << 63, math.MaxUint64})...)
	ss := []string{"", "a", "aa", "a\x00b", "a\x00c", "b", "é", "雪", "😀", "\x00"}
	for _, x := range ss {
		for _, y := range ss {
			f.Strings = append(f.Strings, StringCase{x, y, cmp.Compare(x, y), cmp.Less(x, y)})
		}
	}
	return f
}

// main emits fixtures, or times deterministic public scalar comparison/selection loops.
func main() {
	if len(os.Args) == 1 {
		json.NewEncoder(os.Stdout).Encode(fixtures())
		return
	}
	n, _ := strconv.Atoi(os.Args[2])
	mode := os.Args[1]
	checksum := int64(0)
	start := time.Now()
	for i := 0; i < n; i++ {
		x := int64(i%257) - 128
		y := int64(i%193) - 96
		switch mode {
		case "integer":
			checksum += int64(cmp.Compare(x, y))
			if cmp.Less(x, y) {
				checksum += 3
			}
		case "float":
			a := float64(x)
			b := float64(y)
			if i%17 == 0 {
				a = math.NaN()
			}
			checksum += int64(cmp.Compare(a, b))
			if cmp.Less(a, b) {
				checksum += 3
			}
		case "choice":
			checksum += cmp.Or(int64(0), int64(0), x, y)
		case "string":
			a := []string{"", "alpha", "alpine", "é", "雪"}[i%5]
			b := []string{"", "alpha", "alpine", "é", "雪"}[i%3]
			checksum += int64(cmp.Compare(a, b))
			if cmp.Less(a, b) {
				checksum += 3
			}
		default:
			panic(mode)
		}
	}
	fmt.Println(time.Since(start).Nanoseconds(), checksum)
}
