// Command mlkem_check reads candidate ML-KEM rows from tooling/mlkem_vectors.py on standard input,
// checks each against the reference SDK (the same encapsulation key from the seed, and the same
// shared keys when decapsulating the ciphertext and the corrupted ciphertext), adds rows whose
// ciphertexts the SDK produced, and lists malformed encapsulation keys the SDK rejects.
//
//	python3 tooling/mlkem_vectors.py | go run tooling/mlkem_check.go > seed/tests/fixtures/mlkem-vectors.json
package main

import (
	"bytes"
	"crypto/mlkem"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
)

type row struct {
	Size     int    `json:"size"`
	Seed     string `json:"seed"`
	M        string `json:"m"`
	Ek       string `json:"ek"`
	C        string `json:"c"`
	Key      string `json:"key"`
	Bad      string `json:"bad"`
	Rejected string `json:"rejected"`
}

func h(s string) []byte { b, _ := hex.DecodeString(s); return b }

type dk interface {
	Decapsulate([]byte) ([]byte, error)
}

func keys(size int, seed []byte) (dk, []byte, func() ([]byte, []byte)) {
	if size == 768 {
		d, err := mlkem.NewDecapsulationKey768(seed)
		if err != nil {
			panic(err)
		}
		e := d.EncapsulationKey()
		return d, e.Bytes(), e.Encapsulate
	}
	d, err := mlkem.NewDecapsulationKey1024(seed)
	if err != nil {
		panic(err)
	}
	e := d.EncapsulationKey()
	return d, e.Bytes(), e.Encapsulate
}

func main() {
	var rows []row
	if err := json.NewDecoder(os.Stdin).Decode(&rows); err != nil {
		panic(err)
	}
	var sdk []row
	for _, r := range rows {
		d, ek, encapsulate := keys(r.Size, h(r.Seed))
		if !bytes.Equal(ek, h(r.Ek)) {
			panic(fmt.Sprintf("encapsulation key mismatch for %d", r.Size))
		}
		for _, pair := range [][2]string{{r.C, r.Key}, {r.Bad, r.Rejected}} {
			k, err := d.Decapsulate(h(pair[0]))
			if err != nil || !bytes.Equal(k, h(pair[1])) {
				panic(fmt.Sprintf("shared key mismatch for %d", r.Size))
			}
		}
		k, c := encapsulate()
		sdk = append(sdk, row{Size: r.Size, Seed: r.Seed, Ek: r.Ek, C: hex.EncodeToString(c), Key: hex.EncodeToString(k)})
	}
	var invalid []map[string]any
	for _, r := range rows[:1] {
		ek := h(r.Ek)
		over := append([]byte(nil), ek...)
		over[0], over[1] = 0xff, 0x0f // first coefficient 4095 >= q
		for _, e := range [][]byte{ek[:len(ek)-1], over} {
			_, err := mlkem.NewEncapsulationKey768(e)
			invalid = append(invalid, map[string]any{"size": 768, "ek": hex.EncodeToString(e), "rejected": err != nil})
		}
	}
	json.NewEncoder(os.Stdout).Encode(map[string]any{"rows": rows, "sdk": sdk, "invalid": invalid})
}
