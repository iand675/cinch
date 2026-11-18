// Measure actual secure public operations with retained result ownership.
package main

import (
	"crypto/rand"
	"fmt"
	"os"
	"runtime"
	"strconv"
	"time"
	"uuid"
)

// sink retains the final fresh mutable result across the closing clock.
var sink []byte

// fingerprint observes length and boundary/middle bytes without hashing the buffer.
func fingerprint(bytes []byte) uint64 {
	if len(bytes) == 0 {
		return 0
	}
	return uint64(len(bytes)) + uint64(bytes[0]) + uint64(bytes[len(bytes)/2]) + uint64(bytes[len(bytes)-1])
}

// mix preserves the same complete wrapping observation checksum as the other pipeline.
func mix(checksum, observation uint64) uint64 {
	return (checksum ^ observation) * 1099511628211
}

// observeUUID copies the public value into a fresh retained mutable byte snapshot.
func observeUUID(value uuid.UUID, version byte) (uint64, uint64) {
	sink = append([]byte(nil), value[:]...)
	valid := uint64(0)
	if sink[6]>>4 == version && sink[8]>>6 == 2 {
		valid = 1
	}
	observation := uint64(sink[0]) + uint64(sink[7]) + uint64(sink[8]) + uint64(sink[15])
	return valid, observation & 1023
}

// probe marks one actual default crypto-provider draw for untimed path inspection.
func probe(size int) {
	bytes := make([]byte, size)
	fmt.Fprintf(os.Stderr, "BEGIN SDK ENTROPY DRAW len=%d\n", size)
	count, err := rand.Read(bytes)
	fmt.Fprintf(os.Stderr, "END SDK ENTROPY DRAW len=%d\n", size)
	if err != nil || count != size {
		panic("actual secure provider failed")
	}
	sink = bytes
	fmt.Printf("sdk-probe bytes=%d status=ok\n", len(sink))
	runtime.KeepAlive(sink)
}

// main dispatches outside the interval and reports invariants separately from random data.
func main() {
	if len(os.Args) == 3 && os.Args[1] == "probe" {
		size, err := strconv.Atoi(os.Args[2])
		if err != nil || size < 0 || size > 4096 {
			os.Exit(2)
		}
		probe(size)
		return
	}
	if len(os.Args) != 4 {
		os.Exit(2)
	}
	mode := os.Args[1]
	size, sizeError := strconv.Atoi(os.Args[2])
	count, countError := strconv.ParseUint(os.Args[3], 10, 32)
	if sizeError != nil || countError != nil || size < 0 || size > 4096 {
		os.Exit(2)
	}
	sink = make([]byte, 0)
	valid, checksum := uint64(0), uint64(14695981039346656037)
	var start time.Time
	switch mode {
	case "random":
		start = time.Now()
		for index := uint64(0); index < count; index++ {
			bytes := make([]byte, size)
			read, err := rand.Read(bytes)
			if err != nil || read != size {
				panic("actual secure provider failed")
			}
			sink = bytes
			if len(sink) == size {
				valid++
			}
			checksum = mix(checksum, fingerprint(sink))
		}
	case "allocation":
		start = time.Now()
		for index := uint64(0); index < count; index++ {
			sink = make([]byte, size)
			if size > 0 {
				sink[0] = byte(index & 255)
			}
			if len(sink) == size {
				valid++
			}
			checksum = mix(checksum, fingerprint(sink))
		}
	case "counter":
		start = time.Now()
		for index := uint64(0); index < count; index++ {
			valid++
			checksum = mix(checksum, index)
		}
	case "clock":
		start = time.Now()
		for index := uint64(0); index < count; index++ {
			valid++
			checksum = mix(checksum, uint64(time.Now().UnixNano()))
		}
	case "v4":
		start = time.Now()
		for index := uint64(0); index < count; index++ {
			accepted, observation := observeUUID(uuid.NewV4(), 4)
			valid += accepted
			checksum = mix(checksum, observation)
		}
	case "v7":
		start = time.Now()
		for index := uint64(0); index < count; index++ {
			accepted, observation := observeUUID(uuid.NewV7(), 7)
			valid += accepted
			checksum = mix(checksum, observation)
		}
	default:
		os.Exit(2)
	}
	end := time.Now()
	retained := fingerprint(sink)
	fmt.Printf("%d %d %d %d\n", end.Sub(start).Nanoseconds(), valid, checksum, retained)
	runtime.KeepAlive(sink)
	if valid != count {
		os.Exit(1)
	}
}
