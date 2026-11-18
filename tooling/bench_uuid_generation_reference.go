// Benchmark actual secure UUID generation without predicting or replacing entropy or time.
package main

import (
	"fmt"
	"os"
	"strconv"
	"time"
	"uuid"
)

// sink retains each fresh mutable value snapshot as part of the public pipeline.
var sink []byte

// main reports elapsed time, valid result count and a variable random-output fingerprint separately.
func main() {
	if len(os.Args) != 3 {
		os.Exit(2)
	}
	mode := os.Args[1]
	count, err := strconv.Atoi(os.Args[2])
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
