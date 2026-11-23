// Command tar_vectors writes independent tape-archive vectors from the reference SDK's
// implementation. "written" archives come from the reference writer with automatic format
// selection, so a conforming writer reproduces them byte for byte; "read" archives exercise
// GNU long names, PAX global headers, base-256 numbers and V7 headers for readers.
//
//	go run tooling/tar_vectors.go > seed/tests/fixtures/tar-vectors.json
package main

import (
	"archive/tar"
	"bytes"
	"encoding/hex"
	"encoding/json"
	"io"
	"os"
	"sort"
	"strings"
	"time"
)

type entry struct {
	Kind    int        `json:"kind"`
	Name    string     `json:"name"`
	Link    string     `json:"link"`
	Mode    int64      `json:"mode"`
	Uid     int        `json:"uid"`
	Gid     int        `json:"gid"`
	Size    int64      `json:"size"`
	Mtime   int64      `json:"mtime"`
	Uname   string     `json:"uname"`
	Gname   string     `json:"gname"`
	Major   int64      `json:"major"`
	Minor   int64      `json:"minor"`
	Data    string     `json:"data"`
	Records [][]string `json:"records"`
}

type archive struct {
	Archive string  `json:"archive"`
	Entries []entry `json:"entries"`
}

func hexs(s string) string { return hex.EncodeToString([]byte(s)) }

func describe(h *tar.Header, data []byte) entry {
	var records [][]string
	for k, v := range h.PAXRecords {
		switch k {
		case "path", "linkpath", "uname", "gname", "uid", "gid", "size", "mtime", "atime", "ctime":
			continue
		}
		records = append(records, []string{hexs(k), hexs(v)})
	}
	sort.Slice(records, func(i, j int) bool { return records[i][0] < records[j][0] })
	mtime := h.ModTime.Unix()
	if h.ModTime.IsZero() {
		mtime = 0 // global headers carry no time; record the absence as zero seconds
	}
	return entry{int(h.Typeflag), hexs(h.Name), hexs(h.Linkname), h.Mode, h.Uid, h.Gid, h.Size, mtime, hexs(h.Uname), hexs(h.Gname), h.Devmajor, h.Devminor, hex.EncodeToString(data), records}
}

func read(raw []byte) []entry {
	r := tar.NewReader(bytes.NewReader(raw))
	var out []entry
	for {
		h, err := r.Next()
		if err == io.EOF {
			return out
		}
		if err != nil {
			panic(err)
		}
		data, _ := io.ReadAll(r)
		out = append(out, describe(h, data))
	}
}

func write(headers []*tar.Header, data [][]byte, format tar.Format) []byte {
	var buf bytes.Buffer
	w := tar.NewWriter(&buf)
	for i, h := range headers {
		h.Format = format
		if err := w.WriteHeader(h); err != nil {
			panic(err)
		}
		w.Write(data[i])
	}
	w.Close()
	return buf.Bytes()
}

func main() {
	t := time.Unix(1700000000, 0)
	long := strings.Repeat("directory/", 12) + "file.txt"
	cases := [][]*tar.Header{
		{{Typeflag: '0', Name: "hello.txt", Mode: 0644, Uid: 1000, Gid: 1000, Uname: "user", Gname: "staff", ModTime: t, Size: 13}},
		{{Typeflag: '5', Name: "dir/", Mode: 0755, ModTime: t}, {Typeflag: '0', Name: "dir/a", Mode: 0600, ModTime: t, Size: 600},
			{Typeflag: '2', Name: "dir/link", Linkname: "a", Mode: 0777, ModTime: t}, {Typeflag: '1', Name: "dir/hard", Linkname: "dir/a", ModTime: t}},
		{{Typeflag: '0', Name: long, Mode: 0644, ModTime: t, Size: 1}},
		{{Typeflag: '0', Name: strings.Repeat("x", 120), Mode: 0644, ModTime: t, Size: 2}},
		{{Typeflag: '0', Name: "unicodé-ñame.txt", Mode: 0644, ModTime: t, Size: 3, Uname: "üser"}},
		{{Typeflag: '2', Name: "l", Linkname: strings.Repeat("t/", 70), Mode: 0777, ModTime: t}},
		{{Typeflag: '0', Name: "big-ids", Mode: 0644, Uid: 3000000, Gid: 9000000, ModTime: t, Size: 0}},
		{{Typeflag: '0', Name: "old", Mode: 0644, ModTime: time.Unix(-100, 0), Size: 4}},
		{{Typeflag: '3', Name: "dev/tty", Mode: 0620, Devmajor: 4, Devminor: 64, ModTime: t}, {Typeflag: '6', Name: "fifo", Mode: 0644, ModTime: t}},
		{{Typeflag: '0', Name: "empty", Mode: 0644, ModTime: t}},
		{{Typeflag: '0', Name: "a", Mode: 0644, ModTime: t, Size: 512}, {Typeflag: '0', Name: "b", Mode: 0644, ModTime: t, Size: 511}, {Typeflag: '0', Name: "c", Mode: 0644, ModTime: t, Size: 513}},
		{{Typeflag: '0', Name: "with-records", Mode: 0644, ModTime: t, Size: 5, PAXRecords: map[string]string{"comment": "hello", "SCHILY.xattr.user.k": "v"}}},
		{{Typeflag: '0', Name: "long-uname", Mode: 0644, ModTime: t, Size: 1, Uname: strings.Repeat("u", 40), Gname: "g"}},
	}
	payload := func(n int64) []byte {
		b := make([]byte, n)
		for i := range b {
			b[i] = byte((int64(i)*37 + 11) % 256)
		}
		return b
	}
	var written, reads []archive
	for _, c := range cases {
		var data [][]byte
		for _, h := range c {
			data = append(data, payload(h.Size))
		}
		raw := write(c, data, tar.FormatUnknown)
		written = append(written, archive{hex.EncodeToString(raw), read(raw)})
	}
	gnu := []*tar.Header{{Typeflag: '0', Name: long, Linkname: "", Mode: 0644, ModTime: t, Size: 7}, {Typeflag: '2', Name: "s", Linkname: strings.Repeat("y", 150), Mode: 0777, ModTime: t},
		{Typeflag: '0', Name: "huge-uid", Uid: 1 << 30, Gid: 5, Mode: 0644, ModTime: t, Size: 1}}
	raw := write(gnu, [][]byte{payload(7), nil, payload(1)}, tar.FormatGNU)
	reads = append(reads, archive{hex.EncodeToString(raw), read(raw)})
	global := []*tar.Header{{Typeflag: 'g', Name: "global", PAXRecords: map[string]string{"comment": "all"}}, {Typeflag: '0', Name: "after-global", Mode: 0644, ModTime: t, Size: 2}}
	raw = write(global, [][]byte{nil, payload(2)}, tar.FormatPAX)
	reads = append(reads, archive{hex.EncodeToString(raw), read(raw)})
	v7 := write([]*tar.Header{{Typeflag: '0', Name: "v7file", Mode: 0644, ModTime: t, Size: 3}}, [][]byte{payload(3)}, tar.FormatUSTAR)
	// Strip the USTAR magic, version and every field after it, then recompute the checksum,
	// which yields a V7 header (the reader then ignores owner names and device numbers).
	block := v7[:512]
	for i := 257; i < 512; i++ {
		block[i] = 0
	}
	copy(block[148:156], "        ")
	sum := 0
	for _, b := range block {
		sum += int(b)
	}
	copy(block[148:154], octal(sum, 6))
	block[154] = 0
	block[155] = ' '
	if h, err := tar.NewReader(bytes.NewReader(v7)).Next(); err != nil || h.Format != tar.FormatUnknown || h.Uname != "" {
		panic("expected a V7 header without owner names")
	}
	reads = append(reads, archive{hex.EncodeToString(v7), read(v7)})
	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", " ")
	enc.Encode(map[string][]archive{"written": written, "read": reads})
}

// octal renders n in exactly width zero-padded octal digits.
func octal(n, width int) []byte {
	out := make([]byte, width)
	for i := width - 1; i >= 0; i-- {
		out[i] = byte('0' + n%8)
		n /= 8
	}
	return out
}
