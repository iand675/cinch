// Capture scalar and complete buffer observations from the pinned reference SDK.
package main

import (
	"crypto/subtle"
	"encoding/hex"
	"encoding/json"
	"os"
	"runtime"
	"strconv"
)

// row describes public dimensions and complete before/after byte observations.
type row struct {
	Operation  string `json:"operation"`
	Lengths    []int  `json:"lengths"`
	Flag       int    `json:"flag"`
	Alias      int    `json:"alias"`
	Changed    int    `json:"changed"`
	Result     int    `json:"result"`
	Panicked   bool   `json:"panicked"`
	Output     string `json:"output"`
	OutputHash uint64 `json:"output_hash"`
}

// pattern creates reproducible prepared bytes without consulting a tested operation.
func pattern(size, salt int) []byte {
	out := make([]byte, size)
	for i := range out {
		out[i] = byte(i*73 + salt*37 + i/7)
	}
	return out
}

// capture records panic outcomes and the complete buffer even when no progress is permitted.
func capture(r row, destination []byte, operation func() int) row {
	func() {
		defer func() {
			if recover() != nil {
				r.Panicked = true
			}
		}()
		r.Result = operation()
	}()
	r.Output = hex.EncodeToString(destination)
	r.OutputHash = 14695981039346656037
	for _, value := range destination {
		r.OutputHash = (r.OutputHash ^ uint64(value)) * 1099511628211
	}
	return r
}

// main emits actual scalar results and bounded independent buffer operations.
func main() {
	if runtime.Version() != "go1.27.4" || strconv.IntSize != 64 {
		panic("unexpected reference SDK/word size")
	}
	lengths := []int{0, 1, 2, 3, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257}
	rows := []row{}
	for _, n := range lengths {
		for _, changed := range []int{-1, 0, n / 2, n - 1} {
			left, right := pattern(n, 1), pattern(n, 1)
			if changed >= 0 && changed < n {
				right[changed] ^= 0x80
			}
			rows = append(rows, row{Operation: "compare", Lengths: []int{n, n}, Changed: changed, Result: subtle.ConstantTimeCompare(left, right)})
		}
		rows = append(rows, row{Operation: "compare", Lengths: []int{n, n + 1}, Changed: -1, Result: subtle.ConstantTimeCompare(pattern(n, 1), pattern(n+1, 1))})
		for _, flag := range []int{0, 1} {
			for _, delta := range []int{-1, 0, 1} {
				d := n + delta
				if d < 0 {
					continue
				}
				destination, source := pattern(d, 2), pattern(n, 1)
				r := row{Operation: "copy", Lengths: []int{d, n}, Flag: flag}
				rows = append(rows, capture(r, destination, func() int { subtle.ConstantTimeCopy(flag, destination, source); return 0 }))
			}
			destination := pattern(n, 1)
			r := row{Operation: "copy", Lengths: []int{n, n}, Flag: flag, Alias: 1}
			rows = append(rows, capture(r, destination, func() int { subtle.ConstantTimeCopy(flag, destination, destination); return 0 }))
		}
		for _, m := range []int{0, n / 2, n, n + 1} {
			count := min(n, m)
			for _, extra := range []int{-1, 0, 3} {
				d := count + extra
				if d < 0 {
					continue
				}
				destination, left, right := pattern(d, 3), pattern(n, 1), pattern(m, 2)
				r := row{Operation: "xor", Lengths: []int{d, n, m}}
				rows = append(rows, capture(r, destination, func() int { return subtle.XORBytes(destination, left, right) }))
			}
			left, right := pattern(n, 1), pattern(m, 2)
			r := row{Operation: "xor", Lengths: []int{n, n, m}, Alias: 1}
			rows = append(rows, capture(r, left, func() int { return subtle.XORBytes(left, left, right) }))
			left, right = pattern(n, 1), pattern(m, 2)
			r = row{Operation: "xor", Lengths: []int{m, n, m}, Alias: 2}
			rows = append(rows, capture(r, right, func() int { return subtle.XORBytes(right, left, right) }))
		}
	}
	pairs := []int32{-2147483648, -2147483647, -256, -1, 0, 1, 255, 256, 2147483646, 2147483647}
	signed := []int64{-9223372036854775808, -2147483648, -1, 0, 1, 2147483647, 9223372036854775807}
	scalar := map[string]any{"i32": []any{}, "select": []any{}, "less_or_equal": []any{}}
	for _, x := range pairs {
		for _, y := range pairs {
			scalar["i32"] = append(scalar["i32"].([]any), []int64{int64(x), int64(y), int64(subtle.ConstantTimeEq(x, y))})
		}
	}
	for _, v := range []int{0, 1} {
		for _, x := range signed {
			for _, y := range signed {
				scalar["select"] = append(scalar["select"].([]any), []int64{int64(v), x, y, int64(subtle.ConstantTimeSelect(v, int(x), int(y)))})
			}
		}
	}
	for _, x := range []int{0, 1, 2, 255, 256, 65535, 2147483646, 2147483647} {
		for _, y := range []int{0, 1, 2, 255, 256, 65535, 2147483646, 2147483647} {
			scalar["less_or_equal"] = append(scalar["less_or_equal"].([]any), []int{x, y, subtle.ConstantTimeLessOrEq(x, y)})
		}
	}
	hash := uint64(14695981039346656037)
	for x := 0; x < 256; x++ {
		for y := 0; y < 256; y++ {
			hash = (hash ^ uint64(subtle.ConstantTimeByteEq(uint8(x), uint8(y)))) * 1099511628211
		}
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]any{"sdk": runtime.Version(), "word_bits": strconv.IntSize, "byte_equality_cases": 65536, "byte_equality_hash": hash, "scalars": scalar, "rows": rows}); err != nil {
		panic(err)
	}
}
