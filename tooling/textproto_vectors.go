// Command textproto_vectors writes independent text-protocol vectors from the reference SDK's
// implementation: canonical header keys, MIME header blocks, status lines, multi-line responses
// and dot encoding in both directions, including malformed inputs.
//
//	go run tooling/textproto_vectors.go > seed/tests/fixtures/textproto-vectors.json
package main

import (
	"bufio"
	"bytes"
	"encoding/hex"
	"encoding/json"
	"io"
	"net/textproto"
	"os"
	"sort"
	"strings"
)

func h(s string) string { return hex.EncodeToString([]byte(s)) }

func reader(s string) *textproto.Reader { return textproto.NewReader(bufio.NewReader(strings.NewReader(s))) }

type header struct {
	Input  string     `json:"input"`
	Ok     bool       `json:"ok"`
	Fields [][]string `json:"fields"`
	Rest   string     `json:"rest"`
}

type code struct {
	Input    string `json:"input"`
	Expected int    `json:"expected"`
	Multi    bool   `json:"multi"`
	Ok       bool   `json:"ok"`
	Code     int    `json:"code"`
	Message  string `json:"message"`
}

type dot struct {
	Input  string `json:"input"`
	Ok     bool   `json:"ok"`
	Output string `json:"output"`
}

func main() {
	keys := []string{"content-type", "CONTENT-TYPE", "x-forwarded-for", "a", "", "-", "a-", "-a", "foo bar", "user-agent", "Accept-Encoding", "x_y-z", "Ünicode-key", "with:colon", "x-1-2", "WWW-AUTHENTICATE", "te", "abcdefghijklmnopqrstuvwxyz-ABCDEFGH", "a\tb", "e-tag!#$%&'*+.^_`|~"}
	var canon [][]string
	for _, k := range keys {
		canon = append(canon, []string{h(k), h(textproto.CanonicalMIMEHeaderKey(k))})
	}
	blocks := []string{
		"Content-Type: text/plain\r\nContent-Length: 12\r\n\r\nbody",
		"a: 1\nb: 2\na: 3\n\nrest",
		"Subject: hello\r\n  world\r\n\tagain\r\nX: y\r\n\r\n",
		"Empty:\r\nSpaced :   v   \r\n\r\n",
		" leading: space\r\n\r\n",
		"no colon here\r\n\r\n",
		"bad key: v\r\n\r\n",
		"Key: v\r\n",
		"Key: bad\x01value\r\n\r\n",
		"\r\n",
		"lower-case-key: Mixed Value\r\nLOWER-CASE-KEY: second\r\n\r\n",
	}
	var headers []header
	for _, b := range blocks {
		r := reader(b)
		m, err := r.ReadMIMEHeader()
		hd := header{Input: h(b), Ok: err == nil}
		if err == nil {
			var ks []string
			for k := range m {
				ks = append(ks, k)
			}
			sort.Strings(ks)
			for _, k := range ks {
				row := []string{h(k)}
				for _, v := range m[k] {
					row = append(row, h(v))
				}
				hd.Fields = append(hd.Fields, row)
			}
			rest, _ := io.ReadAll(r.R)
			hd.Rest = h(string(rest))
		}
		headers = append(headers, hd)
	}
	lines := []struct {
		in    string
		exp   int
		multi bool
	}{
		{"220 ready\r\n", 220, false}, {"250 ok\r\n", 2, false}, {"250 ok\r\n", 25, false}, {"550 no\r\n", 2, false}, {"250-first\r\n", 0, false},
		{"25 x\r\n", 0, false}, {"250\r\n", 0, false}, {"099 low\r\n", 0, false}, {"abc def\r\n", 0, false},
		{"250-first\r\n250-second\r\n250 last\r\n", 250, true}, {"250-a\r\nplain text\r\n251 other\r\n250 end\r\n", 0, true},
		{"220 single\r\n", 2, true}, {"550-x\r\n550 y\r\n", 2, true},
	}
	var codes []code
	for _, l := range lines {
		r := reader(l.in)
		var c int
		var msg string
		var err error
		if l.multi {
			c, msg, err = r.ReadResponse(l.exp)
		} else {
			c, msg, err = r.ReadCodeLine(l.exp)
		}
		codes = append(codes, code{h(l.in), l.exp, l.multi, err == nil, c, h(msg)})
	}
	bodies := []string{"line one\r\n..dotted\r\nlast\r\n.\r\nafter", "a\n.b\n.\n", "x\ry\r\n.\r\n", "no end", ".\r\n", "..\r\n.\r\n", "abc\r\n.x\r\n"}
	var reads []dot
	for _, b := range bodies {
		out, err := reader(b).ReadDotBytes()
		reads = append(reads, dot{h(b), err == nil, h(string(out))})
	}
	plain := []string{"", "hello", "hello\n", ".leading\nmiddle\n.\n", "crlf\r\nkept\r\n", "lone\rcr", "ends with cr\r", "\n\n", "..double"}
	var writes []dot
	for _, b := range plain {
		var buf bytes.Buffer
		w := textproto.NewWriter(bufio.NewWriter(&buf))
		dw := w.DotWriter()
		dw.Write([]byte(b))
		dw.Close()
		writes = append(writes, dot{h(b), true, h(buf.String())})
	}
	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", " ")
	enc.Encode(map[string]any{"canonical": canon, "headers": headers, "codes": codes, "dot_reads": reads, "dot_writes": writes})
}
