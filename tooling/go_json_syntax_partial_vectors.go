// Independent observations of malformed-input mutation, lexical commit boundaries and callback timing.
package main

import (
 "encoding/hex"
 "encoding/json"
 "encoding/json/jsontext"
 jsonv2 "encoding/json/v2"
 "errors"
 "fmt"
 "os"
)

type record struct { Before int `json:"before"`; Value int `json:"value"`; After int `json:"after"` }
type pointerRecord struct { Before int `json:"before"`; Value *int `json:"value"`; After int `json:"after"` }
type jsonHook int
type textHook string
var calls []string
func (v *jsonHook) UnmarshalJSON(input []byte) error { calls=append(calls,"json:"+hex.EncodeToString(input));*v=52;return nil }
func (v *textHook) UnmarshalText(input []byte) error { calls=append(calls,"text:"+hex.EncodeToString(input));*v=textHook("method:"+string(input));return nil }
func safeText(err error) (text, panicText string) { defer func(){if v:=recover();v!=nil {panicText=fmt.Sprint(v)}}();return err.Error(),"" }
func failure(err error) map[string]any {
 if err==nil {return map[string]any{"family":"none"}}
 text,panicText:=safeText(err);out:=map[string]any{"family":fmt.Sprintf("%T",err),"text":text};if panicText!="" {out["text_panic"]=panicText}
 var semantic *jsonv2.SemanticError;var mismatch *json.UnmarshalTypeError;var syntactic *jsontext.SyntacticError;var syntax *json.SyntaxError
 switch {
 case errors.As(err,&semantic):
  out["offset"]=semantic.ByteOffset;out["pointer_hex"]=hex.EncodeToString([]byte(semantic.JSONPointer));out["kind"]=uint8(semantic.JSONKind);out["reported_hex"]=hex.EncodeToString(semantic.JSONValue)
  if semantic.GoType!=nil {out["type"]=semantic.GoType.String()} else {out["type_nil"]=true}
 case errors.As(err,&mismatch):
  out["offset"]=mismatch.Offset;out["field"]=mismatch.Field;out["record"]=mismatch.Struct;out["value"]=mismatch.Value;if mismatch.Type!=nil {out["type"]=mismatch.Type.String()} else {out["type_nil"]=true}
 case errors.As(err,&syntactic):out["offset"]=syntactic.ByteOffset;out["pointer_hex"]=hex.EncodeToString([]byte(syntactic.JSONPointer))
 case errors.As(err,&syntax):out["offset"]=syntax.Offset
 }
 return out
}
func observe(model string,input []byte,legacy bool) (json.RawMessage,map[string]any) {
 var target any
 switch model {
 case "int":v:=7;target=&v
 case "record":v:=record{7,9,11};target=&v
 case "pointer-record":v:=pointerRecord{7,nil,11};target=&v
 case "slice":v:=[]int{7,8,9};target=&v
 case "array":v:=[3]int{7,8,9};target=&v
 case "map":v:=map[string]int{"keep":7,"a":9};target=&v
 case "any":var v any=map[string]any{"old":float64(7)};target=&v
 case "raw":v:=json.RawMessage("7");target=&v
 case "json-hook":v:=jsonHook(7);target=&v
 case "text-hook":v:=textHook("prior");target=&v
 default:panic(model)
 }
 err:=jsonv2.Unmarshal(input,target,json.DefaultOptionsV1(),json.ReportErrorsWithLegacySemantics(legacy))
 var state []byte;var marshalErr error
 if raw,ok:=target.(*json.RawMessage);ok {state,marshalErr=json.Marshal(hex.EncodeToString(*raw))} else {state,marshalErr=json.Marshal(target)}
 if marshalErr!=nil {panic(marshalErr)}
 return state,failure(err)
}
func main() {
 models:=[]struct{name,canonical string}{
  {"int","123"},{"record",`{"before":1,"value":2,"after":3}`},{"pointer-record",`{"before":1,"value":2,"after":3}`},
  {"slice",`[1,2,3]`},{"array",`[1,2,3]`},{"map",`{"a":1,"b":2}`},{"any",`{"a":[1,2],"b":true}`},{"raw",`{"a":[1,2]}`},{"json-hook",`{"a":[1,2]}`},{"text-hook",`"new"`},
 }
 malformed:=[]string{"", " ", "@", "-", "1e", "1e+", "1x", "truex", `"ok"x`, `1 2`, `nullx`, `[1]x`, `[`, `[1,`, `[1,@]`, `[true,@]`, `[1,,2]`, `{"before":1 "value":2}`, `{"before":1,"value":@}`, `{"before":1,"value":`, `{"before":1,"value":"x}`, `{"before":1,"value":2,"after":`, `{"before":1,"value":2,"after":3}x`, `{"a":1,"a":2,}`, `{"a":1,"b":"x`, `{"a":[1,2`, `{"a":1} {"a":2}`, `{"before":1,"unknown":[2,@],"after":3}`, `"\u12"`, `"\x"`}
 rows:=[]map[string]any{}
 for _,model:=range models {
  inputs:=[]struct{kind string;at int;wire string}{}
  for at:=0;at<=len(model.canonical);at++ {inputs=append(inputs,struct{kind string;at int;wire string}{"canonical-prefix",at,model.canonical[:at]})}
  for at,wire:=range malformed {inputs=append(inputs,struct{kind string;at int;wire string}{"malformed",at,wire})}
  for _,input:=range inputs {for _,legacy:=range []bool{true,false} {
   calls=[]string{};state,err:=observe(model.name,[]byte(input.wire),legacy)
   rows=append(rows,map[string]any{"model":model.name,"state_encoding":func() string {if model.name=="raw" {return "raw-bytes-hex"};return "json"}(),"case":input.kind,"case_index":input.at,"input_hex":hex.EncodeToString([]byte(input.wire)),"legacy_errors":legacy,"state":state,"error":err,"calls":calls})
  }}
 }
 encoder:=json.NewEncoder(os.Stdout);encoder.SetIndent("","  ");if err:=encoder.Encode(rows);err!=nil {panic(err)}
}
