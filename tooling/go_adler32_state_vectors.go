// Generate independent Adler-32 checksum, checkpoint and restored-state fixtures.
package main

import (
	"encoding"
	"encoding/hex"
	"encoding/json"
	"hash/adler32"
	"os"
	"strings"
)

// corpus makes a compactly described input independently of the Cinch implementation.
func corpus(size int, fill bool) []byte {
	b := make([]byte, size)
	for i := range b {
		if fill {
			b[i] = 255
		} else {
			b[i] = byte(i*37 + 11)
		}
	}
	return b
}

// main includes segment boundaries and arbitrary accepted words that exercise 32-bit wrapping.
func main() {
	valid := []map[string]interface{}{}
	arbitrary := []map[string]interface{}{}
	loads := []map[string]interface{}{}
	prefix := []byte{0xaa, 0xbb, 0xcc}
	for _, size := range []int{0, 1, 2, 3, 4, 7, 8, 15, 16, 17, 31, 32, 33, 255, 256, 257, 5551, 5552, 5553, 11103, 11104, 11105, 65536} {
		for _, fill := range []bool{false, true} {
			input := corpus(size, fill)
			h := adler32.New()
			h.Write(input[:size/2])
			checkpoint, _ := h.(encoding.BinaryMarshaler).MarshalBinary()
			h.Write(input[size/2:])
			saved, _ := h.(encoding.BinaryMarshaler).MarshalBinary()
			appended, _ := h.(encoding.BinaryAppender).AppendBinary(append([]byte{}, prefix...))
			points := []int{}
			if size <= 257 {
				for split := 0; split <= size; split++ {
					points = append(points, split)
				}
			} else {
				for _, split := range []int{0, 1, 15, 16, 17, size / 2, 5551, 5552, 5553, 11103, 11104, size - 1, size} {
					if split <= size {
						points = append(points, split)
					}
				}
			}
			valid = append(valid, map[string]interface{}{"size": size, "fill": fill, "word": h.Sum32(), "digest": hex.EncodeToString(h.Sum(nil)), "saved": hex.EncodeToString(saved), "mid": size / 2, "checkpoint": hex.EncodeToString(checkpoint), "append_sum": hex.EncodeToString(h.Sum(append([]byte{}, prefix...))), "append_saved": hex.EncodeToString(appended), "splits": points})
			for _, seed := range []uint32{0, 1, 0x12345678, 0x80000000, 0xffffffff, 0xfff0fff0, 0xfff1fff1, 0xffff0000, 0x0000ffff} {
				state := []byte{'a', 'd', 'l', 1, byte(seed >> 24), byte(seed >> 16), byte(seed >> 8), byte(seed)}
				g := adler32.New()
				if err := g.(encoding.BinaryUnmarshaler).UnmarshalBinary(state); err != nil {
					panic(err)
				}
				g.Write(input)
				arbitrary = append(arbitrary, map[string]interface{}{"size": size, "fill": fill, "seed": seed, "state": hex.EncodeToString(state), "word": g.Sum32()})
			}
		}
	}
	for size := 0; size <= 12; size++ {
		for _, version := range []byte{0, 1, 2, 255} {
			b := make([]byte, size)
			copy(b, []byte{'a', 'd', 'l', version})
			err := adler32.New().(encoding.BinaryUnmarshaler).UnmarshalBinary(b)
			kind := ""
			if err != nil {
				if strings.Contains(err.Error(), "identifier") {
					kind = "identifier"
				} else {
					kind = "size"
				}
			}
			loads = append(loads, map[string]interface{}{"input": hex.EncodeToString(b), "kind": kind})
		}
	}
	for at := 0; at < 4; at++ {
		for _, replacement := range []byte{0, 255} {
			b := []byte{'a', 'd', 'l', 1, 0, 0, 0, 1}
			b[at] = replacement
			if err := adler32.New().(encoding.BinaryUnmarshaler).UnmarshalBinary(b); err == nil || !strings.Contains(err.Error(), "identifier") {
				panic("corrupt identifier accepted")
			}
			loads = append(loads, map[string]interface{}{"input": hex.EncodeToString(b), "kind": "identifier"})
		}
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]interface{}{"valid": valid, "arbitrary": arbitrary, "loads": loads}); err != nil {
		panic(err)
	}
}
