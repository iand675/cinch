// Audit mutable separator/input and pull lifecycle behavior with the pinned public iterator API.
package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"iter"
	"os"
	"runtime"
)

// snapshot captures actual aliased bytes and one pull result at a precise mutation boundary.
type snapshot struct {
	Action    string   `json:"action"`
	Input     string   `json:"input"`
	Separator string   `json:"separator"`
	Value     string   `json:"value"`
	Present   bool     `json:"present"`
	Held      []string `json:"held"`
}

// record observes all retained yielded slices after the most recent checked mutation.
func record(action string, raw, sep, value []byte, present bool, held [][]byte) snapshot {
	values := []string{}
	for _, part := range held {
		values = append(values, hex.EncodeToString(part))
	}
	return snapshot{action, hex.EncodeToString(raw), hex.EncodeToString(sep), hex.EncodeToString(value), present, values}
}

// changedSeparator mutates current raw separator contents between coroutine pulls.
func changedSeparator(after bool) []snapshot {
	raw, sep := []byte("a,b;c,d"), []byte(",")
	var sequence iter.Seq[[]byte]
	if after {
		sequence = bytes.SplitAfterSeq(raw, sep)
	} else {
		sequence = bytes.SplitSeq(raw, sep)
	}
	next, stop := iter.Pull(sequence)
	defer stop()
	rows, held := []snapshot{}, [][]byte{}
	value, ok := next()
	held = append(held, value)
	rows = append(rows, record("first", raw, sep, value, ok, held))
	sep[0] = ';'
	raw[0] = 'z'
	value, ok = next()
	held = append(held, value)
	rows = append(rows, record("separator-and-prior-result-mutation", raw, sep, value, ok, held))
	sep[0] = ','
	value, ok = next()
	held = append(held, value)
	rows = append(rows, record("separator-restored", raw, sep, value, ok, held))
	for _, action := range []string{"last", "exhausted", "exhausted-again"} {
		value, ok = next()
		if ok {
			held = append(held, value)
		}
		rows = append(rows, record(action, raw, sep, value, ok, held))
	}
	return rows
}

// aliasedSeparator demonstrates that input writes may simultaneously change the retained delimiter.
func aliasedSeparator() []snapshot {
	raw := []byte("a|b|c")
	sep := raw[1:2]
	next, stop := iter.Pull(bytes.SplitSeq(raw, sep))
	defer stop()
	rows, held := []snapshot{}, [][]byte{}
	raw[1] = ';'
	value, ok := next()
	held = append(held, value)
	rows = append(rows, record("aliased-separator-before-first-pull", raw, sep, value, ok, held))
	raw[1] = '|'
	raw[3] = ':'
	value, ok = next()
	held = append(held, value)
	rows = append(rows, record("aliased-separator-and-future-input", raw, sep, value, ok, held))
	value, ok = next()
	rows = append(rows, record("exhausted", raw, sep, value, ok, held))
	return rows
}

// stoppedPull checks explicit stop and repeated pulls after the coroutine is closed.
func stoppedPull() []snapshot {
	raw, sep := []byte("a,b,c"), []byte(",")
	next, stop := iter.Pull(bytes.SplitSeq(raw, sep))
	defer stop()
	value, ok := next()
	held := [][]byte{value}
	rows := []snapshot{record("first", raw, sep, value, ok, held)}
	stop()
	raw[2] = 'z'
	sep[0] = ';'
	for _, action := range []string{"after-stop", "after-stop-again"} {
		value, ok = next()
		rows = append(rows, record(action, raw, sep, value, ok, held))
	}
	return rows
}

// repeatedSequence distinguishes reusing a returned single-use sequence from calling its factory again.
func repeatedSequence() map[string][]string {
	raw, sep := []byte("a,b,c"), []byte(",")
	sequence := bytes.SplitSeq(raw, sep)
	collect := func(value iter.Seq[[]byte]) []string {
		parts := []string{}
		for part := range value {
			parts = append(parts, hex.EncodeToString(part))
		}
		return parts
	}
	return map[string][]string{"first_invocation": collect(sequence), "same_sequence_second_invocation": collect(sequence), "fresh_factory_invocation": collect(bytes.SplitSeq(raw, sep))}
}

// independentPulls contrasts two fresh factory cursors with copied callbacks sharing one cursor.
func independentPulls() []snapshot {
	raw, sep := []byte("a,b,c"), []byte(",")
	first, stopFirst := iter.Pull(bytes.SplitSeq(raw, sep))
	defer stopFirst()
	second, stopSecond := iter.Pull(bytes.SplitSeq(raw, sep))
	defer stopSecond()
	copyFirst := first
	rows, held := []snapshot{}, [][]byte{}
	for _, item := range []struct {
		name string
		next func() ([]byte, bool)
	}{{"first-factory", first}, {"second-factory", second}, {"copied-first-callback", copyFirst}, {"first-after-copy", first}, {"second-independent", second}} {
		value, ok := item.next()
		if ok {
			held = append(held, value)
		}
		rows = append(rows, record(item.name, raw, sep, value, ok, held))
	}
	return rows
}

// emptyMutation verifies that future malformed scalar boundaries are observed at each pull.
func emptyMutation() []snapshot {
	raw, sep := []byte{'a', 240, 159, 153, 130, 'b'}, []byte{}
	next, stop := iter.Pull(bytes.SplitSeq(raw, sep))
	defer stop()
	rows, held := []snapshot{}, [][]byte{}
	value, ok := next()
	held = append(held, value)
	rows = append(rows, record("first", raw, sep, value, ok, held))
	raw[1] = 255
	for i := 0; i < 7; i++ {
		value, ok = next()
		if ok {
			held = append(held, value)
		}
		rows = append(rows, record("after-future-scalar-mutation", raw, sep, value, ok, held))
	}
	return rows
}

// linesMutation verifies live line-boundary changes and carriage-return preservation.
func linesMutation() []snapshot {
	raw, sep := []byte("a\nb\r\nc\n"), []byte{'\n'}
	next, stop := iter.Pull(bytes.Lines(raw))
	defer stop()
	rows, held := []snapshot{}, [][]byte{}
	value, ok := next()
	held = append(held, value)
	rows = append(rows, record("first-line", raw, sep, value, ok, held))
	raw[4] = 'x'
	for i := 0; i < 3; i++ {
		value, ok = next()
		if ok {
			held = append(held, value)
		}
		rows = append(rows, record("after-line-boundary-mutation", raw, sep, value, ok, held))
	}
	return rows
}

// multibyteMutation observes raw delimiter changes before first pull and after the suspended yield.
func multibyteMutation() []snapshot {
	raw, sep := []byte("a--b==c--d"), []byte("--")
	next, stop := iter.Pull(bytes.SplitSeq(raw, sep))
	defer stop()
	rows, held := []snapshot{}, [][]byte{}
	sep[0], sep[1] = '=', '='
	value, ok := next()
	held = append(held, value)
	rows = append(rows, record("changed-before-first", raw, sep, value, ok, held))
	sep[0], sep[1] = '-', '-'
	for i := 0; i < 4; i++ {
		value, ok = next()
		if ok {
			held = append(held, value)
		}
		rows = append(rows, record("restored-after-first", raw, sep, value, ok, held))
	}
	return rows
}

// yieldedSeparator checks mutation through a yielded fragment that also aliases the raw delimiter.
func yieldedSeparator() []snapshot {
	raw := []byte("a|b|c")
	sep := raw[1:2]
	next, stop := iter.Pull(bytes.SplitAfterSeq(raw, sep))
	defer stop()
	value, ok := next()
	held := [][]byte{value}
	rows := []snapshot{record("first-after", raw, sep, value, ok, held)}
	value[1] = ';'
	for i := 0; i < 3; i++ {
		value, ok = next()
		if ok {
			held = append(held, value)
		}
		rows = append(rows, record("delimiter-mutated-through-yielded-alias", raw, sep, value, ok, held))
	}
	return rows
}

// counterBoundaries observes exact fragment extents across byte and word-sized cursor offsets.
func counterBoundaries() []int {
	raw := make([]byte, 65540)
	for _, at := range []int{254, 256, 65534, 65535, 65536} {
		raw[at] = ','
	}
	lengths := []int{}
	for part := range bytes.SplitSeq(raw, []byte{','}) {
		lengths = append(lengths, len(part))
	}
	return lengths
}

// main records the exact reference epoch and all public pull/mutation observations.
func main() {
	if runtime.Version() != "go1.27.4" {
		panic("unexpected reference SDK")
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(map[string]any{"sdk_version": runtime.Version(), "split": changedSeparator(false), "after": changedSeparator(true), "aliased": aliasedSeparator(), "stop": stoppedPull(), "repeat": repeatedSequence(), "independent": independentPulls(), "empty_mutation": emptyMutation(), "lines_mutation": linesMutation(), "multibyte_mutation": multibyteMutation(), "yielded_separator": yieldedSeparator(), "counter_boundaries": counterBoundaries()}); err != nil {
		panic(err)
	}
}
