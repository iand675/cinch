// Observe public buffered byte pipelines with explicit owned-source and fresh-token adapters.
package main

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"os"
	"runtime"
	"strconv"
	"time"
)

// ownedSource models a fresh byte-prefix event before it enters the private buffered destination.
type ownedSource struct {
	input []byte
	at    int
}

// Read reports end only after the final nonempty prefix, copying through the explicitly owned event.
func (s *ownedSource) Read(destination []byte) (int, error) {
	if s.at == len(s.input) {
		return 0, io.EOF
	}
	n := min(len(destination), len(s.input)-s.at)
	event := make([]byte, n)
	copy(event, s.input[s.at:s.at+n])
	copy(destination, event)
	s.at += n
	runtime.KeepAlive(event)
	return n, nil
}

// memorySink exposes written bytes without advertising direct-transfer or text-write shortcuts.
type memorySink struct {
	output []byte
	at     int
}

// Write confirms a complete bounded offer and preserves its inspectable advanced cursor.
func (s *memorySink) Write(input []byte) (int, error) {
	if len(input) > len(s.output)-s.at {
		return 0, io.ErrShortWrite
	}
	n := copy(s.output[s.at:], input)
	s.at += n
	return n, nil
}

// mix accumulates unsigned checksums with exactly defined wraparound.
func mix(a, b uint64) uint64 { return (a ^ b) * 1099511628211 }

// sample observes only present bytes, including empty outputs safely.
func sample(input []byte, at int) uint64 {
	if at >= 0 && at < len(input) {
		return uint64(input[at])
	}
	return 0
}

// fingerprint retains length and selected byte positions without a second full traversal.
func fingerprint(input []byte) uint64 {
	value := uint64(len(input)) + sample(input, 0) + sample(input, 11) + sample(input, len(input)/2) + sample(input, len(input)-1)
	runtime.KeepAlive(input)
	return value
}

// byteOperation consumes every byte through the public reader and preserves its actual status.
func byteOperation(input []byte) uint64 {
	reader := bufio.NewReaderSize(&ownedSource{input: input}, 4096)
	var total uint64
	for {
		value, err := reader.ReadByte()
		if errors.Is(err, io.EOF) {
			return total
		}
		if err != nil {
			return 4294967295
		}
		total += uint64(value)
	}
}

// runeOperation observes public scalar values and actual malformed-byte widths.
func runeOperation(input []byte) uint64 {
	reader := bufio.NewReaderSize(&ownedSource{input: input}, 4096)
	var total uint64
	for {
		value, width, err := reader.ReadRune()
		if errors.Is(err, io.EOF) {
			return total
		}
		if err != nil {
			return 4294967295
		}
		total += uint64(value) + uint64(width)
	}
}

// lineOperation observes every public line fragment through a fresh owned byte snapshot.
func lineOperation(input []byte) uint64 {
	reader := bufio.NewReaderSize(&ownedSource{input: input}, 4096)
	var hash uint64
	for {
		line, prefix, err := reader.ReadLine()
		if errors.Is(err, io.EOF) {
			return hash
		}
		if err != nil {
			return 4294967295
		}
		owned := make([]byte, len(line))
		copy(owned, line)
		value := fingerprint(owned)
		if prefix {
			value++
		}
		hash = mix(hash, value)
	}
}

// writeOperation includes writer creation, one raw write, one-layer flush and output ownership.
func writeOperation(input []byte) uint64 {
	sink := &memorySink{output: make([]byte, len(input))}
	writer := bufio.NewWriterSize(sink, 4096)
	n, err := writer.Write(input)
	if err != nil || writer.Flush() != nil {
		return 4294967295
	}
	return uint64(n) + fingerprint(sink.output)
}

// transferOperation uses ordinary public reader-to-sink transfer without hidden source or sink capabilities.
func transferOperation(input []byte) uint64 {
	sink := &memorySink{output: make([]byte, len(input))}
	reader := bufio.NewReaderSize(&ownedSource{input: input}, 4096)
	n, err := reader.WriteTo(sink)
	if err != nil || n < 0 {
		return 4294967295
	}
	return uint64(n) + fingerprint(sink.output)
}

// scanOperation observes fresh token owners for one selected public split capability.
func scanOperation(input []byte, split bufio.SplitFunc) uint64 {
	scanner := bufio.NewScanner(&ownedSource{input: input})
	scanner.Split(split)
	var hash uint64
	for scanner.Scan() {
		token := scanner.Bytes()
		owned := make([]byte, len(token))
		copy(owned, token)
		hash = mix(hash, fingerprint(owned))
	}
	if scanner.Err() != nil {
		return 4294967295
	}
	return hash
}

// scanLineOperation uses the default line tokenization contract with fresh token observations.
func scanLineOperation(input []byte) uint64 { return scanOperation(input, bufio.ScanLines) }

// scanRuneOperation includes replacement tokens for malformed source bytes.
func scanRuneOperation(input []byte) uint64 { return scanOperation(input, bufio.ScanRunes) }

// scanWordOperation includes the complete Unicode whitespace boundary semantics.
func scanWordOperation(input []byte) uint64 { return scanOperation(input, bufio.ScanWords) }

// copyOperation supplies a distinct fresh allocation/copy control over the same prepared owners.
func copyOperation(input []byte) uint64 {
	output := make([]byte, len(input))
	copy(output, input)
	return fingerprint(output)
}

// prepare creates alternating ASCII-line, mixed UTF-8 or arbitrary-byte owners before timing.
func prepare(size, pattern int) [2][]byte {
	a := make([]byte, size)
	unicode := []byte("Aé β😀\n")
	for i := range a {
		if pattern == 1 {
			a[i] = unicode[i%len(unicode)]
		} else if pattern == 2 {
			a[i] = byte(i*13 + 7)
		} else if i%64 == 63 {
			a[i] = 10
		} else {
			a[i] = byte(65 + i%26)
		}
	}
	b := make([]byte, size)
	copy(b, a)
	if size > 0 {
		a[0] = 65
		b[0] = 66
	}
	return [2][]byte{a, b}
}

// runByte dispatches before the bounded iteration loop, retaining alternating prepared owners.
func runByte(inputs [2][]byte, count uint32) uint64 {
	hash := uint64(14695981039346656037)
	for i := uint32(0); i < count; i++ {
		hash = mix(hash, byteOperation(inputs[i&1]))
	}
	runtime.KeepAlive(inputs)
	return hash
}

// runRune dispatches before the bounded iteration loop, retaining alternating prepared owners.
func runRune(inputs [2][]byte, count uint32) uint64 {
	hash := uint64(14695981039346656037)
	for i := uint32(0); i < count; i++ {
		hash = mix(hash, runeOperation(inputs[i&1]))
	}
	runtime.KeepAlive(inputs)
	return hash
}

// runLine dispatches before the bounded iteration loop, retaining alternating prepared owners.
func runLine(inputs [2][]byte, count uint32) uint64 {
	hash := uint64(14695981039346656037)
	for i := uint32(0); i < count; i++ {
		hash = mix(hash, lineOperation(inputs[i&1]))
	}
	runtime.KeepAlive(inputs)
	return hash
}

// runWrite dispatches before the bounded iteration loop, retaining alternating prepared owners.
func runWrite(inputs [2][]byte, count uint32) uint64 {
	hash := uint64(14695981039346656037)
	for i := uint32(0); i < count; i++ {
		hash = mix(hash, writeOperation(inputs[i&1]))
	}
	runtime.KeepAlive(inputs)
	return hash
}

// runTransfer dispatches before the bounded iteration loop, retaining alternating prepared owners.
func runTransfer(inputs [2][]byte, count uint32) uint64 {
	hash := uint64(14695981039346656037)
	for i := uint32(0); i < count; i++ {
		hash = mix(hash, transferOperation(inputs[i&1]))
	}
	runtime.KeepAlive(inputs)
	return hash
}

// runScanLine dispatches before the bounded iteration loop, retaining alternating prepared owners.
func runScanLine(inputs [2][]byte, count uint32) uint64 {
	hash := uint64(14695981039346656037)
	for i := uint32(0); i < count; i++ {
		hash = mix(hash, scanLineOperation(inputs[i&1]))
	}
	runtime.KeepAlive(inputs)
	return hash
}

// runScanRune dispatches before the bounded iteration loop, retaining alternating prepared owners.
func runScanRune(inputs [2][]byte, count uint32) uint64 {
	hash := uint64(14695981039346656037)
	for i := uint32(0); i < count; i++ {
		hash = mix(hash, scanRuneOperation(inputs[i&1]))
	}
	runtime.KeepAlive(inputs)
	return hash
}

// runScanWord dispatches before the bounded iteration loop, retaining alternating prepared owners.
func runScanWord(inputs [2][]byte, count uint32) uint64 {
	hash := uint64(14695981039346656037)
	for i := uint32(0); i < count; i++ {
		hash = mix(hash, scanWordOperation(inputs[i&1]))
	}
	runtime.KeepAlive(inputs)
	return hash
}

// runCopy dispatches before the bounded iteration loop, retaining alternating prepared owners.
func runCopy(inputs [2][]byte, count uint32) uint64 {
	hash := uint64(14695981039346656037)
	for i := uint32(0); i < count; i++ {
		hash = mix(hash, copyOperation(inputs[i&1]))
	}
	runtime.KeepAlive(inputs)
	return hash
}

// perform selects one concrete public pipeline before entering its operation loop.
func perform(mode string, inputs [2][]byte, count uint32) uint64 {
	switch mode {
	case "byte":
		return runByte(inputs, count)
	case "rune":
		return runRune(inputs, count)
	case "line":
		return runLine(inputs, count)
	case "write":
		return runWrite(inputs, count)
	case "transfer":
		return runTransfer(inputs, count)
	case "scan-line":
		return runScanLine(inputs, count)
	case "scan-rune":
		return runScanRune(inputs, count)
	case "scan-word":
		return runScanWord(inputs, count)
	case "copy":
		return runCopy(inputs, count)
	default:
		return 0
	}
}

// main keeps command parsing and input preparation outside the measured interval.
func main() {
	if len(os.Args) != 5 {
		os.Exit(2)
	}
	size, e1 := strconv.Atoi(os.Args[2])
	iterations, e2 := strconv.ParseUint(os.Args[3], 10, 32)
	pattern, e3 := strconv.Atoi(os.Args[4])
	if e1 != nil || e2 != nil || e3 != nil || size < 0 || size > 65536 {
		os.Exit(2)
	}
	inputs := prepare(size, pattern)
	start := time.Now()
	hash := perform(os.Args[1], inputs, uint32(iterations))
	elapsed := time.Since(start).Nanoseconds()
	fmt.Printf("%d %d\n", elapsed, hash)
}
