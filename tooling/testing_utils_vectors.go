// Generate independent Go 1.24 reference vectors for testing/iotest,
// testing/fstest, and testing/quick parity verification.
package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"io/fs"
	"math/rand"
	"os"
	"strings"
	"testing/fstest"
	"testing/iotest"
	"testing/quick"
)

type Vectors struct {
	IoTest IoTestVectors `json:"iotest"`
	FsTest FsTestVectors `json:"fstest"`
	Quick  QuickVectors  `json:"quick"`
}

type IoTestVectors struct {
	OneByte      []OneByteCase      `json:"one_byte"`
	HalfReader   []HalfCase         `json:"half_reader"`
	Timeout      TimeoutCase        `json:"timeout"`
	DataErr      []DataErrCase      `json:"data_err"`
	Truncate     []TruncateCase     `json:"truncate"`
	TestReader   []TestReaderCase   `json:"test_reader"`
}

type OneByteCase struct {
	Input  string   `json:"input"`
	Chunks []string `json:"chunks"`
}

type HalfCase struct {
	Input    string   `json:"input"`
	Requests []int    `json:"requests"`
	Chunks   []string `json:"chunks"`
}

type TimeoutStep struct {
	Request   int    `json:"request"`
	Data      string `json:"data"`
	IsTimeout bool   `json:"is_timeout"`
	IsEof     bool   `json:"is_eof"`
}

type TimeoutCase struct {
	Input string        `json:"input"`
	Steps []TimeoutStep `json:"steps"`
}

type DataErrCase struct {
	Input   string `json:"input"`
	Wrapper string `json:"wrapper"`
	Output  string `json:"output"`
}

type TruncateCase struct {
	Chunks   []string `json:"chunks"`
	Limit    int64    `json:"limit"`
	Reported []int    `json:"reported"`
	Written  string   `json:"written"`
}

type TestReaderCase struct {
	Name       string `json:"name"`
	Kind       string `json:"kind"`
	Input      string `json:"input"`
	Expected   string `json:"expected"`
	ShouldPass bool   `json:"should_pass"`
}

type FsTestVectors struct {
	ValidPath []ValidPathCase `json:"valid_path"`
	Files     []FsFileSpec    `json:"files"`
	ReadFile  []ReadFileCase  `json:"read_file"`
	Stat      []StatCase      `json:"stat"`
	ReadDir   []ReadDirCase   `json:"read_dir"`
	Glob      []GlobCase      `json:"glob"`
	Sub       []SubCase       `json:"sub"`
	TestFS    []TestFSCase    `json:"test_fs"`
}

type ValidPathCase struct {
	Path  string `json:"path"`
	Valid bool   `json:"valid"`
}

type FsFileSpec struct {
	Path    string `json:"path"`
	Data    string `json:"data"`
	Mode    uint32 `json:"mode"`
	ModTime int64  `json:"mod_time"`
	IsDir   bool   `json:"is_dir"`
}

type ReadFileCase struct {
	Path  string `json:"path"`
	Data  string `json:"data"`
	Error string `json:"error"`
}

type StatCase struct {
	Path  string `json:"path"`
	Name  string `json:"name"`
	Size  int64  `json:"size"`
	Mode  uint32 `json:"mode"`
	IsDir bool   `json:"is_dir"`
	Error string `json:"error"`
}

type DirEntryOut struct {
	Name  string `json:"name"`
	IsDir bool   `json:"is_dir"`
}

type ReadDirCase struct {
	Path    string        `json:"path"`
	Entries []DirEntryOut `json:"entries"`
	Error   string        `json:"error"`
}

type GlobCase struct {
	Pattern string   `json:"pattern"`
	Matches []string `json:"matches"`
	Error   string   `json:"error"`
}

type SubCase struct {
	Dir        string        `json:"dir"`
	Entries    []DirEntryOut `json:"entries"`
	SamplePath string        `json:"sample_path"`
	SampleData string        `json:"sample_data"`
	Error      string        `json:"error"`
}

type TestFSCase struct {
	Name       string   `json:"name"`
	UseEmpty   bool     `json:"use_empty"`
	Expected   []string `json:"expected"`
	ShouldPass bool     `json:"should_pass"`
}

type QuickVectors struct {
	Checks []QuickCase `json:"checks"`
}

type QuickCase struct {
	Name       string `json:"name"`
	MaxCount   int    `json:"max_count"`
	ShouldPass bool   `json:"should_pass"`
	FailCount  int    `json:"fail_count"`
}

func classifyFsErr(err error) string {
	if err == nil {
		return ""
	}
	if errors.Is(err, fs.ErrInvalid) {
		return "invalid"
	}
	if errors.Is(err, fs.ErrNotExist) {
		return "not_found"
	}
	if strings.Contains(err.Error(), "is a directory") {
		return "is_dir"
	}
	if strings.Contains(err.Error(), "not a directory") || strings.Contains(err.Error(), "not implemented") {
		return "not_dir"
	}
	return "other:" + err.Error()
}

func buildIoTestVectors() IoTestVectors {
	var out IoTestVectors

	// OneByteReader vectors
	for _, inp := range []string{"", "a", "hello", "0123456789"} {
		r := iotest.OneByteReader(strings.NewReader(inp))
		chunks := []string{}
		buf := make([]byte, 8)
		for {
			n, err := r.Read(buf)
			if n > 0 {
				chunks = append(chunks, string(buf[:n]))
			}
			if err == io.EOF {
				break
			}
			if err != nil {
				panic(err)
			}
		}
		out.OneByte = append(out.OneByte, OneByteCase{Input: inp, Chunks: chunks})
	}

	// HalfReader vectors
	reqs := []int{1, 2, 3, 4, 5, 6, 10, 20}
	for _, inp := range []string{"abcdefghijklmnopqrstuvwx", "abcde", ""} {
		r := iotest.HalfReader(strings.NewReader(inp))
		chunks := []string{}
		for _, req := range reqs {
			buf := make([]byte, req)
			n, err := r.Read(buf)
			if n > 0 {
				chunks = append(chunks, string(buf[:n]))
			} else {
				chunks = append(chunks, "")
			}
			if err != nil && err != io.EOF {
				panic(err)
			}
		}
		out.HalfReader = append(out.HalfReader, HalfCase{Input: inp, Requests: reqs, Chunks: chunks})
	}

	// TimeoutReader vectors
	{
		inp := "abcdefghij"
		r := iotest.TimeoutReader(strings.NewReader(inp))
		steps := []TimeoutStep{}
		for i := 0; i < 6; i++ {
			buf := make([]byte, 3)
			n, err := r.Read(buf)
			steps = append(steps, TimeoutStep{
				Request:   3,
				Data:      string(buf[:n]),
				IsTimeout: errors.Is(err, iotest.ErrTimeout),
				IsEof:     errors.Is(err, io.EOF),
			})
		}
		out.Timeout = TimeoutCase{Input: inp, Steps: steps}
	}

	// DataErrReader vectors
	for _, tc := range []struct {
		input   string
		wrapper string
	}{
		{"", "plain"},
		{"a", "plain"},
		{"hello world", "plain"},
		{"hello world", "one_byte"},
		{"0123456789abcdef", "half"},
	} {
		var base io.Reader = strings.NewReader(tc.input)
		if tc.wrapper == "one_byte" {
			base = iotest.OneByteReader(base)
		} else if tc.wrapper == "half" {
			base = iotest.HalfReader(base)
		}
		r := iotest.DataErrReader(base)
		data, err := io.ReadAll(r)
		if err != nil {
			panic(err)
		}
		out.DataErr = append(out.DataErr, DataErrCase{
			Input:   tc.input,
			Wrapper: tc.wrapper,
			Output:  string(data),
		})
	}

	// TruncateWriter vectors
	chunks := []string{"hello", " ", "world!"}
	for _, limit := range []int64{0, 1, 5, 6, 11, 12, 50} {
		var buf bytes.Buffer
		w := iotest.TruncateWriter(&buf, limit)
		reported := []int{}
		for _, ch := range chunks {
			n, err := w.Write([]byte(ch))
			if err != nil {
				panic(err)
			}
			reported = append(reported, n)
		}
		out.Truncate = append(out.Truncate, TruncateCase{
			Chunks:   chunks,
			Limit:    limit,
			Reported: reported,
			Written:  buf.String(),
		})
	}

	// TestReader vectors
	trCases := []struct {
		name     string
		kind     string
		input    string
		expected string
		makeR    func(string) io.Reader
	}{
		{"plain_ok", "plain", "hello world 12345", "hello world 12345", func(s string) io.Reader { return strings.NewReader(s) }},
		{"empty_ok", "plain", "", "", func(s string) io.Reader { return strings.NewReader(s) }},
		{"one_byte_ok", "one_byte", "cinch stream test", "cinch stream test", func(s string) io.Reader { return iotest.OneByteReader(strings.NewReader(s)) }},
		{"half_ok", "half", "cinch half reader verification", "cinch half reader verification", func(s string) io.Reader { return iotest.HalfReader(strings.NewReader(s)) }},
		{"one_byte_half_ok", "one_byte_half", "lookahead payload", "lookahead payload", func(s string) io.Reader { return iotest.OneByteReader(iotest.HalfReader(strings.NewReader(s))) }},
		{"timeout_fail", "timeout", "abcdef", "abcdef", func(s string) io.Reader { return iotest.TimeoutReader(strings.NewReader(s)) }},
		{"err_fail", "err", "abcdef", "abcdef", func(s string) io.Reader { return iotest.ErrReader(errors.New("boom")) }},
		{"mismatch_fail", "plain", "hello", "world", func(s string) io.Reader { return strings.NewReader(s) }},
		{"short_fail", "plain", "hel", "hello", func(s string) io.Reader { return strings.NewReader(s) }},
		{"long_fail", "plain", "hello world", "hello", func(s string) io.Reader { return strings.NewReader(s) }},
	}
	for _, tc := range trCases {
		err := iotest.TestReader(tc.makeR(tc.input), []byte(tc.expected))
		out.TestReader = append(out.TestReader, TestReaderCase{
			Name:       tc.name,
			Kind:       tc.kind,
			Input:      tc.input,
			Expected:   tc.expected,
			ShouldPass: err == nil,
		})
	}

	return out
}

func buildFsTestVectors() FsTestVectors {
	var out FsTestVectors

	validPaths := []string{
		".", "", "/", "..", "./a", "a/.", "a/..", "a/../b", "a/./b",
		"/a", "a/", "a//b", "a", "a/b", "a/b/c.txt", ".hidden", "a/.hidden",
		"a/..dots", "a/dots..", "a\\b", "a:b", "a b/c d", "café/naïve.txt",
		"...", "a/.../b", "/.", "./", "../", "a///", "dir/sub/file.tar.gz",
	}
	for _, p := range validPaths {
		out.ValidPath = append(out.ValidPath, ValidPathCase{
			Path:  p,
			Valid: fs.ValidPath(p),
		})
	}

	specs := []FsFileSpec{
		{Path: "README.md", Data: "# Title\n", Mode: 0644, ModTime: 1700000000, IsDir: false},
		{Path: "a/b/c.txt", Data: "hello", Mode: 0644, ModTime: 1700000001, IsDir: false},
		{Path: "a/b/d.log", Data: "logdata", Mode: 0600, ModTime: 1700000002, IsDir: false},
		{Path: "a/d.txt", Data: "world!", Mode: 0644, ModTime: 1700000003, IsDir: false},
		{Path: "a/x.txt", Data: "", Mode: 0444, ModTime: 1700000004, IsDir: false},
		{Path: "empty", Data: "", Mode: uint32(fs.ModeDir | 0755), ModTime: 1700000005, IsDir: true},
		{Path: "z/nested/deep/file.bin", Data: "binary", Mode: 0644, ModTime: 1700000006, IsDir: false},
	}
	out.Files = specs

	mfs := fstest.MapFS{}
	for _, s := range specs {
		mode := fs.FileMode(s.Mode)
		if s.IsDir {
			mode |= fs.ModeDir
		}
		mfs[s.Path] = &fstest.MapFile{
			Data: []byte(s.Data),
			Mode: mode,
		}
	}

	// ReadFile vectors
	for _, p := range []string{"README.md", "a/b/c.txt", "a/x.txt", "z/nested/deep/file.bin", ".", "a", "empty", "missing.txt", "/a", "a/", "a/../README.md"} {
		data, err := mfs.ReadFile(p)
		errKind := ""
		if err != nil {
			if !fs.ValidPath(p) {
				errKind = "invalid"
			} else if st, serr := mfs.Stat(p); serr == nil && st.IsDir() {
				errKind = "is_dir"
			} else {
				errKind = classifyFsErr(err)
			}
		}
		out.ReadFile = append(out.ReadFile, ReadFileCase{
			Path:  p,
			Data:  string(data),
			Error: errKind,
		})
	}

	// Stat vectors
	for _, p := range []string{".", "README.md", "a", "a/b", "a/b/c.txt", "a/x.txt", "empty", "z", "z/nested", "missing", "", "/a", "a//b"} {
		st, err := mfs.Stat(p)
		if err != nil {
			errKind := classifyFsErr(err)
			if !fs.ValidPath(p) {
				errKind = "invalid"
			}
			out.Stat = append(out.Stat, StatCase{Path: p, Error: errKind})
		} else {
			out.Stat = append(out.Stat, StatCase{
				Path:  p,
				Name:  st.Name(),
				Size:  st.Size(),
				Mode:  uint32(st.Mode()),
				IsDir: st.IsDir(),
			})
		}
	}

	// ReadDir vectors
	for _, p := range []string{".", "a", "a/b", "empty", "z", "z/nested", "z/nested/deep", "README.md", "missing", "/a", "a/"} {
		entries, err := mfs.ReadDir(p)
		if err != nil {
			errKind := classifyFsErr(err)
			if !fs.ValidPath(p) {
				errKind = "invalid"
			} else if st, serr := mfs.Stat(p); serr == nil && !st.IsDir() {
				errKind = "not_dir"
			}
			out.ReadDir = append(out.ReadDir, ReadDirCase{Path: p, Error: errKind})
		} else {
			es := []DirEntryOut{}
			for _, e := range entries {
				es = append(es, DirEntryOut{Name: e.Name(), IsDir: e.IsDir()})
			}
			out.ReadDir = append(out.ReadDir, ReadDirCase{Path: p, Entries: es})
		}
	}

	// Glob vectors
	for _, pat := range []string{
		"*", "*/*", "*/*/*", "a/*.txt", "a/*/*.txt", "a/*/*",
		"a/b/c.txt", "a/missing", ".", "empty", "[a-z]/*.txt",
		"a/[dx].txt", "a/?.txt", "z/*/*/file.*", "[invalid", "a/[invalid",
	} {
		matches, err := mfs.Glob(pat)
		if err != nil {
			out.Glob = append(out.Glob, GlobCase{Pattern: pat, Matches: []string{}, Error: "bad_pattern"})
		} else {
			if matches == nil {
				matches = []string{}
			}
			out.Glob = append(out.Glob, GlobCase{Pattern: pat, Matches: matches})
		}
	}

	// Sub vectors
	subChecks := []struct {
		dir    string
		sample string
	}{
		{".", "README.md"},
		{"a", "b/c.txt"},
		{"a/b", "c.txt"},
		{"empty", ""},
		{"/a", ""},
	}
	for _, sc := range subChecks {
		sub, err := mfs.Sub(sc.dir)
		if err != nil {
			out.Sub = append(out.Sub, SubCase{Dir: sc.dir, Error: classifyFsErr(err)})
			continue
		}
		entries, rerr := fs.ReadDir(sub, ".")
		if rerr != nil {
			panic(rerr)
		}
		es := []DirEntryOut{}
		for _, e := range entries {
			es = append(es, DirEntryOut{Name: e.Name(), IsDir: e.IsDir()})
		}
		sampleData := ""
		if sc.sample != "" {
			b, berr := fs.ReadFile(sub, sc.sample)
			if berr != nil {
				panic(berr)
			}
			sampleData = string(b)
		}
		out.Sub = append(out.Sub, SubCase{
			Dir:        sc.dir,
			Entries:    es,
			SamplePath: sc.sample,
			SampleData: sampleData,
		})
	}

	// TestFS vectors
	emptyFS := fstest.MapFS{}
	testFSCases := []struct {
		name     string
		useEmpty bool
		fsys     fstest.MapFS
		expected []string
	}{
		{"empty_ok", true, emptyFS, []string{}},
		{"empty_missing", true, emptyFS, []string{"a.txt"}},
		{"nonempty_no_expected", false, mfs, []string{}},
		{"subset_ok", false, mfs, []string{"README.md", "a/b/c.txt"}},
		{"all_ok", false, mfs, []string{"README.md", "a/b/c.txt", "a/b/d.log", "a/d.txt", "a/x.txt", "empty", "z/nested/deep/file.bin"}},
		{"missing_fail", false, mfs, []string{"README.md", "does/not/exist"}},
	}
	for _, tc := range testFSCases {
		err := fstest.TestFS(tc.fsys, tc.expected...)
		out.TestFS = append(out.TestFS, TestFSCase{
			Name:       tc.name,
			UseEmpty:   tc.useEmpty,
			Expected:   tc.expected,
			ShouldPass: err == nil,
		})
	}

	return out
}

func buildQuickVectors() QuickVectors {
	var out QuickVectors
	runCheck := func(name string, maxCount int, fn any) {
		cfg := &quick.Config{MaxCount: maxCount, Rand: rand.New(rand.NewSource(12345))}
		err := quick.Check(fn, cfg)
		if err == nil {
			out.Checks = append(out.Checks, QuickCase{Name: name, MaxCount: maxCount, ShouldPass: true, FailCount: 0})
		} else if ce, ok := err.(*quick.CheckError); ok {
			out.Checks = append(out.Checks, QuickCase{Name: name, MaxCount: maxCount, ShouldPass: false, FailCount: ce.Count})
		} else {
			panic(fmt.Sprintf("unexpected error: %v", err))
		}
	}
	runEq := func(name string, maxCount int, f, g any) {
		cfg := &quick.Config{MaxCount: maxCount, Rand: rand.New(rand.NewSource(12345))}
		err := quick.CheckEqual(f, g, cfg)
		if err == nil {
			out.Checks = append(out.Checks, QuickCase{Name: name, MaxCount: maxCount, ShouldPass: true, FailCount: 0})
		} else if ce, ok := err.(*quick.CheckEqualError); ok {
			out.Checks = append(out.Checks, QuickCase{Name: name, MaxCount: maxCount, ShouldPass: false, FailCount: ce.Count})
		} else {
			panic(fmt.Sprintf("unexpected error: %v", err))
		}
	}

	runCheck("always_true_100", 100, func(x int64) bool { return true })
	runCheck("always_false_10", 10, func(x int64) bool { return false })
	runCheck("add_comm_i64", 100, func(x, y int64) bool { return x+y == y+x })
	runCheck("xor_self_zero_u32", 100, func(x uint32) bool { return (x ^ x) == 0 })
	runCheck("double_neg_bool", 50, func(b bool) bool { return !!b == b })
	runCheck("string_concat_assoc", 50, func(a, b, c string) bool { return (a+b)+c == a+(b+c) })
	runEq("eq_double", 100, func(x int64) int64 { return x + x }, func(x int64) int64 { return x * 2 })
	runEq("eq_mismatch", 20, func(x int64) int64 { return x }, func(x int64) int64 { return x ^ 1 })

	return out
}

func main() {
	v := Vectors{
		IoTest: buildIoTestVectors(),
		FsTest: buildFsTestVectors(),
		Quick:  buildQuickVectors(),
	}
	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	if err := enc.Encode(v); err != nil {
		fmt.Fprintf(os.Stderr, "encode error: %v\n", err)
		os.Exit(1)
	}
}
