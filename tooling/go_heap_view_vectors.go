//go:build ignore

// Generate independently checked indexed-heap callback and storage observations.
package main

import (
	"container/heap"
	"encoding/json"
	"os"
	"sort"
)

// item carries an independently tracked identity and mutable priority.
type item struct {
	Priority uint64 `json:"priority"`
	ID       uint64 `json:"id"`
}

// queue records every comparison and storage mutation in callback order.
type queue struct {
	Values    []item
	Positions []uint64
	Trace     uint64
	Reverse   bool
}

// record folds one observable callback and its arguments into an exact word.
func (q *queue) record(values ...uint64) {
	for _, value := range values {
		q.Trace = q.Trace*1099511628211 + value
	}
}

// Len supplies the current indexed storage length.
func (q *queue) Len() int { return len(q.Values) }

// Less compares priorities and then identities, recording the exact indices read.
func (q *queue) Less(i, j int) bool {
	q.record(1, uint64(i), uint64(j))
	a, b := q.Values[i], q.Values[j]
	first := a.Priority < b.Priority || a.Priority == b.Priority && a.ID < b.ID
	if q.Reverse {
		return a.Priority > b.Priority || a.Priority == b.Priority && a.ID > b.ID
	}
	return first
}

// Swap updates both stored elements and their externally maintained positions.
func (q *queue) Swap(i, j int) {
	q.record(2, uint64(i), uint64(j))
	q.Values[i], q.Values[j] = q.Values[j], q.Values[i]
	q.Positions[q.Values[i].ID] = uint64(i)
	q.Positions[q.Values[j].ID] = uint64(j)
}

// Push appends exactly one element and initializes its tracked position.
func (q *queue) Push(value any) {
	v := value.(item)
	q.record(3, v.ID, v.Priority)
	q.Positions[v.ID] = uint64(len(q.Values))
	q.Values = append(q.Values, v)
}

// Pop removes exactly the final element and invalidates its tracked position.
func (q *queue) Pop() any {
	v := q.Values[len(q.Values)-1]
	q.record(4, v.ID, v.Priority)
	q.Values = q.Values[:len(q.Values)-1]
	q.Positions[v.ID] = 1000000
	return v
}

// observation retains operation inputs and independent post-operation checksums.
type observation struct {
	Kind      string `json:"kind"`
	Index     uint64 `json:"index"`
	Priority  uint64 `json:"priority"`
	ID        uint64 `json:"id"`
	Length    uint64 `json:"length"`
	Storage   uint64 `json:"storage"`
	Positions uint64 `json:"positions"`
	Trace     uint64 `json:"trace"`
	Result    *item  `json:"result"`
	Valid     bool   `json:"valid"`
}

// fixture groups one initial sequence and a stateful indexed operation trace.
type fixture struct {
	Initial    []item        `json:"initial"`
	Slots      int           `json:"slots"`
	Operations []observation `json:"operations"`
	Ascending  []uint64      `json:"ascending"`
	Replace    uint64        `json:"replace_hash"`
	Fix        uint64        `json:"fix_hash"`
	Remove     uint64        `json:"remove_hash"`
	Reverse    uint64        `json:"reverse_hash"`
}

// orderedHash checks an owned heap through independently sorted public values.
func orderedHash(values []item, target int, replacement uint64, remove, backwards bool) uint64 {
	ordered := []item{}
	for _, value := range values {
		if value.ID == uint64(target) {
			if remove {
				continue
			}
			if replacement != 0 {
				value.Priority = replacement
			}
		}
		ordered = append(ordered, value)
	}
	sort.Slice(ordered, func(i, j int) bool {
		a, b := ordered[i], ordered[j]
		if backwards {
			return a.Priority > b.Priority || a.Priority == b.Priority && a.ID > b.ID
		}
		return a.Priority < b.Priority || a.Priority == b.Priority && a.ID < b.ID
	})
	words := []uint64{}
	for _, value := range ordered {
		words = append(words, value.Priority, value.ID)
	}
	return fold(words)
}

// fold is independent observable hashing rather than a heap implementation.
func fold(values []uint64) uint64 {
	h := uint64(1)
	for _, value := range values {
		h = h*1099511628211 + value
	}
	return h
}

// snapshot freezes all callback, element-order and identity-position observations.
func snapshot(q *queue, op observation) observation {
	values := make([]uint64, 0, len(q.Values)*2)
	for _, value := range q.Values {
		values = append(values, value.Priority, value.ID)
	}
	op.Length = uint64(q.Len())
	op.Storage = fold(values)
	op.Positions = fold(q.Positions)
	op.Trace = q.Trace
	return op
}

// makeFixture obtains expected heap behavior exclusively from the reference package.
func makeFixture(size int) fixture {
	f := fixture{Slots: size + 32, Initial: []item{}, Operations: []observation{}}
	q := &queue{Trace: 1, Values: []item{}, Positions: make([]uint64, f.Slots)}
	for i := range q.Positions {
		q.Positions[i] = 1000000
	}
	seed := uint64(size + 17)
	for i := 0; i < size; i++ {
		seed = (seed*1664525 + 1013904223) & 0xffffffff
		v := item{Priority: seed % 257, ID: uint64(i)}
		f.Initial = append(f.Initial, v)
		q.Values = append(q.Values, v)
		q.Positions[i] = uint64(i)
		f.Ascending = append(f.Ascending, v.Priority)
	}
	sort.Slice(f.Ascending, func(i, j int) bool { return f.Ascending[i] < f.Ascending[j] })
	f.Replace = orderedHash(f.Initial, size/2, 5000, false, false)
	f.Fix = orderedHash(f.Initial, size/2, 10000, false, false)
	f.Remove = orderedHash(f.Initial, size/2, 0, true, false)
	f.Reverse = orderedHash(f.Initial, size/2, 0, false, true)
	heap.Init(q)
	f.Operations = append(f.Operations, snapshot(q, observation{Kind: "init", Valid: true}))
	nextID := uint64(size)
	for step := 0; step < 64; step++ {
		seed = (seed*1664525 + 1013904223) & 0xffffffff
		op := observation{Kind: "push", ID: nextID, Priority: seed % 1024, Valid: true}
		switch step % 8 {
		case 0, 1, 6:
			heap.Push(q, item{Priority: op.Priority, ID: nextID})
			nextID++
		case 2:
			op.Kind = "fix"
			if q.Len() > 0 {
				op.Index = seed % uint64(q.Len())
				op.ID = q.Values[op.Index].ID
				q.Values[op.Index].Priority = op.Priority
				heap.Fix(q, int(op.Index))
			} else {
				op.Valid = false
			}
		case 3:
			op.Kind = "remove"
			if q.Len() > 0 {
				op.Index = seed % uint64(q.Len())
				value := heap.Remove(q, int(op.Index)).(item)
				op.Result = &value
			} else {
				op.Valid = false
			}
		case 4:
			op.Kind = "pop"
			if q.Len() > 0 {
				value := heap.Pop(q).(item)
				op.Result = &value
			} else {
				op.Valid = false
			}
		case 5:
			op.Kind = "reverse"
			q.Reverse = !q.Reverse
			heap.Init(q)
		case 7:
			op.Kind = "invalid"
			op.Index = uint64(q.Len()) + 7
			op.Valid = false
		}
		f.Operations = append(f.Operations, snapshot(q, op))
	}
	return f
}

// main emits the offline corpus to standard output without requiring runtime SDK access in tests.
func main() {
	all := []fixture{}
	for _, size := range []int{0, 1, 2, 3, 4, 5, 7, 8, 15, 16, 31, 32, 63, 64, 127, 257} {
		all = append(all, makeFixture(size))
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(all); err != nil {
		panic(err)
	}
}
