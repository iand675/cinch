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

// opA observes public workload 0 without a per-operation string or integer dispatch.
func opA(table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,index uint32,cp,other rune,checksum uint64)uint64{return mix(checksum,flag(unicode.IsLetter(cp),1))}

// opB observes public workload 1 without a per-operation string or integer dispatch.
func opB(table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,index uint32,cp,other rune,checksum uint64)uint64{return mix(checksum,flag(unicode.IsSpace(cp),1))}

// opC observes public workload 2 without a per-operation string or integer dispatch.
func opC(table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,index uint32,cp,other rune,checksum uint64)uint64{return mix(checksum,classification(cp))}

// opD observes public workload 3 without a per-operation string or integer dispatch.
func opD(table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,index uint32,cp,other rune,checksum uint64)uint64{checksum=mix(checksum,uint32(unicode.ToUpper(cp)));checksum=mix(checksum,uint32(unicode.ToLower(cp)));checksum=mix(checksum,uint32(unicode.ToTitle(cp)));return mix(checksum,uint32(unicode.To(int(index&3),cp)))}

// opE observes public workload 4 without a per-operation string or integer dispatch.
func opE(table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,index uint32,cp,other rune,checksum uint64)uint64{checksum=mix(checksum,uint32(unicode.SimpleFold(cp)));return mix(checksum,flag(equalFold(cp,other),1))}

// opF observes public workload 5 without a per-operation string or integer dispatch.
func opF(table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,index uint32,cp,other rune,checksum uint64)uint64{return mix(checksum,flag(unicode.Is(table,cp),1))}

// opG observes public workload 6 without a per-operation string or integer dispatch.
func opG(table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,index uint32,cp,other rune,checksum uint64)uint64{return mix(checksum,flag(unicode.Is(table,cp),1))}

// opH observes public workload 7 without a per-operation string or integer dispatch.
func opH(table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,index uint32,cp,other rune,checksum uint64)uint64{return mix(checksum,flag(unicode.Is(table,cp),1))}

// opI observes public workload 8 without a per-operation string or integer dispatch.
func opI(table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,index uint32,cp,other rune,checksum uint64)uint64{return mix(checksum,flag(unicode.Is(table,cp),1))}

// opJ observes public workload 9 without a per-operation string or integer dispatch.
func opJ(table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,index uint32,cp,other rune,checksum uint64)uint64{return mix(checksum,flag(unicode.Is(table,cp),1))}

// opK observes public workload 10 without a per-operation string or integer dispatch.
func opK(table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,index uint32,cp,other rune,checksum uint64)uint64{return mix(checksum,flag(unicode.In(cp,tables...),1)|flag(unicode.IsOneOf(tables,cp),2))}

// opL observes public workload 11 without a per-operation string or integer dispatch.
func opL(table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,index uint32,cp,other rune,checksum uint64)uint64{return mix(checksum,catalog(index,cp))}

// opM observes public workload 12 without a per-operation string or integer dispatch.
func opM(table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,index uint32,cp,other rune,checksum uint64)uint64{checksum=mix(checksum,uint32(special.ToUpper(cp)));checksum=mix(checksum,uint32(special.ToLower(cp)));return mix(checksum,uint32(special.ToTitle(cp)))}

// opN observes public workload 13 without a per-operation string or integer dispatch.
func opN(table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,index uint32,cp,other rune,checksum uint64)uint64{checksum=mix(checksum,uint32(special.ToUpper(cp)));checksum=mix(checksum,uint32(special.ToLower(cp)));return mix(checksum,uint32(special.ToTitle(cp)))}

// opO observes public workload 14 without a per-operation string or integer dispatch.
func opO(table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,index uint32,cp,other rune,checksum uint64)uint64{value:=unicode.CaseRanges[index];checksum=mix(checksum,value.Lo);checksum=mix(checksum,value.Hi);for _,delta:=range value.Delta{checksum=mix(checksum,uint32(delta))};return checksum}

// opP observes public workload 15 without a per-operation string or integer dispatch.
func opP(table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,index uint32,cp,other rune,checksum uint64)uint64{return mix(checksum,uint32(cp))}

// repeat dispatches once per timed sample and uses fixed-array inputs with a bounded machine phase.
func repeat(mode string,table *unicode.RangeTable,tables []*unicode.RangeTable,special unicode.SpecialCase,values,partners *[16]rune,iterations uint32)uint64{
 checksum:=uint64(14695981039346656037)
 switch mode{
 case "letter":for index:=uint32(0);index<iterations;index++{at:=index&15;checksum=opA(table,tables,special,at,values[at],partners[at],checksum)}
 case "space":for index:=uint32(0);index<iterations;index++{at:=index&15;checksum=opB(table,tables,special,at,values[at],partners[at],checksum)}
 case "classify":for index:=uint32(0);index<iterations;index++{at:=index&15;checksum=opC(table,tables,special,at,values[at],partners[at],checksum)}
 case "case":for index:=uint32(0);index<iterations;index++{at:=index&15;checksum=opD(table,tables,special,at,values[at],partners[at],checksum)}
 case "fold":for index:=uint32(0);index<iterations;index++{at:=index&15;checksum=opE(table,tables,special,at,values[at],partners[at],checksum)}
 case "table-letter":for index:=uint32(0);index<iterations;index++{at:=index&15;checksum=opF(table,tables,special,at,values[at],partners[at],checksum)}
 case "table-surrogates":for index:=uint32(0);index<iterations;index++{at:=index&15;checksum=opG(table,tables,special,at,values[at],partners[at],checksum)}
 case "table-greek":for index:=uint32(0);index<iterations;index++{at:=index&15;checksum=opH(table,tables,special,at,values[at],partners[at],checksum)}
 case "table-space":for index:=uint32(0);index<iterations;index++{at:=index&15;checksum=opI(table,tables,special,at,values[at],partners[at],checksum)}
 case "table-custom":for index:=uint32(0);index<iterations;index++{at:=index&15;checksum=opJ(table,tables,special,at,values[at],partners[at],checksum)}
 case "table-union":for index:=uint32(0);index<iterations;index++{at:=index&15;checksum=opK(table,tables,special,at,values[at],partners[at],checksum)}
 case "catalog":for index:=uint32(0);index<iterations;index++{at:=index&15;checksum=opL(table,tables,special,at,values[at],partners[at],checksum)}
 case "special-turkish":for index:=uint32(0);index<iterations;index++{at:=index&15;checksum=opM(table,tables,special,at,values[at],partners[at],checksum)}
 case "special-custom":for index:=uint32(0);index<iterations;index++{at:=index&15;checksum=opN(table,tables,special,at,values[at],partners[at],checksum)}
 case "case-observe":for index:=uint32(0);index<iterations;index++{at:=index&15;checksum=opO(table,tables,special,at,values[at],partners[at],checksum)}
 case "harness":for index:=uint32(0);index<iterations;index++{at:=index&15;checksum=opP(table,tables,special,at,values[at],partners[at],checksum)}
 default:panic("unknown Unicode workload")
 };return checksum
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
 var numeric,paired [16]rune
 if len(values.Patterns[pattern])!=16||len(values.Partners[pattern])!=16{panic("wrong bounded input shape")}
 copy(numeric[:],values.Patterns[pattern]);copy(paired[:],values.Partners[pattern])
 start:=time.Now();checksum:=repeat(os.Args[2],table,tables,special,&numeric,&paired,iterations)
 fmt.Println(time.Since(start).Nanoseconds(),checksum)
}
