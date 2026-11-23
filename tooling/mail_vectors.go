// Command mail_vectors writes independent mail vectors from the reference SDK's implementation:
// address lists and single addresses (with groups, comments, quoting, domain literals, UTF-8 and
// encoded words), address formatting, message header blocks and dates with numeric zones.
// Errors are recorded as stable kind names; raw address inputs are also recorded as hex, since
// JSON text cannot carry invalid UTF-8.
//
//	go run tooling/mail_vectors.go > seed/tests/fixtures/mail-vectors.json
package main

import (
	"encoding/hex"
	"encoding/json"
	"io"
	"net/mail"
	"os"
	"sort"
	"strings"
)

// kind maps an error message to a stable name.
func kind(err error) string {
	m := err.Error()
	if strings.HasPrefix(m, "mail: missing word in phrase: ") {
		rest := strings.TrimPrefix(m, "mail: missing word in phrase: ")
		if strings.HasPrefix(rest, "charset not supported") {
			return "phrase:charset"
		}
		return "phrase:" + kind(errorString(rest))
	}
	table := []struct{ prefix, name string }{
		{"mail: no address", "no-address"},
		{"mail: expected comma", "expected-comma"},
		{"mail: expected single address", "expected-single"},
		{"mail: empty group", "empty-group"},
		{"mail: group with multiple addresses", "group-multiple"},
		{"mail: missing '@' or angle-addr", "missing-at-or-angle"},
		{"mail: no angle-addr", "no-angle"},
		{"mail: unclosed angle-addr", "unclosed-angle"},
		{"mail: misformatted parenthetical comment", "bad-comment"},
		{"mail: no addr-spec", "no-addr-spec"},
		{"mail: empty quoted-string in addr-spec", "empty-quoted-local"},
		{"mail: missing @ in addr-spec", "missing-at-in-spec"},
		{"mail: no domain in addr-spec", "no-domain"},
		{"mail: invalid string", "invalid-atom"},
		{"mail: leading dot in atom", "leading-dot"},
		{"mail: double dot in atom", "double-dot"},
		{"mail: trailing dot in atom", "trailing-dot"},
		{"mail: invalid utf-8", "invalid-utf8"},
		{"mail: unclosed quoted-string", "unclosed-quoted"},
		{"mail: bad character in quoted-string", "bad-quoted-char"},
		{"mail: unclosed domain-literal", "unclosed-literal"},
		{"mail: bad character in domain-literal", "bad-literal-char"},
		{"mail: invalid IP address in domain-literal", "invalid-literal"},
		{"charset not supported", "charset"},
		{"mail: header not in message", "not-present"},
		{"malformed initial line", "malformed-initial"},
		{"malformed header line", "malformed-line"},
		{"mail: header could not be parsed", "date"},
		{"mail: header has a CR without LF", "bare-cr"},
	}
	for _, t := range table {
		if strings.HasPrefix(m, t.prefix) {
			return t.name
		}
	}
	if err == io.EOF {
		return "empty-message"
	}
	return "other:" + m
}

type errorString string

func (e errorString) Error() string { return string(e) }

type addr struct {
	Name    string `json:"name"`
	Address string `json:"address"`
}

type listCase struct {
	Input string `json:"input"`
	Hex   string `json:"hex"`
	Error string `json:"error"`
	List  []addr `json:"list"`
}

type formatCase struct {
	Name    string `json:"name"`
	Address string `json:"address"`
	Text    string `json:"text"`
}

type messageCase struct {
	Input  string     `json:"input"`
	Error  string     `json:"error"`
	Fields [][]string `json:"fields"`
	Body   string     `json:"body"`
}

type dateCase struct {
	Input  string `json:"input"`
	Error  string `json:"error"`
	Unix   int64  `json:"unix"`
	Offset int    `json:"offset"`
}

var addresses = []string{
	"jdoe@machine.example",
	"John Doe <jdoe@machine.example>",
	`"Joe Q. Public" <john.q.public@example.com>`,
	"Mary Smith <mary@x.test>, jdoe@example.org, Who? <one@y.test>",
	`<boss@nil.test>, "Giant; \"Big\" Box" <sysservices@example.net>`,
	"A Group:Ed Jones <c@a.test>,joe@where.test,John <jdoe@one.test>;",
	"Undisclosed recipients:;",
	"Pete(A nice \\) chap) <pete(his account)@silly.test(his host)>",
	"John Doe <jdoe@machine(comment).  example>",
	"jdoe@example.com (John Doe)",
	"jdoe@example.com (John (Middle) Doe)",
	"jdoe@example.com (=?utf-8?q?J=C3=B6rg?= Doe)",
	"=?iso-8859-1?q?J=F6rg_Doe?= <joerg@example.com>",
	"=?utf-8?b?SsO2cmc=?= =?utf-8?q?_Doe?= <joerg@example.com>",
	"=?UTF-8?Q?Andr=C3=A9?= Pirard <PIRARD@vm1.ulg.ac.be>",
	"=?ISO-8859-15?Q?J=F6rg?= <joerg@example.com>",
	"=?us-ascii?q?J=6Frg?= <joerg@example.com>",
	"Jörg Doe <joerg@example.com>",
	"Jörg <jörg@例え.テスト>",
	`"\"quoted\"" <quoted@example.com>`,
	`"embedded\\backslash" <b@example.com>`,
	`"local part"@example.com`,
	`"@"@example.com`,
	`""@example.com`,
	"user@[127.0.0.1]",
	"user@[::1]",
	"user@[IPv6:::1]",
	"user@[1.2.3.256]",
	"user@[127.0.0.1",
	"Name <user@[192.168.0.1]>",
	"a@b, , c@d",
	",, a@b,,",
	"a@b,",
	"",
	"   ",
	"no at sign",
	"noatsign",
	"foo.bar",
	"Full Name",
	"<unclosed@example.com",
	"Name <unclosed@example.com",
	"Name <@example.com>",
	"Name <user@>",
	"Name <user>",
	".lead@example.com",
	"trail.@example.com",
	"dou..ble@example.com",
	"user@.example.com",
	"user@example..com",
	"Name.With.Dots <x@y.z>",
	"J. Random. Hacker <jrh@example.com>",
	"a@b c@d",
	"a@b (unclosed comment",
	"Group: a@b, c@d",
	"Group: a@b; tail@x",
	"G: a@b;, H: c@d;",
	"Nested: Inner: a@b;;",
	"\"unclosed <a@b>",
	"\"bad\x01char\" <a@b>",
	"\"tab\tinside\" <a@b>",
	"A\x01B <a@b>",
	"<a@b>",
	"  <a@b>  ",
	"<a@b> (comment)",
	"Name<a@b>",
	"Name <a@b> trailing",
	"a@b;",
	"weird!#$%&'*+-/=?^_`{|}~chars@example.com",
	"\"very.unusual.@.unusual.com\"@example.com",
	"user+tag@sub.example.co.uk",
	"Ünïcödé Nämé <u@example.com>, \"Ascii\" <a@example.com>",
	"=?utf-8?q?broken <a@b>",
	"=?utf-8?x?unknownenc?= <a@b>",
	"=?bogus?q?abc?= <a@b>",
	"a@b (=?bogus?q?abc?=)",
	"\xff <a@b>",
	"a\xff@b",
	"Name <a@b\xff>",
	"Long Name With Many Words In It <long@example.com>",
	"\"Quoted\" Mixed Phrase <m@example.com>",
	"Phrase \"with quoted\" words <p@example.com>",
	"(comment only)",
	"a@b (comment) , c@d",
	"Name (comment) <a@b>",
	"Name <a@b>, Group:;, c@d",
}

var single = []string{
	"jdoe@machine.example",
	"John Doe <jdoe@machine.example>",
	"a@b, c@d",
	"Group:;",
	"Group: a@b;",
	"Group: a@b, c@d;",
	"Name <a@b> (comment)",
	"Name <a@b> extra",
	"",
}

var formats = [][2]string{
	{"", "jdoe@machine.example"},
	{"John Doe", "jdoe@machine.example"},
	{"Joe Q. Public", "john.q.public@example.com"},
	{`Giant; "Big" Box`, "sysservices@example.net"},
	{"Jörg Doe", "joerg@example.com"},
	{"Jörg (Doe)", "joerg@example.com"},
	{"André Pirard", "pirard@vm1.ulg.ac.be"},
	{"Rob", "local part@example.com"},
	{"", ".lead@example.com"},
	{"", "trail.@example.com"},
	{"", "dou..ble@example.com"},
	{"", "a.b.c@example.com"},
	{"", "weird\"quote@example.com"},
	{"", "noat"},
	{"", "multi@at@example.com"},
	{"tab\tname", "t@example.com"},
	{"ctrl\x01name", "c@example.com"},
	{"日本語の名前", "jp@example.com"},
	{"A very long name that will need to be split across several encoded words because it is long ünïcödé", "long@example.com"},
	{"x", "ünï@example.com"},
}

var messages = []string{
	"From: John Doe <jdoe@machine.example>\r\nTo: Mary Smith <mary@example.net>\r\nSubject: Saying Hello\r\nDate: Fri, 21 Nov 1997 09:55:06 -0600\r\nMessage-ID: <1234@local.machine.example>\r\n\r\nThis is a message just to say hello.\r\nSo, \"Hello\".\r\n",
	"Subject: folded\r\n  across\r\n\tlines\r\nX-Empty:\r\nx-lower-case: v\r\n\r\nbody",
	"Subject: unix newlines\nTo: a@b\n\nbody\nmore\n",
	"Subject: no body\r\n",
	"Subject: no newline at all",
	"Key With Space: value\r\n\r\n",
	": empty key\r\nSubject: after\r\n\r\nB",
	"Received: one\r\nReceived: two\r\n\r\n",
	" leading continuation\r\n\r\n",
	"No colon here\r\n\r\n",
	"",
	"\r\nbody only",
	"Subject:   lots of leading blanks   \r\n\r\n",
}

var dates = []string{
	"Fri, 21 Nov 1997 09:55:06 -0600",
	"Tue, 1 Jul 2003 10:52:37 +0200",
	"Thu, 13 Feb 1969 23:32:54 -0330",
	"21 Nov 1997 09:55:06 +0000",
	"Fri, 21 Nov 1997 09:55 -0600",
	"Fri, 21 Nov 1997 09:55:06 -0600 (CST)",
	"Fri, 21 Nov 1997 09:55:06\r\n -0600",
	"Fri, 21 Nov 1997 09:55:06 GMT",
	"Fri, 21 Nov 1997 09:55:06 UT",
	"Mon, 02 Jan 2006 15:04:05 +0700",
	"Fri, 21 Nov 1997 09:55:06\r -0600",
	"not a date",
	"Fri, 32 Nov 1997 09:55:06 -0600",
	"",
}

func main() {
	out := map[string]any{}
	var lists []listCase
	for _, s := range addresses {
		c := listCase{Input: s, Hex: hex.EncodeToString([]byte(s)), List: []addr{}}
		got, err := mail.ParseAddressList(s)
		if err != nil {
			c.Error = kind(err)
		} else {
			for _, a := range got {
				c.List = append(c.List, addr{a.Name, a.Address})
			}
		}
		lists = append(lists, c)
	}
	out["lists"] = lists
	var singles []listCase
	for _, s := range single {
		c := listCase{Input: s, Hex: hex.EncodeToString([]byte(s)), List: []addr{}}
		a, err := mail.ParseAddress(s)
		if err != nil {
			c.Error = kind(err)
		} else {
			c.List = append(c.List, addr{a.Name, a.Address})
		}
		singles = append(singles, c)
	}
	out["singles"] = singles
	var fs []formatCase
	for _, f := range formats {
		a := &mail.Address{Name: f[0], Address: f[1]}
		fs = append(fs, formatCase{f[0], f[1], a.String()})
	}
	out["formats"] = fs
	var ms []messageCase
	for _, s := range messages {
		c := messageCase{Input: s, Fields: [][]string{}}
		m, err := mail.ReadMessage(strings.NewReader(s))
		if err != nil {
			c.Error = kind(err)
		} else {
			keys := make([]string, 0, len(m.Header))
			for k := range m.Header {
				keys = append(keys, k)
			}
			sort.Strings(keys)
			for _, k := range keys {
				c.Fields = append(c.Fields, append([]string{k}, m.Header[k]...))
			}
			body, _ := io.ReadAll(m.Body)
			c.Body = string(body)
		}
		ms = append(ms, c)
	}
	out["messages"] = ms
	var ds []dateCase
	for _, s := range dates {
		c := dateCase{Input: s}
		t, err := mail.ParseDate(s)
		if err != nil {
			c.Error = kind(err)
		} else {
			_, off := t.Zone()
			c.Unix, c.Offset = t.Unix(), off
		}
		ds = append(ds, c)
	}
	out["dates"] = ds
	enc := json.NewEncoder(os.Stdout)
	enc.SetEscapeHTML(false)
	enc.Encode(out)
}
