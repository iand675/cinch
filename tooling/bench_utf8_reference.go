// Measure public raw-byte UTF-8 pipelines with identical input mutation and observable results.
package main

import("bytes";"fmt";"os";"strconv";"strings";"time";"unicode/utf8")

// retained forces returned buffers to escape so fresh copies remain part of the operation.
var retained []byte

// fingerprint observes allocation length and three bounded result positions.
func fingerprint(input []byte) uint64 {
 retained=input
 if len(input)==0{return 0}
 return uint64(len(input))+uint64(input[0])+uint64(input[len(input)/2])+uint64(input[len(input)-1])
}

// eachRepair composes standard decoding with exact per-malformed-byte replacement semantics.
func eachRepair(input,replacement []byte) []byte {
 output:=make([]byte,0,len(input))
 for at:=0;at<len(input); {cp,width:=utf8.DecodeRune(input[at:]);if cp==utf8.RuneError&&width==1{output=append(output,replacement...)}else{output=append(output,input[at:at+width]...)};at+=width}
 return output
}

// operation preserves byte-returning allocation costs and observes every public result.
func operation(mode string,input,replacement []byte) uint64 {
 switch mode {
 case "count":return uint64(utf8.RuneCount(input))
 case "valid":if utf8.Valid(input){return 1};return 0
 case "repair":return fingerprint([]byte(strings.ToValidUTF8(string(input),string(replacement))))
 case "repair-each":return fingerprint(eachRepair(input,replacement))
 case "bytes-copy":return fingerprint(bytes.Clone(input))
 default:panic("unknown UTF-8 workload")
 }
}

// main prepares deterministic input before timing, then retains all iteration outcomes.
func main(){
 if len(os.Args)!=5{panic("expected mode,size,iterations,pattern")}
 size,_:=strconv.Atoi(os.Args[2]);iterations,_:=strconv.Atoi(os.Args[3]);pattern,_:=strconv.Atoi(os.Args[4])
 alphabet:=[]byte{97,98,99,100,101,102,103,104}
 if pattern==1{alphabet=[]byte{97,195,169,206,163,226,130,172}}else if pattern==2{alphabet=[]byte{97,255,128,0,192,240,128,191}}
 input:=make([]byte,size);for i:=range input {input[i]=alphabet[i%len(alphabet)]}
 replacement:=[]byte{239,191,189};start:=time.Now();var checksum uint64
 for i:=0;i<iterations;i++{if len(input)>0{input[0]=byte(97+i%2)};checksum=(checksum+operation(os.Args[1],input,replacement))%1000000007}
 fmt.Println(time.Since(start).Nanoseconds(),checksum)
}
