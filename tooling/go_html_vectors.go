// Generate independent HTML text/byte conversion fixtures and public-pipeline fingerprints.
package main

import (
	"encoding/hex"
	"encoding/json"
	"flag"
	"fmt"
	"hash/fnv"
	"html"
	"os"
	"sort"
	"strconv"
	"strings"
)

// fingerprint retains all output bytes without embedding large repetitive buffers.
func fingerprint(value string) uint64 {
	h := fnv.New64a()
	h.Write([]byte(value))
	return h.Sum64()
}

// corpus builds ASCII, escape-heavy, reference-heavy and arbitrary-byte inputs independently.
func corpus(size, mode int) []byte {
	result := make([]byte, size)
	pattern := []byte("<>&\"'")
	if mode == 2 {
		pattern = []byte("&amp;&#x1F600;&NotEqualTilde;not&a")
	}
	for i := range result {
		switch mode {
		case 0:
			result[i] = 'a'
		case 1, 2:
			result[i] = pattern[i%len(pattern)]
		case 3:
			result[i] = byte(i*37 + 11)
		case 4:
			if i == 0 {
				result[i] = '&'
			} else if i == 1 {
				result[i] = '#'
			} else {
				result[i] = '9'
			}
		case 5:
			if i == 0 {
				result[i] = '&'
			} else if i == 1 {
				result[i] = '#'
			} else if i == 2 {
				result[i] = 'x'
			} else {
				result[i] = 'F'
			}
		}
	}
	return result
}

// main includes every named reference, malformed numeric prefixes, exact boundaries and raw bytes.
func main() {
	input := flag.String("entities", "tooling/data/html-entities.json", "independent HTML5 entity data")
	flag.Parse()
	data, err := os.ReadFile(*input)
	if err != nil {
		panic(err)
	}
	var source struct {
		Entities map[string]string `json:"entities"`
	}
	if err := json.Unmarshal(data, &source); err != nil {
		panic(err)
	}
	rows := []map[string]interface{}{}
	add := func(kind, input, expected string) {
		reference := html.UnescapeString(input)
		rows = append(rows, map[string]interface{}{"kind": kind, "input": hex.EncodeToString([]byte(input)), "escaped": hex.EncodeToString([]byte(html.EscapeString(input))), "unescaped": hex.EncodeToString([]byte(expected)), "reference_unescaped": hex.EncodeToString([]byte(reference)), "full_html5_expansion": reference != expected})
	}
	names := make([]string, 0, len(source.Entities))
	for name := range source.Entities {
		names = append(names, name)
	}
	sort.Strings(names)
	for _, name := range names {
		value := source.Entities[name]
		if html.UnescapeString("&"+name) != value && name != "nLt;" && name != "nGt;" {
			panic("unexpected named-reference difference: " + name)
		}
		add("named", "&"+name, value)
		if !strings.HasSuffix(name, ";") {
			for _, suffix := range []string{"Z", "=", ";", "?", "&amp;"} {
				input := "&" + name + suffix
				add("legacy-context", input, html.UnescapeString(input))
			}
		}
	}
	for _, input := range []string{"", "plain text", "<>&\"'", "é中😀", "a\x00b", "&", "&&", "&;", "&#", "&#x", "&#X", "&#;", "&#x;", "&#X;", "&#1", "&#12", "&#1x", "&#12x", "&#x1", "&#x1x", "&#xg;", "&#z;", "&notit;", "&notin", "&NotEqualTilde", "&unknown;", "&amp;lt;", "&amp;amp;", "&AMPx", "&COPY=", "&amp\n;", "&aacute&amp;", "&#00000000000000000000000000000000000000000000001;", "&#99999999999999999999999999999999999999;", "&#xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;"} {
		add("boundary", input, html.UnescapeString(input))
	}
	for _, value := range []uint64{0, 1, 8, 9, 10, 13, 31, 32, 127, 128, 129, 130, 141, 143, 144, 157, 159, 160, 55295, 55296, 56319, 56320, 57343, 57344, 65533, 65535, 65536, 1114111, 1114112, 2147483647, 2147483648, 4294967295, 4294967296, 4294967297} {
		for _, input := range []string{"&#" + strconv.FormatUint(value, 10) + ";", "&#x" + strconv.FormatUint(value, 16) + ";", "&#X" + strings.ToUpper(strconv.FormatUint(value, 16)), "&#" + strconv.FormatUint(value, 10) + "z"} {
			add("numeric", input, html.UnescapeString(input))
		}
	}
	for value := 0; value < 256; value++ {
		input := string([]byte{byte(value)})
		add("raw-byte", input, html.UnescapeString(input))
		input = string([]byte{byte(value)}) + "&amp;&copy;" + string([]byte{byte(value)})
		add("raw-context", input, html.UnescapeString(input))
	}
	for input, expected := range map[string]string{"a&nLt;": "a≪⃒", "&nLt;a": "≪⃒a", "&nLt;&nGt;": "≪⃒≫⃒", "a&nGt;z": "a≫⃒z"} {
		add("expanding-boundary", input, expected)
	}
	// Keep stable fixture order despite the explicit expansion map above.
	sort.SliceStable(rows, func(i, j int) bool {
		left, right := rows[i]["kind"].(string), rows[j]["kind"].(string)
		if left != right {
			return left < right
		}
		return rows[i]["input"].(string) < rows[j]["input"].(string)
	})
	long := []map[string]interface{}{}
	for _, size := range []int{0, 1, 2, 3, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 255, 256, 257, 1023, 1024, 1025, 4095, 4096, 4097, 65536} {
		for mode := 0; mode < 4; mode++ {
			input := string(corpus(size, mode))
			escaped, unescaped := html.EscapeString(input), html.UnescapeString(input)
			long = append(long, map[string]interface{}{"size": size, "mode": mode, "input_hash": fingerprint(input), "escaped_size": len(escaped), "escaped_hash": fingerprint(escaped), "unescaped_size": len(unescaped), "unescaped_hash": fingerprint(unescaped)})
		}
	}
	for _, mode := range []int{4, 5} {
		input := string(corpus(65536, mode))
		escaped, unescaped := html.EscapeString(input), html.UnescapeString(input)
		long = append(long, map[string]interface{}{"size": 65536, "mode": mode, "input_hash": fingerprint(input), "escaped_size": len(escaped), "escaped_hash": fingerprint(escaped), "unescaped_size": len(unescaped), "unescaped_hash": fingerprint(unescaped)})
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]interface{}{"rows": rows, "long": long, "intentional_differences": "Full HTML5 includes nLt; and nGt; and permits their five-byte references to expand to six UTF-8 bytes."}); err != nil {
		panic(fmt.Sprint(err))
	}
}
