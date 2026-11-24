// Command template_vectors writes independent template vectors from the reference SDK's text
// templates: each case is a template text (with optional associated definitions, custom
// delimiters and a missing-key mode), data given as JSON (integral numbers become integers),
// and the output, or whether parsing or execution fails. Two user functions are available:
// "upper" (ASCII upper case) and "add" (integer sum).
//
//	go run tooling/template_vectors.go > seed/tests/fixtures/template-vectors.json
package main

import (
	"bytes"
	"encoding/json"
	"os"
	"strings"
	"text/template"
)

type tcase struct {
	Text       string `json:"text"`
	Data       string `json:"data"`
	Left       string `json:"left"`
	Right      string `json:"right"`
	MissingKey string `json:"missingkey"`
	Output     string `json:"output"`
	Error      string `json:"error"`
}

func convert(v any) any {
	switch x := v.(type) {
	case json.Number:
		if i, err := x.Int64(); err == nil {
			return int(i)
		}
		f, _ := x.Float64()
		return f
	case map[string]any:
		for k, e := range x {
			x[k] = convert(e)
		}
		return x
	case []any:
		for i, e := range x {
			x[i] = convert(e)
		}
		return x
	}
	return v
}

var funcs = template.FuncMap{
	"upper": strings.ToUpper,
	"add":   func(a, b int) int { return a + b },
}

const people = `{"name":"Ann","age":34,"tags":["a","b","c"],"pets":{"dog":"Rex","cat":"Tom"},"empty":[],"none":null,"zero":0,"pi":3.14159,"big":1e21,"million":1000000,"t":true,"f":false,"nested":{"inner":{"value":"deep"}},"n":3,"html":"<a href=\"x\">&'</a>","s":"héllo wörld"}`

var cases = []tcase{
	{Text: "Hello, {{.name}}!"},
	{Text: "{{.age}} {{.pi}} {{.big}} {{.million}} {{.t}} {{.f}} {{.zero}}"},
	{Text: "{{.tags}} {{.pets}} {{.empty}} {{.missing}} {{.none}}"},
	{Text: "{{.nested.inner.value}}"},
	{Text: "{{range .tags}}[{{.}}]{{end}}"},
	{Text: "{{range $i, $t := .tags}}{{$i}}={{$t}};{{end}}"},
	{Text: "{{range $k, $v := .pets}}{{$k}}:{{$v}} {{end}}"},
	{Text: "{{range .empty}}x{{else}}none{{end}}"},
	{Text: "{{range .n}}{{.}}{{end}}"},
	{Text: "{{range .tags}}{{if eq . \"b\"}}{{break}}{{end}}{{.}}{{end}}"},
	{Text: "{{range .tags}}{{if eq . \"b\"}}{{continue}}{{end}}{{.}}{{end}}"},
	{Text: "{{if .t}}yes{{else}}no{{end}} {{if .f}}yes{{else if .zero}}zero{{else}}neither{{end}}"},
	{Text: "{{with .nested.inner}}{{.value}}{{end}} {{with .missing}}x{{else}}absent{{end}} {{with .missing}}a{{else with .name}}{{.}}{{end}}"},
	{Text: "{{$x := .name}}{{$x}} {{$x = \"Bob\"}}{{$x}} {{$}}"[0:len("{{$x := .name}}{{$x}} {{$x = \"Bob\"}}{{$x}}")]},
	{Text: "{{if true}}{{$y := 1}}{{$y}}{{end}}"},
	{Text: "  {{- .name -}}  !"},
	{Text: "a {{- /* comment */ -}} b {{/* another */}}c"},
	{Text: "{{.name | printf \"%q\"}} {{printf \"%5d|%-5d|%05d\" .age .age .age}} {{printf \"%x %X %o %b\" 255 255 8 5}}"},
	{Text: "{{printf \"%.2f %e %g %v\" .pi .pi .pi .pi}} {{printf \"%8.3f|%-8.1f|%+d\" .pi .pi .age}}"},
	{Text: "{{printf \"%s %v %t %c %%\" .name .tags .t 65}} {{printf \"%d\" .name}} {{printf \"%d\"}} {{printf \"%d\" 1 2}}"},
	{Text: "{{print .name .age \"x\" 1 2}} {{println .name 1}}"},
	{Text: "{{len .tags}} {{len .pets}} {{len .name}} {{index .tags 1}} {{index .pets \"dog\"}} {{slice .name 1 3}} {{slice .tags 1}}"},
	{Text: "{{and .t .name}} {{and .f .name}} {{or .zero .name}} {{or .zero .f}} {{not .t}} {{not .empty}}"},
	{Text: "{{eq .age 34}} {{eq .name \"Bob\" \"Ann\"}} {{ne .age 1}} {{lt .age 40}} {{le 2 2}} {{gt \"b\" \"a\"}} {{ge .pi 3.0}}"},
	{Text: "{{html .html}} {{js .html}} {{urlquery .s \"a b&c\"}}"},
	{Text: "{{upper .name}} {{add .age 1}} {{.name | upper | printf \"<%s>\"}}"},
	{Text: "{{define \"item\"}}<{{.}}>{{end}}{{range .tags}}{{template \"item\" .}}{{end}}"},
	{Text: "{{block \"greet\" .name}}Hi {{.}}{{end}}!"},
	{Text: "{{template \"missing\"}}"},
	{Text: "{{(index .tags 0)}} {{(.nested).inner.value}} {{len (slice .tags 1)}}"},
	{Text: "{{1}} {{-2}} {{3.5}} {{0x1F}} {{1e3}} {{'a'}} {{\"s\\tq\"}} {{`raw\\n`}} {{true}}"},
	{Text: "{{.name.first}}"},
	{Text: "{{index .tags 9}}"},
	{Text: "{{if}}x{{end}}"},
	{Text: "{{range .tags}}"},
	{Text: "{{end}}"},
	{Text: "{{.name"},
	{Text: "{{nofunc .name}}"},
	{Text: "{{$undefined}}"},
	{Text: "{{\"unterminated}}"},
	{Text: "{{break}}"},
	{Text: "plain text with no actions at all, long enough to cross several sixteen byte spans of input"},
	{Text: "<<.name>> {{.name}}", Left: "<<", Right: ">>"},
	{Text: "{{.missing}}", MissingKey: "error"},
	{Text: "{{.missing}}|{{.nested.nope}}", MissingKey: "zero"},
	{Text: "{{.s}} {{len .s}} {{printf \"%q %x\" .s \"hi\"}}"},
	{Text: "{{range $i, $e := .tags}}{{if $i}}, {{end}}{{$e}}{{end}}"},
	{Text: "{{with $x := .name}}{{$x}}{{end}}"},
	{Text: "{{printf \"%v %v\" .none .nested}}"},
	{Text: "{{eq .age 34.0}}"},
	{Text: "{{.age | add 2}}"},
	{Text: "{{printf \"%6.2s|%-6s|%q\" \"abc\" \"de\" 'x'}}"},
	{Text: "{{printf \"%v %.3v %10.4g %G %E\" 0.000012345 3.14159 123456.789 1e-7 12345.678}}"},
}

func run(c tcase) tcase {
	t := template.New("t").Funcs(funcs)
	if c.Left != "" {
		t = t.Delims(c.Left, c.Right)
	}
	if c.MissingKey != "" {
		t = t.Option("missingkey=" + c.MissingKey)
	}
	if _, err := t.Parse(c.Text); err != nil {
		c.Error = "parse"
		return c
	}
	dec := json.NewDecoder(strings.NewReader(c.Data))
	dec.UseNumber()
	var data any
	dec.Decode(&data)
	var buf bytes.Buffer
	if err := t.Execute(&buf, convert(data)); err != nil {
		c.Error = "exec"
		return c
	}
	c.Output = buf.String()
	return c
}

func main() {
	var out []tcase
	for _, c := range cases {
		if c.Data == "" {
			c.Data = people
		}
		out = append(out, run(c))
	}
	enc := json.NewEncoder(os.Stdout)
	enc.SetEscapeHTML(false)
	enc.Encode(out)
}
