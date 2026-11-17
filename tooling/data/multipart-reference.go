// Multipart fixtures exercise the pinned public API independently of candidate source.
package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"math"
	"mime/multipart"
	"net/textproto"
	"os"
	"sort"
	"strconv"
	"strings"
	"syscall"
)

// wireHex preserves every byte, including invalid UTF-8, in portable JSON.
func wireHex(value []byte) string { return hex.EncodeToString(value) }

// errorText preserves diagnostics while stabilizing random scratch paths.
func errorText(err error) string {
	if err == nil {
		return ""
	}
	var path *os.PathError
	if errors.As(err, &path) {
		return path.Op + " <scratch>: " + path.Err.Error()
	}
	return err.Error()
}

// headerRows records sorted original keys and ordered values without normalization.
func headerRows(header textproto.MIMEHeader) []any {
	keys := make([]string, 0, len(header))
	for key := range header {
		keys = append(keys, key)
	}
	sort.Strings(keys)
	rows := []any{}
	for _, key := range keys {
		values := []string{}
		for _, value := range header[key] {
			values = append(values, wireHex([]byte(value)))
		}
		rows = append(rows, map[string]any{"key": wireHex([]byte(key)), "values": values})
	}
	return rows
}

// closeScratchFiles closes only this oracle process's actual multipart scratch descriptors.
// The source hook runs after all file bodies have been copied but before the final
// framing line is returned, exposing the public deferred-close form/error result.
func closeScratchFiles() int {
	entries, err := os.ReadDir("/proc/self/fd")
	if err != nil {
		panic(err)
	}
	count := 0
	for _, entry := range entries {
		target, err := os.Readlink("/proc/self/fd/" + entry.Name())
		if err != nil {
			continue
		}
		slash := strings.LastIndexByte(target, '/')
		if !strings.HasPrefix(target[slash+1:], "multipart-") {
			continue
		}
		fd, err := strconv.Atoi(entry.Name())
		if err != nil {
			panic(err)
		}
		if err = syscall.Close(fd); err != nil {
			panic(err)
		}
		count++
	}
	return count
}

// sourceScript records bounded reads with simultaneous terminal prefixes.
type sourceScript struct {
	// data owns the input.
	data []byte
	// at records consumed bytes.
	at int
	// chunk bounds positive reads.
	chunk int
	// terminal chooses EOF or a caller failure.
	terminal string
	// trace records actual callbacks.
	trace []any
	// closeScratch enables the actual final-close failure hook for one form case.
	closeScratch bool
	// closedScratch counts actual process-owned temporary descriptors interrupted.
	closedScratch int
}

// Read returns prefixes without suppressing repeated post-terminal callbacks.
func (s *sourceScript) Read(out []byte) (int, error) {
	n := len(out)
	if s.chunk > 0 && n > s.chunk {
		n = s.chunk
	}
	if n > len(s.data)-s.at {
		n = len(s.data) - s.at
	}
	copy(out, s.data[s.at:s.at+n])
	s.at += n
	if s.closeScratch && s.at == len(s.data) {
		s.closedScratch += closeScratchFiles()
		s.closeScratch = false
	}
	var err error
	if s.at == len(s.data) {
		if s.terminal == "failure" {
			err = errors.New("fixture source failure")
		} else {
			err = io.EOF
		}
	}
	s.trace = append(s.trace, map[string]any{"request": len(out), "output": wireHex(out[:n]), "error": errorText(err)})
	return n, err
}

// sinkScript models short output with independent simultaneous errors.
type sinkScript struct {
	// limits bounds each callback's accepted prefix; negative means all.
	limits []int
	// failures supplies errors by callback index.
	failures map[int]string
	// calls records callback count.
	calls int
	// output owns accepted bytes.
	output []byte
	// trace retains each complete offer.
	trace []any
}

// Write records one callback and never performs internal retries.
func (s *sinkScript) Write(input []byte) (int, error) {
	n := len(input)
	if s.calls < len(s.limits) && s.limits[s.calls] >= 0 && n > s.limits[s.calls] {
		n = s.limits[s.calls]
	}
	var err error
	if label := s.failures[s.calls]; label != "" {
		err = errors.New(label)
	}
	s.calls++
	s.output = append(s.output, input[:n]...)
	s.trace = append(s.trace, map[string]any{"offer": wireHex(input), "count": n, "error": errorText(err)})
	return n, err
}

// writerCase observes replacement, stale aliases, last-error and repeatable finish.
func writerCase(limits []int, failures map[int]string) map[string]any {
	sink := &sinkScript{limits: limits, failures: failures, trace: []any{}}
	writer := multipart.NewWriter(sink)
	set := writer.SetBoundary("a b")
	header := textproto.MIMEHeader{"z-raw": {"last", "again"}, "A": {"one"}, "empty": {}, "": {"\x00"}}
	actions := []any{map[string]any{"operation": "set", "error": errorText(set)}}
	first, err := writer.CreatePart(header)
	actions = append(actions, map[string]any{"operation": "create", "error": errorText(err)})
	if first != nil {
		n, e := first.Write([]byte("body"))
		actions = append(actions, map[string]any{"operation": "write", "count": n, "error": errorText(e)})
		n, e = first.Write(nil)
		actions = append(actions, map[string]any{"operation": "zero-write", "count": n, "error": errorText(e)})
	}
	_, err = writer.CreateFormFile("na\r\n\"\\me", "path/file")
	actions = append(actions, map[string]any{"operation": "replace", "error": errorText(err)})
	if first != nil {
		n, e := first.Write([]byte("stale"))
		actions = append(actions, map[string]any{"operation": "stale", "count": n, "error": errorText(e)})
	}
	err = writer.SetBoundary("new")
	actions = append(actions, map[string]any{"operation": "set-live", "error": errorText(err)})
	for index := 0; index < 2; index++ {
		err = writer.Close()
		actions = append(actions, map[string]any{"operation": "close", "error": errorText(err)})
	}
	err = writer.SetBoundary("new")
	actions = append(actions, map[string]any{"operation": "set-after-close", "error": errorText(err)})
	err = writer.WriteField("after", "value")
	actions = append(actions, map[string]any{"operation": "field-after-close", "error": errorText(err)})
	return map[string]any{"limits": limits, "failures": failures, "actions": actions, "trace": sink.trace, "output": wireHex(sink.output), "boundary": wireHex([]byte(writer.Boundary())), "content_type": wireHex([]byte(writer.FormDataContentType()))}
}

// readerCase retains all read prefixes, statuses, headers and source callbacks.
func readerCase(input []byte, boundary string, raw bool, chunk int, terminal string) map[string]any {
	source := &sourceScript{data: input, chunk: chunk, terminal: terminal, trace: []any{}}
	reader := multipart.NewReader(source, boundary)
	parts := []any{}
	var final error
	for index := 0; index < 20; index++ {
		var part *multipart.Part
		var err error
		if raw {
			part, err = reader.NextRawPart()
		} else {
			part, err = reader.NextPart()
		}
		if err != nil {
			final = err
			break
		}
		header := headerRows(part.Header)
		name, filename := part.FormName(), part.FileName()
		reads := []any{}
		body := []byte{}
		for step := 0; step < 4096; step++ {
			sizes := []int{0, 1, 2, 7, 3, 4096}
			size := sizes[step%len(sizes)]
			out := make([]byte, size)
			n, e := part.Read(out)
			body = append(body, out[:n]...)
			reads = append(reads, map[string]any{"request": size, "output": wireHex(out[:n]), "error": errorText(e)})
			if e != nil {
				break
			}
		}
		closeErr := part.Close()
		parts = append(parts, map[string]any{"headers": header, "name": wireHex([]byte(name)), "filename": wireHex([]byte(filename)), "body": wireHex(body), "reads": reads, "close_error": errorText(closeErr)})
	}
	return map[string]any{"input": wireHex(input), "boundary": wireHex([]byte(boundary)), "raw": raw, "chunk": chunk, "terminal": terminal, "parts": parts, "error": errorText(final), "source_trace": source.trace}
}

// sampleForm produces deterministic fields, memory/disk candidates and a skipped part.
func sampleForm() []byte {
	var out bytes.Buffer
	writer := multipart.NewWriter(&out)
	if err := writer.SetBoundary("form-boundary"); err != nil {
		panic(err)
	}
	for _, value := range []string{"one", "two"} {
		if err := writer.WriteField("v", value); err != nil {
			panic(err)
		}
	}
	for _, row := range [][3]string{{"f", "dir/a.txt", "abcdef"}, {"f", "b.bin", "0123456789"}, {"g", "", "plain"}} {
		part, err := writer.CreateFormFile(row[0], row[1])
		if err != nil {
			panic(err)
		}
		if _, err = io.WriteString(part, row[2]); err != nil {
			panic(err)
		}
	}
	if _, err := writer.CreatePart(textproto.MIMEHeader{"X-Skip": {"yes"}}); err != nil {
		panic(err)
	}
	if err := writer.Close(); err != nil {
		panic(err)
	}
	return out.Bytes()
}

// fileRead observes a sequential prefix and simultaneous status.
func fileRead(file multipart.File, size int) map[string]any {
	out := make([]byte, size)
	n, err := file.Read(out)
	return map[string]any{"request": size, "output": wireHex(out[:n]), "error": errorText(err)}
}

// fileReadAt observes a positioned prefix without changing the sequential cursor.
func fileReadAt(file multipart.File, size int, offset int64) map[string]any {
	out := make([]byte, size)
	n, err := file.ReadAt(out, offset)
	return map[string]any{"request": size, "offset": offset, "output": wireHex(out[:n]), "error": errorText(err)}
}

// fileObservations covers all four capabilities and backing-specific close behavior.
func fileObservations(header *multipart.FileHeader) map[string]any {
	file, err := header.Open()
	row := map[string]any{"open_error": errorText(err), "filename": wireHex([]byte(header.Filename)), "size": header.Size, "headers": headerRows(header.Header)}
	if err != nil {
		return row
	}
	row["reads"] = []any{fileRead(file, 0), fileRead(file, 2), fileReadAt(file, 3, 1), fileRead(file, 2), fileReadAt(file, 2, -1), fileReadAt(file, 5, header.Size-1)}
	seeks := []any{}
	for _, seek := range [][2]int64{{-1, 0}, {0, 77}, {-1, 2}, {2, 1}, {0, 0}} {
		at, e := file.Seek(seek[0], int(seek[1]))
		seeks = append(seeks, map[string]any{"offset": seek[0], "whence": seek[1], "result": at, "error": errorText(e)})
	}
	row["seeks"] = seeks
	row["close_error"] = errorText(file.Close())
	row["after_close"] = fileRead(file, 3)
	row["close_again_error"] = errorText(file.Close())
	original := header.Size
	header.Size = 1
	small, e := header.Open()
	row["edited_size_open_error"] = errorText(e)
	if e == nil {
		row["edited_size_read"] = fileRead(small, 99)
		small.Close()
	}
	header.Size = original
	return row
}

// formCase observes real spill storage, editable metadata and unlink/open lifecycle.
func formCase(memory int64, distinct bool, limits string, interruptClose bool) map[string]any {
	old := os.Getenv("GODEBUG")
	policy := limits
	if distinct {
		if policy != "" {
			policy += ","
		}
		policy += "multipartfiles=distinct"
	}
	os.Setenv("GODEBUG", policy)
	defer os.Setenv("GODEBUG", old)
	source := &sourceScript{data: sampleForm(), chunk: 7, terminal: "eof", trace: []any{}, closeScratch: interruptClose}
	if interruptClose {
		source.chunk = 1
	}
	form, err := multipart.NewReader(source, "form-boundary").ReadForm(memory)
	row := map[string]any{"memory": memory, "distinct": distinct, "policy": limits, "error": errorText(err), "too_large": errors.Is(err, multipart.ErrMessageTooLarge)}
	row["form_present"], row["interrupted_close_count"] = form != nil, source.closedScratch
	if form == nil {
		return row
	}
	values := []any{}
	names := []string{}
	for name := range form.Value {
		names = append(names, name)
	}
	sort.Strings(names)
	for _, name := range names {
		vs := []string{}
		for _, value := range form.Value[name] {
			vs = append(vs, wireHex([]byte(value)))
		}
		values = append(values, map[string]any{"name": wireHex([]byte(name)), "values": vs})
	}
	row["values"] = values
	files := []any{}
	names = nil
	for name := range form.File {
		names = append(names, name)
	}
	sort.Strings(names)
	var retained multipart.File
	for _, name := range names {
		for _, header := range form.File[name] {
			if retained == nil {
				retained, _ = header.Open()
			}
			entry := fileObservations(header)
			entry["name"] = wireHex([]byte(name))
			files = append(files, entry)
		}
	}
	row["files"] = files
	row["remove_error"] = errorText(form.RemoveAll())
	row["remove_again_error"] = errorText(form.RemoveAll())
	if retained != nil {
		row["retained_after_remove"] = fileRead(retained, 99)
		retained.Close()
	}
	reopened := []any{}
	for _, name := range names {
		for _, header := range form.File[name] {
			file, e := header.Open()
			reopened = append(reopened, errorText(e))
			if e == nil {
				file.Close()
			}
		}
	}
	row["reopened"] = reopened
	return row
}

// cacheCase isolates editable headers and the shared lazy disposition cache.
func cacheCase() map[string]any {
	input := []byte("--b\r\nContent-Disposition: form-data; name=old; filename=../before\r\n\r\nfirst\r\n--b\r\nContent-Disposition: form-data; name=second\r\n\r\nlast\r\n--b--\r\n")
	reader := multipart.NewReader(bytes.NewReader(input), "b")
	first, err := reader.NextPart()
	if err != nil {
		panic(err)
	}
	first.Header.Set("Content-Disposition", "form-data; name=edited; filename=\"dir/after\"")
	name := first.FormName()
	first.Header.Set("Content-Disposition", "form-data; name=late; filename=late")
	filename := first.FileName()
	second, e := reader.NextPart()
	old, oldErr := io.ReadAll(first)
	body, bodyErr := io.ReadAll(second)
	return map[string]any{"name": wireHex([]byte(name)), "filename": wireHex([]byte(filename)), "late_name": wireHex([]byte(first.FormName())), "next_error": errorText(e), "old_body": wireHex(old), "old_error": errorText(oldErr), "next_body": wireHex(body), "next_body_error": errorText(bodyErr), "headers": headerRows(first.Header)}
}

// boundaryCase stabilizes random construction by observing only explicit-boundary success.
func boundaryCase(value string) map[string]any {
	var sink bytes.Buffer
	writer := multipart.NewWriter(&sink)
	err := writer.SetBoundary(value)
	content := ""
	if err == nil {
		content = wireHex([]byte(writer.FormDataContentType()))
	}
	return map[string]any{"input": wireHex([]byte(value)), "error": errorText(err), "content_type": content}
}

// main emits exact deterministic protocol, callback and resource observations.
func main() {
	root := map[string]any{"format": 1, "package": "mime/multipart", "records": 37}
	boundaries, dispositions := []any{}, []any{}
	for number := 0; number < 256; number++ {
		value := string([]byte{'a', byte(number), 'z'})
		boundaries = append(boundaries, boundaryCase(value))
		dispositions = append(dispositions, map[string]any{"name": wireHex([]byte(value)), "filename": wireHex([]byte(value)), "output": wireHex([]byte(multipart.FileContentDisposition(value, value)))})
	}
	for _, value := range []string{"", "x", strings.Repeat("b", 70), strings.Repeat("b", 71), "a ", " ", " x", "a\tb"} {
		boundaries = append(boundaries, boundaryCase(value))
	}
	root["boundaries"], root["dispositions"] = boundaries, dispositions
	writers := []any{}
	for _, limits := range [][]int{nil, {0}, {1}, {-1, 0}, {-1, 1}, {-1, -1, -1, 1}, {-1, -1, -1, -1, 1}, {-1, -1, -1, -1, -1, 1}} {
		for _, failures := range []map[int]string{{}, {0: "header failure"}, {1: "body failure"}, {1: "first error", 2: "last error"}, {4: "footer failure"}} {
			writers = append(writers, writerCase(limits, failures))
		}
	}
	root["writers"] = writers
	wires := []string{
		"--b--", "preamble\r\n--b\r\nContent-Disposition: form-data; name=x; filename=../a\r\nX-Dup: a\r\nx-dup: b\r\n\r\nbody\r\n--b--\r\nepilogue",
		"--b\n\nbody\n--b--", "--b\r\nContent-Transfer-Encoding: quoted-printable\r\n\r\na=3Db=\r\nc\r\n--b--\r\n",
		"--b\r\nContent-Transfer-Encoding: quoted-printable\r\n\r\n=\x01\r\n--b--\r\n", "--b\r\nBad\r\n\r\nx\r\n--b--\r\n",
		"--b\r\n X: a\r\n\r\nx\r\n--b--\r\n", "--b\r\nX Space : v\r\nX-Fold: first\r\n\t second\r\n\r\nx\r\n--b--\r\n",
		"--b\r\n\r\ntruncated", "--b\r\n\r\nx\r\n--bogus\r\n", "--b\r\n\r\n--bX\r\n--b-\r\n--b--",
	}
	readers := []any{}
	for _, wire := range wires {
		for _, raw := range []bool{false, true} {
			for _, chunk := range []int{1, 2, 7, 4096} {
				for _, terminal := range []string{"eof", "failure"} {
					readers = append(readers, readerCase([]byte(wire), "b", raw, chunk, terminal))
				}
			}
		}
	}
	for number := 0; number < 256; number++ {
		wire := []byte("...\r\n--b\r\n\r\nprefix\r\n--b")
		wire = append(wire, byte(number))
		wire = append(wire, []byte("suffix\r\n--b--\r\n")...)
		readers = append(readers, readerCase(wire, "b", true, 3, "eof"))
	}
	readers = append(readers, readerCase([]byte("anything"), "", true, 1, "eof"))
	root["readers"] = readers
	forms := []any{}
	for _, memory := range []int64{-1, 0, 1, 5, 6, 9, 16, 17, 100, math.MaxInt64} {
		for _, distinct := range []bool{false, true} {
			forms = append(forms, formCase(memory, distinct, "", false))
		}
	}
	for _, policy := range []string{"multipartmaxparts=0", "multipartmaxparts=2", "multipartmaxparts=6", "multipartmaxheaders=0", "multipartmaxheaders=1", "multipartmaxheaders=3"} {
		forms = append(forms, formCase(0, false, policy, false))
	}
	forms = append(forms, formCase(0, false, "", true))
	root["forms"], root["cache"] = forms, cacheCase()
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(root); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
