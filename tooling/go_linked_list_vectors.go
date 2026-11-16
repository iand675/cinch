// Generate independent mutable linked-list value, identity and membership observations.
package main

import (
	"container/list"
	"encoding/json"
	"os"
)

// Item separates a fixture node's stable identity from its replaceable value.
type Item struct{ ID, Value uint64 }

// Operation records one public action and its complete post-action observation.
type Operation struct {
	Kind     int    `json:"kind"`
	List     int    `json:"list"`
	Handle   int    `json:"handle"`
	Marker   int    `json:"marker"`
	Value    uint64 `json:"value"`
	Code     uint64 `json:"code"`
	Identity uint64 `json:"identity"`
	Returned uint64 `json:"returned"`
	State    uint64 `json:"state"`
}

// Case supplies one initial size and an exact ordered mutation trace.
type Case struct {
	Initial    int         `json:"initial"`
	Operations []Operation `json:"operations"`
}

// World retains detached handles while tracking the explicit checked membership contract.
type World struct {
	Lists   [2]*list.List
	Handles []*list.Element
	Owner   []int
	Active  []bool
}

// mix preserves the order of every independent observation using wrapping words.
func mix(hash, value uint64) uint64 { return hash*1099511628211 + value }

// identify assigns a fresh fixture identity without changing the logical stored value.
func (world *World) identify(owner int, element *list.Element, value uint64) uint64 {
	identity := uint64(len(world.Handles))
	element.Value = Item{identity, value}
	world.Handles = append(world.Handles, element)
	world.Owner = append(world.Owner, owner)
	world.Active = append(world.Active, true)
	return identity
}

// refusal distinguishes foreign ownership before removed membership.
func (world *World) refusal(owner, handle int) uint64 {
	if world.Owner[handle] != owner {
		return 3
	}
	if !world.Active[handle] {
		return 4
	}
	return 0
}

// navigation observes a valid neighbor, an end, or the exact checked refusal.
func (world *World) navigation(hash uint64, owner, handle int, forward bool) uint64 {
	failure := world.refusal(owner, handle)
	if failure != 0 {
		return mix(hash, failure)
	}
	element := world.Handles[handle]
	var neighbor *list.Element
	if forward {
		neighbor = element.Next()
	} else {
		neighbor = element.Prev()
	}
	if neighbor == nil {
		return mix(hash, 0)
	}
	value := neighbor.Value.(Item)
	hash = mix(mix(hash, 1), value.ID+1)
	if neighbor == world.Handles[value.ID] {
		return mix(hash, 1)
	}
	return mix(hash, 0)
}

// snapshot folds both complete traversals, all detached values and every checked handle relation.
func (world *World) snapshot() uint64 {
	hash := uint64(1)
	for owner, values := range world.Lists {
		hash = mix(hash, uint64(values.Len()))
		for cursor := values.Front(); cursor != nil; cursor = cursor.Next() {
			item := cursor.Value.(Item)
			hash = mix(mix(hash, item.ID), item.Value)
			if cursor == world.Handles[item.ID] {
				hash = mix(hash, 1)
			} else {
				hash = mix(hash, 0)
			}
		}
		hash = mix(hash, 991)
		for cursor := values.Back(); cursor != nil; cursor = cursor.Prev() {
			item := cursor.Value.(Item)
			hash = mix(mix(hash, item.ID), item.Value)
		}
		hash = mix(hash, uint64(owner)+997)
	}
	hash = mix(hash, uint64(len(world.Handles)))
	for handle, element := range world.Handles {
		item := element.Value.(Item)
		hash = mix(mix(hash, item.ID), item.Value)
		for owner := range world.Lists {
			if world.refusal(owner, handle) == 0 {
				hash = mix(hash, 1)
			} else {
				hash = mix(hash, 0)
			}
			hash = world.navigation(hash, owner, handle, true)
			hash = world.navigation(hash, owner, handle, false)
		}
	}
	return hash
}

// perform applies the independent sequence implementation after the checked contract's preconditions.
func (world *World) perform(operation *Operation) {
	values := world.Lists[operation.List]
	handle := world.Handles[operation.Handle]
	marker := world.Handles[operation.Marker]
	switch operation.Kind {
	case 0, 1:
		var element *list.Element
		if operation.Kind == 0 {
			element = values.PushBack(nil)
		} else {
			element = values.PushFront(nil)
		}
		operation.Code = 1
		operation.Identity = world.identify(operation.List, element, operation.Value)
	case 2, 3:
		operation.Code = world.refusal(operation.List, operation.Marker)
		if operation.Code != 0 {
			return
		}
		var element *list.Element
		if operation.Kind == 2 {
			element = values.InsertBefore(nil, marker)
		} else {
			element = values.InsertAfter(nil, marker)
		}
		operation.Code = 1
		operation.Identity = world.identify(operation.List, element, operation.Value)
	case 4:
		operation.Code = world.refusal(operation.List, operation.Handle)
		if operation.Code != 0 {
			return
		}
		item := values.Remove(handle).(Item)
		world.Active[operation.Handle] = false
		operation.Code, operation.Identity, operation.Returned = 2, item.ID, item.Value
	case 5, 6, 7, 8:
		operation.Code = world.refusal(operation.List, operation.Handle)
		if operation.Code == 0 && (operation.Kind == 7 || operation.Kind == 8) {
			operation.Code = world.refusal(operation.List, operation.Marker)
		}
		if operation.Code != 0 {
			return
		}
		switch operation.Kind {
		case 5:
			values.MoveToFront(handle)
		case 6:
			values.MoveToBack(handle)
		case 7:
			values.MoveBefore(handle, marker)
		case 8:
			values.MoveAfter(handle, marker)
		}
	case 9:
		item := handle.Value.(Item)
		handle.Value = Item{item.ID, operation.Value}
	case 10, 11:
		source := world.Lists[operation.Marker%2]
		original, count := values.Len(), source.Len()
		if original+count > 512 {
			operation.Kind = 13
			return
		}
		if operation.Kind == 10 {
			values.PushBackList(source)
		} else {
			values.PushFrontList(source)
		}
		cursor := values.Front()
		if operation.Kind == 10 {
			for index := 0; index < original; index++ {
				cursor = cursor.Next()
			}
		}
		for index := 0; index < count; index++ {
			item := cursor.Value.(Item)
			world.identify(operation.List, cursor, item.Value)
			cursor = cursor.Next()
		}
	case 12:
		for handle, owner := range world.Owner {
			if owner == operation.List {
				world.Active[handle] = false
			}
		}
		values.Init()
	case 13:
		// The complete state observation also checks both navigation directions.
	}
}

// next advances a deterministic input stream without depending on runtime randomness.
func next(seed *uint64) uint64 { *seed = *seed*6364136223846793005 + 1442695040888963407; return *seed }

// generate constructs a reproducible trace with empty, detached, foreign and aliased sequence cases.
func generate(initial int) Case {
	world := World{Lists: [2]*list.List{list.New(), list.New()}}
	for index := 0; index < initial; index++ {
		world.identify(0, world.Lists[0].PushBack(nil), uint64((index*17+23)%997))
	}
	for index := 0; index < 3; index++ {
		world.identify(1, world.Lists[1].PushBack(nil), uint64((index*31+7)%997))
	}
	result := Case{Initial: initial}
	seed := uint64(initial) + 39421
	for index := 0; index < 96; index++ {
		operation := Operation{
			Kind: int((next(&seed) >> 32) % 14), List: int((next(&seed) >> 32) % 2),
			Handle: int((next(&seed) >> 32) % uint64(len(world.Handles))),
			Marker: int((next(&seed) >> 32) % uint64(len(world.Handles))), Value: (next(&seed) >> 32) % 1000,
		}
		world.perform(&operation)
		operation.State = world.snapshot()
		result.Operations = append(result.Operations, operation)
	}
	return result
}

// main writes only the canonical independent fixture payload to standard output.
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
