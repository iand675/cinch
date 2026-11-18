// Generate independent CRC-64 table, checkpoint and incremental checksum fixtures.
package main

import (
	"encoding"
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"hash/crc64"
	"os"
	"strings"
)

// corpus describes inputs without using the implementation under test.
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

// failure preserves identifier-before-size-before-table error precedence.
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
	if strings.Contains(err.Error(), "tables") {
		return "table"
	}
	panic(err)
}

// main covers every base entry, arbitrary polynomial bits, exact tails and long inputs.
func main() {
	tables := []map[string]interface{}{}
	valid := []map[string]interface{}{}
	arbitrary := []map[string]interface{}{}
	loads := []map[string]interface{}{}
	polys := []uint64{0, 1, crc64.ISO, crc64.ECMA, 0xffffffffffffffff, 0x8000000000000000, 0xAD93D23594C935A9}
	prefix := []byte{0xaa, 0xbb, 0xcc}
	for _, poly := range polys {
		table := crc64.MakeTable(poly)
		empty, _ := crc64.New(table).(encoding.BinaryMarshaler).MarshalBinary()
		tables = append(tables, map[string]interface{}{"poly": poly, "entries": table[:], "empty_saved": hex.EncodeToString(empty)})
		for _, size := range []int{0, 1, 2, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 255, 256, 257, 2047, 2048, 2049, 16383, 16384, 16385, 65536} {
			for _, fill := range []bool{false, true} {
				input := corpus(size, fill)
				h := crc64.New(table)
				h.Write(input[:size/2])
				checkpoint, _ := h.(encoding.BinaryMarshaler).MarshalBinary()
				h.Write(input[size/2:])
				saved, _ := h.(encoding.BinaryMarshaler).MarshalBinary()
				appended, _ := h.(encoding.BinaryAppender).AppendBinary(append([]byte{}, prefix...))
				points := []int{}
				if size <= 65 {
					for split := 0; split <= size; split++ {
						points = append(points, split)
					}
				} else {
					for _, split := range []int{0, 1, 7, 8, 9, 15, 16, 17, size / 2, 255, 256, 257, 2047, 2048, 2049, size - 1, size} {
						if split <= size {
							points = append(points, split)
						}
					}
				}
				valid = append(valid, map[string]interface{}{"poly": poly, "size": size, "fill": fill, "word": h.Sum64(), "digest": hex.EncodeToString(h.Sum(nil)), "saved": hex.EncodeToString(saved), "mid": size / 2, "checkpoint": hex.EncodeToString(checkpoint), "append_sum": hex.EncodeToString(h.Sum(append([]byte{}, prefix...))), "append_saved": hex.EncodeToString(appended), "splits": points})
			}
		}
		for _, size := range []int{0, 1, 7, 8, 9, 63, 64, 65, 2047, 2048, 2049, 16383, 16384, 16385, 65536} {
			for _, seed := range []uint64{0, 1, 0x123456789abcdef0, 0x8000000000000000, 0xffffffffffffffff} {
				state := append([]byte{}, empty...)
				binary.BigEndian.PutUint64(state[12:], seed)
				h := crc64.New(table)
				if err := h.(encoding.BinaryUnmarshaler).UnmarshalBinary(state); err != nil {
					panic(err)
				}
				h.Write(corpus(size, false))
				arbitrary = append(arbitrary, map[string]interface{}{"poly": poly, "size": size, "seed": seed, "state": hex.EncodeToString(state), "word": h.Sum64()})
			}
		}
		for size := 0; size <= 24; size++ {
			for _, version := range []byte{0, 1, 2, 255} {
				b := make([]byte, size)
				copy(b, empty)
				if size >= 4 {
					b[3] = version
				}
				kind := failure(crc64.New(table).(encoding.BinaryUnmarshaler).UnmarshalBinary(b))
				loads = append(loads, map[string]interface{}{"poly": poly, "input": hex.EncodeToString(b), "kind": kind})
			}
		}
		for at := 0; at < 12; at++ {
			b := append([]byte{}, empty...)
			b[at] ^= 0x80
			kind := failure(crc64.New(table).(encoding.BinaryUnmarshaler).UnmarshalBinary(b))
			loads = append(loads, map[string]interface{}{"poly": poly, "input": hex.EncodeToString(b), "kind": kind})
		}
		other := crc64.New(crc64.MakeTable(poly ^ 0x42))
		loads = append(loads, map[string]interface{}{"poly": poly ^ 0x42, "input": hex.EncodeToString(empty), "kind": failure(other.(encoding.BinaryUnmarshaler).UnmarshalBinary(empty))})
	}
	e := json.NewEncoder(os.Stdout)
	e.SetIndent("", "  ")
	if err := e.Encode(map[string]interface{}{"tables": tables, "valid": valid, "arbitrary": arbitrary, "loads": loads}); err != nil {
		panic(err)
	}
}
