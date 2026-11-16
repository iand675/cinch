// Generate direct immutable-text UTF-8 observations from the pinned public SDK.
package main

import (
	"encoding/hex"
	"encoding/json"
	"os"
	"runtime"
	"strings"
	"unicode/utf8"
)

// inputRow preserves the arbitrary byte fixture before filtering to the valid-text domain.
type inputRow struct {
	Input string `json:"input"`
}

// inputFile identifies the existing independently generated raw-byte corpus.
type inputFile struct {
	SDK  string     `json:"sdk_version"`
	Rows []inputRow `json:"rows"`
}

// textRow records every returned capability of the five string-facing UTF-8 operations.
type textRow struct {
	Input      string `json:"input"`
	First      int32  `json:"first"`
	FirstWidth int    `json:"first_width"`
	Last       int32  `json:"last"`
	LastWidth  int    `json:"last_width"`
	Count      int    `json:"count"`
	Full       bool   `json:"full"`
	Valid      bool   `json:"valid"`
}

// longRow retains only a short pattern and repetition count for bounded-size fixture metadata.
type longRow struct {
	Pattern     string  `json:"pattern"`
	Repeats     int     `json:"repeats"`
	Observation textRow `json:"observation"`
}

// observe calls each public string operation independently and optionally retains exact input bytes.
func observe(text string, retain bool) textRow {
	first, firstWidth := utf8.DecodeRuneInString(text)
	last, lastWidth := utf8.DecodeLastRuneInString(text)
	input := ""
	if retain {
		input = hex.EncodeToString([]byte(text))
	}
	return textRow{input, first, firstWidth, last, lastWidth, utf8.RuneCountInString(text), utf8.FullRuneInString(text), utf8.ValidString(text)}
}

// main rejects edition drift and observes actual string methods on valid raw rows and large text.
func main() {
	if runtime.Version() != "go1.27.4" {
		panic("unexpected reference SDK")
	}
	if len(os.Args) != 2 {
		panic("expected raw UTF-8 fixture path")
	}
	data, err := os.ReadFile(os.Args[1])
	if err != nil {
		panic(err)
	}
	var input inputFile
	if err := json.Unmarshal(data, &input); err != nil {
		panic(err)
	}
	if input.SDK != runtime.Version() {
		panic("fixture/reference SDK mismatch")
	}
	rows := []textRow{}
	for _, item := range input.Rows {
		bytes, err := hex.DecodeString(item.Input)
		if err != nil {
			panic(err)
		}
		text := string(bytes)
		if !utf8.ValidString(text) {
			continue
		}
		rows = append(rows, observe(text, true))
	}
	long := []longRow{}
	for _, pattern := range []string{"abcdefgh", "aéΣ€", "\x00é€😀", "\U0010ffff\uffff\u0800\u0080"} {
		for _, repeats := range []int{0, 1, 7, 8, 15, 16, 17, 32, 63, 64, 65, 4096, 65536} {
			long = append(long, longRow{hex.EncodeToString([]byte(pattern)), repeats, observe(strings.Repeat(pattern, repeats), false)})
		}
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]any{"sdk_version": runtime.Version(), "rows": rows, "long": long}); err != nil {
		panic(err)
	}
}
