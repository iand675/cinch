// Command cookie_vectors writes HTTP cookie and cookie jar vectors from the reference SDK's implementation.
//
// Rows cover Set-Cookie parsing (every field of the result, or the error), strict and lenient Cookie header
// parsing, serialization and validation of constructed cookies, and jar scenarios: scripted stores and
// lookups at chosen times against hosts, ports, paths, schemes, IP addresses, internationalized names and a
// fixed public suffix function. The jar's time-taking entry points are reached by linkname so scenarios can
// advance the clock, so the program needs the linkname check off and UTC as the local zone:
//
//	TZ=UTC go run -ldflags=-checklinkname=0 tooling/cookie_vectors.go > seed/tests/fixtures/cookie-vectors.json
package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/cookiejar"
	"net/url"
	"os"
	"strings"
	"time"
	_ "unsafe"
)

//go:linkname jarSetCookies net/http/cookiejar.(*Jar).setCookies
func jarSetCookies(j *cookiejar.Jar, u *url.URL, cookies []*http.Cookie, now time.Time)

//go:linkname jarCookies net/http/cookiejar.(*Jar).cookies
func jarCookies(j *cookiejar.Jar, u *url.URL, now time.Time) []*http.Cookie

// base is the scenario clock's zero, in Unix seconds.
const base = 1700000000

// testPSL is a small public suffix function: co.uk, the last label otherwise, and two deliberately broken answers.
type testPSL struct{}

func (testPSL) String() string { return "test" }
func (testPSL) PublicSuffix(d string) string {
	if d == "co.uk" || strings.HasSuffix(d, ".co.uk") {
		return "co.uk"
	}
	if d == "www.buggy.psl" {
		return "xy"
	}
	if d == "www2.buggy.psl" {
		return "com"
	}
	return d[strings.LastIndex(d, ".")+1:]
}

func flag(b bool) string {
	if b {
		return "1"
	}
	return "0"
}

// summary renders every field of a parsed cookie.
func summary(c *http.Cookie) string {
	exp := "-"
	if !c.Expires.IsZero() {
		exp = fmt.Sprint(c.Expires.Unix())
	}
	return strings.Join([]string{c.Name, c.Value, flag(c.Quoted), c.Path, c.Domain, exp, c.RawExpires,
		fmt.Sprint(c.MaxAge), flag(c.Secure), flag(c.HttpOnly), fmt.Sprint(int(c.SameSite)), flag(c.Partitioned),
		strings.Join(c.Unparsed, ",")}, "|")
}

// pairs renders request cookies as a Cookie header would carry them, quoting quoted values.
func pairs(cs []*http.Cookie) string {
	var out []string
	for _, c := range cs {
		v := c.Value
		if c.Quoted {
			v = `"` + v + `"`
		}
		out = append(out, c.Name+"="+v)
	}
	return strings.Join(out, "; ")
}

type setRow struct {
	Line   string `json:"line"`
	Result string `json:"result"`
}

type readRow struct {
	Line   string `json:"line"`
	Filter string `json:"filter"`
	Result string `json:"result"`
}

type stringRow struct {
	Fields []string `json:"fields"`
	String string   `json:"string"`
	Valid  string   `json:"valid"`
}

type step struct {
	Op     string   `json:"op"`
	URL    string   `json:"url"`
	Time   int64    `json:"time"`
	Scheme string   `json:"scheme"`
	Host   string   `json:"host"`
	Path   string   `json:"path"`
	Lines  []string `json:"lines"`
	Want   string   `json:"want"`
}

type scenario struct {
	Name  string `json:"name"`
	PSL   bool   `json:"psl"`
	Steps []step `json:"steps"`
}

// s is a scripted step: "set T URL line..." or "get T URL".
type s struct {
	op    string
	t     int64
	url   string
	lines []string
}

func runScenario(name string, psl bool, script []s) scenario {
	var o *cookiejar.Options
	if psl {
		o = &cookiejar.Options{PublicSuffixList: testPSL{}}
	}
	jar, _ := cookiejar.New(o)
	sc := scenario{Name: name, PSL: psl}
	for _, st := range script {
		u, err := url.Parse(st.url)
		if err != nil {
			panic(err)
		}
		now := time.Unix(base+st.t, 0)
		row := step{Op: st.op, URL: st.url, Time: st.t, Scheme: u.Scheme, Host: u.Host, Path: u.Path}
		if st.op == "set" {
			var cs []*http.Cookie
			for _, line := range st.lines {
				if c, err := http.ParseSetCookie(line); err == nil {
					cs = append(cs, c)
				}
			}
			jarSetCookies(jar, u, cs, now)
			row.Lines = st.lines
			if row.Lines == nil {
				row.Lines = []string{}
			}
		} else {
			row.Lines = []string{}
			row.Want = pairs(jarCookies(jar, u, now))
		}
		sc.Steps = append(sc.Steps, row)
	}
	return sc
}

func main() {
	setLines := []string{
		"a=b",
		"  a = b ; Path=/x ; Domain=.Example.COM ",
		`n="quoted v"`,
		`n="unbalanced`,
		`n=bad"q`,
		"n=a\\b",
		"",
		"   ",
		"noequals",
		"=v",
		"bad name=v",
		"b(=v",
		"k=v; Max-Age=0",
		"k=v; max-age=-5",
		"k=v; Max-Age=3600",
		"k=v; Max-Age=05",
		"k=v; Max-Age=abc",
		"k=v; Max-Age=",
		"k=v; Max-Age=+5",
		"k=v; Expires=Wed, 21 Oct 2015 07:28:00 GMT",
		"k=v; expires=Wed, 21-Oct-2015 07:28:00 GMT",
		"k=v; Expires=Thu, 01 Jan 1970 00:00:00 GMT+3",
		"k=v; Expires=Thu, 01 Jan 1970 00:00:00 GMT-11",
		"k=v; Expires=Sun, 06 Nov 1994 08:49:37 PST",
		"k=v; Expires=Sun, 06 Nov 1994 08:49:37 UTC",
		"k=v; Expires=Sun, 06 Nov 1994 08:49:37 +0100",
		"k=v; Expires=Sunday, 06-Nov-94 08:49:37 GMT",
		"k=v; Expires=Fri, 31 Dec 1600 23:59:59 GMT",
		"k=v; Expires=Mon, 01 Jan 1601 00:00:00 GMT",
		"k=v; Expires=Fri, 31 Dec 9999 23:59:59 GMT",
		"k=v; Expires=Tue, 29 Feb 2000 12:00:00 GMT",
		"k=v; Expires=Thu, 30 Feb 2000 12:00:00 GMT",
		"k=v; Expires=Wed, 21 Oct 2015 07:28:00.5 GMT",
		"k=v; Expires=garbage",
		"k=v; Expires=wed, 21 OCT 2015 07:28:00 GMT",
		"k=v; Expires=Wed, 21 Oct 2015 07:28:00 UTCDT",
		"k=v; Expires=Wed, 21 Oct 2015 07:28:00 +03",
		"k=v; Expires=Wed, 21 Oct 2015 07:28:00 GMT+24",
		"k=v; Expires=Wed, 21 Oct 2015 7:28:00 ChST",
		"k=v; Expires=Wed, 21 Oct 2015 07:28:00 WITA",
		"k=v; Expires=Wed, 21 Oct 2015 07:28:00 CEST",
		"k=v; Expires=Wed, 21 Oct 2015 07:28:00 ABCDE",
		"k=v; Expires=Wed, 21 Oct 2015 07:28:00 GMT; Max-Age=60",
		"k=v; Secure; HttpOnly; Partitioned",
		"k=v; secure=yes; httponly=1",
		"k=v; SameSite", "k=v; SameSite=Lax", "k=v; samesite=strict", "k=v; SameSite=NONE", "k=v; SameSite=bogus",
		"k=v; Path=/a/b; path=/c",
		"k=v; Domain=", "k=v; Domain=example.com", "k=v; domain=..x",
		"k=v; Comment=hi; Version=1; Foo",
		"k=v; ;; Path=/",
		"k=v; Path=/a\"b",
		"k=v; Domain=bad\"d",
		"k=; Path=/",
		`k=""`,
		"k=v=w",
		"k=a b c",
		"k=a,b",
		"k=caf\xc3\xa9",
		"k=v;Path=/;Secure",
	}
	var sets []setRow
	for _, line := range setLines {
		c, err := http.ParseSetCookie(line)
		r := ""
		if err != nil {
			r = "error: " + err.Error()
		} else {
			r = summary(c)
		}
		sets = append(sets, setRow{Line: line, Result: r})
	}

	cookieLines := []string{
		"a=b", "a=b; c=d", " a = b ; c=d", `a="q"; b=""`, "", "  ", "a", "a=b;", "a=b; ;c=d", "a=b;c", "bad name=x",
		"a=b\"c", "a=b; a=c", "x=1;y=2;z=3", "a=b\\c", "a=b; =c", "a=caf\xc3\xa9",
	}
	var strict []setRow
	for _, line := range cookieLines {
		cs, err := http.ParseCookie(line)
		r := ""
		if err != nil {
			r = "error: " + err.Error()
		} else {
			r = pairs(cs)
		}
		strict = append(strict, setRow{Line: line, Result: r})
	}
	var lenient []readRow
	for _, line := range cookieLines {
		for _, filter := range []string{"", "a", "c"} {
			req := &http.Request{Header: http.Header{"Cookie": {line}}}
			var cs []*http.Cookie
			if filter == "" {
				cs = req.Cookies()
			} else {
				cs = req.CookiesNamed(filter)
			}
			lenient = append(lenient, readRow{Line: line, Filter: filter, Result: pairs(cs)})
		}
	}

	at := func(y int, mo time.Month, d, h, mi, sec int) time.Time { return time.Date(y, mo, d, h, mi, sec, 0, time.UTC) }
	built := []*http.Cookie{
		{Name: "a", Value: "b"},
		{Name: "a", Value: "b", Quoted: true},
		{Name: "a", Value: "has space"},
		{Name: "a", Value: "has,comma"},
		{Name: "a", Value: "bad\"q;\\x"},
		{Name: "a", Value: ""},
		{Name: "a", Value: "", Quoted: true},
		{Name: "bad name", Value: "v"},
		{Name: "", Value: "v"},
		{Name: "a", Value: "b", Path: "/p;x", Domain: ".example.com", Expires: at(2015, 10, 21, 7, 28, 0), MaxAge: 60, Secure: true, HttpOnly: true, SameSite: http.SameSiteLaxMode, Partitioned: true},
		{Name: "a", Value: "b", Domain: "bad_domain"},
		{Name: "a", Value: "b", Domain: "127.0.0.1"},
		{Name: "a", Value: "b", Domain: "256.0.0.1"},
		{Name: "a", Value: "b", Domain: "a-.com"},
		{Name: "a", Value: "b", Domain: "-a.com"},
		{Name: "a", Value: "b", Domain: "123.com"},
		{Name: "a", Value: "b", Domain: "1.2.3"},
		{Name: "a", Value: "b", Domain: strings.Repeat("x", 64) + ".com"},
		{Name: "a", Value: "b", Domain: strings.Repeat("x", 63) + ".com"},
		{Name: "a", Value: "b", Expires: at(1600, 12, 31, 23, 59, 59)},
		{Name: "a", Value: "b", Expires: at(1601, 1, 1, 0, 0, 0)},
		{Name: "a", Value: "b", Expires: at(1969, 7, 20, 20, 17, 40)},
		{Name: "a", Value: "b", Expires: at(9999, 12, 31, 23, 59, 59)},
		{Name: "a", Value: "b", MaxAge: -1},
		{Name: "a", Value: "b", SameSite: http.SameSiteDefaultMode},
		{Name: "a", Value: "b", SameSite: http.SameSiteStrictMode},
		{Name: "a", Value: "b", SameSite: http.SameSiteNoneMode},
		{Name: "a", Value: "b", Partitioned: true},
		{Name: "a", Value: "x\ty"},
		{Name: "a", Value: "x\x01y"},
		{Name: "a", Value: "caf\xc3\xa9"},
		{Name: "a", Value: "b", Path: "/caf\xc3\xa9"},
		{Name: "a", Value: "b", Path: "/x\ny"},
		{Name: "a", Value: "x'y"},
		{Name: "a", Value: "x\\y"},
		{Name: "a", Value: "x\x7fy"},
	}
	var strs []stringRow
	for _, c := range built {
		exp := "-"
		if !c.Expires.IsZero() {
			exp = fmt.Sprint(c.Expires.Unix())
		}
		v := ""
		if err := c.Valid(); err != nil {
			v = err.Error()
		}
		strs = append(strs, stringRow{
			Fields: []string{c.Name, c.Value, flag(c.Quoted), c.Path, c.Domain, exp, fmt.Sprint(c.MaxAge), flag(c.Secure), flag(c.HttpOnly), fmt.Sprint(int(c.SameSite)), flag(c.Partitioned)},
			String: c.String(), Valid: v,
		})
	}

	future := "Fri, 01 Jan 2100 00:00:00 GMT"
	past := "Thu, 01 Jan 1998 00:00:00 GMT"
	soon := time.Unix(base+30, 0).UTC().Format(time.RFC1123)
	scenarios := []scenario{
		runScenario("host and domain cookies", false, []s{
			{"set", 0, "http://www.host.test/", []string{"A=a", "B=b; domain=host.test", "C=c; domain=.www.host.test"}},
			{"get", 1, "http://www.host.test/", nil},
			{"get", 1, "http://host.test/", nil},
			{"get", 1, "http://sub.www.host.test/", nil},
			{"get", 1, "http://www.host.test./", nil},
			{"get", 1, "http://WWW.Host.Test:8080/x", nil},
			{"get", 1, "http://other.test/", nil},
			{"get", 1, "ftp://www.host.test/", nil},
		}),
		runScenario("paths", false, []s{
			{"set", 0, "http://www.host.test/some/path", []string{"A=a", "B=b; path=/", "C=c; path=/some/path/", "D=d; path=/some/path", "E=e; path=relative"}},
			{"get", 1, "http://www.host.test/", nil},
			{"get", 1, "http://www.host.test", nil},
			{"get", 1, "http://www.host.test/some", nil},
			{"get", 1, "http://www.host.test/some/path", nil},
			{"get", 1, "http://www.host.test/some/path/", nil},
			{"get", 1, "http://www.host.test/some/path/x", nil},
			{"get", 1, "http://www.host.test/somewhere", nil},
			{"get", 1, "http://www.host.test/some/pathology", nil},
			{"set", 2, "http://www.host.test/one", []string{"F=f"}},
			{"set", 2, "http://www.host.test/a/b/c", []string{"G=g"}},
			{"set", 2, "http://www.host.test", []string{"H=h"}},
			{"get", 3, "http://www.host.test/a/b/c", nil},
		}),
		runScenario("secure and schemes", false, []s{
			{"set", 0, "https://www.host.test/", []string{"S=s; secure", "N=n"}},
			{"set", 0, "ftp://www.host.test/", []string{"F=f"}},
			{"set", 0, "http://www.host.test/", []string{"P=p; Secure"}},
			{"get", 1, "http://www.host.test/", nil},
			{"get", 1, "https://www.host.test/", nil},
		}),
		runScenario("expiry", false, []s{
			{"set", 0, "http://www.host.test/", []string{"A=a; max-age=10", "B=b; expires=" + future, "C=c", "D=d; expires=" + past, "E=e; max-age=0", "F=f; expires=" + soon, "G=g; expires=" + future + "; max-age=5"}},
			{"get", 5, "http://www.host.test/", nil},
			{"get", 9, "http://www.host.test/", nil},
			{"get", 10, "http://www.host.test/", nil},
			{"get", 11, "http://www.host.test/", nil},
			{"set", 12, "http://www.host.test/", []string{"B=; max-age=-1", "C=; expires=" + past, "Z=gone; max-age=-1"}},
			{"get", 13, "http://www.host.test/", nil},
			{"get", 31, "http://www.host.test/", nil},
			{"set", 40, "http://www.host.test/", []string{"F=again"}},
			{"get", 41, "http://www.host.test/", nil},
		}),
		runScenario("replacement keeps order", false, []s{
			{"set", 0, "http://www.host.test/", []string{"A=1"}},
			{"set", 1, "http://www.host.test/", []string{"B=2", "C=3; path=/x"}},
			{"set", 2, "http://www.host.test/", []string{"A=4; HttpOnly", "B=5; path=/"}},
			{"set", 3, "http://www.host.test/", []string{"D=6", "D=7"}},
			{"get", 4, "http://www.host.test/x/y", nil},
			{"set", 5, "http://www.host.test/", []string{`Q="x y"`, `R=""`}},
			{"get", 6, "http://www.host.test/", nil},
		}),
		runScenario("public suffixes", true, []s{
			{"set", 0, "http://www.bbc.co.uk/", []string{"A=a; domain=co.uk", "B=b; domain=bbc.co.uk", "C=c"}},
			{"set", 0, "http://co.uk/", []string{"D=d; domain=co.uk"}},
			{"set", 0, "http://www.host.test/", []string{"E=e; domain=test", "F=f; domain=host.test"}},
			{"set", 0, "http://test/", []string{"G=g; domain=test"}},
			{"set", 0, "http://www.buggy.psl/", []string{"H=h"}},
			{"set", 0, "http://www2.buggy.psl/", []string{"I=i; domain=buggy.psl"}},
			{"get", 1, "http://www.bbc.co.uk/", nil},
			{"get", 1, "http://news.bbc.co.uk/", nil},
			{"get", 1, "http://co.uk/", nil},
			{"get", 1, "http://other.co.uk/", nil},
			{"get", 1, "http://a.host.test/", nil},
			{"get", 1, "http://test/", nil},
			{"get", 1, "http://www.buggy.psl/", nil},
			{"get", 1, "http://www2.buggy.psl/", nil},
		}),
		runScenario("no public suffix function", false, []s{
			{"set", 0, "http://www.a.co.uk/", []string{"A=a; domain=co.uk", "B=b; domain=a.co.uk"}},
			{"set", 0, "http://localhost/", []string{"L=l", "M=m; domain=localhost"}},
			{"get", 1, "http://other.co.uk/", nil},
			{"get", 1, "http://x.a.co.uk/", nil},
			{"get", 1, "http://localhost:3000/", nil},
		}),
		runScenario("ip addresses", false, []s{
			{"set", 0, "http://127.0.0.1/", []string{"A=a", "B=b; domain=127.0.0.1", "C=c; domain=.127.0.0.1", "D=d; domain=0.0.1"}},
			{"set", 0, "http://[::1]:8080/", []string{"E=e", "F=f; domain=::1"}},
			{"set", 0, "http://[fe80::1%25eth0]/", []string{"G=g"}},
			{"get", 1, "http://127.0.0.1:80/", nil},
			{"get", 1, "http://[::1]/", nil},
			{"get", 1, "http://1.0.0.127/", nil},
			{"get", 1, "http://[fe80::1%25eth0]:9/", nil},
		}),
		runScenario("illegal domains", false, []s{
			{"set", 0, "http://www.host.test/", []string{"A=a; domain=other.test", "B=b; domain=..host.test", "C=c; domain=host.test.", "D=d; domain=b\xc3\xbccher.test", "E=e; domain=.", "F=f; domain=HOST.TEST", "G=g; domain=ww.host.test"}},
			{"get", 1, "http://www.host.test/", nil},
			{"get", 1, "http://host.test/", nil},
		}),
		runScenario("internationalized hosts", false, []s{
			{"set", 0, "http://www.b\xc3\xbccher.test/", []string{"A=a", "B=b; domain=xn--bcher-kva.test"}},
			{"set", 0, "http://M\xc3\xbcnchen.test:81/", []string{"C=c"}},
			{"get", 1, "http://www.xn--bcher-kva.test/", nil},
			{"get", 1, "http://xn--bcher-kva.test/", nil},
			{"get", 1, "http://www.b\xc3\xbccher.test/", nil},
			{"get", 1, "http://xn--mnchen-3ya.test/", nil},
			{"get", 1, "http://m\xc3\xbcnchen.test/", nil},
		}),
	}

	enc := json.NewEncoder(os.Stdout)
	enc.SetEscapeHTML(false)
	if err := enc.Encode(map[string]any{"set": sets, "cookie": strict, "read": lenient, "string": strs, "jar": scenarios}); err != nil {
		panic(err)
	}
}
