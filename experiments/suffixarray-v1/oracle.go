// Generate an independent, version-pinned suffix-index capability corpus.
package main

import (
 "bytes"
 "encoding/hex"
 "encoding/json"
 "fmt"
 "index/suffixarray"
 "os"
 "regexp"
 "slices"
 "sort"
)

// Input describes only test data; all expected observations are computed here.
type Input struct { Data []string `json:"data"`; Queries []string `json:"queries"`; Patterns []string `json:"patterns"`; Limits []int `json:"limits"` }
// Query retains both the observed implementation ordering and the sorted position set.
type Query struct { Needle string `json:"needle"`; Limit int `json:"limit"`; Offsets []int `json:"offsets"`; Positions []int `json:"positions"` }
// Regex retains bounded suffix-index choices separately from leftmost search semantics.
type Regex struct { Pattern string `json:"pattern"`; Limit int `json:"limit"`; Indexed [][]int `json:"indexed"`; Leftmost [][]int `json:"leftmost"` }
// Persist records exact stream progress including deliberate short-write behavior.
type Persist struct { Mode string `json:"mode"`; Error string `json:"error"`; Panic string `json:"panic,omitempty"`; Consumed int `json:"consumed"`; Bytes string `json:"bytes"` }
// Case contains direct SDK observations without using its private suffix array.
type Case struct { Data string `json:"data"`; SuffixOrder []int `json:"suffix_order"`; Queries []Query `json:"queries"`; Regex []Regex `json:"regex"`; Wire string `json:"wire"`; Persistence []Persist `json:"persistence"` }
// Writer optionally returns a short count without error, or a chosen failure.
type Writer struct { bytes.Buffer; Maximum int; Fail bool }
// Write preserves the actual number accepted, independently of the caller's contract.
func (w *Writer) Write(p []byte) (int,error) { if len(p)>w.Maximum {p=p[:w.Maximum]};n,_:=w.Buffer.Write(p);if w.Fail{return n,fmt.Errorf("planned-write")};return n,nil }
// readObservation bounds all input and records panics as observations, never expected Cinch behavior.
func readObservation(wire []byte, name string) (out Persist) {
 out.Mode=name;r:=bytes.NewReader(wire);index:=suffixarray.New([]byte("old-index"))
 defer func(){out.Consumed=len(wire)-r.Len();out.Bytes=hex.EncodeToString(index.Bytes());if failure:=recover();failure!=nil {out.Panic=fmt.Sprint(failure)}}()
 if err:=index.Read(r);err!=nil {out.Error=err.Error()};return
}
// positionMatches is an independent overlapping literal scan including embedded zero bytes.
func positionMatches(data,needle []byte) []int { var out []int;if len(needle)==0{return out};for at:=0;at+len(needle)<=len(data);at++ {if bytes.Equal(data[at:at+len(needle)],needle){out=append(out,at)}};return out }
// main requires explicit cases and emits deterministic direct SDK observations.
func main() {
 var input Input;if err:=json.NewDecoder(os.Stdin).Decode(&input);err!=nil {panic(err)}
 encoder:=json.NewEncoder(os.Stdout)
 for _,encoded:=range input.Data {
  data,err:=hex.DecodeString(encoded);if err!=nil {panic(err)};index:=suffixarray.New(data);out:=Case{Data:encoded}
  for at:=range data {out.SuffixOrder=append(out.SuffixOrder,at)}
  sort.Slice(out.SuffixOrder,func(a,b int)bool{return bytes.Compare(data[out.SuffixOrder[a]:],data[out.SuffixOrder[b]:])<0})
  for _,query:=range input.Queries {
   needle,err:=hex.DecodeString(query);if err!=nil {panic(err)}
   complete:=index.Lookup(needle,-1);positions:=slices.Clone(complete);slices.Sort(positions)
   if !slices.Equal(positions,positionMatches(data,needle)){panic("independent literal position disagreement")}
   for _,limit:=range input.Limits {offsets:=index.Lookup(needle,limit);ordered:=slices.Clone(offsets);slices.Sort(ordered);out.Queries=append(out.Queries,Query{query,limit,offsets,ordered})}
  }
  for _,pattern:=range input.Patterns {r,err:=regexp.Compile(pattern);if err!=nil {panic(err)};for _,limit:=range input.Limits {out.Regex=append(out.Regex,Regex{pattern,limit,index.FindAllIndex(r,limit),r.FindAllIndex(data,limit)})}}
  var wire bytes.Buffer;if err:=index.Write(&wire);err!=nil {panic(err)};out.Wire=hex.EncodeToString(wire.Bytes())
  seen:=map[int]bool{};for _,cut:=range []int{0,1,9,10,wire.Len()-1,wire.Len()} {if cut>=0&&cut<=wire.Len()&&!seen[cut]{seen[cut]=true;out.Persistence=append(out.Persistence,readObservation(wire.Bytes()[:cut],fmt.Sprintf("prefix-%d",cut)))}}
  trailer:=append(slices.Clone(wire.Bytes()),0x51,0x52);out.Persistence=append(out.Persistence,readObservation(trailer,"trailing-marker"))
  for _,fail:=range []bool{false,true} {w:=&Writer{Maximum:1,Fail:fail};err:=index.Write(w);observation:=Persist{Mode:fmt.Sprintf("short-writer-error-%v",fail),Bytes:hex.EncodeToString(w.Bytes())};if err!=nil {observation.Error=err.Error()};out.Persistence=append(out.Persistence,observation)}
  if err:=encoder.Encode(out);err!=nil {panic(err)}
 }
}
