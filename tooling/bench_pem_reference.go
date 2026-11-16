// Benchmark public PEM byte pipelines with observable fresh outputs and a bounded scalar checksum loop.
package main

import (
	"bytes"
	"encoding/pem"
	"fmt"
	"os"
	"runtime"
	"strconv"
	"time"
)

// sinkBytes keeps each fresh encoded/copied payload allocation observable to the optimizer.
var sinkBytes []byte

// sinkLabel keeps the independently mutable decoded-label snapshot observable too.
var sinkLabel []byte

// fingerprint observes allocation length and representative boundary/body bytes without a full extra traversal.
func fingerprint(data []byte) uint64 {
	result := uint64(len(data))
	if len(data) > 0 {
		result += uint64(data[0]) + uint64(data[len(data)/2]) + uint64(data[len(data)-1])
	}
	if len(data) > 11 {
		result += uint64(data[11])
	}
	return result
}

// input constructs the exact deterministic two-block workload outside the measured interval.
func input(mode string, size, pattern int) ([2]*pem.Block, [2][]byte) {
	data := make([]byte, size)
	for at := range data {
		data[at] = byte(at*73 + 19)
	}
	headers := map[string]string{}
	if pattern == 1 {
		headers = map[string]string{"Z": "last", "A": "first", "Proc-Type": "4,ENCRYPTED", "DEK-Info": "NONE"}
	}
	if pattern == 2 {
		for index := 0; index < 256; index++ {
			name := "k" + strconv.Itoa(index)
			headers[name] = name
		}
	}
	blocks := [2]*pem.Block{{Type: "DATA", Headers: headers, Bytes: data}, {Type: "EATA", Headers: headers, Bytes: data}}
	var sources [2][]byte
	for index, block := range blocks {
		sources[index] = pem.EncodeToMemory(block)
		if mode == "preamble" || mode == "recovery" {
			char := byte('p')
			if pattern == 1 {
				char = '-'
			} else if pattern == 2 {
				char = '\n'
			}
			prefix := append(bytes.Repeat([]byte{char}, size), '\n')
			if mode == "recovery" {
				prefix = append([]byte("-----BEGIN BAD-----\n?\n-----END BAD-----\n"), prefix...)
			}
			sources[index] = append(prefix, sources[index]...)
		}
	}
	return blocks, sources
}

// observeDecode uses explicit mutable snapshots matching the facade's fresh byte observations.
func observeDecode(source []byte) uint64 {
	block, rest := pem.Decode(source)
	if block == nil {
		return 4294967296 + fingerprint(rest)
	}
	data := append([]byte(nil), block.Bytes...)
	label := []byte(block.Type)
	sinkBytes = data
	sinkLabel = label
	return fingerprint(data) + fingerprint(label) + uint64(len(block.Headers)) + fingerprint(rest)
}

// main times only public operation pipelines, rotating two prepared sources to retain live inputs.
func main() {
	if runtime.Version() != "go1.27.4" {
		panic("unexpected reference SDK")
	}
	if len(os.Args) != 5 {
		panic("expected mode size iterations pattern")
	}
	mode := os.Args[1]
	size, err := strconv.Atoi(os.Args[2])
	if err != nil || size < 0 {
		panic("invalid size")
	}
	count, err := strconv.ParseUint(os.Args[3], 10, 32)
	if err != nil {
		panic(err)
	}
	pattern, err := strconv.Atoi(os.Args[4])
	if err != nil || pattern < 0 || pattern > 2 {
		panic("invalid pattern")
	}
	blocks, sources := input(mode, size, pattern)
	hash := uint64(14695981039346656037)
	start := time.Now()
	for index := uint32(0); index < uint32(count); index++ {
		selected := index & 1
		var observed uint64
		switch mode {
		case "encode":
			output := pem.EncodeToMemory(blocks[selected])
			sinkBytes = output
			observed = fingerprint(output)
		case "writer":
			var output bytes.Buffer
			if err := pem.Encode(&output, blocks[selected]); err != nil {
				observed = 4294967295
			} else {
				sinkBytes = output.Bytes()
				observed = fingerprint(sinkBytes)
			}
		case "copy":
			output := append([]byte(nil), sources[selected]...)
			sinkBytes = output
			observed = fingerprint(output)
		default:
			observed = observeDecode(sources[selected])
		}
		hash = (hash ^ observed) * 1099511628211
	}
	elapsed := time.Since(start).Nanoseconds()
	fmt.Printf("%d %d\n", elapsed, hash)
}
