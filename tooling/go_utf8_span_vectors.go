// Generate direct bounded-slice UTF-8 observations from the pinned public SDK.
package main

import (
	"encoding/hex"
	"encoding/json"
	"os"
	"runtime"
	"unicode/utf8"
)

// spanInput identifies one arbitrary owner buffer independently of span boundaries.
type spanInput struct {
	Input string `json:"input"`
}

// spanFile retains the original SDK identity and arbitrary-byte corpus.
type spanFile struct {
	SDK  string      `json:"sdk_version"`
	Rows []spanInput `json:"rows"`
}

// spanRow fingerprints all legal subspans and selected absolute decoding positions.
type spanRow struct {
	Input   string `json:"input"`
	Hash    uint64 `json:"hash"`
	Spans   int    `json:"spans"`
	Decodes int    `json:"decodes"`
}

// word hashes every byte of one observation in a fixed little-endian wire order.
func word(hash uint64, value uint32) uint64 {
	for shift := uint(0); shift < 32; shift += 8 {
		hash = (hash ^ uint64(byte(value>>shift))) * 1099511628211
	}
	return hash
}

// observation uses actual bounded slices; a scalar crossing either endpoint cannot influence a result.
func observation(data []byte) spanRow {
	hash := uint64(14695981039346656037)
	spans, decodes := 0, 0
	for start := 0; start <= len(data); start++ {
		for end := start; end <= len(data); end++ {
			spans++
			hash = word(hash, uint32(start))
			hash = word(hash, uint32(end))
			hash = word(hash, uint32(utf8.RuneCount(data[start:end])))
			positions := []int{0, start, start + 1, end, end + 1, len(data) + 1}
			if end > 0 {
				positions = append(positions, end-1)
			}
			for _, at := range positions {
				decodes++
				cp, width, status := int32(utf8.RuneError), 0, uint32(3)
				if at >= start && at <= end {
					if at == end {
						status = 0
					} else {
						cp, width = utf8.DecodeRune(data[at:end])
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
	return spanRow{hex.EncodeToString(data), hash, spans, decodes}
}

// main fixes the SDK epoch and selects diverse malformed bytes, alignments and scalar fringes.
func main() {
	if runtime.Version() != "go1.27.4" {
		panic("unexpected reference SDK")
	}
	if len(os.Args) != 2 {
		panic("expected raw UTF-8 fixture path")
	}
	data, err := os.ReadFile(os.Args[1])
	if err != nil {
		panic(err)
	}
	var input spanFile
	if err = json.Unmarshal(data, &input); err != nil {
		panic(err)
	}
	if input.SDK != runtime.Version() {
		panic("fixture/reference SDK mismatch")
	}
	inputs := [][]byte{}
	seen := map[string]bool{}
	add := func(bytes []byte) {
		key := string(bytes)
		if !seen[key] {
			seen[key] = true
			inputs = append(inputs, append([]byte{}, bytes...))
		}
	}
	add(nil)
	for value := 0; value < 256; value++ {
		add([]byte{byte(value)})
	}
	for index, row := range input.Rows {
		if index%17 == 0 {
			bytes, err := hex.DecodeString(row.Input)
			if err != nil {
				panic(err)
			}
			if len(bytes) <= 40 {
				add(bytes)
			}
		}
	}
	patterns := []string{"00", "7f", "c280", "dfbf", "e0a080", "ed9fbf", "ee8080", "efbfbd", "efbfbf", "f0908080", "f48fbfbf", "80", "e08080", "eda080", "f4908080", "f080"}
	for alignment := 0; alignment < 32; alignment++ {
		prefix := make([]byte, alignment)
		for index := range prefix {
			prefix[index] = 'a'
		}
		for _, pattern := range patterns {
			suffix, err := hex.DecodeString(pattern)
			if err != nil {
				panic(err)
			}
			bytes := append(append([]byte{}, prefix...), suffix...)
			bytes = append(bytes, 'z')
			add(bytes)
		}
	}
	rows := []spanRow{}
	spans, decodes := 0, 0
	for _, bytes := range inputs {
		row := observation(bytes)
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
