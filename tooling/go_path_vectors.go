// Generate independent slash-path lexical, join and shell-pattern fixtures.
package main

import (
	"encoding/json"
	"os"
	"path"
	"sort"
	"strings"
)

// lexical records all public lexical observations for one exact UTF-8 input.
func lexical(input string) map[string]interface{} {
	dir, base := path.Split(input)
	return map[string]interface{}{"input": input, "clean": path.Clean(input), "base": path.Base(input), "dir": path.Dir(input), "ext": path.Ext(input), "absolute": path.IsAbs(input), "split_dir": dir, "split_base": base}
}

// next advances a fixed-width deterministic generator without global randomness.
func next(state *uint32) uint32 {
	*state = *state*1664525 + 1013904223
	return *state
}

// main combines targeted byte boundaries and reproducible independent corpora.
func main() {
	inputs := map[string]bool{}
	for _, input := range []string{"", ".", "..", "/", "//", "///", "/.", "/..", "//../", "a", "a/", "a//", "a/.", "a/..", "../a", "../../a/../..", "a/../../b", "/a/../../b", ".hidden", "/a/.hidden", "a..", "a.b/c", "a/b.c", "a/b.", "a/b.c/", "C:\\a\\..\\b", "C:/a/../b", "é/界/../😀.txt", "a\x00b/./c", "a\nb/..", "a/./.././../../b"} {
		inputs[input] = true
	}
	for _, n := range []int{1, 2, 3, 7, 8, 9, 15, 16, 17, 31, 32, 33, 63, 64, 65, 255, 256, 257, 4095, 4096, 4097} {
		for _, input := range []string{strings.Repeat("x", n), strings.Repeat("/", n), "/" + strings.Repeat("é", n) + "/../z", strings.Repeat("x", n) + ".ext", "a/" + strings.Repeat("x", n) + "/.."} {
			inputs[input] = true
		}
	}
	parts := []string{"", "a", "bb", ".", "..", "...", "é", "界.x", "😀", "a\\b", "\x00", ".x", "x."}
	state := uint32(0x10203040)
	for i := 0; i < 240; i++ {
		var b strings.Builder
		if next(&state)&3 == 0 {
			b.WriteByte('/')
		}
		count := 1 + int(next(&state)%11)
		for j := 0; j < count; j++ {
			if j > 0 {
				b.WriteString(strings.Repeat("/", 1+int(next(&state)%4)))
			}
			b.WriteString(parts[next(&state)%uint32(len(parts))])
		}
		if next(&state)&3 == 0 {
			b.WriteByte('/')
		}
		inputs[b.String()] = true
	}
	ordered := make([]string, 0, len(inputs))
	for input := range inputs {
		ordered = append(ordered, input)
	}
	sort.Strings(ordered)
	lex := make([]map[string]interface{}, 0, len(ordered))
	for _, input := range ordered {
		lex = append(lex, lexical(input))
	}
	joins := []map[string]interface{}{}
	for _, elements := range [][]string{{}, {""}, {"", ""}, {"/"}, {"a", "/b"}, {"", "a", "", "b"}, {"/a", "../b"}, {"a/", "/b/", "..", "c"}, {"é", "界", "..", "x"}, {"//host", "share", ".."}} {
		joins = append(joins, map[string]interface{}{"elements": elements, "output": path.Join(elements...)})
	}
	for i := 0; i < 160; i++ {
		elements := []string{}
		count := int(next(&state) % 17)
		for j := 0; j < count; j++ {
			element := parts[next(&state)%uint32(len(parts))]
			if next(&state)&3 == 0 {
				element = "/" + element
			}
			elements = append(elements, element)
		}
		joins = append(joins, map[string]interface{}{"elements": elements, "output": path.Join(elements...)})
	}
	patterns := []string{"", "*", "**", "?", "??", "*?", "*??", "*a", "a*", "a?c", "a*c", "a**b", "*.*", "*.txt", "*/*", "*/?", "a/b", "[/]", "[^/]", "[a]", "[^a]", "[a-z]", "[z-a]", "[a-cx-z]", "[é-界]", "[😀-🙏]", "*[�]", "*�", "*[a/]*/x", "*[/]*/x", "[]", "[^]", "[", "[a", "[a-]", "[-a]", "[a-b-c]", "\\", "\\*", "\\?", "\\é", "[\\]]", "[\\-]", "[^\\-]", "[\\a]", "a*[", "z[", "?[]", "a\\", "*\\", "[a\\]", "a/[界]", "[a/]", "[^a]/*", "*[!a]", "[[]", "]", "a\x00*", "a\n?"}
	names := []string{"", "a", "b", "c", "abc", "axc", "ab", "a/b", "a/b/x", "a/b/c", "a//b", "/", "//", ".", "..", "a.txt", "é", "界", "é.txt", "😀", "🙏", "é", "�", "é�", "é/x", "[", "]", "-", "*", "?", "\\", "a\x00b", "a\nb"}
	globs := []map[string]interface{}{}
	for _, pattern := range patterns {
		for _, name := range names {
			matched, err := path.Match(pattern, name)
			globs = append(globs, map[string]interface{}{"pattern": pattern, "name": name, "matched": matched, "error": err != nil})
		}
	}
	for i := 0; i < 400; i++ {
		pattern, name := "", ""
		for j, count := 0, int(next(&state)%9); j < count; j++ {
			pattern += []string{"a", "b", "?", "*", "[a-z]", "[/]", "\\*", "é", "[^x]", "/", "[", "\\"}[next(&state)%12]
		}
		for j, count := 0, int(next(&state)%12); j < count; j++ {
			name += []string{"a", "b", "/", "é", "界", "*", "x"}[next(&state)%7]
		}
		matched, err := path.Match(pattern, name)
		globs = append(globs, map[string]interface{}{"pattern": pattern, "name": name, "matched": matched, "error": err != nil})
	}
	e := json.NewEncoder(os.Stdout)
	e.SetIndent("", "  ")
	if err := e.Encode(map[string]interface{}{"lexical": lex, "joins": joins, "globs": globs}); err != nil {
		panic(err)
	}
}
