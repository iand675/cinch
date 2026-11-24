// Command float_vectors writes binary64 math vectors from the reference SDK's portable implementations.
//
// Each function is evaluated on special values (signed zeros, infinities, NaN, subnormals, the extremes),
// values around its thresholds and branch points, and seeded random inputs spread over every binade, with
// results recorded as exact bit patterns. Built for 386 the package uses its portable algorithms with SSE2
// arithmetic (no fused multiply-add, no extended precision); hypot, whose 386 version uses the x87 unit, is
// reached by linkname at its portable version:
//
//	GOARCH=386 go run -ldflags=-checklinkname=0 tooling/float_vectors.go > seed/tests/fixtures/float-vectors.json
//
// Every list is hexadecimal little-endian 64-bit words: (input, result) for one-argument functions,
// (x, y, result) for two, (x, fraction, exponent) for frexp, (x, integer, fraction) for modf,
// (x, sin, cos) for sincos, (fraction, exponent, result) for ldexp, (x, result) for ilogb and (n, result)
// for pow10.
package main

import (
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"math"
	"math/rand/v2"
	"os"
	_ "unsafe"
)

//go:linkname hypot math.hypot
func hypot(p, q float64) float64

var rng = rand.New(rand.NewPCG(1, 2))

func words(ws ...uint64) []byte {
	b := make([]byte, 0, 8*len(ws))
	for _, w := range ws {
		b = binary.LittleEndian.AppendUint64(b, w)
	}
	return b
}

func f(x float64) uint64 { return math.Float64bits(x) }

var specials = []float64{
	0, math.Copysign(0, -1), 1, -1, 0.5, -0.5, 2, -2, 3, 10, 100, 1e10, 1e-10, 1e300, -1e300, 1e-300,
	math.Inf(1), math.Inf(-1), math.NaN(), math.MaxFloat64, -math.MaxFloat64, math.SmallestNonzeroFloat64,
	-math.SmallestNonzeroFloat64, 2.2250738585072014e-308, 2.225073858507201e-308, math.Pi, math.Pi / 2,
	math.Pi / 4, -math.Pi, math.E, 0.1, 0.7, 0.9999999999999999, 1.0000000000000002, 1 << 29, 1 << 28,
	1 << 53, 1<<53 + 2, 1 << 63, 4.5, -4.5, 2.5, -2.5, 0.49999999999999994, 1.5, 709.78, -745.1, 710, -746,
	21, 22, 0.625, 44, 1e-14, 3e-8, 7.450580596923828e-9, 0.66, 2.414213562373095, 0.4142135623730951,
	-0.2928932188134524, 0.34657359027997264, 1.0397207708399179, 38.816242111356935, 1023.9, -1074.5, 1024, -1075,
}

// randoms returns n values with random sign, binade (from lo to hi exponents) and mantissa.
func randoms(n int, lo, hi int, signed bool) []float64 {
	out := make([]float64, n)
	for i := range out {
		e := lo + rng.IntN(hi-lo+1)
		x := math.Ldexp(1+rng.Float64(), e)
		if signed && rng.IntN(2) == 0 {
			x = -x
		}
		out[i] = x
	}
	return out
}

// uniform returns n values in [a, b).
func uniform(n int, a, b float64) []float64 {
	out := make([]float64, n)
	for i := range out {
		out[i] = a + (b-a)*rng.Float64()
	}
	return out
}

func cat(parts ...[]float64) []float64 {
	var out []float64
	for _, p := range parts {
		out = append(out, p...)
	}
	return out
}

// nearMultiples returns values within a few ulps of k * step for k in [1, n].
func nearMultiples(step float64, n int) []float64 {
	var out []float64
	for k := 1; k <= n; k++ {
		x := float64(k) * step
		out = append(out, x, math.Nextafter(x, 0), math.Nextafter(x, math.Inf(1)))
	}
	return out
}

func main() {
	generic := cat(specials, randoms(220, -1074, 1023, true), uniform(80, -10, 10))
	unit := cat(specials, uniform(150, -1, 1), randoms(60, -60, 0, true))
	positive := cat(specials, randoms(200, -1074, 1023, false), uniform(80, 0, 10))
	trig := cat(specials, uniform(120, -10, 10), randoms(80, -40, 30, true), randoms(60, 29, 1023, true), nearMultiples(math.Pi/4, 24))
	expIn := cat(specials, uniform(160, -750, 712), uniform(80, -2, 2), randoms(60, -60, 3, true))
	unary := map[string]struct {
		fn func(float64) float64
		in []float64
	}{
		"sqrt": {math.Sqrt, positive}, "floor": {math.Floor, generic}, "ceil": {math.Ceil, generic},
		"trunc": {math.Trunc, generic}, "round": {math.Round, cat(generic, uniform(60, -6, 6))},
		"roundeven": {math.RoundToEven, cat(generic, []float64{0.5, 1.5, 2.5, -0.5, -1.5, -2.5, 4503599627370495.5})},
		"cbrt": {math.Cbrt, generic}, "exp": {math.Exp, expIn}, "exp2": {math.Exp2, cat(expIn, uniform(80, -1080, 1030))},
		"expm1": {math.Expm1, expIn}, "log": {math.Log, positive}, "log2": {math.Log2, positive},
		"log10": {math.Log10, positive}, "log1p": {math.Log1p, cat(positive, uniform(80, -1, 1), randoms(40, -60, -1, true))},
		"logb": {math.Logb, generic}, "sin": {math.Sin, trig}, "cos": {math.Cos, trig}, "tan": {math.Tan, trig},
		"asin": {math.Asin, unit}, "acos": {math.Acos, unit}, "atan": {math.Atan, generic},
		"sinh": {math.Sinh, cat(expIn, uniform(60, -25, 25))}, "cosh": {math.Cosh, cat(expIn, uniform(60, -25, 25))},
		"tanh": {math.Tanh, cat(generic, uniform(80, -50, 50))}, "asinh": {math.Asinh, generic},
		"acosh": {math.Acosh, cat(positive, uniform(80, 1, 3))}, "atanh": {math.Atanh, unit},
	}
	out := map[string]string{}
	for name, u := range unary {
		var b []byte
		for _, x := range u.in {
			b = append(b, words(f(x), f(u.fn(x)))...)
		}
		out[name] = hex.EncodeToString(b)
	}
	pairs := func(xs, ys []float64) [][2]float64 {
		var ps [][2]float64
		for _, x := range xs {
			for _, y := range ys {
				ps = append(ps, [2]float64{x, y})
			}
		}
		return ps
	}
	edge := []float64{0, math.Copysign(0, -1), 1, -1, 0.5, -0.5, 2, -2, 3, -3, 0.25, 1.5, math.Inf(1), math.Inf(-1), math.NaN(), 1 << 63, 1e300, 5e-324}
	randPairs := func(n int, xs, ys func() float64) [][2]float64 {
		ps := make([][2]float64, n)
		for i := range ps {
			ps[i] = [2]float64{xs(), ys()}
		}
		return ps
	}
	any := func() float64 { return randoms(1, -1074, 1023, true)[0] }
	mid := func() float64 { return randoms(1, -30, 30, true)[0] }
	binary := map[string]struct {
		fn func(float64, float64) float64
		in [][2]float64
	}{
		"pow": {math.Pow, append(append(pairs(edge, edge), randPairs(200, func() float64 { return uniform(1, 0, 10)[0] }, func() float64 { return uniform(1, -40, 40)[0] })...),
			append(randPairs(150, mid, func() float64 { return float64(rng.IntN(80) - 40) }), randPairs(100, func() float64 { return randoms(1, -1074, 1023, false)[0] }, func() float64 { return uniform(1, -3, 3)[0] })...)...)},
		"atan2":     {math.Atan2, append(pairs(edge, edge), randPairs(250, mid, mid)...)},
		"hypot":     {hypot, append(pairs(edge, edge), randPairs(250, any, any)...)},
		"mod":       {math.Mod, append(pairs(edge, edge), append(randPairs(200, mid, mid), randPairs(80, any, mid)...)...)},
		"remainder": {math.Remainder, append(pairs(edge, edge), append(randPairs(200, mid, mid), randPairs(80, any, any)...)...)},
		"nextafter": {math.Nextafter, append(pairs(edge, edge), randPairs(100, any, any)...)},
		"dim":       {math.Dim, append(pairs(edge, edge), randPairs(60, any, any)...)},
		"max":       {math.Max, append(pairs(edge, edge), randPairs(60, any, any)...)},
		"min":       {math.Min, append(pairs(edge, edge), randPairs(60, any, any)...)},
		"copysign":  {math.Copysign, append(pairs(edge, edge), randPairs(60, any, any)...)},
	}
	for name, bf := range binary {
		var b []byte
		for _, p := range bf.in {
			b = append(b, words(f(p[0]), f(p[1]), f(bf.fn(p[0], p[1])))...)
		}
		out[name] = hex.EncodeToString(b)
	}
	var fr, mf, sc, ld, il, p10 []byte
	for _, x := range generic {
		a, e := math.Frexp(x)
		fr = append(fr, words(f(x), f(a), uint64(int64(e)))...)
		i, r := math.Modf(x)
		mf = append(mf, words(f(x), f(i), f(r))...)
		il = append(il, words(f(x), uint64(int64(math.Ilogb(x))))...)
	}
	for _, x := range trig {
		s, c := math.Sincos(x)
		sc = append(sc, words(f(x), f(s), f(c))...)
	}
	for _, x := range cat(specials, randoms(100, -1074, 1023, true)) {
		for _, e := range []int{0, 1, -1, 52, -52, 1023, -1022, -1074, -1075, 1024, 2000, -2000, rng.IntN(4000) - 2000} {
			ld = append(ld, words(f(x), uint64(int64(e)), f(math.Ldexp(x, e)))...)
		}
	}
	for n := -330; n <= 315; n++ {
		p10 = append(p10, words(uint64(int64(n)), f(math.Pow10(n)))...)
	}
	out["frexp"] = hex.EncodeToString(fr)
	out["modf"] = hex.EncodeToString(mf)
	out["sincos"] = hex.EncodeToString(sc)
	out["ldexp"] = hex.EncodeToString(ld)
	out["ilogb"] = hex.EncodeToString(il)
	out["pow10"] = hex.EncodeToString(p10)
	enc := json.NewEncoder(os.Stdout)
	if err := enc.Encode(out); err != nil {
		panic(err)
	}
}
