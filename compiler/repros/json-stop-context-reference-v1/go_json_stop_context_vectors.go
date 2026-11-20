// Independent observations distinguish precise semantic pointers and stopped application state.
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

type pair struct { A int `json:"a"`; B int `json:"b"` }
func main() {
 rows:=[]map[string]any{}
 for _,input:=range []string{`{"a":"bad","b":2}`,`{"a":1,"b":`} {
  for _,legacy:=range []bool{true,false} {
   value:=pair{7,8}
   err:=jsonv2.Unmarshal([]byte(input), &value, json.DefaultOptionsV1(),json.ReportErrorsWithLegacySemantics(legacy))
   row:=map[string]any{"input_hex":hex.EncodeToString([]byte(input)),"legacy_errors":legacy,"state":[]int{value.A,value.B},"family":fmt.Sprintf("%T",err)}
   if err!=nil {row["text"]=err.Error()}
   var semantic *jsonv2.SemanticError
   var mismatch *json.UnmarshalTypeError
   var syntactic *jsontext.SyntacticError
   var syntax *json.SyntaxError
   switch {
   case errors.As(err,&semantic):
    row["offset"]=semantic.ByteOffset;row["pointer_hex"]=hex.EncodeToString([]byte(semantic.JSONPointer));row["kind"]=uint8(semantic.JSONKind);row["reported_hex"]=hex.EncodeToString(semantic.JSONValue)
    if semantic.GoType!=nil {row["type"]=semantic.GoType.String()} else {row["type_nil"]=true}
   case errors.As(err,&mismatch):
    row["offset"]=mismatch.Offset;row["field"]=mismatch.Field;row["record"]=mismatch.Struct;row["type"]=mismatch.Type.String();row["value"]=mismatch.Value
   case errors.As(err,&syntactic):
    row["offset"]=syntactic.ByteOffset;row["pointer_hex"]=hex.EncodeToString([]byte(syntactic.JSONPointer))
   case errors.As(err,&syntax):row["offset"]=syntax.Offset
   }
   rows=append(rows,row)
  }
 }
 encoder:=json.NewEncoder(os.Stdout);encoder.SetIndent("","  ");if err:=encoder.Encode(rows);err!=nil {panic(err)}
}
