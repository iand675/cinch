// Generate independent bounded-span exact-byte search fingerprints.
package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"os"
	"runtime"
)

// input produces independent deterministic raw bytes, including malformed encodings and carry-sensitive lanes.
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
			if i+1 == size {
				raw[i] = 'b'
			}
		case 5:
			raw[i] = []byte{240, 159, 152, 128, 237, 160, 128, 194, 169, 128, 0, 255}[i%12]
		case 6:
			raw[i] = []byte("ababcaba")[i%8]
		default:
			raw[i] = byte(i % 251)
		}
	}
	return raw
}

// hashWord mixes a complete unsigned observation without dropping any of its field bits.
func hashWord(hash uint64, value uint32) uint64 {
	return (hash ^ uint64(value)) * 1099511628211
}

// spans visits every short-input subspan and every alignment/block fringe of larger inputs.
func spans(size int, visit func(int, int)) {
	if size <= 33 {
		for start := 0; start <= size; start++ {
			for end := start; end <= size; end++ {
				visit(start, end)
			}
		}
		return
	}
	starts := []int{}
	for start := 0; start < 32; start++ {
		starts = append(starts, start)
	}
	starts = append(starts, size/2, size-33, size-17, size-9, size-8, size-1, size)
	for _, start := range starts {
		for _, width := range []int{0, 1, 7, 8, 9, 15, 16, 17, 23, 24, 25, 31, 32, 33} {
			if start+width <= size {
				visit(start, start+width)
			}
		}
		visit(start, size)
	}
}

// main invokes actual public byte searches on independently bounded slices.
func main() {
	if runtime.Version() != "go1.27.4" {
		panic("unexpected reference SDK")
	}
	sizes := []int{}
	for size := 0; size <= 33; size++ {
		sizes = append(sizes, size)
	}
	sizes = append(sizes, 63, 64, 65, 127, 128, 129, 257, 4097, 65537)
	rows := []map[string]any{}
	for mode := 0; mode < 8; mode++ {
		for _, size := range sizes {
			raw := input(mode, size)
			hash := uint64(14695981039346656037)
			count, queries := 0, 0
			keys := []int{}
			if size <= 257 {
				for key := 0; key < 256; key++ {
					keys = append(keys, key)
				}
			} else {
				keys = []int{0, 1, 2, 10, 44, 97, 98, 127, 128, 129, 194, 237, 240, 253, 254, 255}
			}
			spans(size, func(start, end int) {
				count++
				hash = hashWord(hash, uint32(start))
				hash = hashWord(hash, uint32(end))
				for _, key := range keys {
					queries++
					hash = hashWord(hash, uint32(key))
					at := bytes.IndexByte(raw[start:end], byte(key))
					hash = hashWord(hash, uint32(at))
				}
			})
			digest := sha256.Sum256(raw)
			rows = append(rows, map[string]any{"mode": mode, "size": size, "raw_sha256": hex.EncodeToString(digest[:]), "spans": count, "queries": queries, "hash": hash})
		}
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]any{"sdk_version": runtime.Version(), "rows": rows}); err != nil {
		panic(err)
	}
}
