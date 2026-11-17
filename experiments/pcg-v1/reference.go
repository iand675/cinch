// Command reference emits independently observed deterministic generator states.
package main

import (
 "encoding/hex"
 "encoding/json"
 "math/rand/v2"
 "os"
)

// Sample retains a full output sequence and the encoded state on each side.
type Sample struct {
 High uint64 `json:"high"`
 Low uint64 `json:"low"`
 Before string `json:"before"`
 Values []uint64 `json:"values"`
 After string `json:"after"`
 Appended string `json:"appended"`
}

// main observes the public generator without embedding the candidate arithmetic.
func main() {
 seeds := [][2]uint64{{0,0},{0,1},{1,0},{1,2},{^uint64(0),^uint64(0)},{^uint64(0),0},{0,^uint64(0)},{0x0123456789abcdef,0xfedcba9876543210}}
 rows:=make([]Sample,0,len(seeds))
 for _,seed:=range seeds {
  p:=rand.NewPCG(seed[0],seed[1])
  before,err:=p.MarshalBinary();if err!=nil {panic(err)}
  row:=Sample{High:seed[0],Low:seed[1],Before:hex.EncodeToString(before)}
  for i:=0;i<256;i++ {row.Values=append(row.Values,p.Uint64())}
  after,err:=p.MarshalBinary();if err!=nil {panic(err)}
  row.After=hex.EncodeToString(after)
  appended,err:=p.AppendBinary([]byte{0,255,128,1});if err!=nil {panic(err)}
  row.Appended=hex.EncodeToString(appended)
  rows=append(rows,row)
 }
 if err:=json.NewEncoder(os.Stdout).Encode(rows);err!=nil {panic(err)}
}
