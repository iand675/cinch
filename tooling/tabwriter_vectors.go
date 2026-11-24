// Command tabwriter writes independent elastic-tabstop vectors from the reference SDK's
// implementation: each case records a configuration, the input split into writes,
// the expected output, and the output released before the final flush.
//
//	go run tooling/tabwriter_vectors.go > seed/tests/fixtures/tabwriter-vectors.json
package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"os"
	"text/tabwriter"
)

type vector struct {
	Minimum  int      `json:"minimum"`
	TabWidth int      `json:"tab_width"`
	Padding  int      `json:"padding"`
	Byte     int      `json:"byte"`
	Flags    uint     `json:"flags"`
	Writes   []string `json:"writes"`
	Early    string   `json:"early"`
	Output   string   `json:"output"`
}

// flags maps the reference flag set onto the Cinch flag constants (identical bit order).
func flags(f uint) uint {
	var out uint
	for bit, ref := range []uint{tabwriter.FilterHTML, tabwriter.StripEscape, tabwriter.AlignRight, tabwriter.DiscardEmptyColumns, tabwriter.TabIndent, tabwriter.Debug} {
		if f&(1<<bit) != 0 {
			out |= ref
		}
	}
	return out
}

func main() {
	inputs := [][]string{
		{"a\tb\tc\n"},
		{"aaa\tbb\tc\nx\tyyyy\tzz\n"},
		{"a\tb\nccc\tddd\n\nlonger cell\tx\n"},
		{"name\tvalue\tnote\nalpha\t1\t\nbeta\t22222\tlast\n"},
		{"1\t2\t3\t4\n11\t222\t3333\t44444\n", "x\ty\n"},
		{"no tabs at all\nsecond line\n"},
		{"a\tb\tc\n\fd\te\tf\n"},
		{"\tindented\tx\n\t\tdeeper\ty\n"},
		{"é\tü\n日本\tx\n"},
		{"\xffa\tb\xff\tc\n\xff\xff\tlonger\n"},
		{"<b>bold</b>\tx\n&amp;\ty\nplain\tz\n"},
		{"a\vb\vc\n\v\vz\n"},
		{"split", " across\twrites", "\tand\n", "more\tlines\n"},
		{"trailing without newline\tx"},
		{"a\t\tb\n\t\t\n"},
		{"col\tcol\tcol\n", "x\ty\n", "single\n", "p\tq\tr\n"},
	}
	configs := []vector{
		{0, 8, 1, ' ', 0, nil, "", ""}, {5, 0, 2, '.', 0, nil, "", ""}, {0, 4, 0, '\t', 0, nil, "", ""},
		{2, 8, 1, ' ', 4, nil, "", ""}, {0, 8, 1, '-', 32, nil, "", ""}, {0, 4, 1, ' ', 16, nil, "", ""},
		{0, 8, 1, ' ', 8, nil, "", ""}, {0, 8, 1, ' ', 1, nil, "", ""}, {0, 8, 1, ' ', 2, nil, "", ""},
		{3, 8, 2, '*', 1 | 4 | 32, nil, "", ""}, {0, 2, 1, '\t', 16 | 8, nil, "", ""},
	}
	var vectors []vector
	for _, c := range configs {
		for _, in := range inputs {
			var out bytes.Buffer
			w := tabwriter.NewWriter(&out, c.Minimum, c.TabWidth, c.Padding, byte(c.Byte), flags(c.Flags))
			v := c
			for _, chunk := range in {
				w.Write([]byte(chunk))
				v.Writes = append(v.Writes, hex.EncodeToString([]byte(chunk)))
			}
			v.Early = hex.EncodeToString(out.Bytes())
			w.Flush()
			v.Output = hex.EncodeToString(out.Bytes())
			vectors = append(vectors, v)
		}
	}
	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", " ")
	enc.Encode(vectors)
}
