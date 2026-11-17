// Emit independent standard-package observations for value NaNs and callback mutation.
package main

import (
	"encoding/json"
	"maps"
	"math"
	"os"
	"runtime"
)

// valueNaNObservation separates IEEE value comparison from explicit bit comparison.
type valueNaNObservation struct {
	OriginalBits       uint64 `json:"original_bits"`
	ReplacementBits    uint64 `json:"replacement_bits"`
	SelfEqual          bool   `json:"self_equal"`
	CloneEqual         bool   `json:"clone_equal"`
	CloneBitEqual      bool   `json:"clone_bit_equal"`
	ReplacementBitSame bool   `json:"replacement_bit_same"`
	OriginalSize       int    `json:"original_size"`
	CloneSize          int    `json:"clone_size"`
}

// replacementObservation records deletion of the current binding after its predicate replaces it.
type replacementObservation struct {
	Callbacks           int  `json:"callbacks"`
	OriginalValuesMatch bool `json:"original_values_match"`
	FinalSize           int  `json:"final_size"`
}

// comparisonObservation records current lookup behavior after a value predicate clears the right map.
type comparisonObservation struct {
	Equal     bool `json:"equal"`
	Callbacks int  `json:"callbacks"`
	LeftSize  int  `json:"left_size"`
	RightSize int  `json:"right_size"`
}

// edgeObservations retains the actual SDK identity and independent capability groups.
type edgeObservations struct {
	Format               int                    `json:"format"`
	SDK                  string                 `json:"sdk"`
	ValueNaN             valueNaNObservation    `json:"value_nan"`
	DeleteCurrentReplace replacementObservation `json:"delete_current_replacement"`
	EqualityCurrentRead  comparisonObservation  `json:"equality_current_lookup"`
}

// valueNaNs observes equality and clone behavior without serializing non-finite JSON numbers.
func valueNaNs() valueNaNObservation {
	const originalBits uint64 = 0x7ff8000000000001
	const replacementBits uint64 = 0x7ff8000000000002
	original := map[int]float64{1: math.Float64frombits(originalBits)}
	copy := maps.Clone(original)
	bitsEqual := func(first, second float64) bool {
		return math.Float64bits(first) == math.Float64bits(second)
	}
	result := valueNaNObservation{
		OriginalBits:    math.Float64bits(original[1]),
		ReplacementBits: replacementBits,
		SelfEqual:       maps.Equal(original, original),
		CloneEqual:      maps.Equal(original, copy),
		CloneBitEqual:   maps.EqualFunc(original, copy, bitsEqual),
		OriginalSize:    len(original),
	}
	copy[1] = math.Float64frombits(replacementBits)
	result.ReplacementBitSame = maps.EqualFunc(original, copy, bitsEqual)
	result.CloneSize = len(copy)
	return result
}

// currentReplacement runs the actual deletion callback before ordinary key deletion.
func currentReplacement() replacementObservation {
	bindings := map[int]int{1: 10, 2: 20, 3: 30}
	result := replacementObservation{OriginalValuesMatch: true}
	maps.DeleteFunc(bindings, func(key, value int) bool {
		result.Callbacks++
		previous, present := bindings[key]
		result.OriginalValuesMatch = result.OriginalValuesMatch && present && previous == value
		delete(bindings, key)
		bindings[key] = value + 1
		return true
	})
	result.FinalSize = len(bindings)
	return result
}

// currentComparison distinguishes a live right-side lookup from a copied right-side value set.
func currentComparison() comparisonObservation {
	first := map[int]int{1: 10, 2: 20}
	second := maps.Clone(first)
	result := comparisonObservation{}
	result.Equal = maps.EqualFunc(first, second, func(left, right int) bool {
		result.Callbacks++
		clear(second)
		return left == right
	})
	result.LeftSize, result.RightSize = len(first), len(second)
	return result
}

// main emits observations from the pinned external SDK without invoking a Cinch implementation.
func main() {
	output := edgeObservations{Format: 1, SDK: runtime.Version(), ValueNaN: valueNaNs(),
		DeleteCurrentReplace: currentReplacement(), EqualityCurrentRead: currentComparison()}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(output); err != nil {
		panic(err)
	}
}
