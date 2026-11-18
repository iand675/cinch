// Command reference records actual formatting and output protocol observations.
// It is an independent SDK consumer; it contains no formatter implementation.
package main

import (
	"encoding/hex"
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"io"
	"os"
	"runtime"
	"strconv"
	"text/tabwriter"
	"unicode/utf8"
)

// config holds exact machine-sized constructor arguments in a portable input record.
type config struct {
	MinWidth int64  `json:"min_width"`
	TabWidth int64  `json:"tab_width"`
	Padding  int64  `json:"padding"`
	Pad      byte   `json:"pad"`
	Flags    uint64 `json:"flags"`
}

// sinkConfig describes one deterministic output, including explicit malformed reports.
type sinkConfig struct {
	ID         string `json:"id"`
	MaxPerCall *int   `json:"max_per_call,omitempty"`
	FailAt     int    `json:"fail_at,omitempty"`
	FailPrefix *int   `json:"fail_prefix,omitempty"`
	ErrorCode  string `json:"error_code,omitempty"`
	FailEmpty  bool   `json:"fail_empty,omitempty"`
	PanicAt    int    `json:"panic_at,omitempty"`
	ReportAt   int    `json:"report_at,omitempty"`
	Reported   *int   `json:"reported,omitempty"`
	OfferLimit int    `json:"offer_limit,omitempty"`
	ByteLimit  int    `json:"byte_limit,omitempty"`
}

// action selects one public operation; alias writes retain the original receiver.
type action struct {
	Operation string  `json:"operation"`
	Data      string  `json:"data,omitempty"`
	Sink      string  `json:"sink,omitempty"`
	Config    *config `json:"config,omitempty"`
	Alias     bool    `json:"alias,omitempty"`
}

// request is one fresh reference epoch with independent output state.
type request struct {
	ID      string       `json:"id"`
	Initial *config      `json:"initial,omitempty"`
	Zero    bool         `json:"zero,omitempty"`
	Sinks   []sinkConfig `json:"sinks"`
	Actions []action     `json:"actions"`
}

// sinkFailure is the original error payload whose code survives formatter handling.
type sinkFailure struct{ code string }

// Error gives each injected failure a stable independently observed message.
func (e *sinkFailure) Error() string { return "sink:" + e.code }

// errorRecord distinguishes original errors from the short-write sentinel.
type errorRecord struct {
	Kind    string `json:"kind"`
	Message string `json:"message"`
	Code    string `json:"code,omitempty"`
}

// offer captures every call, including zero-length and malformed-result calls.
type offer struct {
	Sink     string       `json:"sink"`
	Number   int          `json:"number"`
	Data     string       `json:"data"`
	Reported *int         `json:"reported,omitempty"`
	Accepted int          `json:"accepted"`
	Error    *errorRecord `json:"error,omitempty"`
	Panic    string       `json:"panic,omitempty"`
}

// spy owns physical accepted bytes and appends exact global output call order.
type spy struct {
	config sinkConfig
	data   []byte
	calls  int
	events *[]offer
}

// recordError preserves the original error family and its opaque injected code.
func recordError(err error) *errorRecord {
	if err == nil {
		return nil
	}
	if errors.Is(err, io.ErrShortWrite) {
		return &errorRecord{Kind: "short-write", Message: err.Error()}
	}
	if value, ok := err.(*sinkFailure); ok {
		return &errorRecord{Kind: "sink", Message: value.Error(), Code: value.code}
	}
	return &errorRecord{Kind: fmt.Sprintf("%T", err), Message: err.Error()}
}

// Write independently records accepted physical bytes and a possibly invalid returned count.
func (s *spy) Write(input []byte) (int, error) {
	s.calls++
	row := offer{Sink: s.config.ID, Number: s.calls, Data: hex.EncodeToString(input)}
	if s.config.PanicAt == s.calls {
		row.Panic = "injected sink panic"
		*s.events = append(*s.events, row)
		panic(row.Panic)
	}
	n := len(input)
	var err error
	if s.config.MaxPerCall != nil && n > *s.config.MaxPerCall {
		n = *s.config.MaxPerCall
	}
	if (s.config.FailAt != 0 && s.config.FailAt == s.calls) || (s.config.FailEmpty && len(input) == 0) {
		if s.config.FailPrefix != nil && n > *s.config.FailPrefix {
			n = *s.config.FailPrefix
		}
		err = &sinkFailure{code: s.config.ErrorCode}
	}
	if s.calls > s.config.OfferLimit || len(s.data)+n > s.config.ByteLimit {
		remaining := s.config.ByteLimit - len(s.data)
		if remaining < 0 {
			remaining = 0
		}
		if n > remaining {
			n = remaining
		}
		err = &sinkFailure{code: "observer-limit"}
	}
	if n < 0 || n > len(input) {
		panic("invalid observer physical count")
	}
	s.data = append(s.data, input[:n]...)
	reported := n
	if s.config.Reported != nil && (s.config.ReportAt == 0 || s.config.ReportAt == s.calls) {
		reported = *s.config.Reported
	}
	row.Reported, row.Accepted, row.Error = &reported, n, recordError(err)
	*s.events = append(*s.events, row)
	return reported, err
}

// actionResult records only a returned Write count; panics have no fabricated count.
type actionResult struct {
	Operation    string       `json:"operation"`
	Count        *int         `json:"count,omitempty"`
	Error        *errorRecord `json:"error,omitempty"`
	Panic        string       `json:"panic,omitempty"`
	SameReceiver *bool        `json:"same_receiver,omitempty"`
	Offers       []offer      `json:"offers"`
	Outputs      []sinkOutput `json:"outputs"`
}

// sinkOutput exposes complete physical progress independently of reported counts.
type sinkOutput struct {
	ID        string `json:"id"`
	Data      string `json:"data"`
	ValidUTF8 bool   `json:"valid_utf8"`
}

// result contains one exact constructor and operation trace, never a predicted expectation.
type result struct {
	ID               string         `json:"id"`
	ConstructorPanic string         `json:"constructor_panic,omitempty"`
	Actions          []actionResult `json:"actions"`
}

// initialize invokes the actual public initializer and preserves an existing receiver on refusal.
func initialize(writer *tabwriter.Writer, output io.Writer, c config) *tabwriter.Writer {
	return writer.Init(output, int(c.MinWidth), int(c.TabWidth), int(c.Padding), c.Pad, uint(c.Flags))
}

// invoke executes one actual method and captures panics without attempting a second operation.
func invoke(writer, alias **tabwriter.Writer, step action, sinks map[string]*spy) (row actionResult) {
	row.Operation = step.Operation
	defer func() {
		if failure := recover(); failure != nil {
			row.Panic = fmt.Sprint(failure)
		}
	}()
	target := *writer
	if step.Alias {
		target = *alias
	}
	switch step.Operation {
	case "write":
		input, err := hex.DecodeString(step.Data)
		if err != nil {
			panic("invalid request hex")
		}
		n, err := target.Write(input)
		row.Count, row.Error = &n, recordError(err)
	case "flush":
		row.Error = recordError(target.Flush())
	case "init":
		before := target
		after := initialize(target, sinks[step.Sink], *step.Config)
		same := before == after
		row.SameReceiver = &same
	case "new":
		c := *step.Config
		*writer = tabwriter.NewWriter(sinks[step.Sink], int(c.MinWidth), int(c.TabWidth), int(c.Padding), c.Pad, uint(c.Flags))
	default:
		panic("unknown request operation")
	}
	return
}

// observe starts fresh for one request and captures physical progress even after a method panic.
func observe(input request) (output result) {
	output.ID = input.ID
	events := []offer{}
	ordered := make([]*spy, 0, len(input.Sinks))
	sinks := map[string]*spy{}
	for _, c := range input.Sinks {
		if c.OfferLimit == 0 {
			c.OfferLimit = 1024
		}
		if c.ByteLimit == 0 {
			c.ByteLimit = 4096
		}
		s := &spy{config: c, events: &events}
		ordered, sinks[c.ID] = append(ordered, s), s
	}
	writer := new(tabwriter.Writer)
	if !input.Zero {
		func() {
			defer func() {
				if failure := recover(); failure != nil {
					output.ConstructorPanic = fmt.Sprint(failure)
				}
			}()
			c := *input.Initial
			writer = tabwriter.NewWriter(ordered[0], int(c.MinWidth), int(c.TabWidth), int(c.Padding), c.Pad, uint(c.Flags))
		}()
	}
	if output.ConstructorPanic != "" {
		return
	}
	alias := writer
	for _, step := range input.Actions {
		before := len(events)
		row := invoke(&writer, &alias, step, sinks)
		row.Offers = append([]offer{}, events[before:]...)
		for _, sink := range ordered {
			row.Outputs = append(row.Outputs, sinkOutput{ID: sink.config.ID, Data: hex.EncodeToString(sink.data), ValidUTF8: utf8.Valid(sink.data)})
		}
		output.Actions = append(output.Actions, row)
	}
	return
}

// main consumes explicit cases and emits actual observations with the executable SDK identity.
func main() {
	requests := flag.String("requests", "", "input request JSON")
	flag.Parse()
	data, err := os.ReadFile(*requests)
	if err != nil {
		panic(err)
	}
	var cases []request
	if err := json.Unmarshal(data, &cases); err != nil {
		panic(err)
	}
	rows := make([]result, 0, len(cases))
	for _, input := range cases {
		rows = append(rows, observe(input))
	}
	output := struct {
		SDK          string   `json:"sdk"`
		Architecture string   `json:"architecture"`
		IntegerBits  int      `json:"integer_bits"`
		Cases        []result `json:"cases"`
	}{runtime.Version(), runtime.GOARCH, strconv.IntSize, rows}
	encoder := json.NewEncoder(os.Stdout)
	if err := encoder.Encode(output); err != nil {
		panic(err)
	}
}
