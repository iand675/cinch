// Command reference captures actual HKDF results, errors and panics without replacing the SDK algorithms.
package main

import (
	"crypto/fips140"
	"crypto/hkdf"
	"crypto/md5"
	"crypto/sha1"
	"crypto/sha256"
	"crypto/sha512"
	"encoding/hex"
	"encoding/json"
	"flag"
	"fmt"
	"hash"
	"os"
	"runtime"
)

// outcome retains successful empty/nil outputs separately from actual errors or panics.
type outcome struct {
	// Output is the complete byte result, rendered without truncation.
	Output string `json:"output_hex"`
	// Nil records the actual returned slice identity property.
	Nil bool `json:"nil"`
	// Returned distinguishes an ordinary return from a recovered panic.
	Returned bool `json:"returned"`
	// Error is the original SDK error string, if any.
	Error string `json:"error"`
	// Panic is the exact recovered panic text, if any.
	Panic string `json:"panic"`
}

// rfcCase names fixed AppendixA expectations independently of the implementation under test.
type rfcCase struct {
	// ID identifies the RFC case or explicit nil/empty variant.
	ID string `json:"id"`
	// Hash selects the actual reference hash constructor.
	Hash string `json:"hash"`
	// Secret holds complete original keying material.
	Secret string `json:"secret_hex"`
	// Salt preserves nil independently from a supplied empty slice.
	Salt *string `json:"salt_hex"`
	// PRK is the fixed expected extracted key used to isolate Expand observations.
	PRK string `json:"prk_hex"`
	// Info preserves all context octets.
	Info string `json:"info_hex"`
	// Length is the expected output width.
	Length int `json:"length"`
	// Output is the fixed full expected key.
	Output string `json:"output_hex"`
}

// toyHash is a segmentation-independent test-only hash proving unrestricted metadata adaptation.
type toyHash struct {
	// block is the declared HMAC block width, including zero.
	block int
	// width is the declared fixed output width, including zero and values above block.
	width int
	// state is nonzero-sized mutable streaming state, so independent constructors are distinct.
	state uint64
}

// Write updates the specified scalar state without modifying the supplied input.
func (h *toyHash) Write(input []byte) (int, error) {
	for _, value := range input {
		h.state = (h.state ^ uint64(value)) * 1099511628211
	}
	return len(input), nil
}

// Sum appends the exact advertised width without mutating streaming state.
func (h *toyHash) Sum(prefix []byte) []byte {
	for index := 0; index < h.width; index++ {
		prefix = append(prefix, byte(h.state>>uint((index%8)*8))^byte(index*29))
	}
	return prefix
}

// Reset restores the specified independent initial state.
func (h *toyHash) Reset() { h.state = 14695981039346656037 }

// Size reports the exact output width used by both HMAC and HKDF.
func (h *toyHash) Size() int { return h.width }

// BlockSize reports the exact key-normalization and pad width.
func (h *toyHash) BlockSize() int { return h.block }

// constructor preserves actual SDK implementations and creates distinct custom fixture states.
func constructor(name string, block, size int) func() hash.Hash {
	switch name {
	case "sha256":
		return sha256.New
	case "sha512":
		return sha512.New
	case "sha1":
		return sha1.New
	case "md5":
		return md5.New
	case "toy":
		return func() hash.Hash { h := &toyHash{block: block, width: size}; h.Reset(); return h }
	default:
		panic("unknown explicit oracle hash")
	}
}

// decode preserves every literal input octet and refuses malformed expected data.
func decode(value string) []byte {
	bytes, err := hex.DecodeString(value)
	if err != nil {
		panic(err)
	}
	return bytes
}

// decodeSalt retains an explicitly supplied empty key separately from nil.
func decodeSalt(value *string) []byte {
	if value == nil {
		return nil
	}
	return append([]byte{}, decode(*value)...)
}

// call records the original SDK outcome, including negative-length panics, without fabricating success.
func call(operation func() ([]byte, error)) (result outcome) {
	result.Nil = true
	defer func() {
		if failure := recover(); failure != nil {
			result.Panic = fmt.Sprint(failure)
		}
	}()
	value, err := operation()
	result.Returned = true
	result.Nil = value == nil
	result.Output = hex.EncodeToString(value)
	if err != nil {
		result.Error = err.Error()
	}
	return
}

// pattern supplies deterministic binary material containing zero and high bytes.
func pattern(size, seed int) []byte {
	result := make([]byte, size)
	for index := range result {
		result[index] = byte(index*137 + seed)
	}
	return result
}

// observe invokes all three actual public HKDF operations with original input ownership retained.
func observe(id, name string, block, size int, secret, salt, prk, info []byte, length int) map[string]any {
	h := constructor(name, block, size)
	before := []string{hex.EncodeToString(secret), hex.EncodeToString(salt), hex.EncodeToString(prk), hex.EncodeToString(info)}
	extract := call(func() ([]byte, error) { return hkdf.Extract(h, secret, salt) })
	expand := call(func() ([]byte, error) { return hkdf.Expand(h, prk, string(info), length) })
	key := call(func() ([]byte, error) { return hkdf.Key(h, secret, salt, string(info), length) })
	after := []string{hex.EncodeToString(secret), hex.EncodeToString(salt), hex.EncodeToString(prk), hex.EncodeToString(info)}
	return map[string]any{"id": id, "hash": name, "block_size": h().BlockSize(), "digest_size": h().Size(),
		"secret_hex": before[0], "secret_nil": secret == nil, "salt_hex": before[1], "salt_nil": salt == nil,
		"prk_hex": before[2], "prk_nil": prk == nil, "info_hex": before[3], "length": length,
		"extract": extract, "expand": expand, "key": key, "retained_before": before, "retained_after": after}
}

// main emits bounded complete families; policy-only runs inherit an explicitly chosen process mode.
func main() {
	family := flag.String("family", "all", "rfc, standard, custom, salt, policy, or all")
	rfcPath := flag.String("rfc", "rfc5869-vectors.json", "committed fixed AppendixA expectations")
	flag.Parse()
	rows := make([]map[string]any, 0)
	if *family == "rfc" || *family == "all" {
		data, err := os.ReadFile(*rfcPath)
		if err != nil { panic(err) }
		var fixed struct {
			// Vectors retains all fixed RFC and explicit empty/nil cases.
			Vectors []rfcCase `json:"vectors"`
		}
		if err := json.Unmarshal(data, &fixed); err != nil { panic(err) }
		for _, vector := range fixed.Vectors {
			row := observe(vector.ID, vector.Hash, 0, 0, decode(vector.Secret), decodeSalt(vector.Salt), decode(vector.PRK), decode(vector.Info), vector.Length)
			row["expected_prk_hex"], row["expected_output_hex"] = vector.PRK, vector.Output
			rows = append(rows, row)
		}
	}
	if *family == "standard" || *family == "all" {
		for _, name := range []string{"sha256", "sha512", "sha1", "md5"} {
			h := constructor(name, 0, 0)()
			n, block := h.Size(), h.BlockSize()
			lengths := []int{0, 1, n-1, n, n+1, 2*n-1, 2*n, 255*n-1, 255*n, 255*n+1, -1, -2}
			for index, length := range lengths {
				secret := pattern([]int{0, 1, 13, 14, 22, 80, block+1}[index%7], index)
				var salt []byte
				if index%3 == 1 { salt = []byte{} } else if index%3 == 2 { salt = pattern(block+7, index+19) }
				info := append([]byte{0, 255, 128, 195, 40}, pattern(index*3, 251)...)
				rows = append(rows, observe(fmt.Sprintf("standard-%s-%d", name, index), name, 0, 0, secret, salt, pattern(n+1, 73), info, length))
			}
		}
	}
	if *family == "custom" || *family == "all" {
		for _, shape := range [][2]int{{0,0}, {0,3}, {1,0}, {1,1}, {7,17}, {16,65}} {
			for index, length := range []int{0, 1, shape[1], 255*shape[1], 255*shape[1]+1, -1} {
				rows = append(rows, observe(fmt.Sprintf("toy-%d-%d-%d", shape[0], shape[1], index), "toy", shape[0], shape[1], pattern(19, 13), nil, pattern(shape[1]+1, 29), []byte{0,255,128}, length))
			}
		}
	}
	if *family == "salt" || *family == "all" {
		for _, name := range []string{"sha256", "sha512", "sha1", "md5", "toy"} {
			h := constructor(name, 7, 17)()
			for index, salt := range [][]byte{nil, {}, make([]byte, h.Size())} {
				rows = append(rows, observe(fmt.Sprintf("salt-%s-%d", name, index), name, 7, 17, pattern(32, 9), salt, pattern(h.Size(), 17), []byte{}, h.Size()+1))
			}
		}
	}
	if *family == "policy" {
		for _, name := range []string{"sha256", "sha512", "sha1", "md5", "toy"} {
			for _, length := range []int{0, 13, 14, 32} {
				h := constructor(name, 7, 17)()
				rows = append(rows, observe(fmt.Sprintf("policy-%s-%d", name, length), name, 7, 17, pattern(length, 1), nil, pattern(length, 2), nil, 255*h.Size()+1))
			}
		}
	}
	if len(rows) == 0 { panic("unknown or empty explicit oracle family") }
	result := map[string]any{"scope": "Actual public SDK observations; no Cinch qualification or elapsed claim", "sdk": runtime.Version(),
		"family": *family, "fips_enabled": fips140.Enabled(), "godebug": os.Getenv("GODEBUG"), "observations": rows}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(result); err != nil { panic(err) }
}
