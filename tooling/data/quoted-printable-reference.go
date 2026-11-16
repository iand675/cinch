// Independent quoted-printable fixtures use only the pinned public SDK package.
package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"mime/quotedprintable"
	"os"
	"strconv"
	"strings"
	"time"
)

// ByteCase freezes complete ordinary and binary encodings and partial decoding outcomes.
type ByteCase struct {
	// Input Original bytes in hexadecimal.
	Input string `json:"input"`
	// Text Complete normalized text encoding in hexadecimal.
	Text string `json:"text"`
	// Binary Complete binary-policy encoding or mode selection.
	Binary string `json:"binary"`
	// Decoded Successfully decoded prefix in hexadecimal.
	Decoded string `json:"decoded"`
	// Error Exact independently observed diagnostic, empty on success.
	Error string `json:"error"`
}

// ReadStep records output plus a simultaneously returned diagnostic and source read-ahead progress.
type ReadStep struct {
	// Request Requested public read capacity.
	Request int `json:"request"`
	// Output Exact output bytes in hexadecimal.
	Output string `json:"output"`
	// Error Exact independently observed diagnostic, empty on success.
	Error string `json:"error"`
	// Source Underlying input bytes consumed, including read-ahead.
	Source int `json:"source"`
}

// ReadCase freezes request-sized decoding against an independently chunked source.
type ReadCase struct {
	// Input Original bytes in hexadecimal.
	Input string `json:"input"`
	// Chunk Maximum positive source-read chunk.
	Chunk int `json:"chunk"`
	// FailAt Exact source offset after which a distinct failure is injected.
	FailAt int `json:"fail_at"`
	// Steps Ordered independent observations.
	Steps []ReadStep `json:"steps"`
}

// WriteStep records each call's accepted input count and immediately observable sink bytes.
type WriteStep struct {
	// Input Original bytes in hexadecimal.
	Input string `json:"input"`
	// Binary Complete binary-policy encoding or mode selection.
	Binary bool `json:"binary"`
	// Accepted Reportable input prefix, independent of accepted output.
	Accepted int `json:"accepted"`
	// Error Exact independently observed diagnostic, empty on success.
	Error string `json:"error"`
	// Output Exact output bytes in hexadecimal.
	Output string `json:"output"`
}

// WriteCase records explicit mode updates, a compliant failing sink and finish behavior.
type WriteCase struct {
	// Limit Negative for unlimited output or the exact sink-acceptance budget.
	Limit int `json:"limit"`
	// Steps Ordered independent observations.
	Steps []WriteStep `json:"steps"`
	// CloseError Final flush diagnostic when writes completed.
	CloseError string `json:"close_error"`
	// Output Exact output bytes in hexadecimal.
	Output string `json:"output"`
	// SinkClosed Whether the borrowed sink close method was invoked.
	SinkClosed bool `json:"sink_closed"`
}

// Corpus groups independent value, read and writer-progress fixtures.
type Corpus struct {
	// Values Whole-value and partial-error cases.
	Values []ByteCase `json:"values"`
	// Readers Request/chunk/source-failure traces.
	Readers []ReadCase `json:"readers"`
	// Writers Mode/write/finish/sink-failure traces.
	Writers []WriteCase `json:"writers"`
}

// errorText keeps exact diagnostics while representing success as an empty string.
func errorText(err error) string {
	if err == nil {
		return ""
	}
	return err.Error()
}

// encoded emits a complete public writer result with the selected newline policy.
func encoded(input []byte, binary bool) []byte {
	var out bytes.Buffer
	w := quotedprintable.NewWriter(&out)
	w.Binary = binary
	if _, err := w.Write(input); err != nil {
		panic(err)
	}
	if err := w.Close(); err != nil {
		panic(err)
	}
	return out.Bytes()
}

// chunkReader models ordered partial input followed by EOF or a distinct source failure.
type chunkReader struct {
	input  []byte
	at     int
	chunk  int
	failAt int
}

// Read advances at most one configured source chunk without returning bytes and error together.
func (r *chunkReader) Read(p []byte) (int, error) {
	if r.failAt >= 0 && r.at >= r.failAt {
		return 0, errors.New("fixture source failure")
	}
	if r.at == len(r.input) {
		return 0, io.EOF
	}
	n := len(p)
	if n > r.chunk {
		n = r.chunk
	}
	if n > len(r.input)-r.at {
		n = len(r.input) - r.at
	}
	if r.failAt >= 0 && n > r.failAt-r.at {
		n = r.failAt - r.at
	}
	copy(p, r.input[r.at:r.at+n])
	r.at += n
	return n, nil
}

// sink records accepted output and gives short writes their required non-nil error.
type sink struct {
	bytes.Buffer
	limit  int
	closed bool
}

// Write either accepts all bytes or records the accepted prefix before a stable failure.
func (w *sink) Write(p []byte) (int, error) {
	if w.limit < 0 {
		return w.Buffer.Write(p)
	}
	left := w.limit - w.Len()
	if left < 0 {
		left = 0
	}
	if len(p) <= left {
		return w.Buffer.Write(p)
	}
	w.Buffer.Write(p[:left])
	return left, errors.New("fixture sink failure")
}

// Close would expose an erroneous attempt to close a borrowed sink.
func (w *sink) Close() error { w.closed = true; return nil }

// readCase observes zero reads, short reads, terminal errors and repeated errors.
func readCase(input []byte, chunk, failAt int, requests []int) ReadCase {
	r := &chunkReader{input: input, chunk: chunk, failAt: failAt}
	q := quotedprintable.NewReader(r)
	out := ReadCase{Input: hex.EncodeToString(input), Chunk: chunk, FailAt: failAt}
	for _, size := range requests {
		p := make([]byte, size)
		n, err := q.Read(p)
		out.Steps = append(out.Steps, ReadStep{size, hex.EncodeToString(p[:n]), errorText(err), r.at})
	}
	return out
}

// writeCase freezes source-call progress; the trace stops after a write failure to avoid replaying output.
func writeCase(parts [][]byte, modes []bool, limit int) WriteCase {
	out := &sink{limit: limit}
	w := quotedprintable.NewWriter(out)
	c := WriteCase{Limit: limit}
	failed := false
	for i, p := range parts {
		w.Binary = modes[i%len(modes)]
		n, err := w.Write(p)
		c.Steps = append(c.Steps, WriteStep{hex.EncodeToString(p), w.Binary, n, errorText(err), hex.EncodeToString(out.Bytes())})
		if err != nil {
			failed = true
			break
		}
	}
	if !failed {
		c.CloseError = errorText(w.Close())
	}
	c.Output = hex.EncodeToString(out.Bytes())
	c.SinkClosed = out.closed
	return c
}

// fixtures includes exhaustive byte classification and line, escape, error and chunk boundaries.
func fixtures() Corpus {
	c := Corpus{}
	inputs := [][]byte{nil, []byte("foo bar"), []byte("foo=\rbar"), []byte("foo=\r\r\r \nbar"), []byte("="), []byte("a="), []byte("= "), []byte("a= \t"), []byte("=A"), []byte("==\n"), []byte("=G0"), []byte("=0G"), []byte("=\rX\n"), []byte("foo= \r\nbar"), []byte("foo\x00bar"), []byte("foo\x7fbar"), []byte("foo\xffbar"), []byte("a \t\r\n b \t"), []byte("\r\n\r\r\n\n"), []byte("¡Hola Señor! 雪 😀")}
	for i := 0; i < 256; i++ {
		inputs = append(inputs, []byte{byte(i)})
	}
	all := make([]byte, 256)
	for i := range all {
		all[i] = byte(i)
	}
	inputs = append(inputs, all)
	for i := 0; i < 256; i++ {
		inputs = append(inputs, []byte(fmt.Sprintf("=%02X", i)), []byte(fmt.Sprintf("=%02x", i)))
		inputs = append(inputs, []byte{'p', '=', byte(i), '0', 'q'}, []byte{'p', '=', '0', byte(i), 'q'})
	}
	inputs = append(inputs, encoded(all, true))
	for _, n := range []int{7, 8, 15, 16, 17, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 149, 150, 151, 4095, 4096, 4097, 8191} {
		inputs = append(inputs, []byte(strings.Repeat("a", n)), []byte(strings.Repeat(" ", n)), []byte(strings.Repeat("a", n)+"=\r\n"))
	}
	for _, p := range inputs {
		decoded, err := io.ReadAll(quotedprintable.NewReader(bytes.NewReader(p)))
		c.Values = append(c.Values, ByteCase{hex.EncodeToString(p), hex.EncodeToString(encoded(p, false)), hex.EncodeToString(encoded(p, true)), hex.EncodeToString(decoded), errorText(err)})
	}
	traces := [][]byte{[]byte("a=3Db=\r\nc \t\r\nd="), []byte("foo\x00bar"), []byte("foo=\rbar"), []byte("==\n"), []byte("=\r\r\n"), []byte("a= \t"), []byte(strings.Repeat("x", 4095) + "\n"), []byte(strings.Repeat("x", 4096)), []byte(strings.Repeat("x", 4096) + "\n")}
	for _, p := range traces {
		for _, chunk := range []int{1, 2, 7, 4096} {
			c.Readers = append(c.Readers, readCase(p, chunk, -1, []int{0, 1, 0, 2, 7, 4096, 4096, 1, 0}))
		}
	}
	for _, failAt := range []int{0, 1, 2, 5, 10} {
		c.Readers = append(c.Readers, readCase([]byte("a=3Db=\r\nxyz"), 2, failAt, []int{1, 2, 7, 1, 0}))
	}
	parts := [][][]byte{{[]byte("foo\r"), []byte("\nbar \t"), nil}, {[]byte(strings.Repeat("a", 74)), []byte("=\n"), []byte("end ")}, {[]byte("\r"), []byte("\n"), []byte("\n")}, {all[:75], all[75:150], all[150:]}}
	for _, p := range parts {
		for _, modes := range [][]bool{{false}, {true}, {false, true, false}} {
			for _, limit := range []int{-1, 0, 5, 76, 80} {
				c.Writers = append(c.Writers, writeCase(p, modes, limit))
			}
		}
	}
	return c
}

// main emits independent fixtures or times complete public encode/decode pipelines.
func main() {
	if len(os.Args) == 1 {
		json.NewEncoder(os.Stdout).Encode(fixtures())
		return
	}
	mode := os.Args[1]
	size, _ := strconv.Atoi(os.Args[2])
	iterations, _ := strconv.Atoi(os.Args[3])
	input := make([]byte, size)
	for i := range input {
		input[i] = byte((i*37 + 11) % 256)
	}
	if strings.HasPrefix(mode, "ascii") {
		for i := range input {
			input[i] = byte(33 + i%90)
		}
	}
	if strings.HasSuffix(mode, "decode") {
		input = encoded(input, true)
	}
	checksum := uint64(0)
	start := time.Now()
	for i := 0; i < iterations; i++ {
		var output []byte
		if strings.HasSuffix(mode, "decode") {
			var err error
			output, err = io.ReadAll(quotedprintable.NewReader(bytes.NewReader(input)))
			if err != nil {
				panic(err)
			}
		} else {
			output = encoded(input, strings.HasPrefix(mode, "binary"))
		}
		for _, b := range output {
			checksum = (checksum*131 + uint64(b)) & 2147483647
		}
	}
	elapsed := time.Since(start).Nanoseconds()
	var retained uint64
	for _, b := range input {
		retained = (retained*131 + uint64(b)) & 2147483647
	}
	fmt.Println(elapsed, checksum, retained)
}
