// Generate independent exact immutable-string construction observations.
package main

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"os"
	"strings"
	"unicode/utf8"
)

// observation records every byte for short results and a full digest for every result.
func observation(text string) map[string]any {
	digest := sha256.Sum256([]byte(text))
	full := ""
	if len(text) <= 512 {
		full = hex.EncodeToString([]byte(text))
	}
	return map[string]any{"bytes": len(text), "scalars": utf8.RuneCountInString(text), "sha256": hex.EncodeToString(digest[:]), "hex": full}
}

// main covers empty, ASCII, control, multibyte, power-of-two and deep-list boundaries.
func main() {
	counts := []int{0, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257, 1023, 1024, 1025, 4095, 4096, 4097, 65535, 65536, 65537}
	units := []string{"", "x", "ab", "é", "λ界", "🙂", "a\x00z", "\r\n", "e\u0301"}
	repeats := []map[string]any{}
	for _, unit := range units {
		for _, count := range counts {
			row := observation(strings.Repeat(unit, count))
			row["text"], row["count"] = unit, count
			repeats = append(repeats, row)
		}
	}
	patterns := [][]string{{}, {""}, {"x"}, {"a", "", "b"}, {"é", "λ", "界", "🙂"}, {"a\x00", "\r", "\n", "z"}, {"e", "\u0301", "", "!"}}
	concats := []map[string]any{}
	for _, pieces := range patterns {
		for _, count := range []int{0, 1, 2, 3, 7, 8, 9, 63, 64, 65, 4095, 4096, 4097, 16384, 65536} {
			if len(pieces) > 1 && count > 16384 {
				continue
			}
			row := observation(strings.Repeat(strings.Join(pieces, ""), count))
			row["pieces"], row["cycles"] = pieces, count
			concats = append(concats, row)
		}
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]any{"repeats": repeats, "concats": concats}); err != nil {
		panic(err)
	}
}
