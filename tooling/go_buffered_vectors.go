// Generate independent buffered reader, writer, splitter and scanner observations from the pinned SDK.
package main

import (
	"bufio"
	"bytes"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"runtime"
)

// operation records a public action without embedding any implementation algorithm.
type operation struct {
	Name string `json:"name"`
	N    int    `json:"n,omitempty"`
	Data string `json:"data,omitempty"`
}

// observation preserves partial results, errors, panic boundaries and complete raw byte outputs.
type observation struct {
	Name      string   `json:"name"`
	Count     int      `json:"count"`
	Data      string   `json:"data"`
	Rune      int32    `json:"rune"`
	Width     int      `json:"width"`
	Prefix    bool     `json:"prefix"`
	Error     string   `json:"error"`
	Message   string   `json:"message"`
	Panic     string   `json:"panic"`
	Size      int      `json:"size"`
	Buffered  int      `json:"buffered"`
	Available int      `json:"available"`
	Capacity  int      `json:"capacity"`
	Nil       bool     `json:"nil"`
	More      bool     `json:"more"`
	Output    string   `json:"output"`
	Calls     []string `json:"calls"`
}

// readerCase retains initialization, transport behavior and each history-sensitive public result.
type readerCase struct {
	Input    string        `json:"input"`
	Size     int           `json:"size"`
	Chunk    int           `json:"chunk"`
	Together bool          `json:"together"`
	Empties  int           `json:"empties"`
	Fault    int           `json:"fault"`
	Fast     bool          `json:"fast"`
	Steps    []operation   `json:"steps"`
	Expected []observation `json:"expected"`
}

// writerCase retains deterministic byte acceptance, sticky failures and reservation observations.
type writerCase struct {
	Size     int           `json:"size"`
	Limit    int           `json:"limit"`
	Budget   int           `json:"budget"`
	Fault    int           `json:"fault"`
	Steps    []operation   `json:"steps"`
	Expected []observation `json:"expected"`
}

// splitCase keeps nil and empty tokens distinct for direct split-function observations.
type splitCase struct {
	Kind    string `json:"kind"`
	Input   string `json:"input"`
	EOF     bool   `json:"eof"`
	Advance int    `json:"advance"`
	Token   string `json:"token"`
	Nil     bool   `json:"nil"`
	Error   string `json:"error"`
	Panic   string `json:"panic"`
}

// scannerCase records complete token/error sequences under bounded chunks and terminal data timing.
type scannerCase struct {
	Kind     string        `json:"kind"`
	Input    string        `json:"input"`
	Chunk    int           `json:"chunk"`
	Together bool          `json:"together"`
	Capacity int           `json:"capacity"`
	Maximum  int           `json:"maximum"`
	Expected []observation `json:"expected"`
}

// source owns a deterministic transport cursor and records every actual requested read width.
type source struct {
	data                      []byte
	at, chunk, empties, fault int
	together                  bool
	calls                     []string
}

// Read implements controlled short, empty, simultaneous-terminal and malformed-count source events.
func (s *source) Read(p []byte) (int, error) {
	s.calls = append(s.calls, fmt.Sprintf("read:%d", len(p)))
	if s.fault < 0 {
		return -1, nil
	}
	if s.fault > 0 {
		return len(p) + 1, nil
	}
	if s.empties > 0 {
		s.empties--
		return 0, nil
	}
	if len(p) == 0 {
		return 0, nil
	}
	if s.at == len(s.data) {
		return 0, io.EOF
	}
	n := len(s.data) - s.at
	if n > len(p) {
		n = len(p)
	}
	if s.chunk > 0 && n > s.chunk {
		n = s.chunk
	}
	copy(p, s.data[s.at:s.at+n])
	s.at += n
	if s.together && s.at == len(s.data) {
		return n, io.EOF
	}
	return n, nil
}

// fastSource exposes an independently observable direct transfer capability.
type fastSource struct{ *source }

// WriteTo reports dispatch explicitly and moves the remaining source through the supplied writer.
func (s fastSource) WriteTo(w io.Writer) (int64, error) {
	s.calls = append(s.calls, "write-to")
	n, e := w.Write(s.data[s.at:])
	s.at += n
	return int64(n), e
}

// sink accepts a controlled prefix and records callback widths without claiming unavailable capacity.
type sink struct {
	output               bytes.Buffer
	limit, budget, fault int
	calls                []string
}

// Write implements positive short acceptance, simultaneous failure and invalid reported counts.
func (s *sink) Write(p []byte) (int, error) {
	s.calls = append(s.calls, fmt.Sprintf("write:%d", len(p)))
	// Bound only a deliberately invalid writer contract; the panic marker belongs to this observer.
	if s.limit == 0 && len(p) > 0 && len(s.calls) > 100 {
		panic("fixture zero-progress writer watchdog")
	}
	if s.fault < 0 {
		return -1, nil
	}
	if s.fault > 0 {
		return len(p) + 1, nil
	}
	n := len(p)
	if s.limit >= 0 && n > s.limit {
		n = s.limit
	}
	if s.budget >= 0 && n > s.budget {
		n = s.budget
	}
	s.output.Write(p[:n])
	if s.budget >= 0 {
		s.budget -= n
		if n < len(p) {
			return n, errors.New("fixture-sink")
		}
	}
	return n, nil
}

// encoded preserves malformed UTF-8 instead of letting JSON repair a string field.
func encoded(p []byte) string { return hex.EncodeToString(p) }

// decoded reconstructs only generator-owned hexadecimal literals and rejects malformed fixture metadata.
func decoded(s string) []byte {
	p, e := hex.DecodeString(s)
	if e != nil {
		panic(e)
	}
	return p
}

// errorCode identifies standard error capabilities independently of their presentation text.
func errorCode(e error) string {
	if e == nil {
		return "none"
	}
	if errors.Is(e, io.EOF) {
		return "end"
	}
	if errors.Is(e, io.ErrNoProgress) {
		return "no-progress"
	}
	if errors.Is(e, io.ErrShortWrite) {
		return "short-write"
	}
	for _, v := range []struct {
		e error
		n string
	}{{bufio.ErrBufferFull, "buffer-full"}, {bufio.ErrNegativeCount, "negative-count"}, {bufio.ErrInvalidUnreadByte, "invalid-unread-byte"}, {bufio.ErrInvalidUnreadRune, "invalid-unread-rune"}, {bufio.ErrTooLong, "too-long"}, {bufio.ErrNegativeAdvance, "negative-advance"}, {bufio.ErrAdvanceTooFar, "advance-too-far"}, {bufio.ErrBadReadCount, "bad-read-count"}, {bufio.ErrFinalToken, "final-token"}} {
		if errors.Is(e, v.e) {
			return v.n
		}
	}
	return "custom"
}

// finishError keeps the original diagnostic bytes alongside its stable capability code.
func finishError(out *observation, e error) {
	out.Error = errorCode(e)
	if e != nil {
		out.Message = encoded([]byte(e.Error()))
	}
}

// readAction invokes one public method and retains panic boundaries without continuing corrupted state.
func readAction(r *bufio.Reader, s *source, op operation) (out observation) {
	out.Name = op.Name
	before := len(s.calls)
	defer func() {
		if p := recover(); p != nil {
			out.Panic = encoded([]byte(fmt.Sprint(p)))
		}
		out.Size = r.Size()
		out.Buffered = r.Buffered()
		out.Calls = append([]string{}, s.calls[before:]...)
	}()
	var p []byte
	var e error
	switch op.Name {
	case "size", "buffered":
	case "peek":
		p, e = r.Peek(op.N)
		out.Count = len(p)
	case "discard":
		out.Count, e = r.Discard(op.N)
	case "read":
		p = bytes.Repeat([]byte{0xa5}, op.N)
		out.Count, e = r.Read(p)
		if out.Count >= 0 && out.Count <= len(p) {
			p = p[:out.Count]
		}
	case "byte":
		var b byte
		b, e = r.ReadByte()
		out.Count = 1
		if e != nil {
			out.Count = 0
		}
		p = []byte{b}
	case "rune":
		out.Rune, out.Width, e = r.ReadRune()
		out.Count = out.Width
	case "unread-byte":
		e = r.UnreadByte()
	case "unread-rune":
		e = r.UnreadRune()
	case "slice":
		p, e = r.ReadSlice(byte(op.N))
		out.Count = len(p)
	case "bytes":
		p, e = r.ReadBytes(byte(op.N))
		out.Count = len(p)
	case "string":
		var text string
		text, e = r.ReadString(byte(op.N))
		p = []byte(text)
		out.Count = len(p)
	case "line":
		p, out.Prefix, e = r.ReadLine()
		out.Count = len(p)
	case "reset-self":
		r.Reset(r)
	case "reset":
		r.Reset(bytes.NewReader(decoded(op.Data)))
	case "write-to":
		dest := &sink{limit: -1, budget: -1}
		var count int64
		count, e = r.WriteTo(dest)
		out.Count = int(count)
		p = dest.output.Bytes()
	default:
		panic("unmapped reader operation " + op.Name)
	}
	out.Nil = p == nil
	out.Data = encoded(p)
	finishError(&out, e)
	return out
}

// observeReader runs an action sequence against a fresh receiver and transport instance.
func observeReader(c readerCase) readerCase {
	s := &source{data: decoded(c.Input), chunk: c.Chunk, together: c.Together, empties: c.Empties, fault: c.Fault}
	var input io.Reader = s
	if c.Fast {
		input = fastSource{s}
	}
	r := bufio.NewReaderSize(input, c.Size)
	for _, op := range c.Steps {
		out := readAction(r, s, op)
		c.Expected = append(c.Expected, out)
		if out.Panic != "" {
			break
		}
	}
	return c
}

// writeAction invokes one public writer method, including append-and-commit available-buffer usage.
func writeAction(w *bufio.Writer, s *sink, op operation) (out observation) {
	out.Name = op.Name
	before := len(s.calls)
	defer func() {
		if p := recover(); p != nil {
			out.Panic = encoded([]byte(fmt.Sprint(p)))
		}
		out.Size = w.Size()
		out.Buffered = w.Buffered()
		out.Available = w.Available()
		out.Output = encoded(s.output.Bytes())
		out.Calls = append([]string{}, s.calls[before:]...)
	}()
	var e error
	data := decoded(op.Data)
	switch op.Name {
	case "size", "buffered", "available":
	case "write":
		out.Count, e = w.Write(data)
	case "string":
		out.Count, e = w.WriteString(string(data))
	case "byte":
		e = w.WriteByte(byte(op.N))
		if e == nil {
			out.Count = 1
		}
	case "rune":
		out.Count, e = w.WriteRune(rune(op.N))
	case "flush":
		e = w.Flush()
	case "space":
		p := w.AvailableBuffer()
		out.Capacity = cap(p)
		out.Nil = p == nil
		p = append(p, data...)
		out.Count, e = w.Write(p)
	case "read-from":
		var count int64
		count, e = w.ReadFrom(bytes.NewReader(data))
		out.Count = int(count)
	case "reset-self":
		w.Reset(w)
	case "reset":
		w.Reset(s)
	default:
		panic("unmapped writer operation " + op.Name)
	}
	finishError(&out, e)
	return out
}

// observeWriter keeps every partial count and sticky error from an ordered writer action sequence.
func observeWriter(c writerCase) writerCase {
	s := &sink{limit: c.Limit, budget: c.Budget, fault: c.Fault}
	w := bufio.NewWriterSize(s, c.Size)
	for _, op := range c.Steps {
		out := writeAction(w, s, op)
		c.Expected = append(c.Expected, out)
		if out.Panic != "" {
			break
		}
	}
	return c
}

// splitter selects actual built-ins and explicit adversarial custom split callbacks.
func splitter(kind string) bufio.SplitFunc {
	switch kind {
	case "bytes":
		return bufio.ScanBytes
	case "lines":
		return bufio.ScanLines
	case "runes":
		return bufio.ScanRunes
	case "words":
		return bufio.ScanWords
	case "negative":
		return func(data []byte, eof bool) (int, []byte, error) { return -1, nil, nil }
	case "overadvance":
		return func(data []byte, eof bool) (int, []byte, error) { return len(data) + 1, nil, nil }
	case "final-nil":
		return func(data []byte, eof bool) (int, []byte, error) { return 0, nil, bufio.ErrFinalToken }
	case "final-empty":
		return func(data []byte, eof bool) (int, []byte, error) { return 0, []byte{}, bufio.ErrFinalToken }
	case "empty":
		return func(data []byte, eof bool) (int, []byte, error) { return 0, []byte{}, nil }
	default:
		panic("unmapped split " + kind)
	}
}

// observeSplit records direct helper panics separately from ordinary Scanner protocol calls.
func observeSplit(kind string, input []byte, eof bool) (out splitCase) {
	out.Kind = kind
	out.Input = encoded(input)
	out.EOF = eof
	out.Nil = true
	defer func() {
		if p := recover(); p != nil {
			out.Panic = encoded([]byte(fmt.Sprint(p)))
		}
	}()
	data := input[:len(input):len(input)]
	advance, token, e := splitter(kind)(data, eof)
	out.Advance = advance
	out.Token = encoded(token)
	out.Nil = token == nil
	out.Error = errorCode(e)
	return out
}

// observeScanner retains token nilness, stopped-state errors and the checked analogue of nonprogress panics.
func observeScanner(c scannerCase) (out scannerCase) {
	out = c
	s := &source{data: decoded(c.Input), chunk: c.Chunk, together: c.Together}
	scan := bufio.NewScanner(s)
	scan.Buffer(make([]byte, c.Capacity), c.Maximum)
	scan.Split(splitter(c.Kind))
	defer func() {
		if p := recover(); p != nil {
			out.Expected = append(out.Expected, observation{Name: "panic", Panic: encoded([]byte(fmt.Sprint(p)))})
		}
	}()
	for index := 0; index < 104; index++ {
		more := scan.Scan()
		token := scan.Bytes()
		event := observation{Name: "scan", More: more, Data: encoded(token), Nil: token == nil}
		finishError(&event, scan.Err())
		out.Expected = append(out.Expected, event)
		if !more {
			break
		}
	}
	return out
}

// main generates independent histories and raw byte boundaries without copying implementation source.
func main() {
	if runtime.Version() != "go1.27.4" {
		panic("unexpected reference SDK")
	}
	readers := []readerCase{}
	writers := []writerCase{}
	splits := []splitCase{}
	scanners := []scannerCase{}
	inputs := [][]byte{{}, []byte("A\r\nBC\n\nZ\r"), []byte("a\xc3\xa9\xf0\x9f\x8c\x8d\xff\x80\xe2\x82"), bytes.Repeat([]byte("0123456789"), 5), []byte{0, 10, 13, 255}, []byte("x\u00a0\u1680\u2000\u2003\u2028\u202f\u205f\u3000z")}
	history := []operation{{Name: "size"}, {Name: "buffered"}, {Name: "unread-byte"}, {Name: "unread-rune"}, {Name: "peek", N: 0}, {Name: "peek", N: -1}, {Name: "peek", N: 1}, {Name: "byte"}, {Name: "unread-byte"}, {Name: "byte"}, {Name: "rune"}, {Name: "unread-rune"}, {Name: "rune"}, {Name: "read", N: 0}, {Name: "read", N: 2}, {Name: "unread-byte"}, {Name: "discard", N: -1}, {Name: "discard", N: 0}, {Name: "discard", N: 2}, {Name: "line"}, {Name: "unread-rune"}, {Name: "slice", N: 10}, {Name: "bytes", N: 10}, {Name: "string", N: 10}, {Name: "reset-self"}, {Name: "write-to"}, {Name: "reset", Data: encoded([]byte("N\r\n"))}, {Name: "line"}, {Name: "read", N: 20}, {Name: "byte"}}
	for _, input := range inputs {
		for _, size := range []int{-1, 0, 1, 16, 17, 32} {
			for _, chunk := range []int{1, 3, 17} {
				for _, together := range []bool{false, true} {
					readers = append(readers, observeReader(readerCase{Input: encoded(input), Size: size, Chunk: chunk, Together: together, Steps: history}))
				}
			}
		}
	}
	for _, fault := range []int{-1, 1} {
		readers = append(readers, observeReader(readerCase{Input: "41", Size: 16, Fault: fault, Steps: []operation{{Name: "peek", N: 1}}}))
	}
	readers = append(readers, observeReader(readerCase{Input: encoded([]byte("Q")), Size: 16, Empties: 101, Steps: []operation{{Name: "peek", N: 1}, {Name: "byte"}, {Name: "byte"}}}))
	readers = append(readers, observeReader(readerCase{Input: encoded([]byte("transfer")), Size: 16, Fast: true, Steps: []operation{{Name: "peek", N: 1}, {Name: "read", N: 1}, {Name: "write-to"}}}))
	writeSteps := []operation{{Name: "size"}, {Name: "available"}, {Name: "write", Data: encoded([]byte("AB"))}, {Name: "byte", N: 0}, {Name: "space", Data: encoded([]byte("CD"))}, {Name: "string", Data: "ff80"}, {Name: "rune", N: 0x1f30d}, {Name: "rune", N: 0xd800}, {Name: "rune", N: -1}, {Name: "flush"}, {Name: "write", Data: encoded(bytes.Repeat([]byte("xyz"), 20))}, {Name: "read-from", Data: encoded([]byte("reader"))}, {Name: "flush"}, {Name: "reset-self"}, {Name: "reset"}, {Name: "write", Data: "5152"}, {Name: "flush"}}
	for _, size := range []int{-1, 0, 1, 2, 3, 16, 17, 32} {
		for _, limit := range []int{-1, 0, 1, 7} {
			for _, budget := range []int{-1, 0, 1, 5, 25} {
				writers = append(writers, observeWriter(writerCase{Size: size, Limit: limit, Budget: budget, Steps: writeSteps}))
			}
		}
	}
	for _, fault := range []int{-1, 1} {
		writers = append(writers, observeWriter(writerCase{Size: 2, Limit: -1, Budget: -1, Fault: fault, Steps: []operation{{Name: "write", Data: "414243"}, {Name: "flush"}}}))
	}
	for _, input := range inputs {
		for end := 0; end <= len(input); end++ {
			for _, eof := range []bool{false, true} {
				for _, kind := range []string{"bytes", "lines", "runes", "words"} {
					splits = append(splits, observeSplit(kind, input[:end], eof))
				}
			}
		}
	}
	for _, kind := range []string{"bytes", "lines", "runes", "words", "negative", "overadvance", "final-nil", "final-empty", "empty"} {
		for _, input := range inputs {
			for _, together := range []bool{false, true} {
				scanners = append(scanners, observeScanner(scannerCase{Kind: kind, Input: encoded(input), Chunk: 1, Together: together, Capacity: 2, Maximum: 32}))
			}
		}
	}
	for _, size := range []int{15, 16, 17, 31, 32, 33, 65535, 65536} {
		for _, together := range []bool{false, true} {
			scanners = append(scanners, observeScanner(scannerCase{Kind: "lines", Input: encoded(bytes.Repeat([]byte("x"), size)), Chunk: 4096, Together: together, Capacity: 0, Maximum: 65536}))
		}
	}
	result := map[string]any{"sdk_version": runtime.Version(), "readers": readers, "writers": writers, "splits": splits, "scanners": scanners, "scope": "Complete operation histories, raw bytes, direct split boundaries, terminal timing and malformed callbacks; additional transfer/readwriter/capacity controls accompany implementation.", "watchdog": "A fixture-only panic bounds repeated invalid Writer.Write(positive)=(0,nil); this is not reported as a native library panic and maps to a named checked refusal."}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if e := encoder.Encode(result); e != nil {
		panic(e)
	}
}
