// Generate independent valid-text scalar observations and legacy raw-prefix counts.
package main

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"os"
	"strings"
	"unicode/utf8"
)

// observation records exact scalar counts, endpoints, widths and full byte identity.
func observation(text string) map[string]any {
	raw := []byte(text)
	digest := sha256.Sum256(raw)
	first, fw := utf8.DecodeRuneInString(text)
	last, lw := utf8.DecodeLastRuneInString(text)
	firstCode, lastCode := int(first), int(last)
	if len(raw) == 0 {
		firstCode, lastCode = -1, -1
	}
	return map[string]any{"bytes": len(raw), "scalars": utf8.RuneCountInString(text), "first": firstCode, "firstWidth": fw, "last": lastCode, "lastWidth": lw, "sha256": hex.EncodeToString(digest[:])}
}

// main exercises block fringes, every scalar width, embedded NUL and all scalar values.
func main() {
	counts := []int{}
	for n := 0; n <= 33; n++ {
		counts = append(counts, n)
	}
	counts = append(counts, 63, 64, 65, 127, 128, 129, 255, 256, 257, 4095, 4096, 4097, 65535, 65536, 65537)
	texts := []map[string]any{}
	for _, unit := range []string{"", "x", "\x00", "a\x00z", "é", "λ", "界", "🙂", "é界🙂", "\u007f\u0080\u07ff\u0800\ud7ff\ue000\uffff\U00010000\U0010ffff", "�", "e\u0301", "\ufeff\u2028\u2029"} {
		for _, count := range counts {
			row := observation(strings.Repeat(unit, count))
			row["text"], row["count"] = unit, count
			texts = append(texts, row)
		}
	}
	edges := []map[string]any{}
	for _, cp := range []rune{0, 0x7f, 0x80, 0x7ff, 0x800, 0xd7ff, 0xe000, 0xffff, 0x10000, 0x10ffff, 0xfffd, 0x301, 0xfeff, 0x2028, 0x2029, 0x1f642} {
		for prefix := 0; prefix < 32; prefix++ {
			for suffix := 0; suffix < 4; suffix++ {
				text := strings.Repeat("a", prefix) + string(cp) + strings.Repeat("z", suffix)
				row := observation(text)
				row["text"], row["count"] = text, 1
				edges = append(edges, row)
			}
		}
	}
	aggregates := []map[string]any{}
	for _, span := range [][2]int{{0, 127}, {128, 2047}, {2048, 55295}, {55296, 57343}, {57344, 65535}, {65536, 1114111}, {0, 1114111}} {
		var text strings.Builder
		for cp := span[0]; cp <= span[1]; cp++ {
			if cp < 0xd800 || cp > 0xdfff {
				text.WriteRune(rune(cp))
			}
		}
		row := observation(text.String())
		row["low"], row["high"] = span[0], span[1]
		aggregates = append(aggregates, row)
	}
	rawRows := []map[string]any{}
	for _, source := range []string{"", "00", "61", "c3a9", "efbfbd", "f09f9982", "61ff62", "618062", "61c262", "61e282", "61f09f99", "c080", "eda080", "f4908080", "ff00", "c3a900f09f9982"} {
		raw, err := hex.DecodeString(source)
		if err != nil {
			panic(err)
		}
		for start := 0; start <= len(raw)+1; start++ {
			for _, initial := range []int{0, 7} {
				count := initial
				at := start
				for at < len(raw) {
					cp, width := utf8.DecodeRune(raw[at:])
					if cp == utf8.RuneError && width == 1 {
						break
					}
					count++
					at += width
				}
				rawRows = append(rawRows, map[string]any{"hex": source, "start": start, "initial": initial, "result": count})
			}
		}
	}
	out := json.NewEncoder(os.Stdout)
	out.SetIndent("", "  ")
	if err := out.Encode(map[string]any{"texts": texts, "edges": edges, "aggregates": aggregates, "raw": rawRows}); err != nil {
		panic(err)
	}
}
