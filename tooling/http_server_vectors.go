// Generate reference vectors from Go 1.24 net/http and net/http/httptest.
// Usage: GOCACHE=/tmp/go-cache GOTELEMETRY=off go run tooling/http_server_vectors.go > seed/tests/fixtures/http-server-vectors.json
package main

import (
	"bufio"
	"bytes"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/http/httptest"
	"net/textproto"
	"os"
	"sort"
	"strings"
)

type StatusVec struct {
	Code int    `json:"code"`
	Text string `json:"text"`
}

type CanonicalVec struct {
	In  string `json:"in"`
	Out string `json:"out"`
}

type PatternRegisterVec struct {
	Name     string   `json:"name"`
	Patterns []string `json:"patterns"`
	Valid    bool     `json:"valid"`
	Error    string   `json:"error"`
}

type MuxRouteQuery struct {
	Method string `json:"method"`
	Host   string `json:"host"`
	Target string `json:"target"`
}

type MuxRouteResult struct {
	Status     int               `json:"status"`
	Pattern    string            `json:"pattern"`
	Handler    string            `json:"handler"`
	Location   string            `json:"location"`
	Allow      string            `json:"allow"`
	PathValues map[string]string `json:"path_values"`
}

type MuxScenarioVec struct {
	Name     string            `json:"name"`
	Routes   []string          `json:"routes"`
	Params   []string          `json:"params"`
	Queries  []MuxRouteQuery   `json:"queries"`
	Expected []MuxRouteResult  `json:"expected"`
}

type WireReqVec struct {
	Name          string            `json:"name"`
	Raw           string            `json:"raw"`
	Valid         bool              `json:"valid"`
	Method        string            `json:"method"`
	Target        string            `json:"target"`
	Path          string            `json:"path"`
	RawQuery      string            `json:"raw_query"`
	Host          string            `json:"host"`
	ProtoMajor    int               `json:"proto_major"`
	ProtoMinor    int               `json:"proto_minor"`
	Headers       [][2]string       `json:"headers"`
	BodyHex       string            `json:"body_hex"`
	Close         bool              `json:"close"`
	ContentLength int64             `json:"content_length"`
	QueryParams   map[string]string `json:"query_params"`
	FormParams    map[string]string `json:"form_params"`
}

type WireRespVec struct {
	Name       string      `json:"name"`
	ReqMethod  string      `json:"req_method"`
	Raw        string      `json:"raw"`
	Valid      bool        `json:"valid"`
	Status     int         `json:"status"`
	ProtoMajor int         `json:"proto_major"`
	ProtoMinor int         `json:"proto_minor"`
	Headers    [][2]string `json:"headers"`
	BodyHex    string      `json:"body_hex"`
}

type RecorderVec struct {
	Name        string      `json:"name"`
	Status      int         `json:"status"`
	Body        string      `json:"body"`
	ContentType string      `json:"content_type"`
	Headers     [][2]string `json:"headers"`
	Flushed     bool        `json:"flushed"`
}

type NewRequestVec struct {
	Name       string `json:"name"`
	Method     string `json:"method"`
	Target     string `json:"target"`
	Body       string `json:"body"`
	GotMethod  string `json:"got_method"`
	GotHost    string `json:"got_host"`
	GotPath    string `json:"got_path"`
	GotQuery   string `json:"got_query"`
	GotURI     string `json:"got_uri"`
	ProtoMajor int    `json:"proto_major"`
	ProtoMinor int    `json:"proto_minor"`
	RemoteAddr string `json:"remote_addr"`
}

type Fixture struct {
	StatusTexts     []StatusVec          `json:"status_texts"`
	CanonicalHeader []CanonicalVec       `json:"canonical_headers"`
	PatternRegister []PatternRegisterVec `json:"pattern_register"`
	MuxScenarios    []MuxScenarioVec     `json:"mux_scenarios"`
	WireRequests    []WireReqVec         `json:"wire_requests"`
	WireResponses   []WireRespVec        `json:"wire_responses"`
	Recorders       []RecorderVec        `json:"recorders"`
	NewRequests     []NewRequestVec      `json:"new_requests"`
}

func tryRegister(patterns []string) (ok bool, errMsg string) {
	defer func() {
		if r := recover(); r != nil {
			ok = false
			errMsg = fmt.Sprint(r)
		}
	}()
	mux := http.NewServeMux()
	for _, p := range patterns {
		pat := p
		mux.HandleFunc(pat, func(w http.ResponseWriter, r *http.Request) {
			_, _ = w.Write([]byte(pat))
		})
	}
	return true, ""
}

func runMuxScenario(name string, routes []string, paramNames []string, queries []MuxRouteQuery) MuxScenarioVec {
	mux := http.NewServeMux()
	for _, pat := range routes {
		p := pat
		mux.HandleFunc(p, func(w http.ResponseWriter, r *http.Request) {
			w.Header().Set("X-Matched-Pattern", p)
			var parts []string
			for _, pn := range paramNames {
				v := r.PathValue(pn)
				if v != "" {
					parts = append(parts, pn+"="+v)
				}
			}
			w.Header().Set("X-Path-Values", strings.Join(parts, "&"))
			w.WriteHeader(http.StatusOK)
			_, _ = w.Write([]byte("handler:" + p))
		})
	}
	var expected []MuxRouteResult
	for _, q := range queries {
		req := httptest.NewRequest(q.Method, "http://"+q.Host+q.Target, nil)
		// Preserve exact host even if empty
		if q.Host == "" {
			req.Host = "example.com"
		} else {
			req.Host = q.Host
		}
		// Keep raw target path/query as requested
		_, pat := mux.Handler(req)
		rec := httptest.NewRecorder()
		mux.ServeHTTP(rec, req)
		res := rec.Result()
		pv := map[string]string{}
		if raw := res.Header.Get("X-Path-Values"); raw != "" {
			for _, kv := range strings.Split(raw, "&") {
				if k, v, ok := strings.Cut(kv, "="); ok {
					pv[k] = v
				}
			}
		}
		handler := ""
		if res.StatusCode == 200 {
			handler = res.Header.Get("X-Matched-Pattern")
		}
		expected = append(expected, MuxRouteResult{
			Status:     res.StatusCode,
			Pattern:    pat,
			Handler:    handler,
			Location:   res.Header.Get("Location"),
			Allow:      res.Header.Get("Allow"),
			PathValues: pv,
		})
	}
	return MuxScenarioVec{
		Name:     name,
		Routes:   routes,
		Params:   paramNames,
		Queries:  queries,
		Expected: expected,
	}
}

func flattenHeaders(h http.Header) [][2]string {
	keys := make([]string, 0, len(h))
	for k := range h {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	var out [][2]string
	for _, k := range keys {
		for _, v := range h[k] {
			out = append(out, [2]string{k, v})
		}
	}
	return out
}

func main() {
	var f Fixture

	// 1. All standard StatusText entries plus boundary unknown codes.
	for code := 99; code <= 600; code++ {
		text := http.StatusText(code)
		if text != "" || code == 99 || code == 104 || code == 299 || code == 419 || code == 599 || code == 600 {
			f.StatusTexts = append(f.StatusTexts, StatusVec{Code: code, Text: text})
		}
	}

	// 2. Canonical header keys.
	for _, k := range []string{
		"content-type",
		"CONTENT-LENGTH",
		"x-forwarded-for",
		"etag",
		"www-authenticate",
		"Accept-Encoding",
		"x-custom-123-abc",
		"a-b-c",
		"bad header",
		"sec-ch-ua",
	} {
		f.CanonicalHeader = append(f.CanonicalHeader, CanonicalVec{
			In:  k,
			Out: textproto.CanonicalMIMEHeaderKey(k),
		})
	}

	// 3. Go 1.22 ServeMux pattern validation & conflict detection.
	regCases := []struct {
		name     string
		patterns []string
	}{
		{"empty-pattern", []string{""}},
		{"no-slash", []string{"example.com"}},
		{"root-valid", []string{"/"}},
		{"exact-root-valid", []string{"/{$}"}},
		{"root-and-exact-root", []string{"/", "/{$}"}},
		{"duplicate-exact", []string{"/a", "/a"}},
		{"duplicate-method", []string{"GET /a", "GET /a"}},
		{"get-and-head-no-conflict", []string{"GET /a", "HEAD /a"}},
		{"get-and-post-no-conflict", []string{"GET /a", "POST /a"}},
		{"method-and-wildcard-method-exact-path", []string{"GET /a", "/a"}},
		{"method-vs-path-conflict", []string{"GET /", "/index.html"}},
		{"wildcard-conflict", []string{"/a/{x}/b", "/a/c/{y}"}},
		{"wildcard-specific-ok", []string{"/a/{x}/b", "/a/c/b"}},
		{"multi-vs-single-ok", []string{"/a/{x...}", "/a/{x}"}},
		{"multi-vs-exact-ok", []string{"/a/{x...}", "/a/b/c"}},
		{"trailing-slash-vs-exact-end", []string{"/a/", "/a/{$}"}},
		{"host-wins-conflict", []string{"example.com/a/{x}", "/a/b"}},
		{"host-wins-method-conflict", []string{"example.com/", "GET /a"}},
		{"two-hosts-disjoint", []string{"a.example/x", "b.example/x"}},
		{"bad-wildcard-unclosed", []string{"/{id"}},
		{"bad-wildcard-partial-prefix", []string{"/a{id}"}},
		{"bad-wildcard-partial-suffix", []string{"/{id}b"}},
		{"bad-wildcard-empty-name", []string{"/{}"}},
		{"bad-wildcard-digit-start", []string{"/{1a}"}},
		{"bad-wildcard-duplicate-name", []string{"/{id}/sub/{id}"}},
		{"bad-multi-not-at-end", []string{"/{rest...}/more"}},
		{"bad-multi-trailing-slash", []string{"/{rest...}/"}},
		{"bad-dollar-not-at-end", []string{"/{$}/a"}},
		{"bad-brace-in-host", []string{"{host}.example/a"}},
		{"bad-method-chars", []string{"G/T /a"}},
		{"valid-custom-method", []string{"PURGE /cache/{key}"}},
		{"valid-tab-after-method", []string{"GET\t/tab"}},
		{"valid-multi-spaces-after-method", []string{"POST   /spaces"}},
		{"valid-escaped-pattern-segment", []string{"/a%2fb/{id}"}},
		{"exact-and-subtree-root", []string{"/files/", "/files"}},
		{"wildcard-name-underscore", []string{"/{_id_1}/items/{item_2...}"}},
	}
	for _, rc := range regCases {
		ok, msg := tryRegister(rc.patterns)
		f.PatternRegister = append(f.PatternRegister, PatternRegisterVec{
			Name:     rc.name,
			Patterns: rc.patterns,
			Valid:    ok,
			Error:    msg,
		})
	}

	// 4. Go 1.22 ServeMux routing, precedence, path values, 405 Allow, redirects.
	f.MuxScenarios = append(f.MuxScenarios,
		runMuxScenario(
			"rest-api-and-precedence",
			[]string{
				"/{$}",
				"GET /items",
				"POST /items",
				"GET /items/{id}",
				"DELETE /items/{id}",
				"GET /items/special",
				"HEAD /items/special",
				"GET /files/{path...}",
				"GET /docs/",
				"GET /docs/{$}",
			},
			[]string{"id", "path"},
			[]MuxRouteQuery{
				{Method: "GET", Host: "example.com", Target: "/"},
				{Method: "POST", Host: "example.com", Target: "/"},
				{Method: "GET", Host: "example.com", Target: "/missing"},
				{Method: "GET", Host: "example.com", Target: "/items"},
				{Method: "HEAD", Host: "example.com", Target: "/items"},
				{Method: "POST", Host: "example.com", Target: "/items"},
				{Method: "PUT", Host: "example.com", Target: "/items"},
				{Method: "DELETE", Host: "example.com", Target: "/items"},
				{Method: "GET", Host: "example.com", Target: "/items/42"},
				{Method: "DELETE", Host: "example.com", Target: "/items/42"},
				{Method: "PATCH", Host: "example.com", Target: "/items/42"},
				{Method: "GET", Host: "example.com", Target: "/items/special"},
				{Method: "HEAD", Host: "example.com", Target: "/items/special"},
				{Method: "DELETE", Host: "example.com", Target: "/items/special"},
				{Method: "GET", Host: "example.com", Target: "/files/a/b/c.txt"},
				{Method: "GET", Host: "example.com", Target: "/files/"},
				{Method: "GET", Host: "example.com", Target: "/files"},
				{Method: "GET", Host: "example.com", Target: "/docs/"},
				{Method: "GET", Host: "example.com", Target: "/docs/guide/intro"},
				{Method: "GET", Host: "example.com", Target: "/docs"},
				{Method: "POST", Host: "example.com", Target: "/docs"},
				{Method: "GET", Host: "example.com", Target: "/items//42"},
				{Method: "GET", Host: "example.com", Target: "/a/../items/99"},
			},
		),
		runMuxScenario(
			"hosts-and-escapes",
			[]string{
				"api.example.com/v1/{resource}",
				"api.example.com/v1/health",
				"/v1/{resource}",
				"GET /buckets/{bucket}/objects/{obj...}",
				"GET /encoded/a%2Fb/{tail}",
			},
			[]string{"resource", "bucket", "obj", "tail"},
			[]MuxRouteQuery{
				{Method: "GET", Host: "api.example.com", Target: "/v1/users"},
				{Method: "GET", Host: "api.example.com:8080", Target: "/v1/users"},
				{Method: "GET", Host: "api.example.com", Target: "/v1/health"},
				{Method: "GET", Host: "other.example.com", Target: "/v1/health"},
				{Method: "GET", Host: "example.com", Target: "/buckets/my-bkt/objects/dir/file.json"},
				{Method: "GET", Host: "example.com", Target: "/buckets/b%2F1/objects/x%20y/z"},
				{Method: "GET", Host: "example.com", Target: "/encoded/a%2Fb/ok"},
				{Method: "GET", Host: "example.com", Target: "/encoded/a/b/ok"},
			},
		),
		runMuxScenario(
			"method-405-and-fallback",
			[]string{
				"POST /only-post",
				"PUT /multi",
				"DELETE /multi",
				"GET /multi",
				"/catch/",
			},
			[]string{},
			[]MuxRouteQuery{
				{Method: "GET", Host: "example.com", Target: "/only-post"},
				{Method: "POST", Host: "example.com", Target: "/only-post"},
				{Method: "PATCH", Host: "example.com", Target: "/multi"},
				{Method: "HEAD", Host: "example.com", Target: "/multi"},
				{Method: "DELETE", Host: "example.com", Target: "/catch/anything"},
			},
		),
	)

	// 5. Wire HTTP/1.1 Request parsing vectors.
	wireReqs := []struct {
		name string
		raw  string
	}{
		{
			"simple-get",
			"GET /hello?a=1&b=two HTTP/1.1\r\nHost: example.com\r\nUser-Agent: cinch/1\r\n\r\n",
		},
		{
			"post-content-length-form",
			"POST /submit?q=url HTTP/1.1\r\nHost: example.com\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: 17\r\n\r\nname=ada&q=bodyval"[:130], // let's compute exact length below
		},
		{
			"chunked-request",
			"POST /upload HTTP/1.1\r\nHost: example.com\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nHello\r\n7;ext=1\r\n, World\r\n0\r\n\r\n",
		},
		{
			"http10-close",
			"GET /legacy HTTP/1.0\r\nHost: old.example\r\n\r\n",
		},
		{
			"http11-explicit-close",
			"GET /bye HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n",
		},
		{
			"duplicate-headers",
			"GET /h HTTP/1.1\r\nHost: example.com\r\nx-tag: one\r\nX-Tag: two\r\n\r\n",
		},
		{
			"bad-request-line",
			"NOT_A_REQUEST\r\n\r\n",
		},
		{
			"bad-content-length",
			"POST /x HTTP/1.1\r\nHost: example.com\r\nContent-Length: abc\r\n\r\n",
		},
	}
	// Fix post-content-length-form body length cleanly:
	formBody := "name=ada&q=bodyval"
	wireReqs[1].raw = fmt.Sprintf(
		"POST /submit?q=url HTTP/1.1\r\nHost: example.com\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: %d\r\n\r\n%s",
		len(formBody), formBody,
	)

	for _, wr := range wireReqs {
		req, err := http.ReadRequest(bufio.NewReader(strings.NewReader(wr.raw)))
		if err != nil {
			f.WireRequests = append(f.WireRequests, WireReqVec{
				Name:  wr.name,
				Raw:   wr.raw,
				Valid: false,
			})
			continue
		}
		body, rerr := io.ReadAll(req.Body)
		if rerr != nil {
			f.WireRequests = append(f.WireRequests, WireReqVec{
				Name:  wr.name,
				Raw:   wr.raw,
				Valid: false,
			})
			continue
		}
		// Restore body for ParseForm
		req.Body = io.NopCloser(bytes.NewReader(body))
		qparams := map[string]string{}
		for k, vs := range req.URL.Query() {
			if len(vs) > 0 {
				qparams[k] = vs[0]
			}
		}
		fparams := map[string]string{}
		if req.ParseForm() == nil {
			for k, vs := range req.Form {
				if len(vs) > 0 {
					fparams[k] = vs[0]
				}
			}
		}
		f.WireRequests = append(f.WireRequests, WireReqVec{
			Name:          wr.name,
			Raw:           wr.raw,
			Valid:         true,
			Method:        req.Method,
			Target:        req.RequestURI,
			Path:          req.URL.Path,
			RawQuery:      req.URL.RawQuery,
			Host:          req.Host,
			ProtoMajor:    req.ProtoMajor,
			ProtoMinor:    req.ProtoMinor,
			Headers:       flattenHeaders(req.Header),
			BodyHex:       hex.EncodeToString(body),
			Close:         req.Close,
			ContentLength: req.ContentLength,
			QueryParams:   qparams,
			FormParams:    fparams,
		})
	}

	// 6. Wire HTTP/1.1 Response parsing vectors.
	wireResps := []struct {
		name   string
		method string
		raw    string
	}{
		{
			"ok-content-length",
			"GET",
			"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 5\r\n\r\nhello",
		},
		{
			"chunked-response",
			"GET",
			"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n4\r\nWiki\r\n5\r\npedia\r\n0\r\n\r\n",
		},
		{
			"head-no-body",
			"HEAD",
			"HTTP/1.1 200 OK\r\nContent-Length: 999\r\n\r\n",
		},
		{
			"no-content-204",
			"DELETE",
			"HTTP/1.1 204 No Content\r\nX-Done: yes\r\n\r\n",
		},
		{
			"not-modified-304",
			"GET",
			"HTTP/1.1 304 Not Modified\r\nETag: \"v1\"\r\n\r\n",
		},
		{
			"eof-delimited",
			"GET",
			"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nuntil eof",
		},
		{
			"bad-status-line",
			"GET",
			"GARBAGE\r\n\r\n",
		},
	}
	for _, wr := range wireResps {
		dummyReq, _ := http.NewRequest(wr.method, "http://example.com/", nil)
		resp, err := http.ReadResponse(bufio.NewReader(strings.NewReader(wr.raw)), dummyReq)
		if err != nil {
			f.WireResponses = append(f.WireResponses, WireRespVec{
				Name:      wr.name,
				ReqMethod: wr.method,
				Raw:       wr.raw,
				Valid:     false,
			})
			continue
		}
		body, rerr := io.ReadAll(resp.Body)
		if rerr != nil {
			f.WireResponses = append(f.WireResponses, WireRespVec{
				Name:      wr.name,
				ReqMethod: wr.method,
				Raw:       wr.raw,
				Valid:     false,
			})
			continue
		}
		f.WireResponses = append(f.WireResponses, WireRespVec{
			Name:       wr.name,
			ReqMethod:  wr.method,
			Raw:        wr.raw,
			Valid:      true,
			Status:     resp.StatusCode,
			ProtoMajor: resp.ProtoMajor,
			ProtoMinor: resp.ProtoMinor,
			Headers:    flattenHeaders(resp.Header),
			BodyHex:    hex.EncodeToString(body),
		})
	}

	// 7. httptest.ResponseRecorder scenarios.
	{
		// Default empty recorder
		rec := httptest.NewRecorder()
		res := rec.Result()
		f.Recorders = append(f.Recorders, RecorderVec{
			Name:        "empty-default",
			Status:      res.StatusCode,
			Body:        rec.Body.String(),
			ContentType: res.Header.Get("Content-Type"),
			Headers:     flattenHeaders(res.Header),
			Flushed:     rec.Flushed,
		})
	}
	{
		// Implicit 200 on write + header snapshot semantics
		rec := httptest.NewRecorder()
		rec.Header().Set("X-Before", "1")
		_, _ = rec.WriteString("hello world")
		// Mutating header after WriteHeader must not alter Result().Header
		rec.Header().Set("X-After", "2")
		// Second WriteHeader is ignored
		rec.WriteHeader(http.StatusNotFound)
		res := rec.Result()
		f.Recorders = append(f.Recorders, RecorderVec{
			Name:        "implicit-200-and-snapshot",
			Status:      res.StatusCode,
			Body:        rec.Body.String(),
			ContentType: res.Header.Get("Content-Type"),
			Headers:     flattenHeaders(res.Header),
			Flushed:     rec.Flushed,
		})
	}
	{
		// Explicit 201 + Flush + explicit Content-Type
		rec := httptest.NewRecorder()
		rec.Header().Set("Content-Type", "application/json")
		rec.WriteHeader(http.StatusCreated)
		_, _ = rec.Write([]byte(`{"ok":true}`))
		rec.Flush()
		res := rec.Result()
		f.Recorders = append(f.Recorders, RecorderVec{
			Name:        "created-json-flushed",
			Status:      res.StatusCode,
			Body:        rec.Body.String(),
			ContentType: res.Header.Get("Content-Type"),
			Headers:     flattenHeaders(res.Header),
			Flushed:     rec.Flushed,
		})
	}
	{
		// Flush without prior WriteHeader triggers 200
		rec := httptest.NewRecorder()
		rec.Header().Add("X-Multi", "a")
		rec.Header().Add("X-Multi", "b")
		rec.Flush()
		res := rec.Result()
		f.Recorders = append(f.Recorders, RecorderVec{
			Name:        "flush-triggers-200",
			Status:      res.StatusCode,
			Body:        rec.Body.String(),
			ContentType: res.Header.Get("Content-Type"),
			Headers:     flattenHeaders(res.Header),
			Flushed:     rec.Flushed,
		})
	}

	// 8. httptest.NewRequest scenarios.
	for _, nr := range []struct {
		name, method, target, body string
	}{
		{"default-get", "", "/hello?x=1", ""},
		{"post-with-host", "POST", "http://api.local:8080/v1/items?k=v", "payload"},
		{"root-path", "DELETE", "/", ""},
	} {
		var bodyReader io.Reader
		if nr.body != "" {
			bodyReader = strings.NewReader(nr.body)
		}
		req := httptest.NewRequest(nr.method, nr.target, bodyReader)
		f.NewRequests = append(f.NewRequests, NewRequestVec{
			Name:       nr.name,
			Method:     nr.method,
			Target:     nr.target,
			Body:       nr.body,
			GotMethod:  req.Method,
			GotHost:    req.Host,
			GotPath:    req.URL.Path,
			GotQuery:   req.URL.RawQuery,
			GotURI:     req.RequestURI,
			ProtoMajor: req.ProtoMajor,
			ProtoMinor: req.ProtoMinor,
			RemoteAddr: req.RemoteAddr,
		})
	}

	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	if err := enc.Encode(f); err != nil {
		fmt.Fprintf(os.Stderr, "encode error: %v\n", err)
		os.Exit(1)
	}
}
