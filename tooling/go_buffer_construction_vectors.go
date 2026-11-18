// Generate independent scalar-pack and typed byte-list construction observations.
package main

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"os"
	"strings"
	"unicode/utf8"
)

// observed records full byte identity and short exact byte sequences.
func observed(raw []byte) map[string]any {
	digest := sha256.Sum256(raw)
	full := ""
	if len(raw) <= 512 {
		full = hex.EncodeToString(raw)
	}
	return map[string]any{"bytes": len(raw), "sha256": hex.EncodeToString(digest[:]), "hex": full}
}

// main covers every UTF-8 width, scalar boundary and deep typed-list length.
func main() {
	counts := []int{0, 1, 2, 3, 4, 7, 8, 9, 15, 16, 17, 63, 64, 65, 255, 256, 257, 4095, 4096, 4097, 65535, 65536, 65537}
	units := []string{"", "x", "é", "λ界🙂", "a\x00z", "e\u0301", "\u007f\u0080\u07ff\u0800\ud7ff\ue000\uffff\U00010000\U0010ffff", "�"}
	packed := []map[string]any{}
	for _, unit := range units {
		for _, count := range counts {
			text := strings.Repeat(unit, count)
			row := observed([]byte(text))
			row["text"], row["count"], row["scalars"] = unit, count, utf8.RuneCountInString(text)
			packed = append(packed, row)
		}
	}
	buffers := []map[string]any{}
	for mode := 0; mode < 4; mode++ {
		for _, count := range counts {
			raw := make([]byte, count)
			for i := range raw {
				switch mode {
				case 0:
					raw[i] = 0
				case 1:
					raw[i] = 255
				case 2:
					raw[i] = byte(i*37 + 11)
				default:
					if i%2 == 0 {
						raw[i] = 0xaa
					} else {
						raw[i] = 0x55
					}
				}
			}
			row := observed(raw)
			row["mode"], row["count"] = mode, count
			buffers = append(buffers, row)
		}
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]any{"packed": packed, "buffers": buffers}); err != nil {
		panic(err)
	}
}
