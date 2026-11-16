// Independent MIME fixtures are produced solely through the pinned public package API.
package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"mime"
	"os"
	"sort"
	"strings"
)

// Pair preserves exact arbitrary-byte key and value strings through hexadecimal JSON.
type Pair struct {
	// Key is an exact original key, with no caller-side case folding.
	Key string `json:"key"`
	// Value is the exact byte value, including invalid UTF-8.
	Value string `json:"value"`
}

// MediaCase records the nullable-map distinction and preserved base on a parameter failure.
type MediaCase struct {
	// Input is the complete wire value in hexadecimal.
	Input string `json:"input"`
	// Base is the returned normalized media type in hexadecimal.
	Base string `json:"base"`
	// Parameters is null on error and a sorted exact list on success.
	Parameters []Pair `json:"parameters"`
	// Error is the exact public diagnostic or empty on success.
	Error string `json:"error"`
}

// FormatCase records exact original-key ordering and arbitrary-byte parameter values.
type FormatCase struct {
	// Base is the original supplied type in hexadecimal.
	Base string `json:"base"`
	// Parameters is the supplied exact-key map, sorted before fixture serialization.
	Parameters []Pair `json:"parameters"`
	// Output is the exact result, including an empty invalid-format sentinel.
	Output string `json:"output"`
}

// WordEncodeCase preserves all byte-valued encoder selectors rather than just the two named constants.
type WordEncodeCase struct {
	// Encoding is the complete public uint8-valued selector.
	Encoding uint8 `json:"encoding"`
	// Charset is the unvalidated original charset label in hexadecimal.
	Charset string `json:"charset"`
	// Input is the original arbitrary-byte payload in hexadecimal.
	Input string `json:"input"`
	// Output is the complete encoded result in hexadecimal.
	Output string `json:"output"`
}

// CallbackRead captures one converted-reader request and its exact progress/error.
type CallbackRead struct {
	// Request is the destination capacity received by the callback reader.
	Request int `json:"request"`
	// Output is the returned byte prefix in hexadecimal.
	Output string `json:"output"`
	// Error is the simultaneous diagnostic, empty on success.
	Error string `json:"error"`
}

// CallbackEvent records lowercase labels, callback input bytes, and converted-reader use.
type CallbackEvent struct {
	// Charset is the exact lowercase label received by the callback.
	Charset string `json:"charset"`
	// Input is the complete decoded input supplied to the callback.
	Input string `json:"input"`
	// Reads is the sequence observed by a non-WriterTo converted reader.
	Reads []CallbackRead `json:"reads"`
	// Copies observes optional complete-copy calls instead of pull reads.
	Copies []CallbackRead `json:"copies"`
}

// WordDecodeCase covers word versus header rules and explicit callback behavior.
type WordDecodeCase struct {
	// Header selects the header method rather than single-word decoding.
	Header bool `json:"header"`
	// Input is the exact encoded header/word in hexadecimal.
	Input string `json:"input"`
	// Callback selects a deterministic reader/error policy; none leaves the field nil.
	Callback string `json:"callback"`
	// Output is the exact returned bytes, empty whenever the method reports an error.
	Output string `json:"output"`
	// Error is the complete public diagnostic.
	Error string `json:"error"`
	// Events preserves callback invocation order and read behavior.
	Events []*CallbackEvent `json:"events"`
}

// RegistryStep records one explicit-context-equivalent association or query.
type RegistryStep struct {
	// Operation is add, type, or extensions.
	Operation string `json:"operation"`
	// Extension is an exact extension key in hexadecimal.
	Extension string `json:"extension"`
	// Type is the exact supplied media type in hexadecimal.
	Type string `json:"type"`
	// Output is the exact lookup result in hexadecimal.
	Output string `json:"output"`
	// Extensions is a sorted list of returned hexadecimal keys; null is distinct from nonempty.
	Extensions []string `json:"extensions"`
	// Error is the exact reported diagnostic.
	Error string `json:"error"`
}

// WriteCase freezes a complete public encode/decode pipeline followed by a compliant short write.
type WriteCase struct {
	// Header selects complete header decoding before the sink is touched.
	Header bool `json:"header"`
	// Encoding is the raw encoder selector when Header is false.
	Encoding uint8 `json:"encoding"`
	// Charset is the exact encoder label in hexadecimal.
	Charset string `json:"charset"`
	// Input is the complete original payload/header in hexadecimal.
	Input string `json:"input"`
	// Budget is negative for unlimited success or the exact accepted prefix budget.
	Budget int `json:"budget"`
	// Output is the complete externally accepted prefix in hexadecimal.
	Output string `json:"output"`
	// Error is a conversion failure or a sink failure, preserving the first error.
	Error string `json:"error"`
}

// shortSink complies with the writer contract by returning an error for a partial acceptance.
type shortSink struct {
	// Data owns all accepted bytes.
	data []byte
	// Budget caps the accepted prefix, or is negative for unlimited acceptance.
	budget int
}

// Write records one exact accepted prefix and returns a stable failure when short.
func (sink *shortSink) Write(input []byte) (int, error) {
	n := len(input)
	if sink.budget >= 0 && n > sink.budget-len(sink.data) {
		n = sink.budget - len(sink.data)
	}
	if n < 0 {
		n = 0
	}
	sink.data = append(sink.data, input[:n]...)
	if n < len(input) {
		return n, errors.New("fixture sink failure")
	}
	return n, nil
}

// Corpus groups independently executable media, word and registry observations.
type Corpus struct {
	// Media contains parse results and partial errors.
	Media []MediaCase `json:"media"`
	// Formats contains complete deterministic serialization results.
	Formats []FormatCase `json:"formats"`
	// Encoded contains complete byte-valued encoder observations.
	Encoded []WordEncodeCase `json:"encoded"`
	// Decoded contains header/word/callback traces.
	Decoded []WordDecodeCase `json:"decoded"`
	// Registry is an ordered trace over uniquely named associations, independent of host databases.
	Registry []RegistryStep `json:"registry"`
	// Writes preserves complete codec and short-output error/progress observations.
	Writes []WriteCase `json:"writes"`
}

// hx retains byte strings without JSON's UTF-8 repair behavior.
func hx(s string) string { return hex.EncodeToString([]byte(s)) }

// errText preserves exact errors and a simple success sentinel.
func errText(err error) string {
	if err == nil {
		return ""
	}
	return err.Error()
}

// pairs freezes a nullable map and sorts its original raw keys deterministically.
func pairs(m map[string]string) []Pair {
	if m == nil {
		return nil
	}
	out := []Pair{}
	keys := make([]string, 0, len(m))
	for k := range m {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	for _, k := range keys {
		out = append(out, Pair{hx(k), hx(m[k])})
	}
	return out
}

// tracedReader exposes a compliant bounded reader without optional copy fast paths.
type tracedReader struct {
	// Content is the private converted byte sequence.
	content []byte
	// At is the exact advanced position.
	at int
	// Chunk caps positive successful reads.
	chunk int
	// Fail selects a terminal failure after the converted prefix.
	fail bool
	// Event receives every request/result observation.
	event *CallbackEvent
	// PrefixError returns the final positive prefix alongside failure.
	prefixError bool
	// PrefixEOF returns the final positive prefix alongside EOF.
	prefixEOF bool
	// Gap inserts one legal empty nonterminal read.
	gap bool
}

// Read returns positive bytes with nil error and records a later distinct EOF/failure.
func (r *tracedReader) Read(p []byte) (n int, err error) {
	if r.gap {
		r.gap = false
	} else if r.at == len(r.content) {
		if r.fail {
			err = errors.New("fixture converted-reader failure")
		} else {
			err = io.EOF
		}
	} else {
		n = len(p)
		if n > r.chunk {
			n = r.chunk
		}
		if n > len(r.content)-r.at {
			n = len(r.content) - r.at
		}
		copy(p, r.content[r.at:r.at+n])
		r.at += n
		if r.prefixError && r.at == len(r.content) {
			err = errors.New("fixture converted-reader failure")
		} else if r.prefixEOF && r.at == len(r.content) {
			err = io.EOF
		}
	}
	r.event.Reads = append(r.event.Reads, CallbackRead{len(p), hex.EncodeToString(p[:n]), errText(err)})
	return
}

// copyReader exposes a direct complete-copy capability whose effects remain observable.
type copyReader struct {
	// Source retains the fallback read capability.
	source *tracedReader
	// Failure selects a whole-copy error after writing the complete prefix.
	failure bool
}

// Read is the ordinary fallback and should remain unused when the complete-copy path exists.
func (r *copyReader) Read(p []byte) (int, error) { return r.source.Read(p) }

// WriteTo records the optional copy operation and can fail after a positive written prefix.
func (r *copyReader) WriteTo(writer io.Writer) (int64, error) {
	n, err := writer.Write(r.source.content[r.source.at:])
	r.source.at += n
	if err == nil && r.failure {
		err = errors.New("fixture converted-reader failure")
	}
	r.source.event.Copies = append(r.source.event.Copies, CallbackRead{0, hex.EncodeToString(r.source.content[r.source.at-n : r.source.at]), errText(err)})
	return int64(n), err
}

// decodeCase installs only public callback state and freezes complete observable progress.
func decodeCase(input string, header bool, policy string) WordDecodeCase {
	c := WordDecodeCase{Header: header, Input: hx(input), Callback: policy, Events: []*CallbackEvent{}}
	d := mime.WordDecoder{}
	if policy != "none" {
		d.CharsetReader = func(charset string, input io.Reader) (io.Reader, error) {
			data, err := io.ReadAll(input)
			if err != nil {
				panic(err)
			}
			event := &CallbackEvent{Charset: hx(charset), Input: hex.EncodeToString(data), Reads: []CallbackRead{}, Copies: []CallbackRead{}}
			c.Events = append(c.Events, event)
			if policy == "error" {
				return nil, errors.New("fixture charset callback failure")
			}
			if policy == "both-error" {
				return bytes.NewReader([]byte("unused")), errors.New("fixture charset callback failure")
			}
			if policy == "invalid-utf8" {
				data = []byte{0xff, 0xfe, 0}
			}
			reader := &tracedReader{content: data, chunk: 2, fail: policy == "read-error", prefixError: policy == "prefix-error", prefixEOF: policy == "prefix-eof", gap: policy == "empty-more", event: event}
			if policy == "copy-fast" || policy == "copy-fast-error" {
				return &copyReader{source: reader, failure: policy == "copy-fast-error"}, nil
			}
			return reader, nil
		}
	}
	var out string
	var err error
	if header {
		out, err = d.DecodeHeader(input)
	} else {
		out, err = d.Decode(input)
	}
	c.Output = hx(out)
	c.Error = errText(err)
	return c
}

// fixtures covers byte boundaries, liberal syntax, Unicode casing and callback/data ownership observations.
func fixtures() Corpus {
	c := Corpus{}
	media := []string{"", " ", "text", "TEXT/PLAIN", " attachment ", "text/plain; charset=UTF-8", "text/plain;", "text/plain; \t", "text/plain;;", "text /plain", "text/", "text/plain junk", "text/plain; a=1; A=1", "text/plain; a=1; A=2", "text/plain; a=\"\"", "text/plain; a=\"C:\\dev\\file.txt\"", "text/plain; a=\"x\\\"y\"", "text/plain; a=\"x\ry\"", "text/plain; a=\"\x00\xff\"", "text/plain; title*=utf-8'en'%E2%82%AC", "text/plain; title*=x-unknown''abc; title=plain", "text/plain; title*0*=utf-8''a%20; title*1=b; title*2*=%63", "text/plain; title*0*=bad''lost; title*1*=%62", "text/plain; title*0=a; title*0*=utf-8''b; title*1=c", "text/plain; title*=utf-8''single; title*0=numbered", "text/plain; title*0=a; title*2=c", "text/plain; title*01=b; title*1=c", "text/plain; title*abc=ignored", "text/plain; title*=bad''single; title*0=fallback; title=plain", "text/plain; title*0*=utf-8''bad%Q0; title*1=b", "text/plain; title*0*=utf-8''a; title*1*=%Q0; title*2=c", "text/plain; title*0=a; TITLE*0=b", "text/plain; title*=utf-8''a; TITLE*=utf-8''b", "text/plain; a=1; a=1; bad", "text/plain; a=\"\\a\"", "text/plain; a=\"\\;\"", "text/plain; *=utf-8''empty-key", "\u212a/plain; A=b", "\u00a0TEXT/PLAIN\u2003;\u0085A\u2009=\u3000b"}
	for b := 0; b < 256; b++ {
		media = append(media, "x/x; a=\""+string([]byte{byte(b)})+"\"", "x/x; a*=utf-8''%"+fmt.Sprintf("%02X", b))
	}
	for _, s := range media {
		base, p, e := mime.ParseMediaType(s)
		c.Media = append(c.Media, MediaCase{hx(s), hx(base), pairs(p), errText(e)})
	}
	bases := []string{"Text/Plain", "attachment", "bad type", "text/", "\u212a/plain", "x/x"}
	maps := []map[string]string{nil, {}, {"Z": "", "A": "a b", "a": "other"}, {"file": "a\"b\\c"}, {"name": "café;雪"}, {"name": "\t"}, {"name": "\x00\xff"}, {"bad key": "v"}, {"": "v"}, {"x*": "a%b'c"}}
	for _, base := range bases {
		for _, p := range maps {
			c.Formats = append(c.Formats, FormatCase{hx(base), pairs(p), hx(mime.FormatMediaType(base, p))})
		}
	}
	all := make([]byte, 256)
	for i := range all {
		all[i] = byte(i)
	}
	inputs := []string{"", "plain =?ASCII?=\t", "café 雪", "\x00\xff", string(all)}
	for _, n := range []int{1, 15, 16, 17, 44, 45, 46, 62, 63, 64, 74, 75, 76, 149, 150} {
		inputs = append(inputs, strings.Repeat("a", n)+"é", strings.Repeat("雪", n))
	}
	for _, charset := range []string{"UTF-8", "uTf-8", "iso-8859-1", "US-ASCII", "x-custom", "", "odd?label", "\xff"} {
		for _, s := range inputs {
			for _, e := range []byte{'b', 'q'} {
				c.Encoded = append(c.Encoded, WordEncodeCase{e, hx(charset), hx(s), hx(mime.WordEncoder(e).Encode(charset, s))})
			}
		}
	}
	for e := 0; e < 256; e++ {
		c.Encoded = append(c.Encoded, WordEncodeCase{byte(e), hx("UTF-8"), hx("é=?_\t"), hx(mime.WordEncoder(byte(e)).Encode("UTF-8", "é=?_\t"))})
	}
	words := []string{"", "plain", "=?UTF-8?q?hello_world?=", "=?UTF-8?Q?=c3=a9?=", "=?UTF-8?b?YQ==?=", "=?UTF-8?B?YQ==?=", "=?UTF-8?b??=", "=??q?text?=", "=?UTF-8?x?bad?=", "=?UTF-8?q?=GG?=", "=?UTF-8?q?=?=", "=?UTF-8?q?a?b?=", "=?UTF-8?q?\xff?=", "=?UTF-8?q?=FF=00?=", "=?iso-8859-1?q?=80=FF?=", "=?US-ASCII?q?=80=FF?=", "=?uſ-aſcii?q?=80?=", "=?IſO-8859-1?q?=E9?=", "=?X-CUSTOM?q?a_b?=", "=?x-custom?q?ab=FF?=", "=?x-custom?q??=", "=?UTF-8?b?YQ=?=", "=?UTF-8?b?YQ=\n?=", "=?UTF-8?b?YQ=\nA?=", "=?UTF-8?b?a\r\n?=", "=?UTF-8?b?abc?=", "=?UTF-8?b?YQ==x?="}
	for b := 0; b < 256; b++ {
		words = append(words, "=?iso-8859-1?q?="+fmt.Sprintf("%02X", b)+"?=", "=?US-ASCII?q?="+fmt.Sprintf("%02x", b)+"?=")
	}
	for _, label := range []string{"\xff", "\xfe", "\x00", "\u0085", "\u00a0", "\u200b", "\ufffd", "雪", "😀", "X-\xff-CASE"} {
		words = append(words, "=?"+label+"?q?a?=")
		c.Decoded = append(c.Decoded, decodeCase("=?"+label+"?q?a?=", false, "copy"))
	}
	for _, body := range []string{"A", "AA", "AAA", "AAAA", "=", "==", "A=", "AA=", "AAA=", "AA==", "AAAA=", "AA===", "AA==\r\n", "AA=\r\n=", "AA=\r\nA", "AA==\r\nx", "A\r\n", "AA\r\n", "AAA\r\n", "AA==?", "AA==\x00", "AA==\xff"} {
		words = append(words, "=?UTF-8?b?"+body+"?=")
	}
	for _, value := range []byte{0, 9, 10, 13, 31, 32, 33, 61, 63, 64, 65, 90, 95, 96, 122, 126, 127, 128, 255} {
		words = append(words, "=?UTF-8?q?"+string([]byte{value})+"?=", "=?UTF-8?q?="+string([]byte{value})+"0?=", "=?UTF-8?q?=0"+string([]byte{value})+"?=")
		for at := 0; at < 4; at++ {
			body := []byte("YQ==")
			body[at] = value
			words = append(words, "=?UTF-8?b?"+string(body)+"?=")
		}
	}
	headers := []string{"plain untouched", "before =?UTF-8?q?a?= \t\r\n =?UTF-8?q?b?= after", "=?UTF-8?q?a?=\v=?UTF-8?q?b?=", "=?UTF-8?q?a?=  =?UTF-8?q?=GG?=  =?UTF-8?q?b?=", "=?broken before =?UTF-8?q?good?=", "=?UTF-8?q?a?b?= next", "=??q?text?=", "=?X-CUSTOM?q?a?= =?x-custom?q?b?=", "=?UTF-8?q?good?= =?X-CUSTOM?q?bad?= later", "literal \xff =?UTF-8?q?=FF?= trailing"}
	for _, word := range words {
		c.Decoded = append(c.Decoded, decodeCase(word, false, "none"), decodeCase(word, true, "none"))
	}
	for _, s := range headers {
		for _, policy := range []string{"none", "copy", "error", "both-error", "read-error", "invalid-utf8", "prefix-error", "prefix-eof", "empty-more", "copy-fast", "copy-fast-error"} {
			c.Decoded = append(c.Decoded, decodeCase(s, true, policy))
		}
	}
	for _, policy := range []string{"copy", "error", "both-error", "read-error", "invalid-utf8", "prefix-error", "prefix-eof", "empty-more", "copy-fast", "copy-fast-error"} {
		c.Decoded = append(c.Decoded, decodeCase("=?X-CUSTOM?q?abcdef?=", false, policy))
	}
	unique := "application/x-cinch-mime-context-"
	operations := [][3]string{{"add", ".cinch-mime-a", unique + "a"}, {"type", ".CINCH-MIME-A", ""}, {"add", ".CINCH-MIME-A", unique + "b"}, {"type", ".cinch-mime-a", ""}, {"type", ".CiNcH-MiMe-A", ""}, {"extensions", "", unique + "a"}, {"extensions", "", unique + "b"}, {"add", ".cinch-mime-text", "text/x-cinch-mime"}, {"type", ".cinch-mime-text", ""}, {"add", ".cinch-mime-TEXT", "Text/x-cinch-mime; Name=Keep"}, {"type", ".cinch-mime-TEXT", ""}, {"extensions", "", "TEXT/X-CINCH-MIME; q=x"}, {"add", "bad\xff", "bad type"}, {"add", ".cinch-mime-bad", "bad type"}, {"extensions", "", "text/plain; a=1; a=2"}, {"add", ".cinch-mime-\xff", unique + "invalid"}, {"type", ".cinch-mime-\xff", ""}, {"type", ".cinch-mime-\xfe", ""}}
	for _, op := range operations {
		r := RegistryStep{Operation: op[0], Extension: hx(op[1]), Type: hx(op[2])}
		switch op[0] {
		case "add":
			r.Error = errText(mime.AddExtensionType(op[1], op[2]))
		case "type":
			r.Output = hx(mime.TypeByExtension(op[1]))
		case "extensions":
			exts, e := mime.ExtensionsByType(op[2])
			r.Error = errText(e)
			for _, ext := range exts {
				r.Extensions = append(r.Extensions, hx(ext))
			}
		}
		c.Registry = append(c.Registry, r)
	}
	return c
}

// writeCases covers short-output progress and conversion-before-output ordering.
func writeCases() []WriteCase {
	out := []WriteCase{}
	all := make([]byte, 256)
	for i := range all {
		all[i] = byte(i)
	}
	for _, input := range []string{"", "plain", "café 雪", strings.Repeat("a", 74) + "é", string(all)} {
		for _, encoder := range []byte{'b', 'q'} {
			for _, charset := range []string{"UTF-8", "iso-8859-1"} {
				encoded := mime.WordEncoder(encoder).Encode(charset, input)
				for _, budget := range []int{-1, 0, 1, 5, len(encoded) - 1, len(encoded)} {
					sink := &shortSink{budget: budget}
					_, err := sink.Write([]byte(encoded))
					out = append(out, WriteCase{false, encoder, hx(charset), hx(input), budget, hex.EncodeToString(sink.data), errText(err)})
				}
			}
		}
	}
	for _, input := range []string{"plain", "=?UTF-8?q?café?=", "=?UTF-8?q?a?= =?UTF-8?q?b?=", "=?UTF-8?q?good?= =?X-CUSTOM?q?bad?="} {
		for _, budget := range []int{-1, 0, 1, 5} {
			sink := &shortSink{budget: budget}
			decoded, err := new(mime.WordDecoder).DecodeHeader(input)
			if err == nil {
				_, err = sink.Write([]byte(decoded))
			}
			out = append(out, WriteCase{true, 0, "", hx(input), budget, hex.EncodeToString(sink.data), errText(err)})
		}
	}
	return out
}

// main freezes public observations; this generator never needs implementation package access.
func main() {
	corpus := fixtures()
	corpus.Writes = writeCases()
	json.NewEncoder(os.Stdout).Encode(corpus)
}
