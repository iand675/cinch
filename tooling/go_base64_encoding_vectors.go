// Generate independent configurable Base64 wire and malformed-input fixtures.
package main

import (
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"hash/fnv"
	"os"
	"strings"
)

// scheme records a complete byte alphabet, padding policy and trailing-bit policy.
type scheme struct {
	Name     string `json:"name"`
	Alphabet string `json:"alphabet"`
	Padding  int32  `json:"padding"`
	Strict   bool   `json:"strict"`
}

// encoding constructs a fresh independent encoder from the recorded configuration.
func (s scheme) encoding() *base64.Encoding {
	b, _ := hex.DecodeString(s.Alphabet)
	e := base64.NewEncoding(string(b)).WithPadding(s.Padding)
	if s.Strict {
		e = e.Strict()
	}
	return e
}

// corpus produces full-byte data without depending on any encoding operation.
func corpus(n int) []byte {
	b := make([]byte, n)
	for i := range b {
		b[i] = byte(i*37 + 11)
	}
	return b
}

// fingerprint observes every output byte using an independent fixed-width recurrence.
func fingerprint(b []byte) uint64 {
	h := fnv.New64a()
	h.Write(b)
	return h.Sum64()
}

// points includes every small split and representative larger quantum boundaries.
func points(n int) []int {
	if n <= 32 {
		p := make([]int, n+1)
		for i := range p {
			p[i] = i
		}
		return p
	}
	return []int{0, 1, 2, 3, 4, 5, 6, 7, 8, n / 2, n - 1, n}
}

// main records exact short wires, complete long fingerprints and independent failure prefixes.
func main() {
	std := "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
	url := "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"
	high := make([]byte, 64)
	for i := range high {
		high[i] = byte(i + 128)
	}
	reverse := []byte(std)
	for i := 0; i < 32; i++ {
		reverse[i], reverse[63-i] = reverse[63-i], reverse[i]
	}
	equals := "=" + std[1:]
	schemes := []scheme{
		{"standard", hex.EncodeToString([]byte(std)), '=', false},
		{"standard-strict", hex.EncodeToString([]byte(std)), '=', true},
		{"url", hex.EncodeToString([]byte(url)), '=', false},
		{"raw-standard", hex.EncodeToString([]byte(std)), -1, false},
		{"raw-url-strict", hex.EncodeToString([]byte(url)), -1, true},
		{"reverse-dot", hex.EncodeToString(reverse), '.', false},
		{"high-pad", hex.EncodeToString(high), 255, true},
		{"high-raw", hex.EncodeToString(high), -1, false},
		{"equals-zero", hex.EncodeToString([]byte(equals)), 0, true},
	}
	valid := []map[string]interface{}{}
	bad := []map[string]interface{}{}
	for config, s := range schemes {
		e := s.encoding()
		for _, n := range []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129, 255, 256, 257, 1023, 1024, 1025, 4095, 4096, 4097, 65536} {
			input := corpus(n)
			encoded := e.EncodeToString(input)
			hexwire := ""
			if n <= 257 {
				hexwire = hex.EncodeToString([]byte(encoded))
			}
			valid = append(valid, map[string]interface{}{"config": config, "size": n, "encoded": hexwire, "fingerprint": fingerprint([]byte(encoded)), "encoded_length": e.EncodedLen(n), "decoded_bound": e.DecodedLen(len(encoded)), "encoder_splits": points(n), "decoder_splits": points(len(encoded))})
		}
		for _, n := range []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 12} {
			wire := e.EncodeToString(corpus(n))
			mutations := []string{wire + "!", "!" + wire, wire + "\r\n!", wire[:len(wire)-1]}
			for i := range wire {
				mutations = append(mutations, wire[:i]+"!"+wire[i+1:])
			}
			for _, input := range mutations {
				decoded, err := e.DecodeString(input)
				offset := int64(-1)
				if value, ok := err.(base64.CorruptInputError); ok {
					offset = int64(value)
				}
				bad = append(bad, map[string]interface{}{"config": config, "input": hex.EncodeToString([]byte(input)), "decoded": hex.EncodeToString(decoded), "error": err != nil, "reference_offset": offset})
			}
		}
	}
	strict := []map[string]interface{}{}
	for _, input := range []string{"Zg==", "Zh==", "Zi==", "Zv==", "Zm8=", "Zm9=", "Zm+=", "Zm8", "Zm9", "Zh", "Zg", "Zh\r\n==", "Zm9\r\n=", "Zm9vZh==", "Zm9vZh", "Zm9vZm9="} {
		for _, raw := range []bool{false, true} {
			e := base64.StdEncoding.Strict()
			if raw {
				e = e.WithPadding(base64.NoPadding)
			}
			out, err := e.DecodeString(input)
			strict = append(strict, map[string]interface{}{"raw": raw, "input": input, "decoded": hex.EncodeToString(out), "error": err != nil})
		}
	}
	construct := []map[string]interface{}{}
	for _, input := range []string{std[:63], std + "!", std[:63] + "A", "\r" + std[1:], std[:63] + "\n", "\n" + strings.Repeat("A", 63)} {
		failure := ""
		func() {
			defer func() {
				if p := recover(); p != nil {
					failure = fmt.Sprint(p)
				}
			}()
			base64.NewEncoding(input)
		}()
		construct = append(construct, map[string]interface{}{"input": hex.EncodeToString([]byte(input)), "reference_failure": failure})
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]interface{}{"schemes": schemes, "valid": valid, "malformed": bad, "strict": strict, "construct": construct}); err != nil {
		panic(err)
	}
}
