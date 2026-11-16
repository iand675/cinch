// Generate independent owned byte-buffer construction observations.
package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"os"
)

// observation records complete short outputs and every full output digest.
func observation(raw []byte) map[string]any {
	digest := sha256.Sum256(raw)
	full := ""
	if len(raw) <= 512 {
		full = hex.EncodeToString(raw)
	}
	return map[string]any{"bytes": len(raw), "hex": full, "sha256": hex.EncodeToString(digest[:])}
}

// decode parses exact independent raw input bytes, including malformed UTF-8.
func decode(text string) []byte {
	raw, err := hex.DecodeString(text)
	if err != nil {
		panic(err)
	}
	return raw
}

// main exercises empty pieces, invalid byte sequences and deep construction boundaries.
func main() {
	units := []string{"", "00", "ff", "0001", "80", "c3a9", "e282", "f09f9982", "0001ff80"}
	repeats := []map[string]any{}
	for _, unit := range units {
		for _, count := range []int{0, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 255, 256, 257, 4095, 4096, 4097, 65535, 65536, 65537} {
			row := observation(bytes.Repeat(decode(unit), count))
			row["input"], row["count"] = unit, count
			repeats = append(repeats, row)
		}
	}
	joins := []map[string]any{}
	for _, pattern := range [][]string{{}, {""}, {"00"}, {"00", "", "ff"}, {"c3", "a9"}, {"ff", "80", "00", "c3a9"}} {
		for _, cycles := range []int{0, 1, 2, 3, 7, 8, 9, 63, 64, 65, 1024, 16384} {
			for _, separator := range []string{"", "00", "ff", "c3a9", "80"} {
				pieces := [][]byte{}
				for n := 0; n < cycles; n++ {
					for _, part := range pattern {
						pieces = append(pieces, decode(part))
					}
				}
				row := observation(bytes.Join(pieces, decode(separator)))
				row["pattern"], row["cycles"], row["separator"] = pattern, cycles, separator
				joins = append(joins, row)
			}
		}
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]any{"repeats": repeats, "joins": joins}); err != nil {
		panic(err)
	}
}
