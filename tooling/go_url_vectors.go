// Generate independent byte-preserving URL component, query and parsing fixtures.
package main

import (
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"hash/fnv"
	"net/url"
	"os"
	"strconv"
	"strings"
)

// fingerprint observes every byte while keeping large fixture rows compact.
func fingerprint(input []byte) uint64 {
	h := fnv.New64a()
	h.Write(input)
	return h.Sum64()
}

// encode invokes public component operations, adding a harmless prefix where a field has special cases.
func encode(mode int, input []byte) []byte {
	s := string(input)
	switch mode {
	case 1:
		u := url.URL{Path: "x" + s}
		return []byte(u.EscapedPath()[1:])
	case 2:
		return []byte(url.PathEscape(s))
	case 3, 4:
		u := url.URL{Scheme: "s", Host: "x" + s}
		return []byte(strings.TrimPrefix(u.String(), "s://x"))
	case 5:
		return []byte(url.User(s).String())
	case 6:
		return []byte(url.QueryEscape(s))
	case 7:
		u := url.URL{Fragment: "x" + s}
		return []byte(u.EscapedFragment()[1:])
	}
	panic("unknown URL encoding context")
}

// failure identifies public typed URL causes without depending on diagnostic wording.
func failure(err error) string {
	if err == nil {
		return ""
	}
	var escape url.EscapeError
	if errors.As(err, &escape) {
		return "escape"
	}
	var host url.InvalidHostError
	if errors.As(err, &host) {
		return "host"
	}
	return "parse"
}

// decode uses public unescaping or a valid decorated authority to exercise host/zone percent rules.
func decode(mode int, input []byte) ([]byte, error) {
	s := string(input)
	if mode == 6 {
		out, err := url.QueryUnescape(s)
		return []byte(out), err
	}
	if mode == 3 {
		u, err := url.Parse("s://x" + s)
		if err != nil {
			return nil, err
		}
		return []byte(u.Host[1:]), nil
	}
	if mode == 4 {
		u, err := url.Parse("s://[::1%25z" + s + "]")
		if err != nil {
			return nil, err
		}
		return []byte(strings.TrimSuffix(strings.TrimPrefix(u.Host, "[::1%z"), "]")), nil
	}
	out, err := url.PathUnescape(s)
	return []byte(out), err
}

// failureOffset finds the first rejected percent triplet using the same public component oracle.
func failureOffset(mode int, input []byte) int {
	for at := 0; at < len(input); at++ {
		if input[at] != '%' {
			continue
		}
		end := at + 3
		if end > len(input) {
			end = len(input)
		}
		if _, err := decode(mode, input[at:end]); err != nil {
			return at
		}
		at += 2
	}
	return 0
}

// corpus creates ordinary, space-heavy, arbitrary-byte, reserved and UTF-8 input independently.
func corpus(size, mode int) []byte {
	result := make([]byte, size)
	pattern := []byte("AZaz09-_.~")
	if mode == 1 {
		pattern = []byte(" ")
	} else if mode == 3 {
		pattern = []byte("a/b?c&d=e+f #\"[]%:;")
	} else if mode == 4 {
		pattern = []byte("é中😀/")
	}
	for i := range result {
		if mode == 2 {
			result[i] = byte(i*37 + 11)
		} else {
			result[i] = pattern[i%len(pattern)]
		}
	}
	return result
}

// main covers all component byte policies and all percent values without requiring UTF-8.
func main() {
	encoded := []map[string]interface{}{}
	decoded := []map[string]interface{}{}
	long := []map[string]interface{}{}
	for mode := 1; mode <= 7; mode++ {
		for value := 0; value < 256; value++ {
			input := []byte{byte(value)}
			encoded = append(encoded, map[string]interface{}{"mode": mode, "input": hex.EncodeToString(input), "output": hex.EncodeToString(encode(mode, input))})
		}
		for _, size := range []int{0, 1, 2, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 255, 256, 257, 4095, 4096, 4097, 65536} {
			for pattern := 0; pattern < 5; pattern++ {
				input := corpus(size, pattern)
				output := encode(mode, input)
				long = append(long, map[string]interface{}{"mode": mode, "size": size, "pattern": pattern, "input_hash": fingerprint(input), "output_size": len(output), "output_hash": fingerprint(output)})
			}
		}
		candidates := [][]byte{}
		for value := 0; value < 256; value++ {
			candidates = append(candidates, []byte(fmt.Sprintf("%%%02X", value)), []byte(fmt.Sprintf("%%%02x", value)))
			if mode != 3 && mode != 4 {
				candidates = append(candidates, []byte{byte(value)})
			}
		}
		for _, value := range []string{"", "%", "%0", "%GG", "%0g", "%g0", "%%", "%2520", "%25%25", "%FF%00", "%C3%A9", "%F0%9F%98%80"} {
			candidates = append(candidates, []byte(value))
		}
		if mode != 3 && mode != 4 {
			for _, value := range []string{"a+b%2Bc", "a%2Fb/c", "%00+%ff", "a%20b%", "é%zz", "é中😀", "%1\xff", "line\r\n%0A"} {
				candidates = append(candidates, []byte(value))
			}
		}
		for _, input := range candidates {
			output, err := decode(mode, input)
			kind := failure(err)
			if kind == "parse" {
				panic("unexpected decorated authority failure: " + strconv.Quote(string(input)) + ": " + err.Error())
			}
			offset := 0
			if err != nil {
				offset = failureOffset(mode, input)
			}
			decoded = append(decoded, map[string]interface{}{"mode": mode, "input": hex.EncodeToString(input), "output": hex.EncodeToString(output), "error": kind, "offset": offset})
		}
	}
	e := json.NewEncoder(os.Stdout)
	e.SetIndent("", "  ")
	if err := e.Encode(map[string]interface{}{"encoded": encoded, "decoded": decoded, "long": long}); err != nil {
		panic(err)
	}
}
