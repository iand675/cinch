// Command asn1_vectors writes independent DER vectors from the reference SDK's implementation.
// Each valid row pairs the reference encoding of a value with the equivalent Cinch Asn1
// expression; invalid rows are DER violations the reference decoder rejects.
//
//	go run tooling/asn1_vectors.go > seed/tests/fixtures/asn1-vectors.json
package main

import (
	"encoding/asn1"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"math/big"
	"os"
	"strings"
	"time"
)

type row struct {
	Der  string `json:"der"`
	Expr string `json:"expr"`
}

func der(v any, params string) string {
	b, err := asn1.MarshalWithParams(v, params)
	if err != nil {
		panic(err)
	}
	return hex.EncodeToString(b)
}

func hx(s string) string { return fmt.Sprintf("(input %q)", hex.EncodeToString([]byte(s))) }

func intExpr(s string) string {
	if strings.HasPrefix(s, "-") {
		return "(0 - " + s[1:] + ")"
	}
	return s
}

type pair struct {
	A int
	B string `asn1:"utf8"`
}

type tagged struct {
	X int `asn1:"explicit,tag:0"`
	Y int `asn1:"tag:1"`
	Z int `asn1:"optional,explicit,tag:5"`
}

func main() {
	var rows []row
	for _, n := range []int64{0, 1, 127, 128, 255, 256, -1, -128, -129, -256, 32767, -32768, 1<<63 - 1, -1 << 63} {
		rows = append(rows, row{der(n, ""), "Asn1Integer " + intExpr(fmt.Sprint(n))})
	}
	for _, s := range []string{"1267650600228229401496703205376", "-1267650600228229401496703205376", "-1267650600228229401496703205377"} {
		v, _ := new(big.Int).SetString(s, 10)
		rows = append(rows, row{der(v, ""), "Asn1Integer " + intExpr(s)})
	}
	rows = append(rows, row{der(true, ""), "Asn1Boolean True"}, row{der(false, ""), "Asn1Boolean False"})
	for _, oid := range []asn1.ObjectIdentifier{{1, 2, 840, 113549}, {2, 5, 4, 3}, {1, 3, 6, 1, 4, 1, 311, 21, 20}, {2, 999, 3}, {0, 39}, {1, 0}} {
		arcs := make([]string, len(oid))
		for i, a := range oid {
			arcs[i] = fmt.Sprint(a)
		}
		rows = append(rows, row{der(oid, ""), "Asn1Oid [" + strings.Join(arcs, ", ") + "]"})
	}
	rows = append(rows, row{der("héllo", "utf8"), "Asn1String tag-utf8-string " + hx("héllo")},
		row{der("Hello World", "printable"), "Asn1String tag-printable-string " + hx("Hello World")},
		row{der("a@b.c", "ia5"), "Asn1String tag-ia5-string " + hx("a@b.c")},
		row{der("12 34", "numeric"), "Asn1String tag-numeric-string " + hx("12 34")})
	rows = append(rows, row{der([]byte{1, 2, 3}, ""), "Asn1OctetString " + hx("\x01\x02\x03")}, row{der([]byte{}, ""), "Asn1OctetString (bytes-new 0)"})
	rows = append(rows, row{der(asn1.RawValue{FullBytes: asn1.NullBytes}, ""), "Asn1Null"})
	for _, bs := range []asn1.BitString{{Bytes: []byte{0x80}, BitLength: 1}, {Bytes: []byte{0xa0, 0x00}, BitLength: 9}, {Bytes: []byte{0xff, 0xff}, BitLength: 16}, {Bytes: []byte{}, BitLength: 0}} {
		rows = append(rows, row{der(bs, ""), fmt.Sprintf("Asn1BitString %s %d", hx(string(bs.Bytes)), bs.BitLength)})
	}
	t1 := time.Date(2024, 3, 15, 13, 45, 31, 0, time.UTC)
	t2 := time.Date(2051, 1, 2, 3, 4, 5, 0, time.UTC)
	rows = append(rows, row{der(t1, "utc"), "Asn1UtcTime " + hx(t1.Format("060102150405Z"))}, row{der(t2, ""), "Asn1GeneralizedTime " + hx(t2.Format("20060102150405Z"))})
	rows = append(rows, row{der(asn1.Enumerated(3), ""), "Asn1Enumerated 3"})
	rows = append(rows, row{der(pair{7, "x"}, ""), "asn1-sequence [Asn1Integer 7, Asn1String tag-utf8-string " + hx("x") + "]"})
	rows = append(rows, row{der([]int{300, 2, 1}, "set"), "asn1-set [Asn1Integer 300, Asn1Integer 2, Asn1Integer 1]"})
	rows = append(rows, row{der([]pair{{1, "a"}, {2, "bb"}}, ""), "asn1-sequence [asn1-sequence [Asn1Integer 1, Asn1String tag-utf8-string " + hx("a") + "], asn1-sequence [Asn1Integer 2, Asn1String tag-utf8-string " + hx("bb") + "]]"})
	rows = append(rows, row{der(tagged{X: 5, Y: 6, Z: 0}, ""), "asn1-sequence [asn1-explicit 0 (Asn1Integer 5), asn1-implicit 1 (Asn1Integer 6)]"})
	rows = append(rows, row{der(tagged{X: -1, Y: 300, Z: 9}, ""), "asn1-sequence [asn1-explicit 0 (Asn1Integer (0 - 1)), asn1-implicit 1 (Asn1Integer 300), asn1-explicit 5 (Asn1Integer 9)]"})
	rows = append(rows, row{der(asn1.RawValue{Class: asn1.ClassApplication, Tag: 100, IsCompound: false, Bytes: []byte{9}}, ""), "Asn1Raw class-application 100 False " + hx("\x09")})
	long := strings.Repeat("y", 300)
	rows = append(rows, row{der([]byte(long), ""), "Asn1OctetString " + hx(long)})
	// DER violations: non-minimal or indefinite lengths, non-minimal or empty integers, non-canonical
	// booleans, non-empty NULL, truncation, trailing data, bad bit-string padding, non-minimal OID arcs
	// and tag numbers, and empty OIDs.
	invalid := []string{"0281000105", "02020001", "0202ff80", "010101", "01020000", "0500ff", "050100", "0201", "02010100", "30030201",
		"30800201010000", "0303070001", "0302080a", "030100ff", "06028001", "1f1e00", "1f800100", "0200", "048200050000000000", "04",
		"0481050000000000", "0600"}
	times := [][]any{{der(t1, "utc"), t1.Unix()}, {der(t2, ""), t2.Unix()}}
	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", " ")
	enc.Encode(map[string]any{"valid": rows, "invalid": invalid, "times": times})
}
