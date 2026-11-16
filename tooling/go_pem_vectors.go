// Generate independent byte-preserving PEM parser and encoder observations from the pinned SDK.
package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"encoding/pem"
	"errors"
	"fmt"
	"os"
	"runtime"
	"sort"
	"strings"
)

// header records arbitrary string bytes without JSON's invalid-UTF8 repair.
type header struct {
	Key   string `json:"key"`
	Value string `json:"value"`
}

// block records exact owned metadata and payload bytes before or after encoding.
type block struct {
	Label   string   `json:"label"`
	Headers []header `json:"headers"`
	Data    string   `json:"data"`
}

// decodeRow retains both a found block and every byte of the actual returned remainder.
type decodeRow struct {
	Input string `json:"input"`
	Found bool   `json:"found"`
	Block block  `json:"block"`
	Rest  string `json:"rest"`
}

// encodeRow retains full encoding or the actual nil-memory refusal for malformed header keys.
type encodeRow struct {
	Block  block  `json:"block"`
	Output string `json:"output"`
	Valid  bool   `json:"valid"`
}

// writerRow observes actual stream encoding with a byte budget and exact returned prefix.
type writerRow struct {
	Budget int    `json:"budget"`
	Output string `json:"output"`
	Failed bool   `json:"failed"`
}

// budgetWriter satisfies the writer contract by returning a failure with every short acceptance.
type budgetWriter struct {
	Output bytes.Buffer
	Budget int
}

// Write accepts only the remaining budget, preserving a directly observable partial prefix.
func (writer *budgetWriter) Write(input []byte) (int, error) {
	if len(input) > writer.Budget {
		count := writer.Budget
		writer.Output.Write(input[:count])
		writer.Budget = 0
		return count, errors.New("pem-budget")
	}
	writer.Output.Write(input)
	writer.Budget -= len(input)
	return len(input), nil
}

// encoded preserves bytes in JSON fixture fields, including empty and malformed UTF-8 strings.
func encoded(data []byte) string { return hex.EncodeToString(data) }

// observed exposes fields in deterministic byte order, independently of map iteration order.
func observed(value *pem.Block) block {
	result := block{Headers: []header{}}
	if value == nil {
		return result
	}
	result.Label = encoded([]byte(value.Type))
	result.Data = encoded(value.Bytes)
	keys := []string{}
	for key := range value.Headers {
		keys = append(keys, key)
	}
	sort.Strings(keys)
	for _, key := range keys {
		result.Headers = append(result.Headers, header{encoded([]byte(key)), encoded([]byte(value.Headers[key]))})
	}
	return result
}

// materialized reconstructs map fields using last-key-wins semantics rather than a reference encoder copy.
func materialized(label string, headers [][2]string, data []byte) *pem.Block {
	result := &pem.Block{Type: label, Headers: map[string]string{}, Bytes: data}
	for _, pair := range headers {
		result.Headers[pair[0]] = pair[1]
	}
	return result
}

// main fixes the toolchain epoch and retains successful, malformed, nested and Unicode cases.
func main() {
	if runtime.Version() != "go1.27.4" {
		panic("unexpected reference SDK")
	}
	encodes := []encodeRow{}
	decodes := []decodeRow{}
	seen := map[string]bool{}
	add := func(data []byte) {
		key := string(data)
		if seen[key] {
			return
		}
		seen[key] = true
		value, rest := pem.Decode(data)
		decodes = append(decodes, decodeRow{encoded(data), value != nil, observed(value), encoded(rest)})
	}
	headers := [][][2]string{
		{}, {{"Proc-Type", "4,ENCRYPTED"}, {"DEK-Info", "AES-256-CBC,ABC"}, {"Z", "last"}, {"A", "first"}},
		{{"", "empty-key"}, {"a", "one"}, {"a", "two"}, {"proc-type", "ordinary"}, {"Proc-Type", "priority"}},
		{{"\u00a0K\u2003", "\u0085 V \u3000"}, {"\xff\x80", "\x00\xff"}},
		{{"line\nkey", "line\r\nvalue"}}, {{"bad:key", "refused"}},
	}
	labels := []string{"TEST", "", "RSA PRIVATE KEY", "X:Y", "a\nb", "\xff\x00"}
	sizes := []int{0, 1, 2, 3, 47, 48, 49, 63, 64, 65, 95, 96, 97, 256, 4096}
	for li, label := range labels {
		for hi, fields := range headers {
			for _, size := range sizes {
				if size > 97 && (li != 0 || hi > 1) {
					continue
				}
				data := make([]byte, size)
				for index := range data {
					data[index] = byte(index*73 + 19)
				}
				value := materialized(label, fields, data)
				output := pem.EncodeToMemory(value)
				encodes = append(encodes, encodeRow{observed(value), encoded(output), output != nil})
				if output != nil {
					add(output)
					if size <= 97 {
						add(append([]byte("preamble\r\n"), append(output, []byte("tail\x00\xff")...)...))
					}
				}
			}
		}
	}
	good := []byte("-----BEGIN TEST-----\nYWJj\n-----END TEST-----\n")
	add(nil)
	add([]byte("plain text\x00\xff"))
	for index := 0; index <= len(good); index++ {
		add(good[:index])
	}
	for index := range good {
		for _, replacement := range []byte{0, 9, 10, 13, 32, 45, 58, 61, 128, 255} {
			copy := append([]byte{}, good...)
			copy[index] = replacement
			add(copy)
		}
	}
	for _, prefix := range []string{"", "preamble", "preamble\n", " ", "\t", "\r", "\r\n", "x\n", "-----BEGIN OLD-----\n", "-----BEGIN OLD-----\n?\n-----END WRONG-----\n", "\n-----END -----BEGIN TEST-----\nYWJj"} {
		add(append([]byte(prefix), good...))
	}
	for _, middle := range []string{"", "\n", "YWJj", "YWJj\n", "Y W\tJj\n", "Y\rW\nJj\n", "YR==\n", "YWI=\n", "YWL=\n", "YQ===\n", "YQ==junk\n", "?\n", "-w==\n", "_w==\n", "\v\n", "\f\n", "K: V\n", "K: V\n\n", "K: first\nK: last\n\n", " \u00a0K\u2003 : \u0085V\u3000 \n\n", "\xff K \x80: \x80 V \xff\n\n", "-----BEGIN TEST-----\nYWJj\n", "x-----BEGIN TEST-----\nYWJj\n"} {
		for _, ending := range []string{"-----END TEST-----\n", "-----END TEST-----", "-----END TEST----- \t\r\n", "-----END TEST-----\r", "-----END TEST-----junk\n", "-----END OTHER-----\n"} {
			input := []byte("-----BEGIN TEST-----\n" + middle + ending)
			add(input)
			add(append(append([]byte{}, input...), good...))
		}
	}
	for _, input := range []string{
		"\n-----END -----BEGIN TEST-----\nYWJj\n-----END TEST-----\n",
		"-----BEGIN X:Y-----\n-----END X:Y-----\n",
		"-----BEGIN X:Y-----\n\n-----END X:Y-----\n",
		"-----BEGIN TEST-----\nK: V\n-----END TEST-----\n",
		"-----BEGIN TEST----- \t\r\nYWJj\n-----END TEST-----\t\n",
		"-----BEGIN TEST-----\nYWJj\nxx-----BEGIN TEST-----\nYWJj\n-----END TEST-----\n",
	} {
		add([]byte(input))
		add(append([]byte(input), good...))
	}
	for width := 0; width < 40; width++ {
		prefix := bytes.Repeat([]byte("p"), width)
		prefix = append(prefix, '\n')
		add(append(prefix, good...))
	}
	for index := 0; index < 128; index++ {
		seed := uint32(index + 1)
		input := make([]byte, index%33)
		for at := range input {
			seed = seed*1664525 + 1013904223
			input[at] = byte(seed >> 24)
		}
		add(append(append(input, '\n'), good...))
	}
	for _, ending := range []string{"\n", "\r\n", "\r", ""} {
		input := strings.ReplaceAll(string(good), "\n", ending)
		add([]byte(input))
	}
	// Stress map canonicalization independently of the implementation's sorting strategy.
	large := &pem.Block{Type: "MANY", Headers: map[string]string{}, Bytes: []byte{}}
	for index := 0; index < 1000; index++ {
		name := fmt.Sprintf("k%d", index)
		large.Headers[name] = name
	}
	largeOutput := pem.EncodeToMemory(large)
	encodes = append(encodes, encodeRow{observed(large), encoded(largeOutput), largeOutput != nil})
	add(largeOutput)
	var duplicates strings.Builder
	duplicates.WriteString("-----BEGIN TEST-----\n")
	for index := 0; index < 1000; index++ {
		fmt.Fprintf(&duplicates, " K \t: v%d\n", index)
	}
	duplicates.WriteString("\n-----END TEST-----\n")
	add([]byte(duplicates.String()))
	sample := materialized("TEST", nil, []byte("abcdef"))
	sampleOutput := pem.EncodeToMemory(sample)
	writers := []writerRow{}
	for budget := 0; budget <= len(sampleOutput)+1; budget++ {
		writer := &budgetWriter{Budget: budget}
		err := pem.Encode(writer, sample)
		writers = append(writers, writerRow{budget, encoded(writer.Output.Bytes()), err != nil})
	}
	result := map[string]any{"sdk_version": runtime.Version(), "encodes": encodes, "decodes": decodes, "writers": writers, "scope": fmt.Sprintf("%d encodes, %d decode/remainder observations", len(encodes), len(decodes))}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(result); err != nil {
		panic(err)
	}
}
