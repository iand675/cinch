// Generate independent dynamic-value observations without recursively encoding the observation tree.
package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	jsonv2 "encoding/json/v2"
	"errors"
	"fmt"
	"math"
	"os"
	"sort"
)

// edgeObservations adds exact pointer-byte metadata only to the focused edge reference.
var edgeObservations bool

// dynamicNodes records exact application bytes, numeric spellings and floating bits in preorder.
func dynamicNodes(value any) []map[string]any {
	nodes := []map[string]any{}
	var visit func(any)
	visit = func(value any) {
		switch value := value.(type) {
		case nil:
			nodes = append(nodes, map[string]any{"kind": "null"})
		case bool:
			nodes = append(nodes, map[string]any{"kind": "bool", "value": value})
		case float64:
			nodes = append(nodes, map[string]any{"kind": "float", "bits": fmt.Sprintf("%016x", math.Float64bits(value))})
		case string:
			nodes = append(nodes, map[string]any{"kind": "string", "hex": hex.EncodeToString([]byte(value))})
		case json.Number:
			nodes = append(nodes, map[string]any{"kind": "number", "hex": hex.EncodeToString([]byte(value))})
		case []any:
			nodes = append(nodes, map[string]any{"kind": "array", "nil": value == nil, "length": len(value)})
			for _, child := range value {
				visit(child)
			}
			nodes = append(nodes, map[string]any{"kind": "end-array"})
		case map[string]any:
			nodes = append(nodes, map[string]any{"kind": "object", "nil": value == nil, "length": len(value)})
			keys := []string{}
			for key := range value {
				keys = append(keys, key)
			}
			sort.Strings(keys)
			for _, key := range keys {
				nodes = append(nodes, map[string]any{"kind": "key", "hex": hex.EncodeToString([]byte(key))})
				visit(value[key])
			}
			nodes = append(nodes, map[string]any{"kind": "end-object"})
		default:
			panic(fmt.Sprintf("unobserved type %T", value))
		}
	}
	visit(value)
	return nodes
}

// dynamicError preserves the actual error family and its structured public metadata.
func dynamicError(err error) map[string]any {
	if err == nil {
		return map[string]any{"kind": "none"}
	}
	out := map[string]any{"kind": fmt.Sprintf("%T", err), "message": err.Error()}
	var mismatch *json.UnmarshalTypeError
	var syntax *json.SyntaxError
	var semantic *jsonv2.SemanticError
	var custom *json.MarshalerError
	var unsupported *json.UnsupportedValueError
	switch {
	case errors.As(err, &mismatch):
		out["offset"], out["value"], out["type"], out["struct"], out["field"] = mismatch.Offset, mismatch.Value, mismatch.Type.String(), mismatch.Struct, mismatch.Field
	case errors.As(err, &syntax):
		out["offset"] = syntax.Offset
	case errors.As(err, &semantic):
		out["offset"], out["pointer"], out["value_hex"] = semantic.ByteOffset, string(semantic.JSONPointer), hex.EncodeToString(semantic.JSONValue)
		if edgeObservations {
			out["pointer_hex"] = hex.EncodeToString([]byte(semantic.JSONPointer))
		}
		if semantic.GoType != nil {
			out["type"] = semantic.GoType.String()
		}
		if semantic.Err != nil {
			out["cause"] = semantic.Err.Error()
		}
	case errors.As(err, &custom):
		out["type"], out["cause"] = custom.Type.String(), custom.Err.Error()
	case errors.As(err, &unsupported):
		out["value"] = unsupported.Str
	}
	return out
}

// dynamicPrior creates independently observable interface payloads, including typed nil containers.
func dynamicPrior(kind string) any {
	switch kind {
	case "null":
		return nil
	case "bool":
		return true
	case "float":
		return -7.25
	case "string":
		return "prior"
	case "array":
		return []any{true, "prior", map[string]any{"old": "kept"}}
	case "object":
		return map[string]any{"old": "kept", "a": map[string]any{"old": true}}
	case "nil-array":
		return []any(nil)
	case "nil-object":
		return map[string]any(nil)
	}
	panic(kind)
}

// dynamicMarshal observes both HTML modes and omission of a non-nil dynamic payload.
func dynamicMarshal(value any) []map[string]any {
	results := []map[string]any{}
	for _, html := range []bool{false, true} {
		var buffer bytes.Buffer
		encoder := json.NewEncoder(&buffer)
		encoder.SetEscapeHTML(html)
		err := encoder.Encode(value)
		output := bytes.TrimSuffix(buffer.Bytes(), []byte{'\n'})
		wrapped, wrappedErr := json.Marshal(struct {
			Value any `json:"value,omitempty"`
		}{value})
		results = append(results, map[string]any{"html": html, "output_hex": hex.EncodeToString(output), "error": dynamicError(err), "omitted_hex": hex.EncodeToString(wrapped), "omitted_error": dynamicError(wrappedErr)})
	}
	return results
}

// main writes complete independent partial-value/error/encoding observations for the dynamic family.
func main() {
	edgeObservations = len(os.Args) == 2 && os.Args[1] == "--edge"
	inputs := []string{"null", "false", "-0", "9007199254740993", "1e-9999", "1e9999", "-1e9999", `"<>&\u2028é"`, `"\ud800A\udc00"`, "[]", "{}", "[null,false,1,\"x\",{},[]]", `{"z":1,"a":2,"z":3}`, `{"a":{"b":true},"a":{"c":false}}`, `[1,1e9999,3]`, `{"a":1e9999,"b":2}`, `{"a":[1,1e9999,3],"b":2}`, `[{"x":1e9999,"y":2},3]`, `{"a":{"x":1e9999,"y":2},"b":3}`, `[1e9999,-1e9999,2]`, `[1,]`, `{"a":1,"b":}`, "true false", "", " "}
	inputs = append(inputs, string([]byte{34, 255, 192, 175, 34}), string([]byte{'{', '"', 255, '"', ':', '1', ',', '"', 254, '"', ':', '2', '}'}))
	priors := []string{"null", "bool", "float", "string", "array", "object", "nil-array", "nil-object"}
	if edgeObservations {
		inputs = []string{`{"a/b":{"~":[1e9999]}}`, `{"":1e9999}`, " \n{ \"a\" :  1e9999 , \"b\" : 2}", "[  1e9999 , 2 ]", `{"a.b":[1e9999]}`, `{"é":1e9999}`, " 1e9999 \n", string([]byte{'{', '"', 255, '"', ':', '1', 'e', '9', '9', '9', '9', '}'})}
		priors = []string{"null", "object"}
	}
	decoded := []map[string]any{}
	for _, input := range inputs {
		for _, prior := range priors {
			for _, numbers := range []bool{false, true} {
				if numbers && !json.Valid([]byte(input)) {
					continue // Decoder syntax mutation differs from whole-input Unmarshal.
				}
				value := dynamicPrior(prior)
				var err error
				if numbers {
					decoder := json.NewDecoder(bytes.NewBufferString(input))
					decoder.UseNumber()
					err = decoder.Decode(&value)
				} else {
					err = json.Unmarshal([]byte(input), &value)
				}
				decoded = append(decoded, map[string]any{"input_hex": hex.EncodeToString([]byte(input)), "prior": prior, "use_number": numbers, "nodes": dynamicNodes(value), "error": dynamicError(err), "marshal": dynamicMarshal(value)})
			}
		}
	}
	aborted := []map[string]any{}
	for _, input := range inputs {
		if !json.Valid([]byte(input)) {
			continue
		}
		value := dynamicPrior("object")
		err := jsonv2.Unmarshal([]byte(input), &value, json.DefaultOptionsV1(), json.ReportErrorsWithLegacySemantics(false))
		aborted = append(aborted, map[string]any{"input_hex": hex.EncodeToString([]byte(input)), "prior": "object", "nodes": dynamicNodes(value), "error": dynamicError(err)})
	}
	application := []map[string]any{}
	values := []any{nil, false, float64(0), math.Copysign(0, -1), "", []any{}, map[string]any{}, []any(nil), map[string]any(nil), string([]byte{255, 254}), map[string]any{string([]byte{255}): 1.0, "z": true}, json.Number(""), json.Number("01"), json.Number("1e9999"), math.Inf(1), math.NaN()}
	if edgeObservations {
		values = []any{}
	}
	for _, value := range values {
		application = append(application, map[string]any{"nodes": dynamicNodes(value), "marshal": dynamicMarshal(value)})
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]any{"decode": decoded, "abort": aborted, "application": application}); err != nil {
		panic(err)
	}
}
