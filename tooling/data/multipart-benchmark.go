// Complete public multipart pipelines with independent scalar output observations.
package main

import (
	"bytes"
	"fmt"
	"io"
	"mime/multipart"
	"mime/quotedprintable"
	"os"
	"strconv"
	"strings"
	"time"
)

// checksum consumes every returned byte with bounded scalar arithmetic.
func checksum(value uint64, input []byte) uint64 {
	for _, b := range input {
		value = (value*131 + uint64(b)) & 2147483647
	}
	return value
}

// digestSink observes complete public writer offers without materializing another wire copy.
type digestSink struct {
	// value contains the digest of every accepted byte in order.
	value uint64
}

// Write accepts the complete offer exactly once, including empty writes.
func (sink *digestSink) Write(input []byte) (int, error) {
	sink.value = checksum(sink.value, input)
	return len(input), nil
}

// prepare constructs deterministic inputs before the measurement begins.
func prepare(mode string, size int) ([]byte, []byte) {
	raw := make([]byte, size)
	for at := range raw {
		if mode == "read-header" {
			raw[at] = byte('A' + at%26)
		} else if strings.Contains(mode, "sparse") || strings.Contains(mode, "dense") {
			if strings.Contains(mode, "dense") || at%31 == 0 {
				raw[at] = []byte{'\r', '\n', '"', '\\'}[at%4]
			} else {
				raw[at] = byte('a' + at%26)
			}
		} else {
			raw[at] = byte(at*37 + 11)
		}
	}
	if !strings.HasPrefix(mode, "read") {
		return raw, raw
	}
	header := "--bench-boundary\r\nContent-Disposition: form-data; name=\"field\"\r\n"
	body := raw
	if mode == "read-header" {
		header += "X-Pad: " + string(raw) + "\r\n"
		body = []byte("value")
	} else if mode == "read-qp" {
		header += "Content-Transfer-Encoding: quoted-printable\r\n"
		var encoded bytes.Buffer
		writer := quotedprintable.NewWriter(&encoded)
		writer.Binary = true
		if _, err := writer.Write(raw); err != nil {
			panic(err)
		}
		if err := writer.Close(); err != nil {
			panic(err)
		}
		body = encoded.Bytes()
	}
	input := append([]byte(header+"\r\n"), body...)
	input = append(input, []byte("\r\n--bench-boundary--\r\n")...)
	return raw, input
}

// operation observes a complete public allocation/parse/write/finalization pipeline.
func operation(mode string, input []byte, writer *multipart.Writer, sink *digestSink) uint64 {
	if strings.HasPrefix(mode, "disposition") {
		return checksum(0, []byte(multipart.FileContentDisposition(string(input), string(input))))
	}
	if strings.HasPrefix(mode, "write") {
		sink.value = 0
		name, value := "field", string(input)
		if mode != "write-body" {
			name, value = string(input), "value"
		}
		if err := writer.WriteField(name, value); err != nil {
			panic(err)
		}
		if err := writer.Close(); err != nil {
			panic(err)
		}
		return sink.value
	}
	reader := multipart.NewReader(bytes.NewReader(input), "bench-boundary")
	value := uint64(0)
	buffer := make([]byte, 4096)
	for {
		part, err := reader.NextPart()
		if err == io.EOF {
			return value
		}
		if err != nil {
			panic(err)
		}
		value = checksum(value, []byte(part.FormName()))
		value = checksum(value, []byte(part.Header.Get("X-Pad")))
		for {
			count, err := part.Read(buffer)
			value = checksum(value, buffer[:count])
			if err == io.EOF {
				break
			}
			if err != nil {
				panic(err)
			}
		}
	}
}

// main times complete operations and output consumption, then checks retained inputs afterward.
func main() {
	if len(os.Args) != 4 {
		os.Exit(2)
	}
	size, sizeError := strconv.Atoi(os.Args[2])
	count, countError := strconv.Atoi(os.Args[3])
	if sizeError != nil || countError != nil || size < 0 || count <= 0 {
		os.Exit(2)
	}
	mode := os.Args[1]
	raw, input := prepare(mode, size)
	sink := &digestSink{}
	writer := multipart.NewWriter(sink)
	if err := writer.SetBoundary("bench-boundary"); err != nil {
		panic(err)
	}
	value := uint64(0)
	start := time.Now()
	for at := 0; at < count; at++ {
		value = (value + operation(mode, input, writer, sink)) & 2147483647
	}
	elapsed := time.Since(start).Nanoseconds()
	retained := checksum(checksum(0, raw), input)
	retained = checksum(retained, []byte(mode))
	if strings.HasPrefix(mode, "write") {
		retained = checksum(retained, []byte(writer.Boundary()))
	}
	fmt.Printf("%d %d %d\n", elapsed, value, retained)
}
