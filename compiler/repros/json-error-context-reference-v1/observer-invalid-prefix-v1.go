// Independent quoted-field and nested method error-context observations.
package main

import (
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

type pointerMethod bool

func (v *pointerMethod) MarshalJSON() ([]byte, error) {
	calls = append(calls, "pointer-json")
	return []byte(`"method"`), nil
}

type plainBool bool
type jsonRefusal bool

func (v jsonRefusal) MarshalJSON() ([]byte, error) {
	calls = append(calls, "json-refusal")
	return nil, sentinel
}

type textRefusal bool

func (v textRefusal) MarshalText() ([]byte, error) {
	calls = append(calls, "text-refusal")
	return nil, sentinel
}

type jsonSemantic bool

func (v jsonSemantic) MarshalJSON() ([]byte, error) {
	calls = append(calls, "json-semantic")
	return nil, &jsonv2.SemanticError{ByteOffset: 2, JSONPointer: "/inner~0", JSONKind: '0', JSONValue: []byte("12"), Err: sentinel}
}
func (v *jsonSemantic) UnmarshalJSON(input []byte) error {
	calls = append(calls, "json-semantic-read:"+hex.EncodeToString(input))
	*v = true
	return &jsonv2.SemanticError{ByteOffset: 2, JSONPointer: "/inner~0", JSONKind: '0', JSONValue: []byte("12"), Err: sentinel}
}

type textSemantic bool

func (v textSemantic) MarshalText() ([]byte, error) {
	calls = append(calls, "text-semantic")
	return nil, &jsonv2.SemanticError{ByteOffset: 2, JSONPointer: "/inner~0", JSONKind: '0', JSONValue: []byte("12"), Err: sentinel}
}
func (v *textSemantic) UnmarshalText(input []byte) error {
	calls = append(calls, "text-semantic-read:"+hex.EncodeToString(input))
	*v = true
	return &jsonv2.SemanticError{ByteOffset: 2, JSONPointer: "/inner~0", JSONKind: '0', JSONValue: []byte("12"), Err: sentinel}
}

type tagged[T any] struct {
	Value T `json:"value,string"`
}
type prefixed[T any] struct {
	Before string `json:"before"`
	Value  T      `json:"value,string"`
	After  int    `json:"after"`
}
type ordinary[T any] struct {
	Before string `json:"before"`
	Value  T      `json:"a/b~"`
	After  int    `json:"after"`
}
type nested[T any] struct {
	Outer T `json:"outer"`
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

func rows[T any](value T, tag bool) []map[string]any {
	out := []map[string]any{}
	var contexts map[string]any
	if tag {
		v := tagged[T]{value}
		p := prefixed[T]{"α<&", value, 9}
		contexts = map[string]any{"quoted": v, "quoted_pointer": &v, "prefixed": p, "prefixed_pointer": &p, "nested": nested[tagged[T]]{v}, "slice": []tagged[T]{v}}
	} else {
		v := ordinary[T]{"α<&", value, 9}
		contexts = map[string]any{"value": value, "pointer": &value, "prefixed": v, "nested": nested[ordinary[T]]{v}, "slice": []T{value}, "map": map[string]T{"a/b~": value}}
	}
	names := []string{"value", "pointer", "quoted", "quoted_pointer", "prefixed", "prefixed_pointer", "nested", "slice", "map"}
	for _, name := range names {
		v, ok := contexts[name]
		if !ok {
			continue
		}
		for _, legacyMethods := range []bool{true, false} {
			for _, legacyErrors := range []bool{true, false} {
				for _, stringify := range []bool{true, false} {
					if !tag && (!legacyMethods || !stringify) {
						continue
					}
					for _, style := range []string{"compact", "pretty", "empty-indent"} {
						for _, html := range []bool{true, false} {
							options := []jsonv2.Options{json.DefaultOptionsV1(), json.CallMethodsWithLegacySemantics(legacyMethods), json.ReportErrorsWithLegacySemantics(legacyErrors), json.StringifyWithLegacySemantics(stringify), jsontext.EscapeForHTML(html)}
							if style == "pretty" {
								options = append(options, jsontext.WithIndent("  "), jsontext.WithIndentPrefix("P>"))
							}
							if style == "empty-indent" {
								options = append(options, jsontext.Multiline(true), jsontext.WithIndent(""))
							}
							calls = []string{}
							output, err := jsonv2.Marshal(v, options...)
							out = append(out, map[string]any{"type": reflect.TypeOf(value).String(), "tagged": tag, "context": name, "legacy_methods": legacyMethods, "legacy_errors": legacyErrors, "legacy_stringify": stringify, "style": style, "html": html, "output_hex": hex.EncodeToString(output), "error": failure(err), "calls": calls})
						}
					}
				}
			}
		}
	}
	return out
}
func readRows[T ~bool](initial T) []map[string]any {
	out := []map[string]any{}
	for _, context := range []string{"value", "field", "nested"} {
		for _, wire := range []string{`"x"`, `true`, `1`} {
			for _, legacy := range []bool{true, false} {
				value := initial
				record := ordinary[T]{"prior", initial, 9}
				outer := nested[ordinary[T]]{record}
				var target any = &value
				input := []byte(wire)
				if context == "field" {
					target = &record
					input = []byte(`{"before":"ok","a/b~":` + wire + `,"after":2}`)
				}
				if context == "nested" {
					target = &outer
					input = []byte(`{"outer":{"before":"ok","a/b~":` + wire + `,"after":2}}`)
				}
				calls = []string{}
				err := jsonv2.Unmarshal(input, target, json.DefaultOptionsV1(), json.ReportErrorsWithLegacySemantics(legacy))
				after := 9
				if context == "field" {
					value = record.Value
					after = record.After
				}
				if context == "nested" {
					value = outer.Outer.Value
					after = outer.Outer.After
				}
				out = append(out, map[string]any{"type": reflect.TypeOf(initial).String(), "context": context, "input_hex": hex.EncodeToString(input), "legacy_errors": legacy, "value": bool(value), "after": after, "calls": calls, "error": failure(err)})
			}
		}
	}
	return out
}
func main() {
	tagged := []map[string]any{}
	tagged = append(tagged, rows(plainBool(true), true)...)
	tagged = append(tagged, rows(pointerMethod(true), true)...)
	tagged = append(tagged, rows("text", true)...)
	tagged = append(tagged, rows(int64(7), true)...)
	encoded := []map[string]any{}
	encoded = append(encoded, rows(jsonRefusal(false), false)...)
	encoded = append(encoded, rows(textRefusal(false), false)...)
	encoded = append(encoded, rows(jsonSemantic(false), false)...)
	encoded = append(encoded, rows(textSemantic(false), false)...)
	decoded := []map[string]any{}
	decoded = append(decoded, readRows(plainBool(false))...)
	decoded = append(decoded, readRows(jsonSemantic(false))...)
	decoded = append(decoded, readRows(textSemantic(false))...)
	e := json.NewEncoder(os.Stdout)
	e.SetIndent("", "  ")
	if err := e.Encode(map[string]any{"tagged": tagged, "method_errors": encoded, "updates": decoded}); err != nil {
		panic(err)
	}
}
