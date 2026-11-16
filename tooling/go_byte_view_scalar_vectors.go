// Generate independent relative scalar observations over every checked byte span.
package main

import (
	"encoding/hex"
	"encoding/json"
	"os"
	"runtime"
	"unicode/utf8"
)

// sourceRow carries only input bytes; existing expected observations are ignored.
type sourceRow struct {
	Input string `json:"input"`
}

// sourceFile supplies a self-contained input corpus for repeatable regeneration.
type sourceFile struct {
	Rows []sourceRow `json:"rows"`
}

// resultRow fingerprints exact scalar, width and status observations for each owner.
type resultRow struct {
	Input   string `json:"input"`
	Hash    uint64 `json:"hash"`
	Spans   int    `json:"spans"`
	Decodes int    `json:"decodes"`
}

// word hashes one complete observation field in fixed little-endian order.
func word(hash uint64, value uint32) uint64 {
	for shift := uint(0); shift < 32; shift += 8 {
		hash = (hash ^ uint64(byte(value>>shift))) * 1099511628211
	}
	return hash
}

// observe derives results from independent bounded slices and relative coordinates.
func observe(input string) resultRow {
	raw, err := hex.DecodeString(input)
	if err != nil {
		panic(err)
	}
	hash := uint64(14695981039346656037)
	spans, decodes := 0, 0
	for start := 0; start <= len(raw); start++ {
		for end := start; end <= len(raw); end++ {
			spans++
			size := end - start
			hash = word(hash, uint32(start))
			hash = word(hash, uint32(end))
			for _, at := range []int{0, 1, size / 2, size, size + 1, len(raw) + 1} {
				decodes++
				cp, width, status := int32(utf8.RuneError), 0, uint32(3)
				if at <= size {
					if at == size {
						status = 0
					} else {
						cp, width = utf8.DecodeRune(raw[start+at : end])
						status = 1
						if cp == utf8.RuneError && width == 1 {
							status = 2
						}
					}
				}
				hash = word(hash, uint32(at))
				hash = word(hash, uint32(cp))
				hash = word(hash, uint32(width))
				hash = word(hash, status)
			}
		}
	}
	return resultRow{input, hash, spans, decodes}
}

// main verifies the reference epoch and regenerates complete observations from the retained input corpus.
func main() {
	if runtime.Version() != "go1.27.4" {
		panic("unexpected reference SDK")
	}
	if len(os.Args) != 2 {
		panic("expected retained input fixture")
	}
	data, err := os.ReadFile(os.Args[1])
	if err != nil {
		panic(err)
	}
	var input sourceFile
	if err = json.Unmarshal(data, &input); err != nil {
		panic(err)
	}
	rows := []resultRow{}
	spans, decodes := 0, 0
	for _, raw := range input.Rows {
		row := observe(raw.Input)
		rows = append(rows, row)
		spans += row.Spans
		decodes += row.Decodes
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err = encoder.Encode(map[string]any{"sdk_version": runtime.Version(), "rows": rows, "spans": spans, "decodes": decodes}); err != nil {
		panic(err)
	}
}
