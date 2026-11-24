// Command html_template_vectors writes independent HTML template vectors from Go's
// html/template standard library across HTML text, comments, double/single/unquoted
// attributes, attribute names, URL attributes, srcset, <script> JS value/string/template/
// regexp/comment contexts, event handlers, <style> & style= CSS contexts, RCDATA
// (<title>/<textarea>), safe typed wrappers (HTML/HTMLAttr/CSS/JS/JSStr/URL/Srcset),
// control flow (if/range/with/define/block/template with context mangling and recursion),
// pipelines, custom delimiters, missingkey modes, and parse/escape/exec error cases.
//
//	GOCACHE=/tmp/go-cache GOTELEMETRY=off go run tooling/html_template_vectors.go > seed/tests/fixtures/html-template-vectors.json
package main

import (
	"bytes"
	"encoding/json"
	"html/template"
	"os"
	"strings"
)

type tcase struct {
	Name       string `json:"name"`
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
		// Recognize single-key safe content markers {"$html": "..."}, etc.
		if len(x) == 1 {
			if s, ok := x["$html"].(string); ok {
				return template.HTML(s)
			}
			if s, ok := x["$htmlAttr"].(string); ok {
				return template.HTMLAttr(s)
			}
			if s, ok := x["$css"].(string); ok {
				return template.CSS(s)
			}
			if s, ok := x["$js"].(string); ok {
				return template.JS(s)
			}
			if s, ok := x["$jsStr"].(string); ok {
				return template.JSStr(s)
			}
			if s, ok := x["$url"].(string); ok {
				return template.URL(s)
			}
			if s, ok := x["$srcset"].(string); ok {
				return template.Srcset(s)
			}
		}
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
	"upper":        strings.ToUpper,
	"add":          func(a, b int) int { return a + b },
	"safeHTML":     func(s string) template.HTML { return template.HTML(s) },
	"safeHTMLAttr": func(s string) template.HTMLAttr { return template.HTMLAttr(s) },
	"safeCSS":      func(s string) template.CSS { return template.CSS(s) },
	"safeJS":       func(s string) template.JS { return template.JS(s) },
	"safeJSStr":    func(s string) template.JSStr { return template.JSStr(s) },
	"safeURL":      func(s string) template.URL { return template.URL(s) },
	"safeSrcset":   func(s string) template.Srcset { return template.Srcset(s) },
}

const defaultData = `{
  "name": "Ann",
  "age": 34,
  "neg": -7,
  "zero": 0,
  "pi": 3.14159,
  "t": true,
  "f": false,
  "none": null,
  "empty": "",
  "tags": ["a", "<b>", "c&d"],
  "nums": [1, 2, 3],
  "pets": {"dog": "Rex", "cat": "Tom"},
  "emptyList": [],
  "html": "<script>alert(\"x\"+'y')&+;</script>",
  "nul": "a\u0000b",
  "spaces": "a b\tc\nd\"e'f=g` + "`" + `h<i+j",
  "badRune": "x\ufdd0\uffff",
  "httpUrl": "https://example.com/a/b?x=1&y=2#top",
  "mailtoUrl": "mailto:user@example.com?subject=hi+there",
  "relUrl": "/search/items?q=a b&p=1",
  "jsUrl": "javascript:alert(1)",
  "jsUrlUpper": "JaVaScRiPt:alert(1)",
  "vbUrl": "vbscript:msgbox(1)",
  "dataUrl": "data:text/html,<script>alert(1)</script>",
  "ftpUrl": "ftp://files.example.com/a",
  "urlPath": "foo bar/baz'qux(1)",
  "urlQuery": "a&b=c+d#e'f\"g h",
  "srcsetGood": "/a.png 1x, https://ex.com/b.png 2x",
  "srcsetMixed": "/a.png 1x, javascript:evil() 2x, /c.png 300w",
  "srcsetBadMeta": "/a.png 1x;evil",
  "jsStr": "a\"b'c\\d/e<f>g&h+i` + "`" + `j${k}\n\r\t\u0000\u000b\u2028\u2029",
  "jsRe": "a+b*c?d(e)[f]{g}|h^i$j.k/l\\m-n",
  "cssSafe": "10px",
  "cssColor": "#336699",
  "cssExpr": "expression(alert(1))",
  "cssEscExpr": "\\65 xpression(1)",
  "cssMoz": "-moz-binding:url(x)",
  "cssInject": "red; background: url(javascript:alert(1))",
  "cssDash": "--custom",
  "cssStr": "a\"b'c\\d/e<f>g&h+i(j)k:l;m{n}o\n\r\t\u0000",
  "cssStrHex": "a\"0b\" c\"",
  "attrOk": "title",
  "attrBad": "onclick",
  "attrBad2": "href",
  "attrBadChars": "a=b",
  "tree": {"val": "root", "kids": [{"val": "<leaf1>", "kids": []}, {"val": "leaf2", "kids": [{"val": "deep", "kids": []}]}]},
  "safeHtml": {"$html": "<b>&amp;Hi!</b>"},
  "safeHtmlScript": {"$html": "<i>ok</i><script>x</script>"},
  "safeHtmlAttr": {"$htmlAttr": "dir=\"ltr\""},
  "safeCss": {"$css": "rgba(0, 0, 255, 0.5)"},
  "safeJs": {"$js": "(1 + 2 * foo())"},
  "safeJsStr": {"$jsStr": "hello\\nworld\"<x>"},
  "safeUrl": {"$url": "javascript:void(0)"},
  "safeUrlQuery": {"$url": "a=1&b=2"},
  "safeUrlComma": {"$url": "/img/a,b.png"},
  "safeSrcset": {"$srcset": "/a.png 1x, /b.png 2x"}
}`

var cases = []tcase{
	// 1. HTML Text Context
	{Name: "text_plain", Text: "Hello, {{.name}}!"},
	{Name: "text_escape_html", Text: "<p>{{.html}}</p>"},
	{Name: "text_nul", Text: "<p>{{.nul}}</p>"},
	{Name: "text_nil_missing", Text: "<div>{{.missing}}|{{.none}}</div>"},
	{Name: "text_stray_lt", Text: "<p>a < b && c <3 {{.name}}</p>"},
	{Name: "text_doctype", Text: "<!DOCTYPE html><html><body>{{.html}}</body></html>"},
	{Name: "text_doctype_lower", Text: "<!doctype html><p>{{.name}}</p>"},
	{Name: "text_html_comment", Text: "a<!-- secret {{.html}} -->b<!-- c -->d"},
	{Name: "text_html_comment_multiline", Text: "x<!-- line1\n{{.name}}\nline2 -->y"},
	{Name: "text_numbers_bools", Text: "{{.age}} {{.neg}} {{.pi}} {{.t}} {{.f}}"},
	{Name: "text_unicode", Text: "<p>héllo {{.name}} 🌍</p>"},

	// 2. HTML Attributes
	{Name: "attr_dq", Text: `<div title="{{.html}}">ok</div>`},
	{Name: "attr_sq", Text: `<div title='{{.html}}'>ok</div>`},
	{Name: "attr_unquoted", Text: `<div title={{.spaces}}>ok</div>`},
	{Name: "attr_unquoted_empty", Text: `<div title={{.empty}}>ok</div>`},
	{Name: "attr_unquoted_nil", Text: `<div title={{.missing}}>ok</div>`},
	{Name: "attr_unquoted_bad_rune", Text: `<div title={{.badRune}}>ok</div>`},
	{Name: "attr_unquoted_nul", Text: `<div title={{.nul}}>ok</div>`},
	{Name: "attr_name_safe", Text: `<div {{.attrOk}}="v">ok</div>`},
	{Name: "attr_name_unsafe_event", Text: `<div {{.attrBad}}="v">ok</div>`},
	{Name: "attr_name_unsafe_url", Text: `<div {{.attrBad2}}="v">ok</div>`},
	{Name: "attr_name_bad_chars", Text: `<div {{.attrBadChars}}="v">ok</div>`},
	{Name: "attr_name_empty", Text: `<div {{.empty}}="v">ok</div>`},
	{Name: "attr_name_after_bool", Text: `<input disabled {{.attrOk}}="v">`},
	{Name: "attr_self_closing", Text: `<img alt="{{.html}}" src="{{.relUrl}}"/>`},

	// 3. URL Attributes
	{Name: "url_https", Text: `<a href="{{.httpUrl}}">link</a>`},
	{Name: "url_mailto", Text: `<a href="{{.mailtoUrl}}">mail</a>`},
	{Name: "url_rel", Text: `<a href="{{.relUrl}}">rel</a>`},
	{Name: "url_javascript", Text: `<a href="{{.jsUrl}}">bad</a>`},
	{Name: "url_javascript_mixed_case", Text: `<a href="{{.jsUrlUpper}}">bad</a>`},
	{Name: "url_vbscript", Text: `<a href="{{.vbUrl}}">bad</a>`},
	{Name: "url_data", Text: `<img src="{{.dataUrl}}">`},
	{Name: "url_ftp", Text: `<form action="{{.ftpUrl}}"></form>`},
	{Name: "url_data_custom_attr", Text: `<div data-url="{{.jsUrl}}" data-action="{{.jsUrl}}" xlink:href="{{.jsUrl}}"></div>`},
	{Name: "url_pre_query", Text: `<a href="https://example.com/{{.urlPath}}">p</a>`},
	{Name: "url_query", Text: `<a href="/search?q={{.urlQuery}}&n={{.age}}">q</a>`},
	{Name: "url_frag", Text: `<a href="/page#{{.urlQuery}}">f</a>`},
	{Name: "url_unquoted", Text: `<a href={{.httpUrl}}>u</a>`},
	{Name: "url_unquoted_bad", Text: `<a href={{.jsUrl}}>u</a>`},
	{Name: "url_formaction", Text: `<button formaction="{{.jsUrl}}">go</button>`},

	// 4. Srcset Attributes
	{Name: "srcset_good", Text: `<img srcset="{{.srcsetGood}}">`},
	{Name: "srcset_mixed", Text: `<img srcset="{{.srcsetMixed}}">`},
	{Name: "srcset_bad_meta", Text: `<img srcset="{{.srcsetBadMeta}}">`},
	{Name: "srcset_safe_url", Text: `<img srcset="{{.safeUrlComma}} 1x">`},
	{Name: "srcset_safe_srcset", Text: `<img srcset="{{.safeSrcset}}">`},

	// 5. JavaScript Contexts (<script> & event handlers)
	{Name: "js_val_string", Text: `<script>var s = {{.html}};</script>`},
	{Name: "js_val_primitives", Text: `<script>var a={{.age}},b={{.neg}},c={{.t}},d={{.f}},e={{.none}},f={{.missing}};</script>`},
	{Name: "js_val_collections", Text: `<script>var a={{.tags}},b={{.pets}},c={{.nums}};</script>`},
	{Name: "js_val_specials", Text: `<script>var s={{.jsStr}};</script>`},
	{Name: "js_dq_str", Text: `<script>var s = "{{.jsStr}}";</script>`},
	{Name: "js_sq_str", Text: `<script>var s = '{{.jsStr}}';</script>`},
	{Name: "js_tmpl_lit", Text: "<script>var s = `{{.jsStr}}`;</script>"},
	{Name: "js_tmpl_lit_interp", Text: "<script>var s = `hi ${ {{.age}} } {{.name}}`;</script>"},
	{Name: "js_regexp", Text: `<script>var r = /{{.jsRe}}/;</script>`},
	{Name: "js_regexp_empty", Text: `<script>var r = /{{.empty}}/;</script>`},
	{Name: "js_div_vs_regexp", Text: `<script>var a = 1 / {{.age}}; var b = return /{{.name}}/;</script>`},
	{Name: "js_comments", Text: "<script>var x = 1; // cmt {{.name}}\nvar y = /* blk {{.name}} */ 2; var z = /* multi\nline */ 3;</script>"},
	{Name: "js_html_comments", Text: "<script><!-- open {{.name}}\nvar x = {{.age}};\n--> close {{.name}}\n</script>"},
	{Name: "js_static_script_tag_in_str", Text: `<script>var s = "</script><script><!-- {{.name}}";</script>`},
	{Name: "js_onclick_str", Text: `<button onclick="alert('{{.jsStr}}')">click</button>`},
	{Name: "js_onclick_val", Text: `<button onclick="handle({{.html}}, {{.age}})">click</button>`},
	{Name: "js_onclick_entity", Text: `<button onclick="alert(&quot;{{.html}}&quot;)">click</button>`},
	{Name: "js_script_type_json", Text: `<script type="application/json">{{.pets}}</script>`},
	{Name: "js_script_type_ldjson", Text: `<script type="application/ld+json; charset=utf-8">{{.pets}}</script>`},
	{Name: "js_script_type_module", Text: `<script type="module">const x = {{.html}};</script>`},
	{Name: "js_script_type_plain", Text: `<script type="text/plain">{{.html}}</script>`},

	// 6. CSS Contexts (<style> & style=)
	{Name: "css_val_safe", Text: `<style>p { width: {{.cssSafe}}; color: {{.cssColor}}; }</style>`},
	{Name: "css_val_unsafe_expr", Text: `<style>p { width: {{.cssExpr}}; }</style>`},
	{Name: "css_val_unsafe_esc_expr", Text: `<style>p { width: {{.cssEscExpr}}; }</style>`},
	{Name: "css_val_unsafe_moz", Text: `<style>p { x: {{.cssMoz}}; }</style>`},
	{Name: "css_val_unsafe_inject", Text: `<style>p { color: {{.cssInject}}; }</style>`},
	{Name: "css_val_unsafe_dash", Text: `<style>p { color: {{.cssDash}}; }</style>`},
	{Name: "css_dq_str", Text: `<style>p::before { content: "{{.cssStr}}"; }</style>`},
	{Name: "css_sq_str", Text: `<style>p::before { content: '{{.cssStrHex}}'; }</style>`},
	{Name: "css_url_unquoted", Text: `<style>p { background: url({{.relUrl}}); }</style>`},
	{Name: "css_url_dq", Text: `<style>p { background: url("{{.httpUrl}}"); }</style>`},
	{Name: "css_url_sq_bad", Text: `<style>p { background: url('{{.jsUrl}}'); }</style>`},
	{Name: "css_comments", Text: "<style>/* cmt {{.name}} */ p { color: {{.cssColor}}; } // line {{.name}}\n</style>"},
	{Name: "css_inline_style", Text: `<p style="color: {{.cssColor}}; width: {{.cssInject}}">hi</p>`},
	{Name: "css_inline_style_url", Text: `<p style="background: url('{{.jsUrl}}')">hi</p>`},

	// 7. RCDATA (<title> and <textarea>)
	{Name: "rcdata_title", Text: `<title>{{.html}}</title>`},
	{Name: "rcdata_textarea", Text: `<textarea><b>{{.html}}</b></textarea>`},
	{Name: "rcdata_textarea_no_tag_parse", Text: `<textarea><a href="{{.jsUrl}}">{{.name}}</a></textarea>`},
	{Name: "rcdata_safe_html", Text: `<title>{{.safeHtml}}</title>`},

	// 8. Safe Typed Wrappers
	{Name: "safe_html_in_text", Text: `<div>{{.safeHtml}}</div>`},
	{Name: "safe_html_in_attr", Text: `<div title="{{.safeHtml}}">x</div>`},
	{Name: "safe_html_in_attr_strip_script", Text: `<div title="{{.safeHtmlScript}}">x</div>`},
	{Name: "safe_html_in_unquoted_attr", Text: `<div title={{.safeHtml}}>x</div>`},
	{Name: "safe_html_in_href_still_filtered", Text: `<a href="{{safeHTML .jsUrl}}">x</a>`},
	{Name: "safe_html_attr", Text: `<div {{.safeHtmlAttr}}>x</div>`},
	{Name: "safe_css_in_style", Text: `<p style="color: {{.safeCss}}">x</p>`},
	{Name: "safe_css_in_text_escaped", Text: `<p>{{safeCSS "<b>"}}</p>`},
	{Name: "safe_js_in_script", Text: `<script>var x = {{.safeJs}};</script>`},
	{Name: "safe_js_str_in_val", Text: `<script>var x = {{.safeJsStr}};</script>`},
	{Name: "safe_js_str_in_str", Text: `<script>var x = "{{.safeJsStr}}";</script>`},
	{Name: "safe_url_in_href", Text: `<a href="{{.safeUrl}}">x</a>`},
	{Name: "safe_url_in_query", Text: `<a href="/p?{{.safeUrlQuery}}">x</a>`},
	{Name: "safe_url_in_script_quoted", Text: `<script>var u = {{.safeUrl}};</script>`},
	{Name: "safe_func_pipeline", Text: `<div>{{.html | upper | safeHTML}}</div>`},

	// 9. Control Flow, Pipelines, Templates, Custom Delims
	{Name: "ctrl_if_else", Text: `{{if .t}}<b>{{.html}}</b>{{else}}<i>{{.name}}</i>{{end}}`},
	{Name: "ctrl_if_nudge_attr", Text: `<p title={{if .t}}{{.name}}{{end}}>ok</p>`},
	{Name: "ctrl_with", Text: `{{with .pets}}<p>{{.dog}}</p>{{else}}<p>none</p>{{end}}`},
	{Name: "ctrl_range_list", Text: `<ul>{{range $i, $e := .tags}}<li>{{$i}}:{{$e}}</li>{{end}}</ul>`},
	{Name: "ctrl_range_break_continue", Text: `<ul>{{range .nums}}{{if eq . 2}}{{continue}}{{end}}<li>{{.}}</li>{{if eq . 3}}{{break}}{{end}}{{end}}</ul>`},
	{Name: "ctrl_range_script", Text: `<script>var a = [{{range $i, $e := .tags}}{{if $i}},{{end}}{{$e}}{{end}}];</script>`},
	{Name: "ctrl_define_multi_ctx", Text: `{{define "v"}}{{.}}{{end}}<p>{{template "v" .html}}</p><a href="{{template "v" .jsUrl}}">x</a><script>var x={{template "v" .html}};</script>`},
	{Name: "ctrl_block", Text: `<p>{{block "b" .html}}Hi {{.}}{{end}}</p>`},
	{Name: "ctrl_recursive_template", Text: `{{define "node"}}<li>{{.val}}{{if .kids}}<ul>{{range .kids}}{{template "node" .}}{{end}}</ul>{{end}}</li>{{end}}<ul>{{template "node" .tree}}</ul>`},
	{Name: "ctrl_predefined_html_ok", Text: `<p>{{.html | html}}</p>`},
	{Name: "ctrl_predefined_urlquery_ok", Text: `<a href="/s?q={{.urlQuery | urlquery}}">x</a>`},
	{Name: "ctrl_vars_and_trim", Text: `{{$x := .html}}  {{- $x -}}  `},
	{Name: "ctrl_custom_delims", Text: `<p><<.html>> {{.name}}</p>`, Left: "<<", Right: ">>"},
	{Name: "ctrl_missingkey_zero", Text: `<p>{{.missing}}</p>`, MissingKey: "zero"},

	// 10. Error Cases (parse, escape, exec)
	{Name: "err_parse_unclosed", Text: `<p>{{.name</p>`},
	{Name: "err_parse_undef_func", Text: `<p>{{nope .name}}</p>`},
	{Name: "err_parse_unbalanced_end", Text: `<p>{{end}}</p>`},
	{Name: "err_escape_end_in_tag", Text: `<div title="x"`},
	{Name: "err_escape_end_in_script", Text: `<script>var x = {{.age}};`},
	{Name: "err_escape_end_in_style", Text: `<style>p { color: {{.cssColor}}`},
	{Name: "err_escape_end_in_comment", Text: `<!-- unclosed {{.name}}`},
	{Name: "err_escape_branch_diff_ctx", Text: `{{if .t}}<div{{end}}>`},
	{Name: "err_escape_branch_script_str", Text: `<script>{{if .t}}"{{end}}</script>`},
	{Name: "err_escape_range_reentry", Text: `{{range .tags}}<a href="{{.}}{{end}}`},
	{Name: "err_escape_ambig_url", Text: `<a href="{{if .t}}?{{end}}{{.name}}">x</a>`},
	{Name: "err_escape_ambig_slash", Text: `<script>{{if .t}}1{{else}}+{{end}}/a/</script>`},
	{Name: "err_escape_bad_attr_name", Text: `<div a"b="1"></div>`},
	{Name: "err_escape_bad_unquoted_attr", Text: `<div a=b"c></div>`},
	{Name: "err_escape_js_partial_esc", Text: `<script>var s = "\{{.name}}";</script>`},
	{Name: "err_escape_css_partial_esc", Text: `<style>p { content: "\{{.name}}"; }</style>`},
	{Name: "err_escape_js_partial_charset", Text: `<script>var r = /[{{.name}}]/;</script>`},
	{Name: "err_escape_predef_mid_pipe", Text: `<p>{{.name | html | upper}}</p>`},
	{Name: "err_escape_predef_html_unquoted", Text: `<div title={{.name | html}}></div>`},
	{Name: "err_escape_missing_template", Text: `<p>{{template "nope"}}</p>`},
	{Name: "err_exec_missingkey_error", Text: `<p>{{.missing}}</p>`, MissingKey: "error"},
	{Name: "err_exec_index_oob", Text: `<p>{{index .tags 9}}</p>`},
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
	if err := dec.Decode(&data); err != nil {
		panic(err)
	}
	var buf bytes.Buffer
	if err := t.Execute(&buf, convert(data)); err != nil {
		if _, ok := err.(*template.Error); ok {
			c.Error = "escape"
		} else {
			c.Error = "exec"
		}
		return c
	}
	c.Output = buf.String()
	return c
}

func main() {
	var out []tcase
	for _, c := range cases {
		if c.Data == "" {
			c.Data = defaultData
		}
		out = append(out, run(c))
	}
	enc := json.NewEncoder(os.Stdout)
	enc.SetEscapeHTML(false)
	enc.SetIndent("", "  ")
	if err := enc.Encode(out); err != nil {
		panic(err)
	}
}
