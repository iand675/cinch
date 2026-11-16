// Generate independent circular-sequence values, stable identities and alias observations.
package main

import (
	"container/ring"
	"encoding/json"
	"os"
)

// Item separates a stable fixture identity from a replaceable logical value.
type Item struct{ ID, Value uint64 }

// Operation records one action and the complete post-action observation.
type Operation struct {
	Kind     int    `json:"kind"`
	Source   int    `json:"source"`
	Other    int    `json:"other"`
	Result   int    `json:"result"`
	Distance int    `json:"distance"`
	Count    int    `json:"count"`
	Value    uint64 `json:"value"`
	Returned uint64 `json:"returned"`
	Visit    uint64 `json:"visit"`
	State    uint64 `json:"state"`
}

// Case supplies an initial size and an exact ordered mutation trace.
type Case struct {
	Initial    int         `json:"initial"`
	Operations []Operation `json:"operations"`
}

// World keeps every original node and eight independently replaceable cursor aliases.
type World struct {
	Handles []*ring.Ring
	Cursors [8]*ring.Ring
}

// mix preserves every observation's order using wrapping machine arithmetic.
func mix(hash, value uint64) uint64 { return hash*1099511628211 + value }

// identity distinguishes an absent cursor from the first fixture identity.
func identity(cursor *ring.Ring) uint64 {
	if cursor == nil {
		return 0
	}
	return cursor.Value.(Item).ID + 1
}

// initialize appends one fresh cycle with independently initialized logical values.
func (world *World) initialize(count int, factor, offset uint64) *ring.Ring {
	first := ring.New(count)
	cursor := first
	for index := 0; index < count; index++ {
		cursor.Value = Item{uint64(len(world.Handles)), (uint64(index)*factor + offset) % 997}
		world.Handles = append(world.Handles, cursor)
		cursor = cursor.Next()
	}
	return first
}

// clone copies only the current cycle while allocating fresh identities for the new nodes.
func (world *World) clone(first *ring.Ring) *ring.Ring {
	if first == nil {
		return nil
	}
	count := first.Len()
	copy := ring.New(count)
	cursor, output := first, copy
	for index := 0; index < count; index++ {
		output.Value = Item{uint64(len(world.Handles)), cursor.Value.(Item).Value}
		world.Handles = append(world.Handles, output)
		cursor, output = cursor.Next(), output.Next()
	}
	return copy
}

// snapshot observes each cursor and both complete traversal orders from every retained node.
func (world *World) snapshot() uint64 {
	hash := uint64(1)
	for _, cursor := range world.Cursors {
		hash = mix(hash, identity(cursor))
		hash = mix(hash, uint64(cursor.Len()))
	}
	hash = mix(hash, uint64(len(world.Handles)))
	for _, start := range world.Handles {
		item := start.Value.(Item)
		hash = mix(mix(hash, item.ID), item.Value)
		hash = mix(mix(hash, identity(start.Next())), identity(start.Prev()))
		count := start.Len()
		hash = mix(hash, uint64(count))
		cursor := start
		for index := 0; index < count; index++ {
			item = cursor.Value.(Item)
			hash = mix(mix(hash, item.ID), item.Value)
			if cursor == world.Handles[item.ID] {
				hash = mix(hash, 1)
			} else {
				hash = mix(hash, 0)
			}
			cursor = cursor.Next()
		}
		hash = mix(hash, 991)
		cursor = start
		for index := 0; index < count; index++ {
			item = cursor.Value.(Item)
			hash = mix(mix(hash, item.ID), item.Value)
			cursor = cursor.Prev()
		}
		hash = mix(hash, 997)
	}
	return hash
}

// perform applies one checked fixture action without losing preexisting cursor aliases.
func (world *World) perform(operation *Operation) {
	first, other := world.Cursors[operation.Source], world.Cursors[operation.Other]
	var result *ring.Ring
	switch operation.Kind {
	case 0:
		if first != nil {
			result = first.Next()
		}
	case 1:
		if first != nil {
			result = first.Prev()
		}
	case 2:
		if first != nil {
			result = first.Move(operation.Distance)
		}
	case 3:
		if first != nil {
			result = first.Link(other)
		}
	case 4:
		if first != nil {
			result = first.Unlink(operation.Distance)
		}
	case 5:
		if first != nil {
			item := first.Value.(Item)
			first.Value = Item{item.ID, operation.Value}
		}
		result = first
	case 6:
		operation.Visit = 1
		first.Do(func(value any) {
			item := value.(Item)
			operation.Visit = mix(mix(operation.Visit, item.ID), item.Value)
		})
		result = first
	case 7:
		result = world.initialize(operation.Count, 17, operation.Value)
	case 8:
		if first != nil && len(world.Handles)+first.Len() <= 1024 {
			result = world.clone(first)
		} else {
			operation.Kind = 9
			result = first
		}
	}
	world.Cursors[operation.Result] = result
	operation.Returned = identity(result)
}

// next advances a deterministic input stream without runtime randomness.
func next(seed *uint64) uint64 { *seed = *seed*6364136223846793005 + 1442695040888963407; return *seed }

// generate retains aliases across joins, splits, signed moves, independent copies and value replacement.
func generate(initial int) Case {
	world := World{}
	world.Cursors[0] = world.initialize(initial, 17, 23)
	if initial > 0 {
		world.Cursors[1] = world.Cursors[0].Move(initial / 2)
		world.Cursors[2] = world.Cursors[0].Prev()
	}
	world.Cursors[3] = world.initialize(3, 31, 7)
	world.Cursors[4] = world.Cursors[3].Next()
	world.Cursors[5] = world.initialize(1, 1, 901)
	world.Cursors[7] = world.Cursors[3]
	result := Case{Initial: initial}
	seed := uint64(initial) + 49733
	for index := 0; index < 96; index++ {
		operation := Operation{
			Kind:   index % 9,
			Source: int((next(&seed) >> 32) % 8), Other: int((next(&seed) >> 32) % 8),
			Result: int((next(&seed) >> 32) % 8), Distance: int((next(&seed)>>32)%1025) - 512,
			Count: int((next(&seed) >> 32) % 9), Value: (next(&seed) >> 32) % 1000,
		}
		// Force overlapping link operands and exact modular unlink boundaries in each trace.
		if operation.Kind == 3 {
			switch index / 9 {
			case 0:
				operation.Other = operation.Source
			case 1:
				operation.Source, operation.Other = 3, 5
			case 2:
				operation.Source, operation.Other = 3, 4
			case 3:
				operation.Source, operation.Other = 3, 6
			}
		}
		if operation.Kind == 4 {
			switch (index / 9) % 5 {
			case 0:
				operation.Distance = 0
			case 1:
				operation.Distance = -1
			case 2:
				operation.Distance = world.Cursors[operation.Source].Len() * 2
			case 3:
				operation.Distance = world.Cursors[operation.Source].Len()*2 + 1
			}
		}
		world.perform(&operation)
		operation.State = world.snapshot()
		result.Operations = append(result.Operations, operation)
	}
	return result
}

// main writes only the reproducible independent fixture payload.
func main() {
	cases := []Case{}
	for _, size := range []int{0, 1, 2, 3, 8, 17, 64, 257} {
		cases = append(cases, generate(size))
	}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(cases); err != nil {
		panic(err)
	}
}
