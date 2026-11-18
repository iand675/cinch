// Generate independent configurable Base32 byte, prefix-error and configuration fixtures.
package main

import (
	"crypto/sha256"
	"encoding/base32"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"strings"
	"unicode/utf8"
)

// corpus builds repeatable full-byte input without using any encoding implementation.
func corpus(n int, fill bool) []byte {
	b := make([]byte, n)
	for i := range b {
		if fill {
			b[i] = 255
		} else {
			b[i] = byte(i*37 + 11)
		}
	}
	return b
}

// configuration constructs one reference alphabet with an exact byte padding policy.
func configuration(alphabet []byte, padding int) *base32.Encoding {
	return base32.NewEncoding(string(alphabet)).WithPadding(rune(padding))
}

// configError captures deterministic validation categories without recording panic text as an API.
func configError(alphabet []byte, padding int) (kind string) {
	defer func() {
		if value := recover(); value != nil {
			text := fmt.Sprint(value)
			switch {
			case strings.Contains(text, "32-bytes"):
				kind = "length"
			case strings.Contains(text, "newline"):
				kind = "newline"
			case strings.Contains(text, "duplicate"):
				kind = "duplicate"
			case strings.Contains(text, "contained"):
				kind = "collision"
			default:
				kind = "padding"
			}
		}
	}()
	configuration(alphabet, padding)
	return ""
}

// main records all short fringes, full quanta, binary alphabets and independent malformed prefixes.
func main() {
	standard := []byte("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567")
	hexAlphabet := []byte("0123456789ABCDEFGHIJKLMNOPQRSTUV")
	high := make([]byte, 32)
	control := []byte{}
	for i := range high {
		high[i] = byte(128 + i)
	}
	for i := 0; len(control) < 32; i++ {
		if i != 10 && i != 13 {
			control = append(control, byte(i))
		}
	}
	configs := []struct {
		alphabet []byte
		padding  int
	}{
		{standard, '='}, {standard, -1}, {hexAlphabet, '='}, {hexAlphabet, -1},
		{[]byte("abcdefghijklmnopqrstuvwxyz234567"), '~'},
		{[]byte("765432ZYXWVUTSRQPONMLKJIHGFEDCBA"), -1},
		{high, 255}, {control, -1}, {[]byte("=BCDEFGHIJKLMNOPQRSTUVWXYZ234567"), '~'},
	}
	valid := []map[string]interface{}{}
	invalid := []map[string]interface{}{}
	configRows := []map[string]interface{}{}
	for id, c := range configs {
		e := configuration(c.alphabet, c.padding)
		for _, n := range []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 16, 17, 31, 32, 33, 63, 64, 65, 255, 256, 257, 4095, 4096, 4097, 65536} {
			for _, fill := range []bool{false, true} {
				if n > 257 && fill {
					continue
				}
				raw := corpus(n, fill)
				encoded := []byte(e.EncodeToString(raw))
				hash := sha256.Sum256(encoded)
				full := ""
				if n <= 257 {
					full = hex.EncodeToString(encoded)
				}
				valid = append(valid, map[string]interface{}{"id": id, "alphabet": hex.EncodeToString(c.alphabet), "padding": c.padding, "size": n, "fill": fill, "encoded_size": len(encoded), "encoded_hex": full, "encoded_sha256": hex.EncodeToString(hash[:]), "decoded_bound": e.DecodedLen(len(encoded)), "text_valid": utf8.Valid(encoded)})
			}
		}
		encoded := []byte(e.EncodeToString(corpus(15, false)))
		bad := byte(0)
		for {
			if bad != '\r' && bad != '\n' && int(bad) != c.padding && !strings.ContainsRune(string(c.alphabet), rune(bad)) {
				present := false
				for _, b := range c.alphabet {
					present = present || b == bad
				}
				if !present {
					break
				}
			}
			bad++
		}
		for at := range encoded {
			input := append([]byte{}, encoded...)
			input[at] = bad
			out := make([]byte, e.DecodedLen(len(input)))
			n, err := e.Decode(out, input)
			position, ok := err.(base32.CorruptInputError)
			if !ok {
				panic(fmt.Sprintf("missing corruption at %d: %v", at, err))
			}
			invalid = append(invalid, map[string]interface{}{"id": id, "alphabet": hex.EncodeToString(c.alphabet), "padding": c.padding, "input": hex.EncodeToString(input), "output": hex.EncodeToString(out[:n]), "position": int64(position), "byte": bad})
		}
	}
	for _, n := range []int{0, 1, 31, 33, 64} {
		b := make([]byte, n)
		configRows = append(configRows, map[string]interface{}{"alphabet": hex.EncodeToString(b), "padding": 61, "kind": configError(b, 61)})
	}
	for _, at := range []int{0, 15, 31} {
		for _, value := range []byte{'\r', '\n'} {
			b := append([]byte{}, standard...)
			b[at] = value
			configRows = append(configRows, map[string]interface{}{"alphabet": hex.EncodeToString(b), "padding": 61, "kind": configError(b, 61)})
		}
	}
	for _, at := range []int{1, 15, 31} {
		b := append([]byte{}, standard...)
		b[at] = b[0]
		configRows = append(configRows, map[string]interface{}{"alphabet": hex.EncodeToString(b), "padding": 61, "kind": configError(b, 61)})
	}
	for _, padding := range []int{-1, 0, 10, 13, 61, 65, 90, 255, 256, 65535} {
		configRows = append(configRows, map[string]interface{}{"alphabet": hex.EncodeToString(standard), "padding": padding, "kind": configError(standard, padding)})
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]interface{}{"valid": valid, "invalid": invalid, "config": configRows}); err != nil {
		panic(err)
	}
}
