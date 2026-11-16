// Measure direct public Unicode scalar, table, case, fold and catalog pipelines with bounded counters.
package main

import("encoding/json";"fmt";"os";"strconv";"time";"unicode";"unicode/utf8")

// inputs preserves the exact bounded numeric observations shared by the independent drivers.
type inputs struct {Patterns [][]int32 `json:"patterns"`;Partners [][]int32 `json:"partners"`}

// mix retains every public result through the same unsigned wraparound arithmetic.
func mix(checksum uint64,value uint32) uint64{return (checksum^uint64(value))*1099511628211}

// flag encodes one observable boolean in a stable result bit.
func flag(value bool,mask uint32) uint32{if value{return mask};return 0}

// classification observes every public basic classification and scalar-validity predicate.
func classification(cp rune) uint32{
 tests:=[]bool{unicode.IsControl(cp),unicode.IsDigit(cp),unicode.IsLetter(cp),unicode.IsLower(cp),unicode.IsUpper(cp),unicode.IsTitle(cp),unicode.IsMark(cp),unicode.IsNumber(cp),unicode.IsPunct(cp),unicode.IsSymbol(cp),unicode.IsGraphic(cp),unicode.IsPrint(cp),unicode.IsSpace(cp),utf8.ValidRune(cp)}
 var result uint32;for index,value:=range tests{result|=flag(value,1<<index)};return result
}

// equalFold walks the actual simple-fold orbit, rather than reproducing the numeric increasing-cycle algorithm.
func equalFold(first,other rune) bool{
 if first==other{return true};next:=unicode.SimpleFold(first)
 for step:=0;step<16&&next!=first;step++{if next==other{return true};next=unicode.SimpleFold(next)}
 return false
}

// catalog resolves actual public maps during the operation and observes both presence and membership.
func catalog(index uint32,cp rune) uint32{
 var table *unicode.RangeTable
 switch index&3{
 case 0:if index<8{table=unicode.Categories["L"]}else{table=unicode.Categories["NoSuchCategory"]}
 case 1:if index<8{table=unicode.Scripts["Greek"]}else{table=unicode.Scripts["Adlam"]}
 case 2:if index<8{table=unicode.Properties["White_Space"]}else{table=unicode.Properties["Noncharacter_Code_Point"]}
 case 3:if index<8{table=unicode.FoldCategory["Ll"]}else{table=unicode.FoldCategory["NoSuchFold"]}
 }
 if table==nil{return 0};if unicode.Is(table,cp){return 3};return 1
}

// operation observes identical public capabilities; returned case structs need no heap boxes in this reference.
func operation(mode string,table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,index uint32,cp,other rune,checksum uint64)uint64{
 switch mode{
 case "harness":return mix(checksum,uint32(cp))
 case "letter":return mix(checksum,flag(unicode.IsLetter(cp),1))
 case "space":return mix(checksum,flag(unicode.IsSpace(cp),1))
 case "classify":return mix(checksum,classification(cp))
 case "case":checksum=mix(checksum,uint32(unicode.ToUpper(cp)));checksum=mix(checksum,uint32(unicode.ToLower(cp)));checksum=mix(checksum,uint32(unicode.ToTitle(cp)));return mix(checksum,uint32(unicode.To(int(index&3),cp)))
 case "fold":checksum=mix(checksum,uint32(unicode.SimpleFold(cp)));return mix(checksum,flag(equalFold(cp,other),1))
 case "table-letter","table-surrogates","table-greek","table-space","table-custom":return mix(checksum,flag(unicode.Is(table,cp),1))
 case "table-union":return mix(checksum,flag(unicode.In(cp,tables...),1)|flag(unicode.IsOneOf(tables,cp),2))
 case "catalog":return mix(checksum,catalog(index,cp))
 case "special-turkish","special-custom":checksum=mix(checksum,uint32(special.ToUpper(cp)));checksum=mix(checksum,uint32(special.ToLower(cp)));return mix(checksum,uint32(special.ToTitle(cp)))
 case "case-observe":value:=unicode.CaseRanges[index];checksum=mix(checksum,value.Lo);checksum=mix(checksum,value.Hi);for _,delta:=range value.Delta{checksum=mix(checksum,uint32(delta))};return checksum
 default:panic("unknown Unicode workload")
 }
}

// main prepares all immutable tables, locale rules and numeric inputs outside the monotonic timer.
func main(){
 if len(os.Args)!=5{panic("expected input snapshot,mode,pattern,iterations")}
 data,err:=os.ReadFile(os.Args[1]);if err!=nil{panic(err)};var values inputs;if err:=json.Unmarshal(data,&values);err!=nil{panic(err)}
 pattern,_:=strconv.Atoi(os.Args[3]);iterations64,_:=strconv.ParseUint(os.Args[4],10,32);iterations:=uint32(iterations64)
 table:=unicode.Letter
 switch os.Args[2]{case "table-surrogates":table=unicode.Cs;case "table-greek":table=unicode.Greek;case "table-space":table=unicode.White_Space;case "table-custom":table=&unicode.RangeTable{R16:[]unicode.Range16{{65,90,2},{931,963,16},{55296,57343,257}},R32:[]unicode.Range32{{65536,1114111,17}},LatinOffset:1}}
 tables:=[]*unicode.RangeTable{table,unicode.White_Space};special:=unicode.TurkishCase
 if os.Args[2]=="special-custom"{special=unicode.SpecialCase{{65,90,[3]rune{0,0,0}},{233,233,[3]rune{-145,-145,-145}},{256,257,[3]rune{unicode.UpperLower,unicode.UpperLower,unicode.UpperLower}},{66600,66639,[3]rune{-40,0,-40}}}}
 checksum:=uint64(14695981039346656037);start:=time.Now()
 for index:=uint32(0);index<iterations;index++{at:=index&15;checksum=operation(os.Args[2],table,tables,special,at,values.Patterns[pattern][at],values.Partners[pattern][at],checksum)}
 fmt.Println(time.Since(start).Nanoseconds(),checksum)
}
