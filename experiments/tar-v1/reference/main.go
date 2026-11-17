// Command tar-reference records independent archive bytes and public operation observations.
// It is intentionally separate from the Cinch implementation and has no generated-code dependency.
package main

import (
	"archive/tar"
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"io"
	"io/fs"
	"os"
	"path/filepath"
	"runtime"
	"sort"
	"strings"
	"testing/fstest"
	"time"
)

// wireTime distinguishes an absent timestamp from the Unix epoch.
type wireTime struct { Absent bool; Seconds int64; Nanos int }
// record preserves arbitrary attribute key and value bytes without JSON Unicode replacement.
type record struct { KeyHex, ValueHex string }
// header captures every public metadata field with byte-exact names and attribute values.
type header struct {
	NameHex, LinkHex, UnameHex, GnameHex string
	Type byte
	Size, Mode int64
	UID, GID int
	Mtime, Atime, Ctime wireTime
	Major, Minor int64
	Format int
	FormatName string
	PAX, Xattrs []record
}
// action retains progress, bytes, error identity and optional metadata from one public call.
type action struct { Op string; N int; DataHex, Error string; Header *header }
// vector retains complete input/output bytes and all ordered public observations.
type vector struct { ID, ArchiveHex, SHA256 string; Actions []action }
// report identifies the actual SDK execution and separates constructed from installed fixture inputs.
type report struct { SDK, OS, Arch, Debug string; Generated, Installed []vector; Formats map[string]string }

// hexString preserves every original byte of a possibly non-UTF-8 name.
func hexString(s string) string { return hex.EncodeToString([]byte(s)) }
// timestamp records normalized Unix seconds and nanoseconds along with explicit absence.
func timestamp(t time.Time) wireTime { return wireTime{t.IsZero(), t.Unix(), t.Nanosecond()} }
// records sorts byte-string keys for deterministic observation output.
func records(m map[string]string) []record {
	keys := make([]string, 0, len(m)); for k := range m { keys = append(keys, k) }; sort.Strings(keys)
	out := make([]record, 0, len(keys)); for _, k := range keys { out = append(out, record{hexString(k), hexString(m[k])}) }; return out
}
// snapshot converts a live public header into a byte-exact independent record.
func snapshot(h *tar.Header) *header {
	if h == nil { return nil }
	return &header{hexString(h.Name), hexString(h.Linkname), hexString(h.Uname), hexString(h.Gname), h.Typeflag,
		h.Size, h.Mode, h.Uid, h.Gid, timestamp(h.ModTime), timestamp(h.AccessTime), timestamp(h.ChangeTime),
		h.Devmajor, h.Devminor, int(h.Format), h.Format.String(), records(h.PAXRecords), records(h.Xattrs)}
}
// errorName records stable public identities while retaining additional diagnostics verbatim.
func errorName(err error) string {
	if err == nil { return "" }
	for _, item := range []struct{name string; err error}{
		{"EOF", io.EOF}, {"UnexpectedEOF", io.ErrUnexpectedEOF}, {"ErrHeader", tar.ErrHeader},
		{"ErrFieldTooLong", tar.ErrFieldTooLong}, {"ErrWriteTooLong", tar.ErrWriteTooLong},
		{"ErrWriteAfterClose", tar.ErrWriteAfterClose}, {"ErrInsecurePath", tar.ErrInsecurePath},
	} { if errors.Is(err, item.err) { return item.name } }
	return err.Error()
}
// observe traverses bodies with varying request sizes, including zero requests and overreads.
func observe(id string, raw []byte) vector {
	digest := sha256.Sum256(raw); v := vector{ID:id, ArchiveHex:hex.EncodeToString(raw), SHA256:hex.EncodeToString(digest[:])}
	r := tar.NewReader(bytes.NewReader(raw))
	for entries := 0; entries < 128; entries++ {
		h, err := r.Next(); v.Actions = append(v.Actions, action{Op:"next", Error:errorName(err), Header:snapshot(h)})
		if err != nil { break }
		for _, size := range []int{0, 1, 3, 511, 512, 513} {
			b := make([]byte, size); n, e := r.Read(b); v.Actions = append(v.Actions, action{Op:fmt.Sprintf("read/%d",size), N:n, DataHex:hex.EncodeToString(b[:n]), Error:errorName(e)})
		}
		// Next deliberately discards remaining physical bytes: it is a separate required operation.
	}
	return v
}
// encode records both writer action results and a fresh read traversal of the resulting bytes.
func encode(id string, h *tar.Header, body []byte) vector {
	var out bytes.Buffer; w := tar.NewWriter(&out)
	var ops []action
	err := w.WriteHeader(h); ops = append(ops, action{Op:"write-header", Error:errorName(err), Header:snapshot(h)})
	if err == nil { n, e := w.Write(body); ops = append(ops, action{Op:"write", N:n, Error:errorName(e)}) }
	ops = append(ops, action{Op:"flush", Error:errorName(w.Flush())})
	ops = append(ops, action{Op:"close", Error:errorName(w.Close())}, action{Op:"close-again", Error:errorName(w.Close())})
	n, e := w.Write([]byte{1}); ops = append(ops, action{Op:"write-after-close", N:n, Error:errorName(e)})
	v := observe(id, out.Bytes()); v.Actions = append(ops, v.Actions...); return v
}
// metadataCases exercises the full header domain without embedding expected codec behavior.
func metadataCases() []vector {
	var out []vector
	for _, format := range []tar.Format{tar.FormatUnknown,tar.FormatUSTAR,tar.FormatPAX,tar.FormatGNU} {
		for _, name := range []string{"plain", "dir/", strings.Repeat("x",100), strings.Repeat("p/",80)+"last", "café/λ", string([]byte{'a',255,'b'}), "nul\x00name"} {
			h := &tar.Header{Name:name, Mode:0640, Size:3, ModTime:time.Unix(123,499999999), Format:format}
			out = append(out, encode(fmt.Sprintf("name/%d/%s",format,hexString(name)),h,[]byte("abc")))
		}
		for _, kind := range []byte{0,tar.TypeReg,tar.TypeLink,tar.TypeSymlink,tar.TypeChar,tar.TypeBlock,tar.TypeDir,tar.TypeFifo,tar.TypeCont,tar.TypeGNUSparse,tar.TypeXHeader,tar.TypeXGlobalHeader,tar.TypeGNULongName,tar.TypeGNULongLink,255} {
			h:= &tar.Header{Name:"kind",Typeflag:kind,Linkname:"target",Mode:04755,Size:3,Devmajor:8,Devminor:1,Format:format}
			out=append(out,encode(fmt.Sprintf("type/%d/%d",format,kind),h,[]byte("abc")))
		}
		for _, value := range []int64{-9223372036854775808,-1,0,1,2097151,2097152,8589934591,8589934592,9223372036854775807} {
			h:= &tar.Header{Name:"numeric",Mode:value,Uid:int(value),Gid:int(value),ModTime:time.Unix(value,123456789),Format:format}
			out=append(out,encode(fmt.Sprintf("numeric/%d/%d",format,value),h,nil))
		}
	}
	for _, size := range []int64{-1,0,1,511,512,513} {
		for _, count := range []int{0,1,512,513} {
			out=append(out,encode(fmt.Sprintf("body/%d/%d",size,count),&tar.Header{Name:"body",Size:size,Mode:0600},bytes.Repeat([]byte{97},count)))
		}
	}
	for _, values := range []map[string]string{{"VENDOR.key":"value"},{"path":"override","size":"999"},{"SCHILY.xattr.user.x":"explicit"},{"GNU.sparse.map":"0,3"},{"bad=key":"x"},{"VENDOR.key":"a\x00b"}} {
		h:= &tar.Header{Name:"pax",Size:3,Mode:0600,Format:tar.FormatPAX,PAXRecords:values,Xattrs:map[string]string{"user.x":"attribute"},AccessTime:time.Unix(-1,1),ChangeTime:time.Unix(1,999999999)}
		out=append(out,encode(fmt.Sprintf("pax/%d",len(out)),h,[]byte("abc")))
	}
	return out
}
// treeCase exercises recursive filesystem ingestion and metadata conversion on an abstract tree.
func treeCase() vector {
	tree:=fstest.MapFS{"a":&fstest.MapFile{Data:[]byte("abc"),Mode:0644},"dir":&fstest.MapFile{Mode:fs.ModeDir|0755},"dir/b":&fstest.MapFile{Data:[]byte("def"),Mode:0600}}
	var out bytes.Buffer; w:=tar.NewWriter(&out); err:=w.AddFS(tree); closeErr:=w.Close()
	v:=observe("add-tree",out.Bytes());v.Actions=append([]action{{Op:"add-fs",Error:errorName(err)},{Op:"close",Error:errorName(closeErr)}},v.Actions...)
	for _, name := range []string{"a","dir"} { info,e:=fs.Stat(tree,name);if e!=nil{panic(e)};h,e:=tar.FileInfoHeader(info,"");v.Actions=append(v.Actions,action{Op:"file-info-header/"+name,Error:errorName(e),Header:snapshot(h)});if h!=nil{round,e:=tar.FileInfoHeader(h.FileInfo(),"");v.Actions=append(v.Actions,action{Op:"metadata-roundtrip/"+name,Error:errorName(e),Header:snapshot(round)})} }
	return v
}
// mutationCases independently construct accepted and malformed checksum/numeric framing inputs.
func mutationCases() []vector {
	var encoded bytes.Buffer;w:=tar.NewWriter(&encoded)
	if err:=w.WriteHeader(&tar.Header{Name:"raw",Mode:0600,Size:0,Format:tar.FormatUSTAR});err!=nil{panic(err)}
	if err:=w.Close();err!=nil{panic(err)}
	base:=encoded.Bytes();var out []vector
	seal:=func(block []byte,signed bool){sum:=int64(0);for i,c:=range block[:512]{if i>=148&&i<156{c=32};if signed{sum+=int64(int8(c))}else{sum+=int64(c)}};text:=fmt.Sprintf("%06o\x00 ",sum);if len(text)!=8{panic("checksum fixture outside field")};copy(block[148:156],text)}
	for _,signed:=range []bool{false,true}{b:=bytes.Clone(base);b[0]=255;seal(b,signed);out=append(out,observe(fmt.Sprintf("checksum/signed-%v",signed),b))}
	for _,field:=range []struct{name string;start,width int}{{"mode",100,8},{"uid",108,8},{"gid",116,8},{"size",124,12},{"mtime",136,12}}{
		for _,value:=range [][]byte{[]byte(" \x0000007\x00"),[]byte("0000008\x00"),[]byte("000\x00bad"),{128,0,0,0,0,0,0,1},{255,255,255,255,255,255,255,255}}{
			b:=bytes.Clone(base);clear(b[field.start:field.start+field.width]);copy(b[field.start:field.start+field.width],value);seal(b,false)
			out=append(out,observe("numeric-raw/"+field.name+"/"+hex.EncodeToString(value),b))
		}
	}
	for _,offset:=range []int{0,147,148,155,156,511}{b:=bytes.Clone(base);b[offset]^=1;out=append(out,observe(fmt.Sprintf("checksum/corrupt-%d",offset),b))}
	for _,size:=range []int{1,511,512,513,1023}{out=append(out,observe(fmt.Sprintf("truncated/%d",size),base[:size]))}
	for _,name:=range []string{"../escape","/absolute","safe/../local","link"}{
		out=append(out,encode("locality/"+hexString(name),&tar.Header{Name:name,Typeflag:tar.TypeSymlink,Linkname:"../../target",Mode:0777},nil))
	}
	return out
}
// main writes one bounded independent corpus; existing installed fixtures add sparse and legacy coverage.
func main() {
	fixtureDir:=flag.String("fixtures","","installed archive fixture directory; raw .tar inputs only");flag.Parse()
	if runtime.Version()!="go1.27.4"{panic("reference SDK identity differs from the pinned version")}
	r:=report{SDK:runtime.Version(),OS:runtime.GOOS,Arch:runtime.GOARCH,Debug:os.Getenv("GODEBUG"),Generated:metadataCases(),Formats:map[string]string{}}
	r.Generated=append(r.Generated,treeCase())
	r.Generated=append(r.Generated,mutationCases()...)
	for _, n:=range []int{-1,0,1,2,3,4,6,8,14,16,31,32}{r.Formats[fmt.Sprint(n)]=tar.Format(n).String()}
	for _, n:=range []int{0,1,511,512,513,1024,1025}{r.Generated=append(r.Generated,observe(fmt.Sprintf("zero-trailer/%d",n),make([]byte,n)))}
	if *fixtureDir!="" { paths,e:=filepath.Glob(filepath.Join(*fixtureDir,"*.tar"));if e!=nil{panic(e)};sort.Strings(paths);for _,p:=range paths{b,e:=os.ReadFile(p);if e!=nil{panic(e)};if len(b)<=2<<20{r.Installed=append(r.Installed,observe("installed/"+filepath.Base(p),b))}} }
	encoder:=json.NewEncoder(os.Stdout);encoder.SetIndent("","  ");if err:=encoder.Encode(r);err!=nil{panic(err)}
}
