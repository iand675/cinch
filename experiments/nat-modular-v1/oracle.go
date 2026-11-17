// Reference-only arithmetic observations; this program is not part of the library.
package main

import (
	"encoding/json"
	"fmt"
	"math/big"
	"os"
)

// Case carries exact decimal values without JSON floating-point conversion.
type Case struct {
	First, Second, Base, Exponent, Modulus string
}

// Observation distinguishes direct arithmetic results from the candidate's checked zero-modulus policy.
type Observation struct {
	Input Case `json:"input"`
	GCD, FirstCoefficient, SecondCoefficient string
	ProductResidue, PowerResidue, InverseResidue *string
	InverseGCD string
	PolicyRefusal string `json:",omitempty"`
}

// integer parses an exact supplied input and fails instead of changing malformed reference data.
func integer(text string) *big.Int {
	value, ok := new(big.Int).SetString(text, 10)
	if !ok { panic("invalid exact integer input") }
	return value
}

// optional copies a result string while preserving a genuine absent inverse or power.
func optional(value *big.Int) *string {
	if value == nil { return nil }
	text := value.String()
	return &text
}

// observe invokes the actual integer implementation for every nonzero-modulus arithmetic result.
func observe(input Case) Observation {
	x, y, base, exponent, modulus := integer(input.First), integer(input.Second), integer(input.Base), integer(input.Exponent), integer(input.Modulus)
	if x.Sign() < 0 || y.Sign() < 0 || modulus.Sign() < 0 { panic("input outside the planned API domain") }
	left, right := new(big.Int), new(big.Int)
	gcd := new(big.Int).GCD(left, right, x, y)
	out := Observation{Input:input, GCD:gcd.String(), FirstCoefficient:left.String(), SecondCoefficient:right.String()}
	if modulus.Sign() == 0 {
		// This is a candidate API policy, not a claimed reference arithmetic refusal.
		out.PolicyRefusal = "zero-modulus; modular SDK calls intentionally not invoked"
		return out
	}
	out.ProductResidue = optional(new(big.Int).Mod(new(big.Int).Mul(x, y), modulus))
	out.PowerResidue = optional(new(big.Int).Exp(base, exponent, modulus))
	out.InverseResidue = optional(new(big.Int).ModInverse(base, modulus))
	out.InverseGCD = new(big.Int).GCD(nil, nil, base, modulus).String()
	return out
}

// main reads prepared cases and emits observations; no oracle execution is claimed by storing this source.
func main() {
	var inputs []Case
	if err := json.NewDecoder(os.Stdin).Decode(&inputs); err != nil { panic(err) }
	outputs := make([]Observation, 0, len(inputs))
	for _, input := range inputs { outputs = append(outputs, observe(input)) }
	if err := json.NewEncoder(os.Stdout).Encode(outputs); err != nil { panic(fmt.Sprint(err)) }
}
