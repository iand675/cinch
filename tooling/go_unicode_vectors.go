// Generate independent public Unicode and scalar-mapping observations for the pinned edition.
package main

import (
	"encoding/json"
	"fmt"
	"os"
	"runtime"
	"sort"
	"unicode"
	"unicode/utf8"
)

// observe records independent class bits and all simple scalar-mapping operations.
func observe(cp rune) []uint32 {
	tests := []bool{unicode.IsControl(cp), unicode.IsDigit(cp), unicode.IsLetter(cp), unicode.IsLower(cp), unicode.IsUpper(cp), unicode.IsTitle(cp), unicode.IsMark(cp), unicode.IsNumber(cp), unicode.IsPunct(cp), unicode.IsSymbol(cp), unicode.IsGraphic(cp), unicode.IsPrint(cp), unicode.IsSpace(cp), utf8.ValidRune(cp)}
	var flags uint32
	for index, value := range tests {
		if value {
			flags |= 1 << index
		}
	}
	return []uint32{flags, uint32(unicode.ToUpper(cp)), uint32(unicode.ToLower(cp)), uint32(unicode.ToTitle(cp)), uint32(unicode.SimpleFold(cp))}
}

// hashWords observes all four bytes of each result in explicit little-endian order.
func hashWords(hash uint64, words []uint32) uint64 {
	for _, word := range words {
		for shift := 0; shift < 32; shift += 8 {
			hash ^= uint64(byte(word >> shift))
			hash *= 1099511628211
		}
	}
	return hash
}

// metadataWords combines public table observations with independently sorted canonical names.
func metadataWords() []uint64 {
	categories := []string{}
	for name := range unicode.Categories {
		if len(name) == 2 && name != "LC" {
			categories = append(categories, name)
		}
	}
	sort.Strings(categories)
	scripts, properties := []string{}, []string{}
	for name := range unicode.Scripts {
		scripts = append(scripts, name)
	}
	for name := range unicode.Properties {
		properties = append(properties, name)
	}
	sort.Strings(scripts)
	sort.Strings(properties)
	words := make([]uint64, unicode.MaxRune+1)
	defaultCategory := uint64(sort.SearchStrings(categories, "Cn"))
	for cp := range words {
		words[cp] = defaultCategory | 255<<5
	}
	visit := func(table *unicode.RangeTable, apply func(int)) {
		for _, span := range table.R16 {
			for cp := uint32(span.Lo); cp <= uint32(span.Hi); cp += uint32(span.Stride) {
				apply(int(cp))
			}
		}
		for _, span := range table.R32 {
			for cp := span.Lo; cp <= span.Hi; cp += span.Stride {
				apply(int(cp))
			}
		}
	}
	for index, name := range categories {
		visit(unicode.Categories[name], func(cp int) { words[cp] = words[cp]&^31 | uint64(index) })
	}
	for index, name := range scripts {
		visit(unicode.Scripts[name], func(cp int) { words[cp] = words[cp]&^(255<<5) | uint64(index)<<5 })
	}
	for index, name := range properties {
		visit(unicode.Properties[name], func(cp int) { words[cp] |= uint64(1) << uint(13+index) })
	}
	index := 0
	for _, tables := range []map[string]*unicode.RangeTable{unicode.FoldCategory, unicode.FoldScript} {
		names := []string{}
		for name := range tables {
			names = append(names, name)
		}
		sort.Strings(names)
		for _, name := range names {
			bit := uint(13 + len(properties) + index)
			visit(tables[name], func(cp int) { words[cp] |= uint64(1) << bit })
			index++
		}
	}
	return words
}

// hashWide observes every byte of one full metadata word, independent of its host byte order.
func hashWide(hash, word uint64) uint64 {
	for shift := 0; shift < 64; shift += 8 {
		hash ^= uint64(byte(word >> shift))
		hash *= 1099511628211
	}
	return hash
}

// scalarChunks includes every code point, with separate scalar-only observations for Char APIs.
func scalarChunks() []interface{} {
	rows := []interface{}{}
	metadata := metadataWords()
	for start := 0; start <= unicode.MaxRune; start += 2048 {
		end := start + 2048
		if end > unicode.MaxRune+1 {
			end = unicode.MaxRune + 1
		}
		numeric, chars := uint64(14695981039346656037), uint64(14695981039346656037)
		packed := uint64(14695981039346656037)
		for cp := start; cp < end; cp++ {
			words := observe(rune(cp))
			packed = hashWide(packed, metadata[cp])
			numeric = hashWords(numeric, words)
			if utf8.ValidRune(rune(cp)) {
				chars = hashWords(chars, words)
			}
		}
		rows = append(rows, map[string]interface{}{"start": start, "end": end, "numeric_hash": numeric, "char_hash": chars, "metadata_hash": packed})
	}
	return rows
}

// numericEdges preserves wrong numeric inputs and every scalar/surrogate boundary explicitly.
func numericEdges() []interface{} {
	rows := []interface{}{}
	for _, cp := range []rune{-2147483648, -1114112, -57344, -55296, -1, 0, 1, 0x7F, 0x80, 0xFF, 0x100, 0xD7FF, 0xD800, 0xDBFF, 0xDC00, 0xDFFF, 0xE000, 0xFFFD, 0xFFFF, 0x10000, 0x10FFFE, 0x10FFFF, 0x110000, 2147483647} {
		selectors := []interface{}{}
		for _, which := range []int{-2147483648, -3, -1, 0, 1, 2, 3, 2147483647} {
			selectors = append(selectors, []int64{int64(which), int64(unicode.To(which, cp))})
		}
		rows = append(rows, map[string]interface{}{"input": cp, "words": observe(cp), "selectors": selectors})
	}
	return rows
}

// rangeRows observes every public named range table, including source stride and Latin metadata.
func rangeRows() []interface{} {
	rows := []interface{}{}
	for _, catalog := range []struct {
		name   string
		tables map[string]*unicode.RangeTable
	}{{"category", unicode.Categories}, {"script", unicode.Scripts}, {"property", unicode.Properties}, {"fold_category", unicode.FoldCategory}, {"fold_script", unicode.FoldScript}} {
		names := []string{}
		for name := range catalog.tables {
			names = append(names, name)
		}
		sort.Strings(names)
		for _, name := range names {
			table := catalog.tables[name]
			r16, r32 := [][]uint32{}, [][]uint32{}
			for _, span := range table.R16 {
				r16 = append(r16, []uint32{uint32(span.Lo), uint32(span.Hi), uint32(span.Stride)})
			}
			for _, span := range table.R32 {
				r32 = append(r32, []uint32{span.Lo, span.Hi, span.Stride})
			}
			rows = append(rows, map[string]interface{}{"catalog": catalog.name, "name": name, "r16": r16, "r32": r32, "latin_offset": table.LatinOffset})
		}
	}
	return rows
}

// caseRows observes every exact source bound and signed delta of the public case tables.
func caseRows(ranges []unicode.CaseRange) [][]int64 {
	rows := [][]int64{}
	for _, span := range ranges {
		rows = append(rows, []int64{int64(span.Lo), int64(span.Hi), int64(span.Delta[0]), int64(span.Delta[1]), int64(span.Delta[2])})
	}
	return rows
}

// specialRows checks locale precedence, explicit identity overrides and default fallback.
func specialRows() []interface{} {
	rows := []interface{}{}
	cases := []struct {
		name  string
		rules unicode.SpecialCase
	}{{"turkish", unicode.TurkishCase}, {"azeri", unicode.AzeriCase}, {"empty", nil}, {"identity", unicode.SpecialCase{{Lo: 'A', Hi: 'Z', Delta: [3]rune{0, 0, 0}}}}, {"alternating", unicode.SpecialCase{{Lo: 0x100, Hi: 0x101, Delta: [3]rune{unicode.UpperLower, unicode.UpperLower, unicode.UpperLower}}}}, {"custom", unicode.SpecialCase{{Lo: 0xE9, Hi: 0xE9, Delta: [3]rune{'X' - 0xE9, 'X' - 0xE9, 'X' - 0xE9}}}}}
	for _, special := range cases {
		for _, cp := range []rune{-1, 0, 'A', 'B', 'I', 'Z', 'a', 'i', 'k', 'z', 0xE9, 0x100, 0x101, 0x130, 0x131, 0x212A, 0xD800, 0x10FFFF, 0x110000} {
			rows = append(rows, map[string]interface{}{"name": special.name, "rules": caseRows(special.rules), "input": cp, "upper": special.rules.ToUpper(cp), "lower": special.rules.ToLower(cp), "title": special.rules.ToTitle(cp)})
		}
	}
	return rows
}

// main rejects accidental edition drift before emitting complete independent observations.
func main() {
	if unicode.Version != "17.0.0" {
		panic(fmt.Sprintf("Unicode edition changed: %s", unicode.Version))
	}
	output := map[string]interface{}{"unicode_version": unicode.Version, "sdk_version": runtime.Version(), "scalar_chunks": scalarChunks(), "numeric_edges": numericEdges(), "ranges": rangeRows(), "case_ranges": caseRows(unicode.CaseRanges), "turkish_ranges": caseRows(unicode.TurkishCase), "azeri_ranges": caseRows(unicode.AzeriCase), "special": specialRows(), "category_aliases": unicode.CategoryAliases}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(output); err != nil {
		panic(err)
	}
}
