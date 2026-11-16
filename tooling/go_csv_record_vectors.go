// Generate independent configurable CSV field, position, error and rendering fixtures.
package main

import (
	"bytes"
	"encoding/csv"
	"encoding/hex"
	"encoding/json"
	"errors"
	"io"
	"os"
	"strings"
)

// dialect describes the public scalar and record-count policies used by each fixture.
type dialect struct {
	Delimiter rune `json:"delimiter"`
	Comment   rune `json:"comment"`
	Fields    int  `json:"fields"`
	Lazy      bool `json:"lazy"`
	Trim      bool `json:"trim"`
}

// outcome retains completed fields and exact source positions even when a read fails.
type outcome struct {
	Fields    []string `json:"fields"`
	Positions [][2]int `json:"positions"`
	Offset    int64    `json:"offset"`
	Kind      string   `json:"kind"`
	StartLine int      `json:"start_line"`
	Line      int      `json:"line"`
	Column    int      `json:"column"`
	Expected  int      `json:"expected"`
	Actual    int      `json:"actual"`
}

// readerCase records every advanced reader result, including later reads after an error.
func readerCase(config dialect, input string) map[string]interface{} {
	r := csv.NewReader(strings.NewReader(input))
	r.Comma, r.Comment = config.Delimiter, config.Comment
	r.FieldsPerRecord, r.LazyQuotes, r.TrimLeadingSpace = config.Fields, config.Lazy, config.Trim
	rows := []outcome{}
	for step := 0; step < 100; step++ {
		fields, err := r.Read()
		if err == io.EOF {
			break
		}
		if fields == nil {
			fields = []string{}
		}
		row := outcome{Fields: fields, Positions: [][2]int{}, Offset: r.InputOffset()}
		for i := range fields {
			line, column := r.FieldPos(i)
			row.Positions = append(row.Positions, [2]int{line, column})
		}
		if err != nil {
			var parse *csv.ParseError
			if !errors.As(err, &parse) {
				panic(err)
			}
			row.StartLine, row.Line, row.Column = parse.StartLine, parse.Line, parse.Column
			switch {
			case errors.Is(err, csv.ErrBareQuote):
				row.Kind = "bare"
			case errors.Is(err, csv.ErrQuote):
				row.Kind = "quote"
			case errors.Is(err, csv.ErrFieldCount):
				row.Kind, row.Expected, row.Actual = "fields", r.FieldsPerRecord, len(fields)
			default:
				panic(err)
			}
		}
		rows = append(rows, row)
	}
	return map[string]interface{}{"config": config, "input": input, "rows": rows}
}

// byteReaderCase encodes arbitrary field/input bytes as hex before JSON can replace invalid UTF-8.
func byteReaderCase(config dialect, input []byte) map[string]interface{} {
	result := readerCase(config, string(input))
	rows := []map[string]interface{}{}
	for _, row := range result["rows"].([]outcome) {
		fields := []string{}
		for _, field := range row.Fields {
			fields = append(fields, hex.EncodeToString([]byte(field)))
		}
		rows = append(rows, map[string]interface{}{"fields_hex": fields, "positions": row.Positions, "offset": row.Offset, "kind": row.Kind, "start_line": row.StartLine, "line": row.Line, "column": row.Column, "expected": row.Expected, "actual": row.Actual})
	}
	return map[string]interface{}{"config": config, "input_hex": hex.EncodeToString(input), "rows": rows}
}

// main spans physical/newline boundaries, quote errors, Unicode spaces and all count policies.
func main() {
	configs := []dialect{
		{',', 0, 0, false, false}, {',', 0, -1, false, false}, {',', 0, 2, false, false},
		{';', '#', -1, false, false}, {',', '#', 0, false, true},
		{',', 0, -1, true, false}, {',', 0, -1, true, true},
		{'λ', '※', -1, false, true},
	}
	inputs := []string{
		"", "\n\r\n\r", "a,b\n", "a,b\r\nc,d\r\n", "a,b\nc\nx,y,z\n",
		"a,,b,\n", "\"a,b\",\"x\"\"y\"\n", "\"a\r\nb\",x\r\n", "\"a\rb\",x\n",
		"a,b\r", "a\rb,c\n", "\"unterminated", "first,\"unterminated\nnext",
		"a\"b,c\nnext,row\n", "a,\"b\"tail\nnext,row\n", "a,\"b\" \n",
		"#comment\n\n a,b\n #literal,c\n", " a,\t b\n", "\u00a0a,\u2003\"b\"\n", "   \n\t\r\n",
		"a;b;\"c;d\"\n#skip\nx;y;z\n", "aλbλ\"cλd\"\r\n※comment\nxλ yλz\n",
		"\"a\"\"\"\n", "\"a\"\"b\"\n", "\"a\n\nb\",z\n", "\"\",\n",
		"a,b\n\"x\nq\",\"r\"z\nnext,row\n", "a,b\n\"x\nq\n", "\ufeffa,b\n", "a,\u0000b\n",
	}
	reads := []map[string]interface{}{}
	for _, config := range configs {
		for _, input := range inputs {
			reads = append(reads, readerCase(config, input))
		}
	}
	extraConfigs := []dialect{{'漢', '※', -1, false, true}, {'😀', '📝', -1, true, true}}
	for _, config := range extraConfigs {
		delimiter, comment := string(config.Delimiter), string(config.Comment)
		for _, input := range []string{
			"a" + delimiter + "b" + delimiter + "\"c" + delimiter + "d\"\r\n" + comment + "skip\nx" + delimiter + "y" + delimiter + "z\n",
			"\u3000a" + delimiter + "\u2003\"b\"\n",
			"\"😀😀😀\r\n😀😀\"" + delimiter + "z\n",
			"a" + delimiter + "\"b\"tail\nnext" + delimiter + "row\n",
			"\"unterminated\n", "\n\r\n",
		} {
			reads = append(reads, readerCase(config, input))
		}
	}
	writes := []map[string]interface{}{}
	rows := [][]string{{}, {""}, {"a", "b"}, {"a,b", " x", "\"q\""}, {"a\rb", "a\nb", "a\r\nb"}, {"\\."}, {"\u00a0lead", "\u2003lead", "tail "}, {"aλb", "c;d", "emoji 😀"}, {"\u0000", "\ufeff"}}
	for _, delimiter := range []rune{',', ';', 'λ', '漢', '😀'} {
		for _, crlf := range []bool{false, true} {
			for _, row := range rows {
				var output bytes.Buffer
				w := csv.NewWriter(&output)
				w.Comma, w.UseCRLF = delimiter, crlf
				if err := w.Write(row); err != nil {
					panic(err)
				}
				w.Flush()
				if err := w.Error(); err != nil {
					panic(err)
				}
				writes = append(writes, map[string]interface{}{"delimiter": delimiter, "crlf": crlf, "fields": row, "output": output.String()})
			}
		}
	}
	byteReads := []map[string]interface{}{}
	byteInputs := [][]byte{
		{}, {0xff}, {'a', ',', 0xff, '\n'}, {0xff, ',', '"', 'x', '"', '\n'},
		{'"', 0xff, 0xfe, '\r', '\n', 'x', '"', ',', 'y', '\n'},
		{'a', ',', '"', 'b', 0x80, '"', 'x', '\n', 'n', ',', 'r', '\n'},
		{0xe3, 0x80, 0x80, 'a', ',', 0x80, '\n'}, {0xce, 0xff, ',', 'a', '\n'},
		{0xff, 0xce, 0xbb, 'x', '\n'}, {'#', 0xff, '\n', 'a', ',', 'b', '\n'},
		{'a', ',', 0, '\n'}, {'"', 0xe3, 0x80, 0xff, '"', '\n'},
		[]byte("a漢\xff漢z\n"), []byte("a😀\xff😀z\n"), {0xe6, 0xff, ',', 'a', '\n'}, {0xf0, 0x80, 0xff, ',', 'a', '\n'},
	}
	for _, config := range append(configs, extraConfigs...) {
		for _, input := range byteInputs {
			byteReads = append(byteReads, byteReaderCase(config, input))
		}
	}
	byteWrites := []map[string]interface{}{}
	byteRows := [][][]byte{
		{}, {{}}, {{0xff}}, {{0xff, ',', 'x'}, {'a', '"', 0x80}},
		{{'a', '\r', 0xff}, {'a', '\n', 0x80}, {'\r', '\n'}},
		{{0xce, 0xff}, {0xce, 0xbb, 0xff}}, {{0xe3, 0x80, 0x80, 0xff}, {0}},
	}
	for _, delimiter := range []rune{',', ';', 'λ', '漢', '😀'} {
		for _, crlf := range []bool{false, true} {
			for _, row := range byteRows {
				fields, encodedFields := []string{}, []string{}
				for _, field := range row {
					fields = append(fields, string(field))
					encodedFields = append(encodedFields, hex.EncodeToString(field))
				}
				var output bytes.Buffer
				w := csv.NewWriter(&output)
				w.Comma, w.UseCRLF = delimiter, crlf
				if err := w.Write(fields); err != nil {
					panic(err)
				}
				w.Flush()
				if err := w.Error(); err != nil {
					panic(err)
				}
				byteWrites = append(byteWrites, map[string]interface{}{"delimiter": delimiter, "crlf": crlf, "fields_hex": encodedFields, "output_hex": hex.EncodeToString(output.Bytes())})
			}
		}
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]interface{}{"read": reads, "write": writes, "read_bytes": byteReads, "write_bytes": byteWrites}); err != nil {
		panic(err)
	}
}
