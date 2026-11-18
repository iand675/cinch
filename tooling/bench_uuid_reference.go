// Benchmark complete public identifier operations with independently observable outputs.
package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
	"uuid"
)

// sink retains fresh output bytes so formatting and snapshot work remain observable.
var sink []byte

// fingerprint matches the complete public-output observation used by the paired driver.
func fingerprint(b []byte) uint64 {
	n := len(b)
	r := uint64(n)
	if n > 0 {
		r += uint64(b[0]) + uint64(b[n/2]) + uint64(b[n-1])
	}
	return r
}

// main prepares live inputs outside timing and measures one public operation pipeline.
func main() {
	if len(os.Args) == 4 && os.Args[1] == "generate" {
		generation(os.Args[2], os.Args[3])
		return
	}
	if len(os.Args) != 5 {
		os.Exit(2)
	}
	mode := os.Args[1]
	size, _ := strconv.Atoi(os.Args[2])
	count, _ := strconv.Atoi(os.Args[3])
	pattern, _ := strconv.Atoi(os.Args[4])
	if size < 0 || count < 0 {
		os.Exit(2)
	}
	values := [2]uuid.UUID{uuid.MustParse("f81d4fae-7dec-11d0-a765-00a0c91e6bf6"), uuid.MustParse("081d4fae-7dec-11d0-a765-00a0c91e6bf7")}
	texts := [2]string{values[0].String(), values[1].String()}
	for i, s := range texts {
		switch pattern {
		case 1:
			s = strings.ReplaceAll(s, "-", "")
		case 2:
			s = "urn:uuid:" + s
		case 3:
			s = "{" + s + "}"
		case 4:
			s = s[:len(s)-1] + "x"
		}
		texts[i] = s
	}
	prefix := make([]byte, size)
	for i := range prefix {
		prefix[i] = byte(i*73 + 19)
	}
	hash := uint64(14695981039346656037)
	start := time.Now()
	for i := 0; i < count; i++ {
		at := i & 1
		var observed uint64
		switch mode {
		case "parse":
			v, err := uuid.Parse(texts[at])
			if err != nil {
				observed = 4294967295
			} else {
				sink = append([]byte(nil), v[:]...)
				observed = fingerprint(sink)
			}
		case "format":
			sink, _ = values[at].MarshalText()
			observed = fingerprint(sink)
		case "append":
			prepared := make([]byte, len(prefix), len(prefix)+36)
			copy(prepared, prefix)
			sink, _ = values[at].AppendText(prepared)
			observed = fingerprint(sink)
		case "compare":
			observed = uint64(values[at].Compare(values[1]) + 1)
		default:
			os.Exit(2)
		}
		hash = (hash ^ observed) * 1099511628211
	}
	fmt.Printf("%d %d\n", time.Since(start).Nanoseconds(), hash)
}

// generation observes actual public secure generation and reports only invariant equality across implementations.
func generation(mode, iterations string) {
	count, err := strconv.Atoi(iterations)
	if err != nil || count < 0 {
		os.Exit(2)
	}
	draw := uuid.New
	version := byte(4)
	if mode == "v4" {
		draw = uuid.NewV4
	} else if mode == "v7" {
		draw = uuid.NewV7
		version = 7
	} else if mode != "default" {
		os.Exit(2)
	}
	valid, checksum := uint64(0), uint64(14695981039346656037)
	start := time.Now()
	for index := 0; index < count; index++ {
		value := draw()
		sink = append([]byte(nil), value[:]...)
		if sink[6]>>4 == version && sink[8]>>6 == 2 {
			valid++
		}
		observed := uint64(sink[0]) + uint64(sink[7]) + uint64(sink[8]) + uint64(sink[15])
		checksum = (checksum ^ observed) * 1099511628211
	}
	fmt.Printf("%d %d %d\n", time.Since(start).Nanoseconds(), valid, checksum)
	if valid != uint64(count) {
		os.Exit(1)
	}
}
