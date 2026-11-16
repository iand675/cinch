// Generate independent UTF-8 byte, scalar, prefix and repair observations.
package main

import (
 "encoding/hex"
 "encoding/json"
 "fmt"
 "os"
 "runtime"
 "sort"
 "strings"
 "unicode/utf8"
)

// word hashes every byte of a complete observation in explicit little-endian order.
func word(hash uint64, value uint32) uint64 {
 for shift:=0;shift<32;shift+=8 {hash^=uint64(byte(value>>shift));hash*=1099511628211};return hash
}

// flag represents a public boolean observation without depending on JSON object order.
func flag(value bool) uint32 {if value{return 1};return 0}

// observe hashes every forward/backward position, including end and explicit out-of-bounds status.
func observe(hash uint64, input []byte) uint64 {
 hash=word(hash,uint32(utf8.RuneCount(input)));hash=word(hash,flag(utf8.Valid(input)));hash=word(hash,flag(utf8.FullRune(input)))
 for at:=0;at<=len(input)+1;at++ {
  var cp rune=utf8.RuneError; width,status:=0,0
  if at<len(input) {cp,width=utf8.DecodeRune(input[at:]);status=1;if cp==utf8.RuneError&&width==1 {status=2}} else if at>len(input){status=3}
  hash=word(hash,uint32(cp));hash=word(hash,uint32(width));hash=word(hash,uint32(status))
  cp=utf8.RuneError;width=0;status=0
  if at>len(input){status=3}else if at>0 {cp,width=utf8.DecodeLastRune(input[:at]);status=1;if cp==utf8.RuneError&&width==1{status=2}}
  hash=word(hash,uint32(cp));hash=word(hash,uint32(width));hash=word(hash,uint32(status))
 }
 first:=uint32(0xFFFFFFFF)
 for at:=0;at<len(input); {cp,width:=utf8.DecodeRune(input[at:]);if cp==utf8.RuneError&&width==1{first=uint32(at);break};at+=width}
 hash=word(hash,first)
 return hash
}

// eachRepair retains valid bytes and replaces every individually malformed byte.
func eachRepair(input,replacement []byte) []byte {
 out:=[]byte{}
 for at:=0;at<len(input); {cp,width:=utf8.DecodeRune(input[at:]);if cp==utf8.RuneError&&width==1 {out=append(out,replacement...)}else{out=append(out,input[at:at+width]...)};at+=width}
 return out
}

// chunk records a bounded group for poisoned ownership validation.
type chunk struct {Start int `json:"start"`;End int `json:"end"`;Hash uint64 `json:"hash"`}
// row preserves malformed byte inputs and arbitrary invalid replacement bytes explicitly.
type row struct {Input string `json:"input"`;Hash uint64 `json:"hash"`;Replacement string `json:"replacement"`;Repair string `json:"repair"`;Each string `json:"each"`}

// main covers every numeric code point and byte pair, plus malformed and alignment boundaries.
func main(){
 if runtime.Version() != "go1.27.4" {panic("unexpected reference SDK: "+runtime.Version())}
 scalar:=[]chunk{}
 for start:=0;start<=utf8.MaxRune;start+=2048 {
  hash:=uint64(14695981039346656037)
  for cp:=start;cp<start+2048;cp++ {
   encoded:=utf8.AppendRune(nil,rune(cp))
   hash=word(hash,flag(utf8.ValidRune(rune(cp))));hash=word(hash,uint32(int32(utf8.RuneLen(rune(cp)))))
   hash=word(hash,uint32(len(encoded)))
   for _,b:=range encoded {hash=word(hash,uint32(b))}
   hash=observe(hash,encoded)
  }
  scalar=append(scalar,chunk{start,start+2048,hash})
 }
 pairs:=[]chunk{}
 for start:=0;start<65536;start+=256 {
  hash:=uint64(14695981039346656037)
  for value:=start;value<start+256;value++ {hash=observe(hash,[]byte{byte(value>>8),byte(value)})}
  pairs=append(pairs,chunk{start,start+256,hash})
 }
 inputs:=map[string]bool{"":true}
 add:=func(input []byte){inputs[string(input)]=true}
 scalars:=[]rune{0,1,127,128,2047,2048,0xD7FF,0xE000,0xFFFF,0x10000,0x10FFFF,0xFFFD,0x1F600,0x16EA0}
 for _,cp:=range scalars {
  encoded:=utf8.AppendRune(nil,cp)
  for split:=0;split<=len(encoded);split++{add(encoded[:split]);add(encoded[split:])}
  for align:=0;align<17;align++ {prefix:=[]byte(strings.Repeat("a",align));add(append(append(prefix,encoded...),[]byte("tail")...))}
 }
 boundaries:=[]byte{0,0x7F,0x80,0x8F,0x90,0x9F,0xA0,0xBF,0xC0,0xFF}
 for _,lead:=range []byte{0xC0,0xC1,0xC2,0xDF,0xE0,0xED,0xEF,0xF0,0xF4,0xF5,0xFF} {
  for _,second:=range boundaries {for _,third:=range boundaries {
   add([]byte{lead,second,third})
   for _,fourth:=range boundaries {add([]byte{lead,second,third,fourth})}
  }}
 }
 for align:=0;align<17;align++ {
  for count:=1;count<=8;count++ {
   input:=[]byte(strings.Repeat("x",align));for j:=0;j<count;j++{input=append(input,0xFF)};input=append(input,[]byte("valid")...);add(input)
  }
 }
 state:=uint32(0x1234ABCD)
 for size:=0;size<257;size++ {input:=make([]byte,size);for i:=range input {state=1664525*state+1013904223;input[i]=byte(state>>24)};add(input)}
 keys:=[]string{};for input:=range inputs{keys=append(keys,input)};sort.Strings(keys)
 rows:=[]row{}
 replacements:=[][]byte{{},{'?'},{0xEF,0xBF,0xBD},{0xFF,0,0x80}}
 for index,input:=range keys {
  replacement:=replacements[index%len(replacements)]
  rows=append(rows,row{hex.EncodeToString([]byte(input)),observe(14695981039346656037,[]byte(input)),hex.EncodeToString(replacement),hex.EncodeToString([]byte(strings.ToValidUTF8(input,string(replacement)))),hex.EncodeToString(eachRepair([]byte(input),replacement))})
 }
 output:=map[string]any{"sdk_version":runtime.Version(),"scalar_chunks":scalar,"pair_chunks":pairs,"rows":rows}
 encoder:=json.NewEncoder(os.Stdout);encoder.SetIndent("","  ");if err:=encoder.Encode(output);err!=nil{panic(fmt.Sprint(err))}
}
