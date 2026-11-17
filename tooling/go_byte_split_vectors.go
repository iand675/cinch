// Generate independent complete splitting outputs with a pinned reference SDK.
package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"runtime"
)

// request is a language-neutral input recipe; negative limits mean unlimited.
type request struct {
	Input     string `json:"input"`
	Separator string `json:"separator"`
	Start     int    `json:"start"`
	End       int    `json:"end"`
	Limit     int    `json:"limit"`
	After     bool   `json:"after"`
	Lines     bool   `json:"lines"`
}

// word hashes a length or byte as a fixed-width little-endian observation.
func word(hash uint64, value uint32) uint64 {
	for shift := 0; shift < 32; shift += 8 {
		hash = (hash ^ uint64(byte(value>>shift))) * 1099511628211
	}
	return hash
}

// main emits actual outputs, not a reimplementation of splitting rules.
func main() {
	if runtime.Version() != "go1.27.4" {
		panic("unexpected reference SDK")
	}
	var inputs []request
	if err := json.NewDecoder(os.Stdin).Decode(&inputs); err != nil {
		panic(err)
	}
	rows := []map[string]any{}
	for _, input := range inputs {
		raw, err := hex.DecodeString(input.Input)
		if err != nil {
			panic(err)
		}
		separator, err := hex.DecodeString(input.Separator)
		if err != nil {
			panic(err)
		}
		view := raw[input.Start:input.End]
		var parts [][]byte
		if input.Lines {
			for part := range bytes.Lines(view) {
				parts = append(parts, part)
			}
		} else if input.After {
			parts = bytes.SplitAfterN(view, separator, input.Limit)
		} else {
			parts = bytes.SplitN(view, separator, input.Limit)
		}
		chunks := []string{}
		hash := uint64(14695981039346656037)
		for _, part := range parts {
			chunks = append(chunks, hex.EncodeToString(part))
			hash = word(hash, uint32(len(part)))
			for _, value := range part {
				hash = word(hash, uint32(value))
			}
		}
		if !input.Lines && input.Limit < 0 {
			sequence := bytes.SplitSeq(view, separator)
			if input.After {
				sequence = bytes.SplitAfterSeq(view, separator)
			}
			index := 0
			for part := range sequence {
				if index >= len(parts) || !bytes.Equal(part, parts[index]) {
					panic("list/sequence mismatch")
				}
				index++
			}
			if index != len(parts) {
				panic("sequence output count mismatch")
			}
		}
		rows = append(rows, map[string]any{"input": input.Input, "separator": input.Separator, "start": input.Start, "end": input.End, "limit": input.Limit, "after": input.After, "lines": input.Lines, "chunks": chunks, "count": len(parts), "hash": hash})
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]any{"sdk_version": runtime.Version(), "rows": rows}); err != nil {
		panic(fmt.Sprint(err))
	}
}
