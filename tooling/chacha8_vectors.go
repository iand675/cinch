// Command chacha8_vectors writes ChaCha8 generator vectors from the reference SDK's implementation.
//
// Each row seeds a generator and runs a script of operations (draw n 64-bit values, read n bytes, encode the
// state, restore a fresh generator from the latest encoding), recording each operation's output in hex.
// Scripts cross block, reseed-period and partial-read boundaries. Malformed encodings record whether they are
// refused.
//
//	go run tooling/chacha8_vectors.go > seed/tests/fixtures/chacha8-vectors.json
package main

import (
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"math/rand/v2"
	"os"
	"strconv"
	"strings"
)

type row struct {
	Seed    string   `json:"seed"`
	Ops     []string `json:"ops"`
	Outputs []string `json:"outputs"`
}

func runScript(seed [32]byte, ops []string) []string {
	g := rand.NewChaCha8(seed)
	var last []byte
	var outs []string
	for _, op := range ops {
		parts := strings.Fields(op)
		switch parts[0] {
		case "u64":
			n, _ := strconv.Atoi(parts[1])
			var b []byte
			for i := 0; i < n; i++ {
				b = binary.BigEndian.AppendUint64(b, g.Uint64())
			}
			outs = append(outs, hex.EncodeToString(b))
		case "read":
			n, _ := strconv.Atoi(parts[1])
			b := make([]byte, n)
			g.Read(b)
			outs = append(outs, hex.EncodeToString(b))
		case "marshal":
			b, _ := g.MarshalBinary()
			last = b
			outs = append(outs, hex.EncodeToString(b))
		case "restore":
			g = new(rand.ChaCha8)
			if err := g.UnmarshalBinary(last); err != nil {
				panic(err)
			}
			outs = append(outs, "")
		}
	}
	return outs
}

func main() {
	scripts := [][]string{
		{"u64 40"},
		{"u64 200", "marshal", "u64 5", "restore", "u64 5"},
		{"read 3", "read 5", "read 1", "read 17", "marshal", "read 4", "restore", "read 4", "u64 2"},
		{"u64 123", "marshal", "u64 3", "restore", "u64 3", "u64 130", "marshal"},
		{"u64 124", "marshal", "u64 1", "restore", "u64 1"},
		{"read 0", "read 7", "marshal", "restore", "read 1", "read 9", "u64 1000", "marshal"},
	}
	var rows []row
	for i, script := range scripts {
		var seed [32]byte
		for j := range seed {
			seed[j] = byte(i*37 + j*11 + 5)
		}
		rows = append(rows, row{Seed: hex.EncodeToString(seed[:]), Ops: script, Outputs: runScript(seed, script)})
	}
	bad := [][]byte{nil, []byte("chacha8:"), append([]byte("chacha9:"), make([]byte, 40)...)}
	over := append([]byte("chacha8:"), make([]byte, 40)...)
	binary.BigEndian.PutUint64(over[8:], 125)
	bad = append(bad, over, append([]byte("readbuf:\x05ab"), over...))
	var refused []string
	for _, b := range bad {
		var g rand.ChaCha8
		err := g.UnmarshalBinary(b)
		msg := ""
		if err != nil {
			msg = err.Error()
		}
		refused = append(refused, hex.EncodeToString(b)+" "+msg)
	}
	enc := json.NewEncoder(os.Stdout)
	if err := enc.Encode(map[string]any{"scripts": rows, "refused": refused}); err != nil {
		panic(err)
	}
}
