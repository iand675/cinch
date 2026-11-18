// Generate complete field outputs and actual predicate/yield traces with the pinned reference SDK.
package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"iter"
	"os"
	"runtime"
	"unicode"
	"unicode/utf8"
)

// request identifies one bounded raw input and an observable traversal policy.
type request struct {
	Input   string `json:"input"`
	Start   int    `json:"start"`
	End     int    `json:"end"`
	Mode    string `json:"mode"`
	Take    int    `json:"take"`
	Replays int    `json:"replays"`
}

// event distinguishes scalar predicate calls from completed yielded byte aliases.
type event struct {
	Kind  string `json:"kind"`
	Rune  int32  `json:"rune"`
	Value string `json:"value,omitempty"`
}

// selected is deterministic for each scalar while its caller records observable visits.
func selected(mode string, r rune) bool {
	switch mode {
	case "space", "default":
		return unicode.IsSpace(r)
	case "comma":
		return r == ','
	case "error":
		return r == utf8.RuneError
	case "nonascii":
		return r >= utf8.RuneSelf
	case "even":
		return r%2 == 0
	default:
		panic("unknown predicate")
	}
}

// chunks records every returned byte and the bounded capacity of each actual reference view.
func chunks(values [][]byte) ([]string, []int) {
	out, capacities := []string{}, []int{}
	for _, value := range values {
		out = append(out, hex.EncodeToString(value))
		capacities = append(capacities, cap(value))
	}
	return out, capacities
}

// main audits the actual eager and reusable iterator implementations, including early stopping.
func main() {
	if runtime.Version() != "go1.27.4" {
		panic("unexpected reference SDK")
	}
	var inputs []request
	if err := json.NewDecoder(os.Stdin).Decode(&inputs); err != nil {
		panic(err)
	}
	rows := []map[string]any{}
	for _, input := range inputs {
		raw, err := hex.DecodeString(input.Input)
		if err != nil || input.Start < 0 || input.End < input.Start || input.End > len(raw) {
			panic("invalid bounded input")
		}
		view := raw[input.Start:input.End]
		eagerTrace := []event{}
		predicate := func(r rune) bool {
			eagerTrace = append(eagerTrace, event{Kind: "predicate", Rune: r})
			return selected(input.Mode, r)
		}
		var eager [][]byte
		if input.Mode == "default" {
			eager = bytes.Fields(view)
		} else {
			eager = bytes.FieldsFunc(view, predicate)
		}
		eagerValues, capacities := chunks(eager)
		var sequence iter.Seq[[]byte]
		trace := []event{}
		seqPredicate := func(r rune) bool {
			trace = append(trace, event{Kind: "predicate", Rune: r})
			return selected(input.Mode, r)
		}
		if input.Mode == "default" {
			sequence = bytes.FieldsSeq(view)
		} else {
			sequence = bytes.FieldsFuncSeq(view, seqPredicate)
		}
		constructionTrace := append([]event{}, trace...)
		invocations := []map[string]any{}
		for replay := 0; replay < input.Replays; replay++ {
			trace = []event{}
			values := []string{}
			if input.Take != 0 {
				sequence(func(value []byte) bool {
					hexValue := hex.EncodeToString(value)
					values = append(values, hexValue)
					trace = append(trace, event{Kind: "yield", Value: hexValue})
					return input.Take < 0 || len(values) < input.Take
				})
			}
			invocations = append(invocations, map[string]any{"values": values, "events": trace})
		}
		rows = append(rows, map[string]any{"input": input.Input, "start": input.Start, "end": input.End,
			"mode": input.Mode, "take": input.Take, "replays": input.Replays,
			"eager": eagerValues, "capacities": capacities, "eager_events": eagerTrace,
			"construction_events": constructionTrace, "invocations": invocations})
	}
	encoded, err := json.MarshalIndent(map[string]any{"sdk_version": runtime.Version(), "rows": rows, "effect_controls": effectControls()}, "", "  ")
	if err != nil {
		panic(err)
	}
	fmt.Println(string(encoded))
}

// effectControls observes valid side effects without changing a predicate's result for a given scalar.
func effectControls() []map[string]any {
	results := []map[string]any{}
	for _, name := range []string{"predicate-future", "predicate-past", "predicate-current-width", "yield-terminator-width", "default-terminator-width", "yield-future-invalid", "yield-alias", "stop-after-first"} {
		mode, input, take := "comma", []byte("a,b,c"), -1
		switch name {
		case "predicate-current-width":
			input = []byte("é,x")
		case "yield-terminator-width", "default-terminator-width":
			input = append(bytes.Repeat([]byte{'a'}, 17), []byte("\u2003bc d")...)
			mode = "space"
			if name == "default-terminator-width" {
				mode = "default"
			}
		case "yield-future-invalid":
			input, mode = []byte("a bé"), "space"
		case "stop-after-first":
			input, mode, take = []byte("ab cd ef"), "space", 1
		}
		original := hex.EncodeToString(input)
		events := []map[string]any{}
		mutated := false
		predicate := func(r rune) bool {
			events = append(events, map[string]any{"kind": "predicate", "rune": r, "input": hex.EncodeToString(input)})
			if !mutated && name == "predicate-future" {
				input[2], mutated = 'x', true
				events = append(events, map[string]any{"kind": "mutation", "input": hex.EncodeToString(input)})
			}
			if !mutated && name == "predicate-current-width" {
				input[0], input[1], mutated = 'a', ' ', true
				events = append(events, map[string]any{"kind": "mutation", "input": hex.EncodeToString(input)})
			}
			if !mutated && name == "predicate-past" && r == 'b' {
				input[0], mutated = 'Z', true
				events = append(events, map[string]any{"kind": "mutation", "input": hex.EncodeToString(input)})
			}
			return selected(mode, r)
		}
		var sequence iter.Seq[[]byte]
		if mode == "default" {
			sequence = bytes.FieldsSeq(input)
		} else {
			sequence = bytes.FieldsFuncSeq(input, predicate)
		}
		construction := append([]map[string]any{}, events...)
		invocations := []map[string]any{}
		for replay := 0; replay < 2; replay++ {
			events = []map[string]any{}
			held := [][]byte{}
			sequence(func(part []byte) bool {
				held = append(held, part)
				events = append(events, map[string]any{"kind": "yield", "value": hex.EncodeToString(part), "input": hex.EncodeToString(input)})
				if len(held) == 1 {
					switch name {
					case "yield-terminator-width", "default-terminator-width":
						input[17], input[18], input[19] = ' ', 'x', 'y'
					case "yield-future-invalid":
						input[2], input[3] = 255, ' '
					case "yield-alias":
						part[0] = 'Z'
					case "stop-after-first":
						input[len(input)-1] = 'z'
					default:
						return take < 0 || len(held) < take
					}
					events = append(events, map[string]any{"kind": "mutation", "input": hex.EncodeToString(input)})
				}
				return take < 0 || len(held) < take
			})
			values, _ := chunks(held)
			invocations = append(invocations, map[string]any{"events": events, "held_after": values, "input_after": hex.EncodeToString(input)})
		}
		results = append(results, map[string]any{"name": name, "mode": mode, "input": original, "take": take, "construction_events": construction, "invocations": invocations})
	}
	return results
}
