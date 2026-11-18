// Generate independent pinned-SDK observations in explicitly separate model and real modes.
package main

import (
	"bytes"
	"crypto/fips140"
	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"math/big"
	"os"
	"runtime"
	"strconv"
	"strings"
)

// fixtureFailure identifies only the deliberately scripted reader-error model.
var fixtureFailure = errors.New("fixture-reader-failure")

// output writes complete independent observations without implementation assertions.
var output = json.NewEncoder(os.Stdout)

// emit writes one JSONL record and fails if evidence cannot be retained.
func emit(record map[string]any) {
	if err := output.Encode(record); err != nil {
		panic(err)
	}
}

// setting returns the final explicit value of one process-local compatibility option.
func setting(key string) string {
	value := ""
	for _, part := range strings.Split(os.Getenv("GODEBUG"), ",") {
		pair := strings.SplitN(part, "=", 2)
		if len(pair) == 2 && pair[0] == key {
			value = pair[1]
		}
	}
	return value
}

// metadata identifies the actual SDK mode before any model or real provider call.
func metadata(mode string) {
	emit(map[string]any{"kind": "metadata", "mode": mode, "version": runtime.Version(),
		"os": runtime.GOOS, "arch": runtime.GOARCH, "godebug": os.Getenv("GODEBUG"),
		"fips_enabled": fips140.Enabled(), "fips_enforced": fips140.Enforced(),
		"runtime_timing_claim": false})
}

// scriptedReader records valid finite byte delivery, optional short chunks and model failures.
// Its bytes are fixtures only; it is never used by real-default or benchmark modes.
type scriptedReader struct {
	data         []byte
	position     int
	chunk        int
	zeroFirst    bool
	errorOnLast  bool
	failAt       int
	calls        int
	requests     []int
}

// Read implements the scripted model while retaining exact callback progress.
func (reader *scriptedReader) Read(destination []byte) (int, error) {
	reader.calls++
	reader.requests = append(reader.requests, len(destination))
	if reader.calls > 1024 {
		return 0, errors.New("fixture read budget exceeded")
	}
	if reader.failAt == reader.calls {
		return 0, fixtureFailure
	}
	if reader.zeroFirst && reader.calls == 1 {
		return 0, nil
	}
	if reader.position == len(reader.data) {
		return 0, io.EOF
	}
	count := len(destination)
	if reader.chunk > 0 && count > reader.chunk {
		count = reader.chunk
	}
	if count > len(reader.data)-reader.position {
		count = len(reader.data) - reader.position
	}
	copy(destination, reader.data[reader.position:reader.position+count])
	reader.position += count
	if reader.errorOnLast && reader.position == len(reader.data) {
		return count, fixtureFailure
	}
	return count, nil
}

// readerObservation preserves source state independently of the sampled value.
func readerObservation(reader *scriptedReader) map[string]any {
	return map[string]any{"calls": reader.calls, "bytes": reader.position,
		"requests": reader.requests, "remaining_hex": hex.EncodeToString(reader.data[reader.position:])}
}

// errorKind classifies relevant boundaries without discarding the actual error string.
func errorKind(err error) string {
	if err == nil {
		return "none"
	}
	if errors.Is(err, io.EOF) {
		return "EOF"
	}
	if errors.Is(err, io.ErrUnexpectedEOF) {
		return "UnexpectedEOF"
	}
	if errors.Is(err, fixtureFailure) {
		return "fixture-error"
	}
	return "other"
}

// integerCase invokes the actual arbitrary-bound API, including its panic boundary.
func integerCase(name string, maximum *big.Int, reader *scriptedReader) {
	record := map[string]any{"kind": "Int", "scope": "deterministic-reader-model",
		"case": name, "maximum": maximum.String(), "input_hex": hex.EncodeToString(reader.data)}
	defer func() {
		if value := recover(); value != nil {
			record["panic"] = fmt.Sprint(value)
		}
		record["reader"] = readerObservation(reader)
		emit(record)
	}()
	value, err := rand.Int(reader, maximum)
	record["error_kind"] = errorKind(err)
	if err != nil {
		record["error"] = err.Error()
	}
	if value != nil {
		record["value"] = value.String()
	}
}

// integerModels covers exact powers, large bounds, masking/rejection and reader failures.
func integerModels() {
	for _, maximum := range []int64{-7, 0, 1, 2, 3, 127, 128, 129, 255, 256, 257, 511, 512, 513} {
		integerCase("small-"+strconv.FormatInt(maximum, 10), big.NewInt(maximum),
			&scriptedReader{data: []byte{255, 255, 0, 0, 0, 0}})
	}
	for _, bits := range []uint{64, 128, 256, 521} {
		for _, delta := range []int64{-1, 0, 1} {
			maximum := new(big.Int).Lsh(big.NewInt(1), bits)
			maximum.Add(maximum, big.NewInt(delta))
			width := (new(big.Int).Sub(maximum, big.NewInt(1)).BitLen() + 7) / 8
			data := append(bytes.Repeat([]byte{255}, width), make([]byte, width)...)
			integerCase(fmt.Sprintf("wide-%d-%d", bits, delta), maximum,
				&scriptedReader{data: data, chunk: 3})
		}
	}
	for _, maximum := range []int64{3, 5, 17, 128, 129, 255, 256} {
		for raw := 0; raw < 256; raw++ {
			integerCase(fmt.Sprintf("mask-%d-%d", maximum, raw), big.NewInt(maximum),
				&scriptedReader{data: []byte{byte(raw), 0}})
		}
	}
	integerCase("immediate-error", big.NewInt(257), &scriptedReader{failAt: 1})
	integerCase("partial-EOF", big.NewInt(257), &scriptedReader{data: []byte{1}, chunk: 1})
	integerCase("partial-with-error", big.NewInt(257), &scriptedReader{data: []byte{1}, errorOnLast: true})
	integerCase("complete-with-error", big.NewInt(128), &scriptedReader{data: []byte{42}, errorOnLast: true})
	integerCase("zero-once-then-progress", big.NewInt(128), &scriptedReader{data: []byte{42}, zeroFirst: true})
}

// textModel temporarily replaces the public reader only inside an isolated model process.
func textModel(name string, input []byte) {
	original := rand.Reader
	reader := &scriptedReader{data: input}
	rand.Reader = reader
	defer func() { rand.Reader = original }()
	text := rand.Text()
	emit(map[string]any{"kind": "Text", "scope": "deterministic-reader-model",
		"case": name, "input_hex": hex.EncodeToString(input), "text": text,
		"reader": readerObservation(reader)})
}

// textModels exercises the actual current Text API and an independent alphabet oracle.
func textModels() {
	const alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"
	for value := 0; value < 256; value++ {
		textModel("repeated-"+strconv.Itoa(value), bytes.Repeat([]byte{byte(value)}, 26))
	}
	for length := 0; length <= 40; length++ {
		input := make([]byte, length)
		for index := range input {
			input[index] = byte(index*37 + length)
		}
		mapped := make([]byte, len(input))
		for index, value := range input {
			mapped[index] = alphabet[value&31]
		}
		emit(map[string]any{"kind": "token-symbol-map", "scope": "alphabet-spec-oracle",
			"input_hex": hex.EncodeToString(input), "text": string(mapped)})
	}
	for lane := 0; lane < 16; lane++ {
		for value := 0; value < 256; value++ {
			input := make([]byte, 32)
			input[lane] = byte(value)
			mapped := make([]byte, len(input))
			for index, raw := range input {
				mapped[index] = alphabet[raw&31]
			}
			emit(map[string]any{"kind": "token-lane", "scope": "alphabet-spec-oracle",
				"lane": lane, "input_hex": hex.EncodeToString(input), "text": string(mapped)})
		}
	}
}

// naturalByteModels uses the SDK's arbitrary-precision byte importer as a separate oracle.
func naturalByteModels() {
	for length := 0; length <= 80; length++ {
		input := make([]byte, length)
		for index := range input {
			input[index] = byte(255 - index*29)
		}
		for _, mask := range []byte{0, 1, 3, 7, 15, 31, 63, 127, 255} {
			masked := append([]byte(nil), input...)
			if len(masked) > 0 {
				masked[0] &= mask
			}
			value := new(big.Int).SetBytes(masked)
			emit(map[string]any{"kind": "natural-from-bytes", "scope": "independent-big-integer-oracle",
				"input_hex": hex.EncodeToString(input), "high_mask": mask, "value": value.String()})
		}
	}
}

// replayReader supplies a known candidate without advancing for optional probe reads.
// This reader is solely a deprecated compatibility-model fixture, never secure entropy.
type replayReader struct {
	bytes    []byte
	calls    int
	requests []int
}

// Read repeats the candidate prefix so an optional one-byte probe cannot shift it.
func (reader *replayReader) Read(destination []byte) (int, error) {
	reader.calls++
	reader.requests = append(reader.requests, len(destination))
	if reader.calls > 1024 {
		return 0, errors.New("prime fixture read budget exceeded")
	}
	for index := range destination {
		destination[index] = reader.bytes[index%len(reader.bytes)]
	}
	return len(destination), nil
}

// primeCompatibility captures the optional-reader branch separately from secure defaults.
func primeCompatibility() {
	if setting("cryptocustomrand") != "1" {
		panic("prime compatibility fixtures require explicit cryptocustomrand=1")
	}
	for _, decimal := range []string{"3", "7", "13", "29", "193", "389", "65521", "18446744073709551557", "340282366920938463463374607431768211297"} {
		candidate, ok := new(big.Int).SetString(decimal, 10)
		if !ok || !candidate.ProbablyPrime(64) || candidate.Bit(candidate.BitLen()-2) != 1 {
			panic("invalid prime fixture")
		}
		reader := &replayReader{bytes: candidate.Bytes()}
		value, err := rand.Prime(reader, candidate.BitLen())
		record := map[string]any{"kind": "Prime", "scope": "deprecated-custom-reader-model",
			"candidate": decimal, "bits": candidate.BitLen(), "calls": reader.calls,
			"requests": reader.requests, "error_kind": errorKind(err)}
		if value != nil {
			record["value"] = value.String()
		}
		if err != nil {
			record["error"] = err.Error()
		}
		emit(record)
	}
}

// secureDefaults observes only the actual default provider, never a scripted source.
func secureDefaults() {
	if setting("cryptocustomrand") == "1" {
		panic("secure-default fixtures must not enable custom-reader compatibility")
	}
	for _, count := range []int{0, 1, 16, 26, 32, 256} {
		data := make([]byte, count)
		read, err := rand.Read(data)
		emit(map[string]any{"kind": "Read", "scope": "real-system-provider",
			"requested": count, "read": read, "error_kind": errorKind(err), "hex": hex.EncodeToString(data)})
	}
	for index := 0; index < 32; index++ {
		emit(map[string]any{"kind": "Text", "scope": "real-system-provider", "index": index, "text": rand.Text()})
	}
	for _, bits := range []uint{1, 7, 8, 9, 64, 65, 128, 256, 521} {
		maximum := new(big.Int).Add(new(big.Int).Lsh(big.NewInt(1), bits), big.NewInt(1))
		value, err := rand.Int(rand.Reader, maximum)
		record := map[string]any{"kind": "Int", "scope": "real-system-provider",
			"maximum": maximum.String(), "error_kind": errorKind(err)}
		if value != nil {
			record["value"] = value.String()
			record["in_range"] = value.Sign() >= 0 && value.Cmp(maximum) < 0
		}
		if err != nil {
			record["error"] = err.Error()
		}
		emit(record)
	}
	for _, bits := range []int{-1, 0, 1, 2, 3, 4, 5, 7, 8, 9, 16, 32, 64, 128} {
		ignored := &scriptedReader{failAt: 1}
		value, err := rand.Prime(ignored, bits)
		record := map[string]any{"kind": "Prime", "scope": "real-system-provider-reader-ignore-check",
			"bits": bits, "supplied_reader": readerObservation(ignored), "error_kind": errorKind(err)}
		if value != nil {
			record["value"] = value.String()
			record["actual_bits"] = value.BitLen()
			record["probably_prime_32"] = value.ProbablyPrime(32)
			record["top_two_set"] = value.Bit(bits-1) == 1 && value.Bit(bits-2) == 1
		}
		if err != nil {
			record["error"] = err.Error()
		}
		emit(record)
	}
}

// readFailure invokes the irrecoverable public Read failure only in its own process.
func readFailure() {
	rand.Reader = &scriptedReader{failAt: 1}
	rand.Read(make([]byte, 16))
	panic("public Read unexpectedly returned after a reader failure")
}

// main isolates deterministic reader models, actual provider cases and fatal behavior.
func main() {
	if len(os.Args) != 2 {
		panic("select int-model, text-model, natural-model, prime-compat, secure-default or read-failure")
	}
	mode := os.Args[1]
	metadata(mode)
	switch mode {
	case "int-model":
		integerModels()
	case "text-model":
		textModels()
	case "natural-model":
		naturalByteModels()
	case "prime-compat":
		primeCompatibility()
	case "secure-default":
		secureDefaults()
	case "read-failure":
		readFailure()
	default:
		panic("unknown fixture mode")
	}
}
