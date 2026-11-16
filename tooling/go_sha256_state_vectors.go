// Generate independent SHA-224/SHA-256 digest, checkpoint and restored-state fixtures.
package main

import (
	"crypto/sha256"
	"encoding"
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"hash"
	"os"
	"strings"
)

// algorithm selects a fresh independent accumulator for the requested variant.
func algorithm(short bool) hash.Hash {
	if short {
		return sha256.New224()
	}
	return sha256.New()
}

// corpus describes deterministic full-byte data independently of the implementation under test.
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

// failure preserves the reference identifier-before-size rejection order.
func failure(err error) string {
	if err == nil {
		return ""
	}
	if strings.Contains(err.Error(), "identifier") {
		return "identifier"
	}
	if strings.Contains(err.Error(), "size") {
		return "size"
	}
	panic(err)
}

// main includes both padding boundaries, every small split, noncanonical tails and wrapped counts.
func main() {
	valid := []map[string]interface{}{}
	arbitrary := []map[string]interface{}{}
	loads := []map[string]interface{}{}
	prefix := []byte{0xaa, 0xbb, 0xcc}
	for _, short := range []bool{false, true} {
		for _, size := range []int{0, 1, 2, 3, 7, 8, 15, 16, 31, 32, 33, 54, 55, 56, 57, 62, 63, 64, 65, 66, 119, 120, 121, 127, 128, 129, 255, 256, 257, 4095, 4096, 4097, 65536} {
			for _, fill := range []bool{false, true} {
				input := corpus(size, fill)
				h := algorithm(short)
				h.Write(input[:size/2])
				checkpoint, _ := h.(encoding.BinaryMarshaler).MarshalBinary()
				h.Write(input[size/2:])
				saved, _ := h.(encoding.BinaryMarshaler).MarshalBinary()
				appended, _ := h.(encoding.BinaryAppender).AppendBinary(append([]byte{}, prefix...))
				points := []int{}
				if size <= 129 {
					for at := 0; at <= size; at++ {
						points = append(points, at)
					}
				} else {
					for _, at := range []int{0, 1, 55, 56, 57, 63, 64, 65, 119, 120, 121, 127, 128, 129, size / 2, size - 1, size} {
						if at <= size {
							points = append(points, at)
						}
					}
				}
				valid = append(valid, map[string]interface{}{"short": short, "size": size, "fill": fill, "digest": hex.EncodeToString(h.Sum(nil)), "saved": hex.EncodeToString(saved), "mid": size / 2, "checkpoint": hex.EncodeToString(checkpoint), "append_sum": hex.EncodeToString(h.Sum(append([]byte{}, prefix...))), "append_saved": hex.EncodeToString(appended), "splits": points})
			}
		}
		initial, _ := algorithm(short).(encoding.BinaryMarshaler).MarshalBinary()
		for _, count := range []uint64{0, 1, 55, 56, 63, 64, 65, 0x800000000000003f, 0xffffffffffffffc0, 0xffffffffffffffff} {
			for _, size := range []int{0, 1, 7, 63, 64, 65, 128, 257} {
				state := append([]byte{}, initial...)
				for at := 4; at < 100; at++ {
					state[at] = byte(at*53 + int(count&255))
				}
				binary.BigEndian.PutUint64(state[100:], count)
				h := algorithm(short)
				if err := h.(encoding.BinaryUnmarshaler).UnmarshalBinary(state); err != nil {
					panic(err)
				}
				canonical, _ := h.(encoding.BinaryMarshaler).MarshalBinary()
				before := hex.EncodeToString(h.Sum(nil))
				h.Write(corpus(size, false))
				saved, _ := h.(encoding.BinaryMarshaler).MarshalBinary()
				arbitrary = append(arbitrary, map[string]interface{}{"short": short, "size": size, "count": count, "state": hex.EncodeToString(state), "canonical": hex.EncodeToString(canonical), "before": before, "digest": hex.EncodeToString(h.Sum(nil)), "saved": hex.EncodeToString(saved)})
			}
		}
		for size := 0; size <= 112; size++ {
			for _, version := range []byte{0, 2, 3, 255} {
				b := make([]byte, size)
				copy(b, initial)
				if size >= 4 {
					b[3] = version
				}
				h := algorithm(short)
				kind := failure(h.(encoding.BinaryUnmarshaler).UnmarshalBinary(b))
				canonical := ""
				if kind == "" {
					v, _ := h.(encoding.BinaryMarshaler).MarshalBinary()
					canonical = hex.EncodeToString(v)
				}
				loads = append(loads, map[string]interface{}{"short": short, "input": hex.EncodeToString(b), "kind": kind, "canonical": canonical})
			}
		}
		for at := 0; at < 4; at++ {
			b := append([]byte{}, initial...)
			b[at] ^= 0x80
			loads = append(loads, map[string]interface{}{"short": short, "input": hex.EncodeToString(b), "kind": failure(algorithm(short).(encoding.BinaryUnmarshaler).UnmarshalBinary(b)), "canonical": ""})
		}
	}
	e := json.NewEncoder(os.Stdout)
	e.SetIndent("", "  ")
	if err := e.Encode(map[string]interface{}{"valid": valid, "arbitrary": arbitrary, "loads": loads}); err != nil {
		panic(err)
	}
}
