// Command suffixarray_vectors generates independent reference vectors from Go's
// index/suffixarray package covering construction, Lookup (n = -1, 0, 1, 2, 5, 100),
// FindAllIndex with regular expressions, and binary Write/Read roundtrips.
//
//	GOCACHE=/tmp/go-cache GOTELEMETRY=off go run tooling/suffixarray_vectors.go > seed/tests/fixtures/suffixarray-vectors.json
package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"index/suffixarray"
	"os"
	"regexp"
	"slices"
)

type lookupVec struct {
	PatternHex string `json:"pattern_hex"`
	Limit      int    `json:"limit"`
	Offsets    []int  `json:"offsets"`
	Sorted     []int  `json:"sorted"`
	Count      int    `json:"count"`
}

type regexVec struct {
	Pattern string  `json:"pattern"`
	Limit   int     `json:"limit"`
	Matches [][]int `json:"matches"`
}

type corpusVec struct {
	Name        string      `json:"name"`
	DataHex     string      `json:"data_hex"`
	SA          []int       `json:"sa"`
	Lookups     []lookupVec `json:"lookups"`
	Regexes     []regexVec  `json:"regexes"`
	GoRoundtrip bool        `json:"go_roundtrip"`
}

type fixture struct {
	Corpora []corpusVec `json:"corpora"`
}

func fibonacciString(order int) []byte {
	a := []byte("a")
	b := []byte("ab")
	if order == 0 {
		return a
	}
	for i := 1; i < order; i++ {
		next := make([]byte, 0, len(a)+len(b))
		next = append(next, b...)
		next = append(next, a...)
		a, b = b, next
	}
	return b
}

func thueMorseString(order int) []byte {
	s := []byte("a")
	for i := 0; i < order; i++ {
		n := len(s)
		next := make([]byte, 2*n)
		copy(next, s)
		for j := 0; j < n; j++ {
			if s[j] == 'a' {
				next[n+j] = 'b'
			} else {
				next[n+j] = 'a'
			}
		}
		s = next
	}
	return s
}

func referenceSA(data []byte) []int {
	n := len(data)
	sa := make([]int, n)
	for i := 0; i < n; i++ {
		sa[i] = i
	}
	slices.SortFunc(sa, func(a, b int) int {
		return bytes.Compare(data[a:], data[b:])
	})
	return sa
}

func verifyGoRoundtrip(idx *suffixarray.Index, data []byte, queries [][]byte) bool {
	var buf bytes.Buffer
	if err := idx.Write(&buf); err != nil {
		panic(err)
	}
	var restored suffixarray.Index
	if err := restored.Read(bytes.NewReader(buf.Bytes())); err != nil {
		panic(err)
	}
	if !bytes.Equal(restored.Bytes(), data) {
		panic("go Write/Read roundtrip bytes mismatch")
	}
	for _, q := range queries {
		got := restored.Lookup(q, -1)
		want := idx.Lookup(q, -1)
		if !slices.Equal(got, want) {
			panic("go Write/Read roundtrip lookup mismatch")
		}
	}
	return true
}

func makeCorpus(name string, data []byte, queries [][]byte, patterns []string) corpusVec {
	idx := suffixarray.New(data)
	sa := referenceSA(data)
	limits := []int{-1, 0, 1, 2, 5, 100}

	lookups := []lookupVec{}
	for _, q := range queries {
		all := idx.Lookup(q, -1)
		for _, lim := range limits {
			raw := idx.Lookup(q, lim)
			if raw == nil {
				raw = []int{}
			}
			sorted := slices.Clone(raw)
			slices.Sort(sorted)
			lookups = append(lookups, lookupVec{
				PatternHex: hex.EncodeToString(q),
				Limit:      lim,
				Offsets:    raw,
				Sorted:     sorted,
				Count:      len(all),
			})
		}
	}

	regexes := []regexVec{}
	regexLimits := []int{-1, 0, 1, 2, 5}
	for _, pat := range patterns {
		re := regexp.MustCompile(pat)
		allSA := idx.FindAllIndex(re, -1)
		allRE := re.FindAllIndex(data, -1)
		if len(allSA) != len(allRE) {
			panic("suffixarray.FindAllIndex(-1) disagreed with regexp.FindAllIndex(-1)")
		}
		for i := range allSA {
			if allSA[i][0] != allRE[i][0] || allSA[i][1] != allRE[i][1] {
				panic("suffixarray.FindAllIndex(-1) pair mismatch")
			}
		}
		for _, lim := range regexLimits {
			// For n < 0 and n == 0, idx.FindAllIndex and re.FindAllIndex are identical.
			// For n > 0, Go's suffixarray.FindAllIndex returns at most n sorted non-overlapping
			// matches (not necessarily successive), whereas re.FindAllIndex returns the first n
			// successive matches, which always satisfies suffixarray.FindAllIndex's contract.
			matches := re.FindAllIndex(data, lim)
			saLimited := idx.FindAllIndex(re, lim)
			if len(matches) != len(saLimited) {
				panic("FindAllIndex limited count mismatch")
			}
			if matches == nil {
				matches = [][]int{}
			}
			regexes = append(regexes, regexVec{
				Pattern: pat,
				Limit:   lim,
				Matches: matches,
			})
		}
	}

	rt := verifyGoRoundtrip(idx, data, queries)

	return corpusVec{
		Name:        name,
		DataHex:     hex.EncodeToString(data),
		SA:          sa,
		Lookups:     lookups,
		Regexes:     regexes,
		GoRoundtrip: rt,
	}
}

func main() {
	all256 := make([]byte, 256)
	rev256 := make([]byte, 256)
	for i := 0; i < 256; i++ {
		all256[i] = byte(i)
		rev256[i] = byte(255 - i)
	}
	binRep := []byte{0x00, 0xff, 0x00, 0x80, 0x00, 0xff, 0x00, 0x00, 0xff, 0x7f, 0x00, 0xff}

	fib := fibonacciString(9)  // 55 bytes
	tm := thueMorseString(6)   // 64 bytes

	corpora := []corpusVec{
		makeCorpus(
			"empty",
			[]byte{},
			[][]byte{{}, []byte("a"), []byte{0x00}},
			[]string{"", "a*", "a+"},
		),
		makeCorpus(
			"one_byte_a",
			[]byte("a"),
			[][]byte{{}, []byte("a"), []byte("b"), []byte("aa")},
			[]string{"a", "b", "a*", "^a$"},
		),
		makeCorpus(
			"one_byte_nul",
			[]byte{0x00},
			[][]byte{{}, {0x00}, {0x01}},
			nil,
		),
		makeCorpus(
			"one_byte_ff",
			[]byte{0xff},
			[][]byte{{}, {0xff}, {0xfe}},
			nil,
		),
		makeCorpus(
			"repeated_aaaa",
			[]byte("aaaa"),
			[][]byte{{}, []byte("a"), []byte("aa"), []byte("aaa"), []byte("aaaa"), []byte("aaaaa"), []byte("b")},
			[]string{"a", "aa", "a*", "a+"},
		),
		makeCorpus(
			"repeated_10a",
			[]byte("aaaaaaaaaa"),
			[][]byte{[]byte("a"), []byte("aa"), []byte("aaa"), []byte("aaaaa")},
			[]string{"aa", "aaa"},
		),
		makeCorpus(
			"overlapping_ababa",
			[]byte("ababa"),
			[][]byte{[]byte("a"), []byte("b"), []byte("ab"), []byte("ba"), []byte("aba"), []byte("bab"), []byte("ababa"), []byte("abaab")},
			[]string{"aba", "a.", "a*", "b*"},
		),
		makeCorpus(
			"banana",
			[]byte("banana"),
			[][]byte{{}, []byte("a"), []byte("an"), []byte("ana"), []byte("na"), []byte("nana"), []byte("banana"), []byte("bananas"), []byte("z")},
			[]string{"an", "an|na", "a*", "a?", "^b", "a$", "[aeiou]+"},
		),
		makeCorpus(
			"mississippi",
			[]byte("mississippi"),
			[][]byte{[]byte("i"), []byte("s"), []byte("ss"), []byte("iss"), []byte("ssi"), []byte("ppi"), []byte("mississippi"), []byte("mip")},
			[]string{"i[ps]+", "s+"},
		),
		makeCorpus(
			"all_256_asc",
			all256,
			[][]byte{{0x00}, {0x7f}, {0x80}, {0xff}, {0x00, 0x01, 0x02}, {0xfd, 0xfe, 0xff}, {0xff, 0x00}},
			nil,
		),
		makeCorpus(
			"all_256_desc",
			rev256,
			[][]byte{{0x00}, {0xff}, {0xff, 0xfe, 0xfd}, {0x02, 0x01, 0x00}, {0x00, 0x01}},
			nil,
		),
		makeCorpus(
			"binary_repeated",
			binRep,
			[][]byte{{0x00}, {0xff}, {0x00, 0xff}, {0x00, 0xff, 0x00}, {0x80}, {0x00, 0x00}},
			nil,
		),
		makeCorpus(
			"fibonacci",
			fib,
			[][]byte{[]byte("a"), []byte("b"), []byte("aa"), []byte("ab"), []byte("ba"), []byte("aba"), []byte("abaab"), []byte("bb")},
			[]string{"aba", "a+b"},
		),
		makeCorpus(
			"thue_morse",
			tm,
			[][]byte{[]byte("a"), []byte("b"), []byte("aa"), []byte("bb"), []byte("abba"), []byte("baab"), []byte("aaa"), []byte("bbb")},
			[]string{"abba|baab", "a+|b+"},
		),
		makeCorpus(
			"english_text",
			[]byte("the quick brown fox jumps over the lazy dog; foo12 bar34 foo56 the end"),
			[][]byte{{}, []byte("the"), []byte(" "), []byte("foo"), []byte("fox"), []byte("lazy"), []byte("missing")},
			[]string{"the", "foo[0-9]+", "\\b[a-z]+\\b", "\\bthe\\b", "[0-9]+", "^the|end$"},
		),
		makeCorpus(
			"utf8_text",
			[]byte("café naïve résumé café π"),
			[][]byte{[]byte("café"), []byte("é"), []byte("π"), []byte(" ")},
			[]string{"café", "é", "[^ ]+"},
		),
	}

	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	if err := enc.Encode(fixture{Corpora: corpora}); err != nil {
		panic(err)
	}
}
