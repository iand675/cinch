// Generate independent valid-text joining and cloning observations.
package main

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"os"
	"strings"
	"unicode/utf8"
)

// observe records complete scalar and byte identity for one immutable result.
func observe(text string) map[string]any {
	raw := []byte(text)
	digest := sha256.Sum256(raw)
	full := ""
	if len(raw) <= 512 {
		full = hex.EncodeToString(raw)
	}
	return map[string]any{"bytes": len(raw), "scalars": utf8.RuneCountInString(text), "hex": full, "sha256": hex.EncodeToString(digest[:])}
}

// main covers empty pieces, every scalar width and deep separator boundaries.
func main() {
	rows := []map[string]any{}
	for _, pattern := range [][]string{{}, {""}, {"x"}, {"", "x", ""}, {"é", "λ界🙂"}, {"a\x00z", "e\u0301", "�"}} {
		for _, count := range []int{0, 1, 2, 3, 7, 8, 9, 31, 32, 33, 255, 256, 257, 1024, 16384} {
			pieces := []string{}
			for i := 0; i < count; i++ {
				pieces = append(pieces, pattern...)
			}
			for _, separator := range []string{"", ",", "\x00", "é", "界🙂"} {
				text := strings.Join(pieces, separator)
				row := observe(text)
				row["pattern"], row["cycles"], row["separator"] = pattern, count, separator
				row["cloneEqual"] = strings.Clone(text) == text
				rows = append(rows, row)
			}
		}
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]any{"joins": rows}); err != nil {
		panic(err)
	}
}
