// Command xml_vectors writes independent XML vectors from the reference SDK's implementation:
// documents decoded to namespace-translated tokens (and raw tokens) up to the reference error
// message and line; token sequences encoded with and without indentation, or the error the
// encoder reports; and text escaping.
//
//	go run tooling/xml_vectors.go > seed/tests/fixtures/xml-vectors.json
package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"encoding/xml"
	"os"
	"strings"
)

type tok struct {
	Kind   string     `json:"kind"`
	Space  string     `json:"space"`
	Local  string     `json:"local"`
	Attrs  [][]string `json:"attrs"`
	Data   string     `json:"data"`
	Target string     `json:"target"`
}

func describe(t xml.Token) tok {
	switch v := t.(type) {
	case xml.StartElement:
		a := [][]string{}
		for _, x := range v.Attr {
			a = append(a, []string{x.Name.Space, x.Name.Local, x.Value})
		}
		return tok{Kind: "start", Space: v.Name.Space, Local: v.Name.Local, Attrs: a}
	case xml.EndElement:
		return tok{Kind: "end", Space: v.Name.Space, Local: v.Name.Local, Attrs: [][]string{}}
	case xml.CharData:
		return tok{Kind: "chardata", Data: hex.EncodeToString(v), Attrs: [][]string{}}
	case xml.Comment:
		return tok{Kind: "comment", Data: hex.EncodeToString(v), Attrs: [][]string{}}
	case xml.ProcInst:
		return tok{Kind: "procinst", Target: v.Target, Data: hex.EncodeToString(v.Inst), Attrs: [][]string{}}
	case xml.Directive:
		return tok{Kind: "directive", Data: hex.EncodeToString(v), Attrs: [][]string{}}
	}
	panic("token")
}

func token(t tok) xml.Token {
	switch t.Kind {
	case "start":
		var a []xml.Attr
		for _, x := range t.Attrs {
			a = append(a, xml.Attr{Name: xml.Name{Space: x[0], Local: x[1]}, Value: x[2]})
		}
		return xml.StartElement{Name: xml.Name{Space: t.Space, Local: t.Local}, Attr: a}
	case "end":
		return xml.EndElement{Name: xml.Name{Space: t.Space, Local: t.Local}}
	}
	data, _ := hex.DecodeString(t.Data)
	switch t.Kind {
	case "chardata":
		return xml.CharData(data)
	case "comment":
		return xml.Comment(data)
	case "procinst":
		return xml.ProcInst{Target: t.Target, Inst: data}
	}
	return xml.Directive(data)
}

type decodeRow struct {
	Input  string `json:"input"`
	Raw    bool   `json:"raw"`
	Tokens []tok  `json:"tokens"`
	Error  string `json:"error"`
}

type encodeRow struct {
	Tokens []tok  `json:"tokens"`
	Prefix string `json:"prefix"`
	Indent string `json:"indent"`
	Output string `json:"output"`
	Error  string `json:"error"`
}

var documents = []string{
	`<?xml version="1.0" encoding="UTF-8"?><root a="1" b='two'><child>text</child><empty/></root>`,
	"<a>\r\nline\rnext\r\n</a>",
	`<a x="&lt;&gt;&amp;&apos;&quot;&#65;&#x42;&#x1F600;">&#9;&#xA;&#13;</a>`,
	`<a><![CDATA[ <not> &tags; ]] ]]]></a>`,
	`<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0//EN" "x.dtd"><html/>`,
	`<!DOCTYPE doc [<!ELEMENT doc (#PCDATA)> <!-- a comment --> <!ATTLIST doc a CDATA "x>y">]><doc/>`,
	`<!-- comment -- inside --><a/>`,
	`<!-- ok comment --><a/><!---->`,
	`<?target  some instruction ?><?x?><a/>`,
	`<root xmlns="urn:default" xmlns:p="urn:p"><p:child p:attr="1" plain="2" xml:lang="en"><inner/></p:child><q:other xmlns:q="urn:q"/></root>`,
	`<a xmlns:p="urn:one"><p:b xmlns:p="urn:two"><p:c/></p:b><p:d/></a>`,
	`<u:a>unbound prefix</u:a>`,
	`<a:b:c/>`,
	`<a x="1"y="2"/>`,
	`<a/><b/>top level text`,
	"<a>tab\tand\nnewline</a>",
	"<a>Ünïcödé ✓ 日本 😀</a>",
	"<élément attribut=\"valeur\">texte</élément>",
	"<a>\x00</a>",
	"<a>\xff</a>",
	"<a>\xef\xbf\xbe</a>",
	"<a>&#0;</a>",
	"<a>&#xD800;</a>",
	"<a>&#x110000;</a>",
	"<a>&bogus;</a>",
	"<a>&lt</a>",
	"<a>&;</a>",
	"<a>& b</a>",
	"<a>&#12</a>",
	"<a>&#;</a>",
	"<a>]]></a>",
	"<a><b></a>",
	"<a></b>",
	"</a>",
	"<a>",
	"<a",
	"<a x>",
	"<a x=1>",
	"<a x=\"<\">",
	"<a x=\"1\"",
	"<a/b>",
	"< a/>",
	"<1a/>",
	"<a></a >",
	"<a></a x>",
	"<!-x>",
	"<![CDAT[x]]>",
	"<![CDATA[unterminated",
	"<?xml version=\"1.1\"?><a/>",
	"<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?><a/>",
	"<?xml version='1.0' encoding='utf-8' standalone='yes'?><a/>",
	"<?xml?><a/>",
	"<?xm",
	"<!DOCTYPE x [ <!ENTITY y 'z'>",
	"line one\nline two\n<a>\n\n<b x=1/>",
	"<p:a xmlns:p=\"urn:p\"></q:a>",
	"<p:a xmlns:p=\"urn:p\"></a>",
	"<a xmlns=\"urn:x\"><b xmlns=\"\"><c/></b></a>",
	strings.Repeat("<n>", 40) + "deep" + strings.Repeat("</n>", 40),
	"<a>" + strings.Repeat("plain text without markup, long enough for several sixteen-byte spans. ", 5) + "&amp;end</a>",
	"<a b=\"" + strings.Repeat("attribute value long enough to span vectors ", 3) + "\"/>",
	"<a>]</a><b>]]</b><c>]x]></c>",
}

func decode(input string, raw bool) decodeRow {
	d := xml.NewDecoder(strings.NewReader(input))
	row := decodeRow{Input: hex.EncodeToString([]byte(input)), Raw: raw, Tokens: []tok{}}
	for {
		var t xml.Token
		var err error
		if raw {
			t, err = d.RawToken()
		} else {
			t, err = d.Token()
		}
		if err != nil {
			if err.Error() != "EOF" {
				row.Error = err.Error()
			}
			return row
		}
		row.Tokens = append(row.Tokens, describe(xml.CopyToken(t)))
	}
}

func s(local string) tok { return tok{Kind: "start", Local: local, Attrs: [][]string{}} }
func e(local string) tok { return tok{Kind: "end", Local: local, Attrs: [][]string{}} }
func c(text string) tok  { return tok{Kind: "chardata", Data: hex.EncodeToString([]byte(text)), Attrs: [][]string{}} }

func main() {
	var decodes []decodeRow
	for _, doc := range documents {
		decodes = append(decodes, decode(doc, false))
	}
	for _, doc := range []string{documents[9], documents[12], "<a></b>", "<a><b/>"} {
		decodes = append(decodes, decode(doc, true))
	}
	ns := tok{Kind: "start", Space: "urn:root", Local: "r", Attrs: [][]string{{"http://example.com/ns/", "a", "1"}, {"http://example.com/ns", "b", "2"}, {"urn:xmlish/xmlfoo", "c", "3"}, {"http://www.w3.org/XML/1998/namespace", "lang", "en"}, {"", "plain", "q\"<>&'\t\n\r"}}}
	sequences := [][]tok{
		{s("a"), c("text"), e("a")},
		{s("a"), s("b"), e("b"), s("c"), c("x"), e("c"), e("a")},
		{s("a"), c("<&>\"'\t\n\r"), e("a")},
		{ns, s("child"), e("child"), {Kind: "end", Space: "urn:root", Local: "r", Attrs: [][]string{}}},
		{{Kind: "procinst", Target: "xml", Data: hex.EncodeToString([]byte(`version="1.0"`)), Attrs: [][]string{}}, s("a"), e("a")},
		{s("a"), {Kind: "procinst", Target: "xml", Data: "", Attrs: [][]string{}}},
		{{Kind: "comment", Data: hex.EncodeToString([]byte(" fine ")), Attrs: [][]string{}}, {Kind: "directive", Data: hex.EncodeToString([]byte(`DOCTYPE a [<!ELEMENT a ANY>]`)), Attrs: [][]string{}}},
		{{Kind: "comment", Data: hex.EncodeToString([]byte("bad --> end")), Attrs: [][]string{}}},
		{{Kind: "directive", Data: hex.EncodeToString([]byte("DOCTYPE >")), Attrs: [][]string{}}},
		{{Kind: "procinst", Target: "p i", Data: "", Attrs: [][]string{}}},
		{{Kind: "procinst", Target: "pi", Data: hex.EncodeToString([]byte("a?>b")), Attrs: [][]string{}}},
		{s("a"), e("b")},
		{e("a")},
		{s("a"), s("b")},
		{s(""), e("")},
		{{Kind: "start", Local: "a", Attrs: [][]string{{"urn:x", "p", "1"}}}, {Kind: "start", Local: "b", Attrs: [][]string{{"urn:x", "q", "2"}, {"urn:y/x", "r", "3"}}}, e("b"), {Kind: "start", Local: "c", Attrs: [][]string{{"urn:z/x", "s", "4"}}}, e("c"), e("a")},
		{s("a"), c("\xff\x01ok"), e("a")},
	}
	var encodes []encodeRow
	for _, seq := range sequences {
		for _, ind := range [][2]string{{"", ""}, {"", "  "}, {">", "\t"}} {
			var buf bytes.Buffer
			enc := xml.NewEncoder(&buf)
			enc.Indent(ind[0], ind[1])
			row := encodeRow{Tokens: seq, Prefix: ind[0], Indent: ind[1]}
			var err error
			for _, t := range seq {
				if err = enc.EncodeToken(token(t)); err != nil {
					break
				}
			}
			if err == nil {
				err = enc.Close()
			}
			if err != nil {
				row.Error = err.Error()
			} else {
				row.Output = hex.EncodeToString(buf.Bytes())
			}
			encodes = append(encodes, row)
		}
	}
	var escapes [][]string
	for _, text := range []string{"plain", "<&>\"'\t\n\r", "ünï", "\x00\x1f\xff", strings.Repeat("ab<cd>ef", 10), "\xef\xbf\xbe"} {
		var buf bytes.Buffer
		xml.EscapeText(&buf, []byte(text))
		escapes = append(escapes, []string{hex.EncodeToString([]byte(text)), hex.EncodeToString(buf.Bytes())})
	}
	json.NewEncoder(os.Stdout).Encode(map[string]any{"decode": decodes, "encode": encodes, "escape": escapes})
}
