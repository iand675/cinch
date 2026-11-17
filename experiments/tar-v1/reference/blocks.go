// Command tar-block-reference records fixed-block observations through the installed public reader.
// Run this file explicitly; main.go is a separate complete-archive corpus program.
package main

import (
 "archive/tar"
 "bytes"
 "crypto/sha256"
 "encoding/hex"
 "encoding/json"
 "fmt"
 "os"
 "runtime"
 "time"
)

// mutation is one nonzero byte needed to reconstruct a full zero-initialized block.
type mutation struct { At int; Value byte }
// blockCase retains raw bytes, public reader observations and independent checksum reductions.
type blockCase struct {
 ID string
 Writes []mutation
 Start, Width int
 Numeric bool
 Number int64
 NumberError bool
 ReaderError string
 ReaderFormat int
 WireFormat int
 Unsigned, Signed int64
 BlockValid bool
 NameHex string
}
// corpus records the exact executing reference epoch and a bounded fixed-block corpus.
type corpus struct { SDK, OS, Arch string; Cases []blockCase; Encodings []encodingCase }
// encodingCase captures exact numeric bytes emitted by the independent public writer.
type encodingCase struct { Width int; Value int64; BytesHex, Error string }

// sums independently reduce byte values, replacing the checksum field by spaces.
func sums(block []byte)(int64,int64){var u,s int64;for i,c:=range block{if i>=148&&i<156{c=32};u+=int64(c);s+=int64(int8(c))};return u,s}
// seal stores either conventional checksum without depending on the candidate's codec.
func seal(block []byte,signed bool){u,s:=sums(block);if signed{u=s};text:=fmt.Sprintf("%06o\x00 ",u);if len(text)!=8{panic("fixture checksum range")};copy(block[148:156],text)}
// observe runs the public reader and preserves byte-exact raw mutations and names.
func observe(id string,block []byte,start,width int,numeric,valid bool,wire int)blockCase{
 c:=blockCase{ID:id,Start:start,Width:width,Numeric:numeric,BlockValid:valid,WireFormat:wire}
 c.Unsigned,c.Signed=sums(block)
 for i,b:=range block{if b!=0{c.Writes=append(c.Writes,mutation{i,b})}}
 h,err:=tar.NewReader(bytes.NewReader(block)).Next();if err!=nil{c.ReaderError=err.Error()};c.NumberError=err!=nil
 if h!=nil{c.ReaderFormat=int(h.Format);c.NameHex=hex.EncodeToString([]byte(h.Name));if start==100{c.Number=h.Mode}else if start==136{c.Number=h.ModTime.Unix()}}
 return c
}
// base constructs a minimal valid legacy regular-file header with no data body.
func base()[]byte{b:=make([]byte,512);b[0]='x';b[156]=tar.TypeReg;return b}
// numericCases checks both fixed widths through real public mode and modification-time fields.
func numericCases()[]blockCase{
 var out []blockCase
 for _,field:=range []struct{start,width int}{{100,8},{136,12}}{
  values:=[][]byte{nil,[]byte("       "),[]byte("\x00 \x00"),[]byte("0000007"),[]byte("0000008"),[]byte("000\x00bad"),[]byte("  123  "),[]byte("1 2"),[]byte("-1"),[]byte("+1")}
  for _,value:=range []int64{-9223372036854775808,-72057594037927937,-72057594037927936,-1,0,1,72057594037927935,72057594037927936,9223372036854775807}{
   raw:=make([]byte,field.width);x:=value;for i:=len(raw)-1;i>=0;i--{raw[i]=byte(x);x>>=8};raw[0]|=128;values=append(values,raw)
  }
  // A wide positive overflow and a noncanonical sign byte exercise liberal binary decoding.
  overflow:=make([]byte,field.width);overflow[0]=129;values=append(values,overflow)
  overflow=make([]byte,field.width);for i:=range overflow{overflow[i]=255};overflow[0]=254;values=append(values,overflow)
  values=append(values,bytes.Repeat([]byte{'7'},field.width))
  for i,v:=range values{b:=base();copy(b[field.start:field.start+field.width],v);seal(b,false);out=append(out,observe(fmt.Sprintf("numeric/%d/%d",field.width,i),b,field.start,field.width,true,true,0))}
 }
 return out
}
// numericEncodings records strict writer choices independently from liberal reader decoding.
func numericEncodings()[]encodingCase{
 var out []encodingCase
 for _,width:=range []int{8,12}{for _,value:=range []int64{-9223372036854775808,-72057594037927937,-72057594037927936,-1,0,1,2097151,2097152,72057594037927935,72057594037927936,9223372036854775807}{
  h:=&tar.Header{Name:"encoded",Format:tar.FormatGNU};start:=100
  if width==8{h.Mode=value}else{h.ModTime=time.Unix(value,0);start=136}
  var raw bytes.Buffer;w:=tar.NewWriter(&raw);err:=w.WriteHeader(h)
  c:=encodingCase{Width:width,Value:value};if err!=nil{c.Error=err.Error()}else{c.BytesHex=hex.EncodeToString(raw.Bytes()[start:start+width])};out=append(out,c)
 }}
 return out
}
// main emits a small standalone corpus and refuses an accidental reference-toolchain substitution.
func main(){
 if runtime.Version()!="go1.27.4"{panic("pinned SDK mismatch")}
 r:=corpus{SDK:runtime.Version(),OS:runtime.GOOS,Arch:runtime.GOARCH,Cases:numericCases(),Encodings:numericEncodings()}
 for _,signed:=range []bool{false,true}{b:=base();b[0]=255;seal(b,signed);r.Cases=append(r.Cases,observe(fmt.Sprintf("checksum/%v",signed),b,100,8,false,true,0))}
 for _,at:=range []int{0,147,148,155,156,511}{
  b:=base();seal(b,false);b[at]^=1
  c:=observe(fmt.Sprintf("corrupt/%d",at),b,100,8,false,false,0)
  // These mutations start from otherwise valid metadata: the public reader is authoritative.
  // Bytes after the checksum NUL do not necessarily invalidate its parsed value.
  c.BlockValid=c.ReaderError==""
  r.Cases=append(r.Cases,c)
 }
 for _,entry:=range []struct{name string;magic,version,trailer []byte;wire int}{{"v7",nil,nil,nil,0},{"ustar",[]byte("ustar\x00"),[]byte("00"),nil,1},{"ustar-version",[]byte("ustar\x00"),[]byte("XY"),nil,1},{"gnu",[]byte("ustar "),[]byte(" \x00"),nil,2},{"gnu-wrong-version",[]byte("ustar "),[]byte("00"),nil,0},{"star",[]byte("ustar\x00"),[]byte("00"),[]byte("tar\x00"),3}}{
  b:=base();copy(b[257:263],entry.magic);copy(b[263:265],entry.version);copy(b[508:512],entry.trailer);seal(b,false);r.Cases=append(r.Cases,observe("format/"+entry.name,b,100,8,false,true,entry.wire))
 }
 for _,end:=range []int{0,1,7,8,9,15,16,31,32,63,64,95,96,99,100}{b:=base();for i:=0;i<end;i++{b[i]=byte('a'+i%26)};if end==0{b[0]=0};seal(b,false);r.Cases=append(r.Cases,observe(fmt.Sprintf("name/%d",end),b,100,8,false,true,0))}
 encoded,err:=json.MarshalIndent(r,"","  ");if err!=nil{panic(err)};digest:=sha256.Sum256(encoded);fmt.Fprintf(os.Stderr,"cases=%d sha256=%x\n",len(r.Cases),digest);if _,err=os.Stdout.Write(append(encoded,'\n'));err!=nil{panic(err)}
}
