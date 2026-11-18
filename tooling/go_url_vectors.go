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
	"unicode/utf8"
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
// truth records booleans without depending on JSON number coercion in consumers.
func truth(value bool) string {
	if value {
		return "1"
	}
	return "0"
}

// observe records every URL field and derived public view as byte-exact hex or presence bits.
func observe(u *url.URL) []string {
	user, password, username, pass := false, false, "", ""
	if u.User != nil {
		user = true
		username = u.User.Username()
		pass, password = u.User.Password()
	}
	h := func(s string) string { return hex.EncodeToString([]byte(s)) }
	return []string{h(u.Scheme), h(u.Opaque), h(u.Host), h(u.Path), h(u.Fragment), h(u.RawQuery), h(u.RawPath), h(u.RawFragment), truth(u.ForceQuery), truth(u.OmitHost), truth(user), h(username), truth(password), h(pass), h(u.String()), h(u.EscapedPath()), h(u.EscapedFragment()), h(u.Hostname()), h(u.Port()), h(u.RequestURI()), h(u.Redacted()), truth(u.IsAbs()), truth(utf8.ValidString(u.String()))}
}

// parseFailure identifies the public typed cause or exact grammar category.
func parseFailure(err error) (string, string) {
	if err == nil {
		return "", ""
	}
	context := ""
	var outer *url.Error
	if errors.As(err, &outer) {
		context = hex.EncodeToString([]byte(outer.URL))
		err = outer.Err
	}
	kind := failure(err)
	if kind != "parse" {
		return kind, context
	}
	message := err.Error()
	for _, pair := range [][2]string{{"invalid port", "port"}, {"invalid IP-literal", "ip"}, {"invalid host:", "ip"}, {"missing ']'", "bracket"}, {"invalid userinfo", "user"}, {"invalid control", "control"}, {"missing protocol scheme", "scheme"}, {"invalid URI for request", "request"}, {"empty url", "empty"}, {"first path segment", "colon"}} {
		if strings.Contains(message, pair[0]) {
			return pair[1], context
		}
	}
	panic("unclassified URL failure: " + message)
}

// parsingRows exercises raw bytes independently in path, fragment, user and host positions.
func parsingRows() []interface{} {
	inputs := []string{"", "*", ":", "http:", "HTTP://User:p%FF@example.com:00080/a%2fb?q=%zz#é", "a:b", "1a:b", "a_b:c", "./a:b", "/a:b", "///a", "//", "http:///", "http:/a", "http:////a", "mailto:user@example.com", "http:opaque%zz?x#f", "a?", "a??", "a#", "a##", "a%2Fb", "%FF%00", "a?q=%ff%zz#%ff", "http://u:p:q@h/x", "http://u@v@h", "http://@h", "http://:p@h", "http://u:@h", "http://é@h", "http://%C3%A9@%C3%A9/中", "http://h:65536/", "http://h:999999999999999999999999/", "http://h:/", "http://h:x/", "http://a:1,b:2/x", "postgres://a:1,b:2/db", "//a:1,b:2/x", "http://h%25/", "http://h%20/", "http://h\\x/", "http://[::1]", "http://[::]", "http://[::1]:", "http://[::ffff:192.0.2.1%25eth0]:80/x", "http://[fe80::1%25a%20b]", "http://[::1%25]", "http://[127.0.0.1]", "http://[v1.a]", "http://[1:2:3:4:5:6:7:8]", "http://[1:2:3:4:5:6:7]", "http://[1:2:3:4:5:6:7:8:9]", "http://[1::2::3]", "http://[:::1]", "http://[::1", "http://x[::1]", "http://[::1]]", "http://[::1]:x", "http://[::ffff:01.2.3.4]", "http://[::ffff:256.2.3.4]", "http://[::ffff:1.2.3]", "http://[::ffff:1.2.3.4:5]", "http://[::%FF]", "http://[::1%25%FF]", "http://[::1%25x%2525]", "/a%", "/a%gg#f", "/a#%gg", "/a#x\r\n", "/a\r\n#x", "/a?x\x00", "a b", "/a b", "//h/a b", "http://<h>/", "http://h|/", "http://h\x7f/"}
	for value := 0; value < 256; value++ {
		inputs = append(inputs, "Abcdefghi"+string([]byte{byte(value)})+":opaque")
		for _, parts := range [][2]string{{"/x", "y"}, {"/x#", "y"}, {"http://x", "/"}, {"http://x:", "/"}, {"http://u", "@h/"}} {
			inputs = append(inputs, parts[0]+string([]byte{byte(value)})+parts[1])
		}
	}
	rows := []interface{}{}
	for _, input := range inputs {
		for _, request := range []bool{false, true} {
			var u *url.URL
			var err error
			if request {
				u, err = url.ParseRequestURI(input)
			} else {
				u, err = url.Parse(input)
			}
			kind, context := parseFailure(err)
			observation := []string{}
			if err == nil {
				observation = observe(u)
			}
			rows = append(rows, map[string]interface{}{"input": hex.EncodeToString([]byte(input)), "request": request, "error": kind, "context": context, "fields": observation})
		}
	}
	return rows
}

// queryRows records partial successful pairs, canonical ordering and explicit failure priority.
func queryRows() []interface{} {
	inputs := []string{"", "&", "&&", "a", "a=", "=x", "=", "a=1&a=2&a=", "b=1&a=2", "a+b=x+y", "a%2Bb=x%2By", "a=%FF%00", "a=1;2&b=3", "a%3Bb=ok", "a=%gg&b=2", "%gg=1&b=2", "a=%gg&b=2;c", "x=1;y&a=%gg", "x=1&bad=%&x=2", "a=b=c", "a=é中😀", "%00=%FF"}
	for value := 0; value < 256; value++ {
		inputs = append(inputs, fmt.Sprintf("z=last&k%%%02X=%%%02X&k%%%02X=two&=empty", value, value, value))
		inputs = append(inputs, "a="+string([]byte{byte(value)})+"&b=ok")
	}
	rows := []interface{}{}
	for _, input := range inputs {
		values, err := url.ParseQuery(input)
		kind := failure(err)
		if err != nil && strings.Contains(err.Error(), "semicolon") {
			kind = "semicolon"
		}
		rows = append(rows, map[string]interface{}{"input": hex.EncodeToString([]byte(input)), "output": hex.EncodeToString([]byte(values.Encode())), "error": kind, "size": len(values)})
	}
	return rows
}

// resolutionRows covers normal/abnormal references, opaque bases, repeated slashes and encoding hints.
func resolutionRows() []interface{} {
	bases := []string{"http://a/b/c/d;p?q#old", "relative/a/b?x#y", "mailto:user@example.com?x#old", "http:///a/b", "scheme:/a//b/", "//host/a/b", ""}
	refs := []string{"g:h", "g", "./g", "g/", "/g", "//g", "?y", "g?y", "#s", "g#s", "g?y#s", ";x", "g;x", "g;x?y#s", "", ".", "./", "..", "../", "../g", "../..", "../../", "../../g", "../../../g", "../../../../g", "/./g", "/../g", "g.", ".g", "g..", "..g", "./../g", "./g/.", "g/./h", "g/../h", "g;x=1/./y", "g;x=1/../y", "g?y/./x", "g?y/../x", "g#s/./x", "g#s/../x", "http:g", "a//b///c", "a/%2e/%2E%2E/b", "?", "#", "//user:pass@host/x", "scheme:opaque", "///g", "//", "%FF", "g%zz"}
	rows := []interface{}{}
	for _, base := range bases {
		b, err := url.Parse(base)
		if err != nil {
			panic(err)
		}
		for _, ref := range refs {
			parsed, err := b.Parse(ref)
			kind, context := parseFailure(err)
			fields := []string{}
			if err == nil {
				fields = observe(parsed)
			}
			rows = append(rows, map[string]interface{}{"base": hex.EncodeToString([]byte(base)), "reference": hex.EncodeToString([]byte(ref)), "error": kind, "context": context, "fields": fields})
		}
	}
	return rows
}

// joiningRows checks path method behavior and checked top-level joining separately.
func joiningRows() []interface{} {
	bases := []string{"http://host/a/b?x#f", "relative/a", "http://host", "http://host/", "scheme:opaque", "", "/a%2fb", "http:///a"}
	elements := [][]string{{}, {""}, {"/"}, {".."}, {"..", "..", "..", "x"}, {"a", "b"}, {"a/", "/b//", "c/"}, {".", "..", "x"}, {"%FF", "%2e%2e", "b"}, {"a%2fb", "c"}, {"bad%zz"}, {"é", "中😀/"}, {string([]byte{255}), "x"}}
	rows := []interface{}{}
	for _, base := range bases {
		u, err := url.Parse(base)
		if err != nil {
			panic(err)
		}
		for _, parts := range elements {
			joined, err := url.JoinPath(base, parts...)
			kind, _ := parseFailure(err)
			encoded := []string{}
			for _, part := range parts {
				encoded = append(encoded, hex.EncodeToString([]byte(part)))
			}
			rows = append(rows, map[string]interface{}{"base": hex.EncodeToString([]byte(base)), "elements": encoded, "fields": observe(u.JoinPath(parts...)), "output": hex.EncodeToString([]byte(joined)), "error": kind})
		}
	}
	return rows
}

// manualRows observes independently supplied fields, invalid hints and arbitrary preserved bytes.
func manualRows() []interface{} {
	inputs := []url.URL{{Path: "a:b"}, {Scheme: "http", Path: "//x", OmitHost: true}, {Scheme: "http", Path: "x"}, {Scheme: "x", Host: "h", Path: "abc"}, {Path: "*"}, {Path: "/a/b", RawPath: "/a%2fb"}, {Path: "/a/b", RawPath: "/a%2Fg"}, {Path: "/a b", RawPath: "/a b"}, {Path: "/'[*]", RawPath: "/'[*]"}, {Fragment: "a?b", RawFragment: "a?b"}, {Fragment: "a#b", RawFragment: "a#b"}, {Fragment: "a/b", RawFragment: "a%2fb"}, {RawQuery: string([]byte{255})}, {Opaque: string([]byte{255})}, {Scheme: "X", Opaque: "//opaque"}, {User: url.User(""), Host: ""}, {User: url.UserPassword("", "")}, {User: url.UserPassword(string([]byte{255}), string([]byte{0, 255})), Host: "h"}, {Host: "[::1]:080"}, {Host: "host:bad"}, {Host: "a:b:80"}, {Host: "[bad]"}, {Path: string([]byte{0, 255}), Fragment: string([]byte{255}), ForceQuery: true}}
	rows := []interface{}{}
	for _, u := range inputs {
		rows = append(rows, observe(&u))
	}
	return rows
}

// queryLongRows observes exact default-boundary and ordered/repeated-key stress results.
func queryLongRows() []interface{} {
	rows := []interface{}{}
	for _, count := range []int{0, 1, 15, 16, 255, 256, 9999, 10000, 10001} {
		for mode := 0; mode < 3; mode++ {
			parts := []string{}
			for i := 0; i < count; i++ {
				if mode == 0 {
					parts = append(parts, fmt.Sprintf("k%d=v%d", 100000+i, 100000+i))
				} else if mode == 1 {
					parts = append(parts, fmt.Sprintf("k=v%d", 100000+i))
				} else {
					parts = append(parts, "")
				}
			}
			raw := strings.Join(parts, "&")
			values, err := url.ParseQuery(raw)
			kind := ""
			if err != nil {
				kind = "limit"
			}
			output := values.Encode()
			rows = append(rows, map[string]interface{}{"count": count, "mode": mode, "input_size": len(raw), "input_hash": fingerprint([]byte(raw)), "output_size": len(output), "output_hash": fingerprint([]byte(output)), "size": len(values), "error": kind})
		}
	}
	return rows
}

// hexPairHash independently observes every two-byte percent-digit combination through the public decoder.
func hexPairHash() uint64 {
	h := fnv.New64a()
	for first := 0; first < 256; first++ {
		for last := 0; last < 256; last++ {
			output, err := url.PathUnescape(string([]byte{'%', byte(first), byte(last)}))
			value := uint16(256)
			if err == nil {
				if len(output) != 1 {
					panic("unexpected pair width")
				}
				value = uint16(output[0])
			}
			h.Write([]byte{byte(value), byte(value >> 8)})
		}
	}
	return h.Sum64()
}

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
	if err := e.Encode(map[string]interface{}{"hex_pair_hash": hexPairHash(), "encoded": encoded, "decoded": decoded, "long": long, "queries": queryRows(), "query_long": queryLongRows(), "parsing": parsingRows(), "resolution": resolutionRows(), "joining": joiningRows(), "manual": manualRows()}); err != nil {
		panic(err)
	}
}
