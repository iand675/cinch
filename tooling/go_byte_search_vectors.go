// Generate independent exact raw-byte search observations for broad-word and scalar fringes.
package main

import (
	"bytes"
	"encoding/json"
	"os"
)

// input constructs exact bytes, including adjacent equal/one-bit-different lanes.
func input(mode, count int, needle byte) []byte {
	raw := make([]byte, count)
	for i := range raw {
		switch mode {
		case 0:
			raw[i] = 0
		case 1:
			raw[i] = 255
		case 2:
			raw[i] = byte(i*37 + 11)
		case 3:
			raw[i] = byte(i % 2)
		default:
			if i%4 == 0 {
				raw[i] = needle
			} else {
				raw[i] = needle ^ 1
			}
		}
	}
	return raw
}

// row records the reference's complete first/last offsets and nonoverlapping one-byte count.
func row(mode, count int, needle byte) map[string]any {
	raw := input(mode, count, needle)
	return map[string]any{"mode": mode, "size": count, "needle": needle, "first": bytes.IndexByte(raw, needle), "last": bytes.LastIndexByte(raw, needle), "count": bytes.Count(raw, []byte{needle})}
}

// main exercises every byte value and exact broad-word/tail boundaries.
func main() {
	rows := []map[string]any{}
	for mode := 0; mode < 5; mode++ {
		for _, count := range []int{0, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257, 1023, 1024, 1025, 4095, 4096, 4097, 65535, 65536, 65537} {
			for _, needle := range []byte{0, 1, 2, 7, 15, 16, 63, 64, 127, 128, 129, 254, 255} {
				rows = append(rows, row(mode, count, needle))
			}
		}
	}
	for needle := 0; needle < 256; needle++ {
		for _, count := range []int{7, 8, 9, 15, 16, 17} {
			rows = append(rows, row(4, count, byte(needle)))
		}
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]any{"rows": rows}); err != nil {
		panic(err)
	}
}
