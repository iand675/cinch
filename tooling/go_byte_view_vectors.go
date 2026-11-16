// Generate independent byte-span, substring and lexicographic observations.
package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"os"
)

// input supplies periodic, malformed, adversarial and exact scalar boundary bytes.
func input(mode, size int) []byte {
	raw := make([]byte, size)
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
		case 4:
			raw[i] = 'a'
			if i == size-1 {
				raw[i] = 'b'
			}
		case 5:
			raw[i] = []byte{0xf0, 0x9f, 0x98, 0x80, 0xed, 0xa0, 0x80, 0xc2, 0xa9, 0x80, 0, 0xff}[i%12]
		case 6:
			raw[i] = []byte("ababcaba")[i%8]
		default:
			raw[i] = byte(i % 251)
		}
	}
	return raw
}

// row preserves complete raw inputs alongside the standard byte-query answers.
func row(mode, size, start, end, nm, ns, source int) map[string]any {
	raw := input(mode, size)
	needle := input(nm, ns)
	if source >= 0 {
		needle = raw[source : source+ns]
	}
	view := raw[start:end]
	hash := sha256.Sum256(raw)
	return map[string]any{"mode": mode, "size": size, "start": start, "end": end, "needle_mode": nm, "needle_size": ns, "needle_source": source, "raw_sha256": hex.EncodeToString(hash[:]), "needle_hex": hex.EncodeToString(needle), "first": bytes.Index(view, needle), "last": bytes.LastIndex(view, needle), "count": bytes.Count(view, needle), "prefix": bytes.HasPrefix(view, needle), "suffix": bytes.HasSuffix(view, needle), "compare": bytes.Compare(view, needle), "equal": bytes.Equal(view, needle)}
}

// main covers block fringes, both cut directions, overlaps and truncated raw scalars.
func main() {
	rows := []map[string]any{}
	for mode := 0; mode < 8; mode++ {
		for _, size := range []int{0, 1, 2, 3, 7, 8, 9, 15, 16, 17, 31, 32, 33, 65, 129} {
			spans := [][2]int{{0, size}, {size, size}}
			if size > 2 {
				spans = append(spans, [2]int{1, size - 1}, [2]int{size / 2, size})
			}
			for _, span := range spans {
				for _, width := range []int{0, 1, 2, 7, 8, 9, 16, 17} {
					rows = append(rows, row(mode, size, span[0], span[1], (mode+1)%8, width, -1))
					if width <= size-span[0] {
						rows = append(rows, row(mode, size, span[0], span[1], 0, width, span[0]))
					}
				}
			}
		}
	}
	for _, size := range []int{1025, 4097, 65537} {
		for _, mode := range []int{4, 5, 6} {
			for _, width := range []int{1, 8, 9, 31, 257} {
				rows = append(rows, row(mode, size, 1, size-1, mode, width, -1))
			}
		}
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]any{"rows": rows}); err != nil {
		panic(err)
	}
}
