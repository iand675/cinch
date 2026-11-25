// Command cmplx_vectors writes complex128 math vectors from Go's math/cmplx package
// and Go's runtime complex128 division.
//
// Each function is evaluated on a full special-value grid (signed zeros, unit values,
// infinities, NaN), branch-cut and pole neighborhoods (including Payne-Hanek range
// reduction above 2^30), rescaling thresholds, Go's cmath_test.go sample vector, and
// seeded random inputs across normal, large, and tiny scales.
//
// Run (built for 386 to use Go's portable IEEE-754 math kernels without amd64 FMA,
// matching tooling/float_vectors.go and Data.Float):
//
//	GOCACHE=/tmp/go-cache GOTELEMETRY=off GOARCH=386 go run tooling/cmplx_vectors.go > seed/tests/fixtures/cmplx-vectors.json
//
// Every list is hexadecimal little-endian 64-bit words:
//   - unary Complex -> Complex: (in_re, in_im, out_re, out_im) [4 words]
//   - unary Complex -> F64:     (in_re, in_im, out)            [3 words]
//   - polar Complex -> (r, th): (in_re, in_im, r, th)          [4 words]
//   - rect (r, th) -> Complex:  (r, th, out_re, out_im)        [4 words]
//   - pred Complex -> Bool:     (in_re, in_im, 0|1)            [3 words]
//   - binary (Complex,Complex)->Complex: (a_re, a_im, b_re, b_im, out_re, out_im) [6 words]
package main

import (
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"math"
	"math/cmplx"
	"math/rand/v2"
	"os"
)

var rng = rand.New(rand.NewPCG(42, 99))

func f(x float64) uint64 { return math.Float64bits(x) }

func b2u(b bool) uint64 {
	if b {
		return 1
	}
	return 0
}

func appendWords(dst []byte, ws ...uint64) []byte {
	for _, w := range ws {
		dst = binary.LittleEndian.AppendUint64(dst, w)
	}
	return dst
}

func main() {
	posZero := 0.0
	negZero := math.Copysign(0, -1)
	posInf := math.Inf(1)
	negInf := math.Inf(-1)
	nan := math.NaN()

	specialScalars := []float64{
		posZero, negZero, 1, -1, 0.5, -0.5, 2, -2, posInf, negInf, nan,
	}

	var inputs []complex128
	// Full 11x11 special grid (121 values).
	for _, r := range specialScalars {
		for _, i := range specialScalars {
			inputs = append(inputs, complex(r, i))
		}
	}

	// Go cmath_test.go reference vc vector.
	vc := []complex128{
		(4.9790119248836735e+00 + 7.7388724745781045e+00i),
		(7.7388724745781045e+00 - 2.7688005719200159e-01i),
		(-2.7688005719200159e-01 - 5.0106036182710749e+00i),
		(-5.0106036182710749e+00 + 9.6362937071984173e+00i),
		(9.6362937071984173e+00 + 2.9263772392439646e+00i),
		(2.9263772392439646e+00 + 5.2290834314593066e+00i),
		(5.2290834314593066e+00 + 2.7279399104360102e+00i),
		(2.7279399104360102e+00 + 1.8253080916808550e+00i),
		(1.8253080916808550e+00 - 8.6859247685756013e+00i),
		(-8.6859247685756013e+00 + 4.9790119248836735e+00i),
	}
	inputs = append(inputs, vc...)

	// Branch cuts, poles, rescaling thresholds, huge/tiny values, Payne-Hanek triggers.
	targeted := []complex128{
		// Tan / Cot pole neighborhoods (small & large, exercising tanSeries + Cody-Waite + Payne-Hanek).
		complex(math.Pi/2, 0),
		complex(-math.Pi/2, 0),
		complex(math.Pi/2, 1e-8),
		complex(math.Pi/2, -1e-8),
		complex(math.Pi/2+1e-12, 1e-12),
		complex(3*math.Pi/2, 1e-9),
		complex(math.Pi, 0),
		complex(math.Pi, 1e-9),
		complex(math.Pi+1e-11, -1e-11),
		complex(0, math.Pi/2),
		complex(0, -math.Pi/2),
		complex(1e-12, math.Pi/2),
		// Payne-Hanek triggers (|2*re| >= 2^30 and |cos(2*re)+cosh(2*im)| or |cosh(2*im)-cos(2*re)| < 0.25).
		complex(math.Ldexp(1, 30), 0),
		complex(math.Ldexp(1, 30), 1e-6),
		complex(math.Floor(math.Ldexp(1, 30)/math.Pi)*math.Pi, 1e-8),
		complex(math.Floor(math.Ldexp(1, 30)/math.Pi)*math.Pi+math.Pi/2, 1e-8),
		complex(math.Floor(math.Ldexp(1, 40)/math.Pi)*math.Pi, 1e-9),
		complex(math.Floor(math.Ldexp(1, 40)/math.Pi)*math.Pi+math.Pi/2, 1e-9),
		complex(math.Floor(math.Ldexp(1, 52)/math.Pi)*math.Pi+math.Pi/2, 1e-10),
		complex(math.Ldexp(1, 100), 1e-8),
		complex(math.Ldexp(1, 200), 1e-8),
		complex(math.Ldexp(1, 500), 1e-8),
		complex(math.Ldexp(1, 900), 1e-8),
		complex(1e300, 1e-8),
		complex(-1e300, 1e-8),
		// Exact bitshift==0 cases in Payne-Hanek: exp+64 = rawExp - 1011 = 64*k => rawExp = 1011+64*k.
		// k=1 => rawExp=1075 => 2^52; k=2 => rawExp=1139 => 2^116; k=10 => rawExp=1651 => 2^628.
		complex(math.Ldexp(1, 51), 0), // 2*re = 2^52
		complex(math.Ldexp(1, 115), 0), // 2*re = 2^116
		complex(math.Ldexp(1, 627), 0), // 2*re = 2^628
		// Sqrt rescaling threshold |a|>4 || |b|>4 and extreme scales.
		complex(3.99, 3.99),
		complex(4.0, 4.0),
		complex(4.01, 4.01),
		complex(-4.01, -4.01),
		complex(1e308, 1e308),
		complex(-1e308, 1e308),
		complex(1e308, -1e308),
		complex(-1e308, -1e308),
		complex(math.MaxFloat64, math.MaxFloat64),
		complex(math.MaxFloat64, 1),
		complex(1, math.MaxFloat64),
		complex(1e-308, 1e-308),
		complex(-1e-308, 1e-308),
		complex(1e-308, -1e-308),
		complex(math.SmallestNonzeroFloat64, math.SmallestNonzeroFloat64),
		complex(-math.SmallestNonzeroFloat64, math.SmallestNonzeroFloat64),
		complex(math.SmallestNonzeroFloat64, -math.SmallestNonzeroFloat64),
		complex(1e-200, -1e-200),
		// Atan unit-circle (1 - x^2 - y^2 == 0) and poles (0, ±1).
		complex(0.6, 0.8),
		complex(-0.6, 0.8),
		complex(0.6, -0.8),
		complex(-0.6, -0.8),
		complex(1e-15, 1.0),
		complex(1e-15, -1.0),
		complex(-1e-15, 1.0),
		complex(-1e-15, -1.0),
		// Sinh/Cosh threshold |x| <= 0.5 and overflow boundaries.
		complex(0.5, 0.5),
		complex(0.500001, 0.500001),
		complex(-0.5, 0.5),
		complex(709.0, 1.0),
		complex(710.0, 1.0),
		complex(-710.0, 1.0),
		complex(1.0, 709.0),
		complex(1.0, 710.0),
		complex(1.0, -710.0),
		// Real axis beyond [-1, 1] with ±0 imaginary (branch cuts of asin, acos, acosh, atanh, log, sqrt).
		complex(1.5, posZero),
		complex(1.5, negZero),
		complex(-1.5, posZero),
		complex(-1.5, negZero),
		complex(0.7, posZero),
		complex(0.7, negZero),
		complex(-0.7, posZero),
		complex(-0.7, negZero),
		complex(posZero, 1.5),
		complex(negZero, 1.5),
		complex(posZero, -1.5),
		complex(negZero, -1.5),
	}
	inputs = append(inputs, targeted...)

	// Seeded random normal complex numbers in [-10, 10]^2.
	for i := 0; i < 40; i++ {
		r := -10 + 20*rng.Float64()
		im := -10 + 20*rng.Float64()
		inputs = append(inputs, complex(r, im))
	}
	// Seeded random multi-scale complex numbers across wide binades.
	for i := 0; i < 30; i++ {
		er := -300 + rng.IntN(601)
		ei := -300 + rng.IntN(601)
		r := math.Ldexp(1+rng.Float64(), er)
		im := math.Ldexp(1+rng.Float64(), ei)
		if rng.IntN(2) == 0 {
			r = -r
		}
		if rng.IntN(2) == 0 {
			im = -im
		}
		inputs = append(inputs, complex(r, im))
	}

	out := map[string]string{}

	encodeUnaryC := func(name string, fn func(complex128) complex128) {
		var buf []byte
		for _, z := range inputs {
			w := fn(z)
			buf = appendWords(buf, f(real(z)), f(imag(z)), f(real(w)), f(imag(w)))
		}
		out[name] = hex.EncodeToString(buf)
	}

	encodeUnaryF := func(name string, fn func(complex128) float64) {
		var buf []byte
		for _, z := range inputs {
			r := fn(z)
			buf = appendWords(buf, f(real(z)), f(imag(z)), f(r))
		}
		out[name] = hex.EncodeToString(buf)
	}

	encodeUnaryBool := func(name string, fn func(complex128) bool) {
		var buf []byte
		for _, z := range inputs {
			buf = appendWords(buf, f(real(z)), f(imag(z)), b2u(fn(z)))
		}
		out[name] = hex.EncodeToString(buf)
	}

	encodeUnaryC("conj", cmplx.Conj)
	encodeUnaryC("exp", cmplx.Exp)
	encodeUnaryC("log", cmplx.Log)
	encodeUnaryC("log10", cmplx.Log10)
	encodeUnaryC("sqrt", cmplx.Sqrt)
	encodeUnaryC("sin", cmplx.Sin)
	encodeUnaryC("cos", cmplx.Cos)
	encodeUnaryC("tan", cmplx.Tan)
	encodeUnaryC("cot", cmplx.Cot)
	encodeUnaryC("sinh", cmplx.Sinh)
	encodeUnaryC("cosh", cmplx.Cosh)
	encodeUnaryC("tanh", cmplx.Tanh)
	encodeUnaryC("asin", cmplx.Asin)
	encodeUnaryC("acos", cmplx.Acos)
	encodeUnaryC("atan", cmplx.Atan)
	encodeUnaryC("asinh", cmplx.Asinh)
	encodeUnaryC("acosh", cmplx.Acosh)
	encodeUnaryC("atanh", cmplx.Atanh)

	encodeUnaryF("abs", cmplx.Abs)
	encodeUnaryF("phase", cmplx.Phase)

	encodeUnaryBool("is_nan", cmplx.IsNaN)
	encodeUnaryBool("is_inf", cmplx.IsInf)

	// Polar.
	{
		var buf []byte
		for _, z := range inputs {
			r, th := cmplx.Polar(z)
			buf = appendWords(buf, f(real(z)), f(imag(z)), f(r), f(th))
		}
		out["polar"] = hex.EncodeToString(buf)
	}

	// Rect.
	{
		var buf []byte
		for _, z := range inputs {
			r, th := real(z), imag(z)
			w := cmplx.Rect(r, th)
			buf = appendWords(buf, f(r), f(th), f(real(w)), f(imag(w)))
		}
		out["rect"] = hex.EncodeToString(buf)
	}

	// Constants.
	{
		z := cmplx.Inf()
		out["complex_inf"] = hex.EncodeToString(appendWords(nil, f(real(z)), f(imag(z))))
		n := cmplx.NaN()
		out["complex_nan"] = hex.EncodeToString(appendWords(nil, f(real(n)), f(imag(n))))
	}

	// Binary pairs for Pow and Div.
	binInputs := []complex128{
		complex(posZero, posZero),
		complex(posZero, negZero),
		complex(negZero, posZero),
		complex(negZero, negZero),
		complex(1, 0),
		complex(-1, 0),
		complex(0, 1),
		complex(0, -1),
		complex(2, 3),
		complex(-2, 3),
		complex(2, -3),
		complex(-2, -3),
		complex(0.5, -0.25),
		complex(1e200, 1e200),
		complex(1e-200, 1e-200),
		complex(1e308, -1e308),
		complex(posInf, 0),
		complex(negInf, 0),
		complex(0, posInf),
		complex(posInf, posInf),
		complex(negInf, posInf),
		complex(nan, 0),
		complex(0, nan),
		complex(nan, nan),
		complex(posInf, nan),
		complex(nan, posInf),
	}

	// Pow: all safe pairs + vc pairs + random pairs.
	{
		var buf []byte
		for _, x := range binInputs {
			for _, y := range binInputs {
				// Go's cmplx.Pow(0, complex(NaN, ±Inf)) panics ("not reached") due to IsNaN(complex(NaN,Inf))==false.
				if x == 0 && math.IsNaN(real(y)) && math.IsInf(imag(y), 0) {
					continue
				}
				w := cmplx.Pow(x, y)
				buf = appendWords(buf, f(real(x)), f(imag(x)), f(real(y)), f(imag(y)), f(real(w)), f(imag(w)))
			}
		}
		for i := range vc {
			x := vc[i]
			y := vc[(i+1)%len(vc)]
			w := cmplx.Pow(x, y)
			buf = appendWords(buf, f(real(x)), f(imag(x)), f(real(y)), f(imag(y)), f(real(w)), f(imag(w)))
		}
		out["pow"] = hex.EncodeToString(buf)
	}

	// Div: all pairs (including overflow-safe 1e308, infinities, zeros, NaNs) + vc pairs.
	{
		var buf []byte
		for _, x := range binInputs {
			for _, y := range binInputs {
				w := x / y
				buf = appendWords(buf, f(real(x)), f(imag(x)), f(real(y)), f(imag(y)), f(real(w)), f(imag(w)))
			}
		}
		for i := range vc {
			x := vc[i]
			y := vc[(i+1)%len(vc)]
			w := x / y
			buf = appendWords(buf, f(real(x)), f(imag(x)), f(real(y)), f(imag(y)), f(real(w)), f(imag(w)))
		}
		out["div"] = hex.EncodeToString(buf)
	}

	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", "  ")
	if err := enc.Encode(out); err != nil {
		panic(err)
	}
}
