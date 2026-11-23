// Command zip_vectors writes independent zip vectors from the reference SDK's implementation:
// archives to read (stored and deflated entries, directories, UTF-8 and legacy names, comments,
// Unix modes, extended timestamps and a Zip64-forced entry) and stored archives that a conforming
// writer reproduces byte for byte.
//
//	go run tooling/zip_vectors.go > seed/tests/fixtures/zip-vectors.json
package main

import (
	"archive/zip"
	"bytes"
	"encoding/hex"
	"encoding/json"
	"hash/crc32"
	"io"
	"io/fs"
	"os"
	"time"
)

type entry struct {
	Name     string `json:"name"`
	Comment  string `json:"comment"`
	Method   int    `json:"method"`
	Flags    int    `json:"flags"`
	Crc      uint32 `json:"crc"`
	Size     uint64 `json:"size"`
	Packed   uint64 `json:"packed"`
	Modified int64  `json:"modified"`
	Mode     uint32 `json:"mode"`
	Data     string `json:"data"`
	Timed    bool   `json:"timed"`
}

type archive struct {
	Archive string  `json:"archive"`
	Comment string  `json:"comment"`
	Entries []entry `json:"entries"`
}

type file struct {
	name, comment string
	method        uint16
	mode          fs.FileMode
	modified      time.Time
	data          []byte
}

func payload(n int, salt int) []byte {
	b := make([]byte, n)
	for i := range b {
		b[i] = byte((i*37 + 11 + salt) % 256)
		if salt == 1 {
			b[i] = byte('a' + i%3)
		}
	}
	return b
}

func build(files []file, comment string) []byte {
	var buf bytes.Buffer
	w := zip.NewWriter(&buf)
	for _, f := range files {
		h := &zip.FileHeader{Name: f.name, Comment: f.comment, Method: f.method, Modified: f.modified}
		if f.mode != 0 {
			h.SetMode(f.mode)
		}
		out, err := w.CreateHeader(h)
		if err != nil {
			panic(err)
		}
		out.Write(f.data)
	}
	w.SetComment(comment)
	w.Close()
	return buf.Bytes()
}

func read(raw []byte, comment string) archive {
	r, err := zip.NewReader(bytes.NewReader(raw), int64(len(raw)))
	if err != nil {
		panic(err)
	}
	a := archive{Archive: hex.EncodeToString(raw), Comment: hex.EncodeToString([]byte(r.Comment))}
	for _, f := range r.File {
		rc, _ := f.Open()
		data, err := io.ReadAll(rc)
		if err != nil {
			panic(err)
		}
		mode := uint32(0)
		if f.CreatorVersion>>8 == 3 {
			mode = f.ExternalAttrs >> 16
		}
		modified := int64(0)
		if !f.Modified.IsZero() {
			modified = f.Modified.Unix()
		}
		a.Entries = append(a.Entries, entry{hex.EncodeToString([]byte(f.Name)), hex.EncodeToString([]byte(f.Comment)), int(f.Method), int(f.Flags), f.CRC32,
			f.UncompressedSize64, f.CompressedSize64, modified, mode, hex.EncodeToString(data), false})
	}
	return a
}

func main() {
	t := time.Date(2024, 3, 15, 13, 45, 31, 0, time.UTC)
	stored := [][]file{
		{{"hello.txt", "", zip.Store, 0, time.Time{}, []byte("hello, zip\n")}},
		{{"dir/", "", zip.Store, fs.ModeDir | 0755, t, nil}, {"dir/a.bin", "first file", zip.Store, 0644, t, payload(300, 0)}, {"dir/empty", "", zip.Store, 0600, t, nil}},
		{{"ünïcode.txt", "", zip.Store, 0644, t, []byte("x")}, {"plain", "commenté", zip.Store, 0, t, []byte("y")}},
		{{"old.txt", "", zip.Store, 0644, time.Date(1985, 7, 4, 9, 2, 58, 0, time.UTC), payload(10, 1)}},
	}
	var written, reads []archive
	for i, files := range stored {
		comment := ""
		if i == 1 {
			comment = "archive comment"
		}
		raw := build(files, comment)
		a := read(raw, comment)
		for j, f := range files {
			a.Entries[j].Timed = !f.modified.IsZero()
		}
		written = append(written, a)
	}
	deflated := [][]file{
		{{"repeat.txt", "", zip.Deflate, 0644, t, payload(5000, 1)}, {"noise.bin", "", zip.Deflate, 0644, t, payload(3000, 0)}, {"tiny", "", zip.Deflate, 0, time.Time{}, []byte("z")}},
		{{"empty-deflate", "", zip.Deflate, 0, t, nil}},
	}
	for _, files := range deflated {
		reads = append(reads, read(build(files, "deflated"), "deflated"))
	}
	// A Zip64 entry: force the 64-bit extra field by writing a raw header with oversized markers.
	var buf bytes.Buffer
	w := zip.NewWriter(&buf)
	data := payload(64, 0)
	h := &zip.FileHeader{Name: "zip64.bin", Method: zip.Store, CRC32: crc32.ChecksumIEEE(data), CompressedSize64: uint64(len(data)), UncompressedSize64: uint64(len(data))}
	h.CompressedSize, h.UncompressedSize = 0xffffffff, 0xffffffff
	out, _ := w.CreateRaw(h)
	out.Write(data)
	w.Close()
	reads = append(reads, read(buf.Bytes(), ""))
	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", " ")
	enc.Encode(map[string][]archive{"written": written, "read": reads})
}
