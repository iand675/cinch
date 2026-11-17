// Independent read/write/text method, addressability and reporting-policy observations.
package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"encoding/json/jsontext"
	jsonv2 "encoding/json/v2"
	"errors"
	"fmt"
	"os"
	"reflect"
)

var calls []string
var sentinel = errors.New("method-refusal")

type writeJSON bool

func (v writeJSON) MarshalJSON() ([]byte, error) {
	calls = append(calls, "write-json")
	if !v {
		return nil, sentinel
	}
	return []byte(`"encoded"`), nil
}

type readJSON bool

func (v *readJSON) UnmarshalJSON(input []byte) error {
	calls = append(calls, "read-json:"+hex.EncodeToString(input))
	*v = true
	if bytes.Contains(input, []byte("error")) {
		return sentinel
	}
	return nil
}

type writeText bool

func (v writeText) MarshalText() ([]byte, error) {
	calls = append(calls, "write-text")
	if !v {
		return nil, sentinel
	}
	return []byte{255, '<', '&'}, nil
}

type readText bool

func (v *readText) UnmarshalText(input []byte) error {
	calls = append(calls, "read-text:"+hex.EncodeToString(input))
	*v = true
	if bytes.Contains(input, []byte("error")) {
		return sentinel
	}
	return nil
}

type combinedMethods bool

func (v combinedMethods) MarshalJSON() ([]byte, error) {
	calls = append(calls, "combined-write-json")
	if !v { return nil, sentinel }
	return []byte(`"combined-json"`), nil
}

func (v combinedMethods) MarshalText() ([]byte, error) {
	calls = append(calls, "combined-write-text")
	return []byte("combined-text"), nil
}

func (v *combinedMethods) UnmarshalJSON(input []byte) error {
	calls = append(calls, "combined-read-json:"+hex.EncodeToString(input))
	*v = true
	if bytes.Contains(input, []byte("error")) { return sentinel }
	return nil
}

func (v *combinedMethods) UnmarshalText(input []byte) error {
	calls = append(calls, "combined-read-text:"+hex.EncodeToString(input))
	*v = false
	return nil
}

type pointerMethods bool

func (v *pointerMethods) MarshalJSON() ([]byte, error) {
	if v == nil {
		calls = append(calls, "nil-pointer-json")
		return []byte(`"nil-pointer-method"`), nil
	}
	calls = append(calls, "pointer-json")
	return []byte(`"pointer-json"`), nil
}

func (v *pointerMethods) MarshalText() ([]byte, error) {
	if v == nil {
		calls = append(calls, "nil-pointer-text")
		return []byte("nil-pointer-text"), nil
	}
	calls = append(calls, "pointer-text")
	return []byte("pointer-text"), nil
}

type tagged[T any] struct {
	Value T `json:"value,string"`
}

type ordinary[T any] struct {
	Value T `json:"value"`
}

type partial struct {
	First int      `json:"first"`
	Value readJSON `json:"value"`
	Last  int      `json:"last"`
}

type methodInterface struct {
	Value json.Marshaler `json:"value"`
}

func failure(err error) map[string]any {
	if err == nil {
		return map[string]any{"kind": "none"}
	}
	out := map[string]any{"kind": fmt.Sprintf("%T", err), "text": err.Error(), "is_sentinel": errors.Is(err, sentinel)}
	var mismatch *json.UnmarshalTypeError
	var marshal *json.MarshalerError
	var semantic *jsonv2.SemanticError
	var syntax *json.SyntaxError
	var syntactic *jsontext.SyntacticError
	switch {
	case errors.As(err, &mismatch):
		out["offset"], out["value"], out["type"], out["struct"], out["field"] = mismatch.Offset, mismatch.Value, mismatch.Type.String(), mismatch.Struct, mismatch.Field
	case errors.As(err, &marshal):
		out["type"], out["cause"] = marshal.Type.String(), marshal.Err.Error()
	case errors.As(err, &semantic):
		out["offset"], out["pointer_hex"], out["value_hex"] = semantic.ByteOffset, hex.EncodeToString([]byte(semantic.JSONPointer)), hex.EncodeToString(semantic.JSONValue)
		out["json_kind"] = uint8(semantic.JSONKind)
		if semantic.GoType != nil {
			out["type"] = semantic.GoType.String()
		}
		if semantic.Err != nil {
			out["cause"] = semantic.Err.Error()
		}
	case errors.As(err, &syntax):
		out["offset"] = syntax.Offset
	case errors.As(err, &syntactic):
		out["offset"], out["pointer_hex"] = syntactic.ByteOffset, hex.EncodeToString([]byte(syntactic.JSONPointer))
		if syntactic.Err != nil {
			out["cause"] = syntactic.Err.Error()
		}
	}
	return out
}

func contexts[T any](value T) map[string]any {
	array := [1]T{value}
	record := ordinary[T]{value}
	quoted := tagged[T]{value}
	return map[string]any{"value": value, "pointer": &value, "record": record, "record_pointer": &record, "quoted": quoted, "quoted_pointer": &quoted, "array": array, "array_pointer": &array, "slice": []T{value}, "map": map[string]T{"value": value}}
}

func encodeRows[T any](value T) []map[string]any {
	rows := []map[string]any{}
	for _, context := range []string{"value", "pointer", "record", "record_pointer", "quoted", "quoted_pointer", "array", "array_pointer", "slice", "map"} {
		for _, methods := range []bool{true, false} {
			for _, reporting := range []bool{true, false} {
				for _, stringify := range []bool{true, false} {
					calls = []string{}
					input := contexts(value)[context]
					output, err := jsonv2.Marshal(input, json.DefaultOptionsV1(), json.CallMethodsWithLegacySemantics(methods), json.ReportErrorsWithLegacySemantics(reporting), json.StringifyWithLegacySemantics(stringify))
					rows = append(rows, map[string]any{"type": reflect.TypeOf(value).String(), "bool": reflect.ValueOf(value).Bool(), "context": context, "legacy_methods": methods, "legacy_errors": reporting, "legacy_stringify": stringify, "output_hex": hex.EncodeToString(output), "error": failure(err), "calls": calls})
				}
			}
		}
	}
	return rows
}

func decodeRows[T ~bool](initial T) []map[string]any {
	rows := []map[string]any{}
	inputs := []string{"null", "true", "false", `"true"`, `"false"`, `"error"`, "1", `"\ud800"`, string([]byte{'"', 255, '"'}), "{}", "[1]", "[1,]"}
	for _, context := range []string{"value", "record", "quoted"} {
		for _, input := range inputs {
			for _, reporting := range []bool{true, false} {
				for _, stringify := range []bool{true, false} {
					value := initial
					record := ordinary[T]{initial}
					quoted := tagged[T]{initial}
					var target any = &value
					wire := []byte(input)
					if context == "record" {
						target, wire = &record, []byte(`{"value":`+input+`}`)
					}
					if context == "quoted" {
						target, wire = &quoted, []byte(`{"value":`+input+`}`)
					}
					calls = []string{}
					err := jsonv2.Unmarshal(wire, target, json.DefaultOptionsV1(), json.ReportErrorsWithLegacySemantics(reporting), json.StringifyWithLegacySemantics(stringify))
					if context == "record" {
						value = record.Value
					}
					if context == "quoted" {
						value = quoted.Value
					}
					rows = append(rows, map[string]any{"type": reflect.TypeOf(initial).String(), "context": context, "input_hex": hex.EncodeToString(wire), "legacy_errors": reporting, "legacy_stringify": stringify, "initial": bool(initial), "value": bool(value), "error": failure(err), "calls": calls})
				}
			}
		}
	}
	return rows
}

func main() {
	encodings := []map[string]any{}
	encodings = append(encodings, encodeRows(writeJSON(true))...)
	encodings = append(encodings, encodeRows(writeJSON(false))...)
	encodings = append(encodings, encodeRows(readJSON(false))...)
	encodings = append(encodings, encodeRows(writeText(true))...)
	encodings = append(encodings, encodeRows(writeText(false))...)
	encodings = append(encodings, encodeRows(readText(false))...)
	encodings = append(encodings, encodeRows(pointerMethods(true))...)
	encodings = append(encodings, encodeRows(combinedMethods(true))...)
	encodings = append(encodings, encodeRows(combinedMethods(false))...)
	decodings := []map[string]any{}
	decodings = append(decodings, decodeRows(writeJSON(true))...)
	decodings = append(decodings, decodeRows(readJSON(false))...)
	decodings = append(decodings, decodeRows(writeText(true))...)
	decodings = append(decodings, decodeRows(readText(false))...)
	decodings = append(decodings, decodeRows(combinedMethods(false))...)
	partials := []map[string]any{}
	for _, input := range []string{`{"first":"bad","value":"error","last":2}`, `{"first":"bad","value":true,"last":2}`, `{"first":1,"value":true,"last":}`, `{"first":1,"value":"error","last":}`} {
		for _, reporting := range []bool{true, false} {
			value := partial{First: 7, Last: 9}
			calls = []string{}
			err := jsonv2.Unmarshal([]byte(input), &value, json.DefaultOptionsV1(), json.ReportErrorsWithLegacySemantics(reporting))
			partials = append(partials, map[string]any{"input_hex": hex.EncodeToString([]byte(input)), "legacy_errors": reporting, "first": value.First, "value": bool(value.Value), "last": value.Last, "error": failure(err), "calls": calls})
		}
	}
	nils := []map[string]any{}
	var missing *pointerMethods
	for _, name := range []string{"pointer", "interface", "nil-interface", "map-key"} {
		for _, methods := range []bool{true, false} {
			for _, reporting := range []bool{true, false} {
				var value any = missing
				if name == "interface" {
					value = methodInterface{missing}
				}
				if name == "nil-interface" {
					value = methodInterface{}
				}
				if name == "map-key" {
					value = map[*pointerMethods]int{missing: 1}
				}
				calls = []string{}
				output, err := jsonv2.Marshal(value, json.DefaultOptionsV1(), json.CallMethodsWithLegacySemantics(methods), json.ReportErrorsWithLegacySemantics(reporting))
				nils = append(nils, map[string]any{"context": name, "legacy_methods": methods, "legacy_errors": reporting, "output_hex": hex.EncodeToString(output), "error": failure(err), "calls": calls})
			}
		}
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]any{"encoding": encodings, "decoding": decodings, "partial": partials, "nil_methods": nils}); err != nil {
		panic(err)
	}
}
