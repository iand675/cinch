// Generate independent FNV state fixtures; the compiler tests read the resulting JSON.
package main

import (
	"encoding"
	"encoding/hex"
	"encoding/json"
	"hash"
	"hash/fnv"
	"os"
	"strings"
)

// stateBytes obtains the reference's complete versioned checkpoint.
func stateBytes(h hash.Hash) []byte {
	b, err := h.(encoding.BinaryMarshaler).MarshalBinary()
	if err != nil {
		panic(err)
	}
	return b
}

// main observes all variants, resumable prefixes, arbitrary payloads and malformed checkpoints.
func main() {
	factories := []func() hash.Hash{
		func() hash.Hash { return fnv.New32() }, func() hash.Hash { return fnv.New32a() },
		func() hash.Hash { return fnv.New64() }, func() hash.Hash { return fnv.New64a() },
		fnv.New128, fnv.New128a,
	}
	inputs := [][]byte{}
	for _, n := range []int{0, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 31, 32, 33, 255, 256, 257} {
		b := make([]byte, n)
		for i := range b {
			b[i] = byte(i*37 + 11)
		}
		inputs = append(inputs, b)
	}
	prefix := []byte{0xaa, 0xbb, 0xcc}
	valid := []map[string]interface{}{}
	arbitrary := []map[string]interface{}{}
	invalid := []map[string]interface{}{}
	identifiers := []map[string]interface{}{}
	for variant, factory := range factories {
		for position := 0; position < 4; position++ {
			for _, replacement := range []byte{0, 0xff} {
				b := stateBytes(factory())
				b[position] = replacement
				err := factory().(encoding.BinaryUnmarshaler).UnmarshalBinary(b)
				if err == nil || !strings.Contains(err.Error(), "identifier") {
					panic("altered checkpoint identifier was accepted")
				}
				identifiers = append(identifiers, map[string]interface{}{"variant": variant, "input": hex.EncodeToString(b)})
			}
		}
		for _, input := range inputs {
			h := factory()
			initial := hex.EncodeToString(stateBytes(h))
			mid := len(input) / 2
			h.Write(input[:mid])
			checkpoint := hex.EncodeToString(stateBytes(h))
			h.Write(input[mid:])
			saved := stateBytes(h)
			appended, err := h.(encoding.BinaryAppender).AppendBinary(append([]byte{}, prefix...))
			if err != nil {
				panic(err)
			}
			valid = append(valid, map[string]interface{}{"variant": variant, "input": hex.EncodeToString(input), "initial": initial, "mid": mid, "checkpoint": checkpoint, "digest": hex.EncodeToString(h.Sum(nil)), "saved": hex.EncodeToString(saved), "append_sum": hex.EncodeToString(h.Sum(append([]byte{}, prefix...))), "append_saved": hex.EncodeToString(appended), "size": h.Size(), "block_size": h.BlockSize()})
		}
		for _, fill := range []byte{0, 0x80, 0xff} {
			for _, input := range inputs[:12] {
				h := factory()
				seed := stateBytes(h)
				for i := 4; i < len(seed); i++ {
					seed[i] = fill
				}
				if err := h.(encoding.BinaryUnmarshaler).UnmarshalBinary(seed); err != nil {
					panic(err)
				}
				h.Write(input)
				arbitrary = append(arbitrary, map[string]interface{}{"variant": variant, "seed": hex.EncodeToString(seed), "input": hex.EncodeToString(input), "digest": hex.EncodeToString(h.Sum(nil))})
			}
		}
		for size := 0; size <= 24; size++ {
			for _, tag := range []byte{byte(variant + 1), 0, 7, byte((variant+1)%6 + 1)} {
				b := make([]byte, size)
				copy(b, []byte{'f', 'n', 'v', tag})
				err := factory().(encoding.BinaryUnmarshaler).UnmarshalBinary(b)
				kind := ""
				if err != nil {
					if strings.Contains(err.Error(), "identifier") {
						kind = "identifier"
					} else {
						kind = "size"
					}
				}
				invalid = append(invalid, map[string]interface{}{"variant": variant, "input": hex.EncodeToString(b), "kind": kind})
			}
		}
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]interface{}{"valid": valid, "arbitrary": arbitrary, "loads": invalid, "identifiers": identifiers}); err != nil {
		panic(err)
	}
}
