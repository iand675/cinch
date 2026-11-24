// Command rand_vectors writes pseudorandom-distribution vectors from the reference SDK's generators.
//
// Each script seeds a PCG or ChaCha8 source, wraps it in the reference Rand and runs operations (batches of
// 64-, 32-bit and non-negative draws, bounded draws for power-of-two and other bounds near the rejection
// threshold, floats, permutations, shuffles, normal, exponential and Zipf variates), recording each batch's
// results as hexadecimal: 8 bytes big-endian per 64-bit or float64 result, 4 per 32-bit or float32 result.
// PCG scripts also record state encodings and continue from restored generators; malformed encodings record
// the reference's error. Built for 386 so that the samplers' exp and log are the portable algorithms:
//
//	GOARCH=386 go run tooling/rand_vectors.go > seed/tests/fixtures/rand-vectors.json
package main

import (
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"math"
	"math/rand/v2"
	"os"
)

type op struct {
	Name string   `json:"name"`
	Args []uint64 `json:"args"`
}

type script struct {
	Kind    string   `json:"kind"`
	Seed    []uint64 `json:"seed"`
	ChaCha  string   `json:"chacha"`
	Ops     []op     `json:"ops"`
	Outputs []string `json:"outputs"`
}

func be64(b []byte, v uint64) []byte { return binary.BigEndian.AppendUint64(b, v) }
func be32(b []byte, v uint32) []byte { return binary.BigEndian.AppendUint32(b, v) }

func run(s *script, src rand.Source, pcg *rand.PCG) {
	r := rand.New(src)
	var last []byte
	for _, o := range s.Ops {
		var b []byte
		n := 0
		if len(o.Args) > 0 {
			n = int(o.Args[len(o.Args)-1])
		}
		switch o.Name {
		case "u64":
			for i := 0; i < n; i++ {
				b = be64(b, r.Uint64())
			}
		case "i64":
			for i := 0; i < n; i++ {
				b = be64(b, uint64(r.Int64()))
			}
		case "u32":
			for i := 0; i < n; i++ {
				b = be32(b, r.Uint32())
			}
		case "i32":
			for i := 0; i < n; i++ {
				b = be32(b, uint32(r.Int32()))
			}
		case "u64n":
			for i := 0; i < n; i++ {
				b = be64(b, r.Uint64N(o.Args[0]))
			}
		case "i64n":
			for i := 0; i < n; i++ {
				b = be64(b, uint64(r.Int64N(int64(o.Args[0]))))
			}
		case "u32n":
			for i := 0; i < n; i++ {
				b = be32(b, r.Uint32N(uint32(o.Args[0])))
			}
		case "i32n":
			for i := 0; i < n; i++ {
				b = be32(b, uint32(r.Int32N(int32(o.Args[0]))))
			}
		case "f64":
			for i := 0; i < n; i++ {
				b = be64(b, math.Float64bits(r.Float64()))
			}
		case "f32":
			for i := 0; i < n; i++ {
				b = be32(b, math.Float32bits(r.Float32()))
			}
		case "perm":
			for _, v := range r.Perm(n) {
				b = be64(b, uint64(v))
			}
		case "shuffle":
			xs := make([]int, n)
			for i := range xs {
				xs[i] = 1000 + i
			}
			r.Shuffle(n, func(i, j int) { xs[i], xs[j] = xs[j], xs[i] })
			for _, v := range xs {
				b = be64(b, uint64(v))
			}
		case "norm":
			for i := 0; i < n; i++ {
				b = be64(b, math.Float64bits(r.NormFloat64()))
			}
		case "exp":
			for i := 0; i < n; i++ {
				b = be64(b, math.Float64bits(r.ExpFloat64()))
			}
		case "zipf":
			z := rand.NewZipf(r, math.Float64frombits(o.Args[0]), math.Float64frombits(o.Args[1]), o.Args[2])
			for i := 0; i < n; i++ {
				b = be64(b, z.Uint64())
			}
		case "marshal":
			last, _ = pcg.MarshalBinary()
			b = last
		case "restore":
			pcg = new(rand.PCG)
			if err := pcg.UnmarshalBinary(last); err != nil {
				panic(err)
			}
			r = rand.New(pcg)
		}
		s.Outputs = append(s.Outputs, hex.EncodeToString(b))
	}
}

func o(name string, args ...uint64) op { return op{name, args} }

func f(x float64) uint64 { return math.Float64bits(x) }

func main() {
	common := []op{
		o("u64", 20), o("i64", 10), o("u32", 10), o("i32", 10),
		o("u64n", 1<<40, 6), o("u64n", 6, 30), o("u64n", 1<<63+1, 12), o("u64n", 1, 3), o("u64n", 0xFFFFFFFFFFFFFFFF, 6),
		o("i64n", 1000000007, 10), o("i64n", 1<<62+3, 10), o("u32n", 7, 20), o("u32n", 0x80000001, 10), o("i32n", 100, 20), o("i32n", 0x7FFFFFFF, 6),
		o("f64", 20), o("f32", 20), o("perm", 0), o("perm", 1), o("perm", 17), o("shuffle", 30),
		o("norm", 400), o("exp", 400),
		o("zipf", f(1.5), f(1), 100, 40), o("zipf", f(2.7), f(3.5), 1<<40, 40), o("zipf", f(1.0001), f(1), 10, 20),
	}
	var scripts []script
	for i, seed := range [][2]uint64{{1, 2}, {0, 0}, {0xDEADBEEFCAFEF00D, 0x0123456789ABCDEF}} {
		ops := append([]op{}, common...)
		if i == 0 {
			ops = append(ops, o("marshal"), o("u64", 3), o("restore"), o("u64", 3), o("norm", 50))
		}
		s := script{Kind: "pcg", Seed: seed[:], Ops: ops}
		p := rand.NewPCG(seed[0], seed[1])
		run(&s, p, p)
		scripts = append(scripts, s)
	}
	var seed [32]byte
	for i := range seed {
		seed[i] = byte(i*7 + 3)
	}
	cs := script{Kind: "chacha8", ChaCha: hex.EncodeToString(seed[:]), Ops: common}
	run(&cs, rand.NewChaCha8(seed), nil)
	scripts = append(scripts, cs)

	var refused []string
	for _, b := range [][]byte{nil, []byte("pcg:"), []byte("pcx:0123456789abcdef"), []byte("pcg:0123456789abcdef!")} {
		var p rand.PCG
		msg := ""
		if err := p.UnmarshalBinary(b); err != nil {
			msg = err.Error()
		}
		refused = append(refused, hex.EncodeToString(b)+" "+msg)
	}
	zipfNil := []bool{rand.NewZipf(nil, 1, 1, 10) == nil, rand.NewZipf(nil, 2, 0.5, 10) == nil, rand.NewZipf(nil, 2, 1, 10) == nil}
	enc := json.NewEncoder(os.Stdout)
	if err := enc.Encode(map[string]any{"scripts": scripts, "refused": refused, "zipf_refused": zipfNil}); err != nil {
		panic(err)
	}
}
