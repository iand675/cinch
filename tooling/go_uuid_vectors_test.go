package uuidvectors

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"strings"
	"testing"
	"testing/synctest"
	"time"
	"uuid"
)

// parseCase records public parsing and receiver-preservation observations.
type parseCase struct {
	InputHex  string `json:"input_hex"`
	OK        bool   `json:"ok"`
	Canonical string `json:"canonical"`
	Receiver  string `json:"receiver"`
	MustPanic bool   `json:"must_panic"`
}

// appendCase retains arbitrary prefix bytes and the actual canonical suffix.
type appendCase struct {
	Prefix string `json:"prefix"`
	Value  string `json:"value"`
	Output string `json:"output"`
}

// compareCase records the complete public unsigned network ordering result.
type compareCase struct {
	First  string `json:"first"`
	Second string `json:"second"`
	Order  int    `json:"order"`
}

// v7Case retains one public clock sequence and actual entropy payload.
type v7Case struct {
	UnixNanos int64  `json:"unix_nanos"`
	Repeat    int    `json:"repeat"`
	Entropy   string `json:"entropy"`
	Canonical string `json:"canonical"`
}

// generatedCase records a real system-entropy generation observation.
type generatedCase struct {
	Algorithm string `json:"algorithm"`
	Canonical string `json:"canonical"`
	Version   byte   `json:"version"`
	Variant   byte   `json:"variant"`
}

// vectors preserves the independent public-operation reference corpus.
type vectors struct {
	SDK       string          `json:"sdk"`
	Parse     []parseCase     `json:"parse"`
	Append    []appendCase    `json:"append"`
	Compare   []compareCase   `json:"compare"`
	V7        []v7Case        `json:"v7"`
	Generated []generatedCase `json:"generated"`
}

// TestVectors emits the independent public UUID operation corpus.
func TestVectors(t *testing.T) {
	out := vectors{SDK: "go1.27.4"}
	// Controlled SDK clock scopes expose real public V7 behavior. Entropy remains
	// actual cryptographic entropy; each observation retains its resulting bytes.
	// sample observes repeated public V7 calls at the current controlled clock.
	sample := func(repeat int) {
		var u uuid.UUID
		now := time.Now().UnixNano()
		for i := 0; i < repeat; i++ {
			u = uuid.NewV7()
		}
		out.V7 = append(out.V7, v7Case{now, repeat, hex.EncodeToString(u[8:]), u.String()})
	}
	synctest.Test(t, func(t *testing.T) {
		sample(1)
		sample(3)
		sample(4097)
		time.Sleep(time.Nanosecond)
		sample(1)
		time.Sleep(500 * time.Millisecond)
		sample(1)
	})
	synctest.Test(t, func(t *testing.T) { sample(1); time.Sleep(2 * time.Second); sample(1) })
	synctest.Test(t, func(t *testing.T) { sample(1); sample(3) })
	sentinel := uuid.MustParse("aabbccdd-eeff-0011-2233-445566778899")
	// add records parse, unmarshal and strict-parser success/refusal together.
	add := func(text []byte) {
		s := string(text)
		u, err := uuid.Parse(s)
		receiver := sentinel
		unmarshal := receiver.UnmarshalText(text)
		if (err == nil) != (unmarshal == nil) {
			t.Fatal("parser mismatch")
		}
		panicked := false
		func() { defer func() { panicked = recover() != nil }(); _ = uuid.MustParse(s) }()
		canonical := ""
		if err == nil {
			canonical = u.String()
			if receiver != u {
				t.Fatal("receiver mismatch")
			}
		} else if receiver != sentinel {
			t.Fatal("failed parse changed receiver")
		}
		out.Parse = append(out.Parse, parseCase{hex.EncodeToString(text), err == nil, canonical, receiver.String(), panicked})
	}
	valid := []uuid.UUID{uuid.Nil(), uuid.Max(), sentinel, uuid.MustParse("f81d4fae-7dec-11d0-a765-00a0c91e6bf6")}
	for i := 0; i < 64; i++ {
		var u uuid.UUID
		for j := range u {
			u[j] = byte(i*37 + j*19)
		}
		valid = append(valid, u)
	}
	for _, u := range valid {
		canonical := u.String()
		for _, s := range []string{canonical, strings.ToUpper(canonical), "{" + canonical + "}", "urn:uuid:" + canonical, strings.ReplaceAll(canonical, "-", "")} {
			add([]byte(s))
		}
		for _, prefix := range [][]byte{nil, []byte("urn:uuid:"), {0, 255, 0, 128}} {
			saved := append([]byte(nil), prefix...)
			appended, err := u.AppendText(prefix)
			if err != nil || !bytes.Equal(appended[:len(saved)], saved) {
				t.Fatal("append failed")
			}
			marshalled, err := u.MarshalText()
			if err != nil || string(marshalled) != canonical {
				t.Fatal("marshal mismatch")
			}
			out.Append = append(out.Append, appendCase{hex.EncodeToString(saved), canonical, hex.EncodeToString(appended)})
		}
	}
	for n := 0; n < 49; n++ {
		if n == 32 || n == 36 || n == 38 || n == 45 {
			continue
		}
		add(bytes.Repeat([]byte{'0'}, n))
	}
	canonical := []byte("f81d4fae-7dec-11d0-a765-00a0c91e6bf6")
	for value := 0; value < 256; value++ {
		bad := append([]byte(nil), canonical...)
		bad[0] = byte(value)
		add(bad)
	}
	for _, index := range []int{8, 13, 18, 23} {
		for _, value := range []byte{'_', 0, 255} {
			bad := append([]byte(nil), canonical...)
			bad[index] = value
			add(bad)
		}
	}
	for _, s := range []string{" " + string(canonical), string(canonical) + " ", "URN:UUID:" + string(canonical), "{urn:uuid:" + string(canonical) + "}", "{f81d4fae7dec11d0a76500a0c91e6bf6}", "urn:uuid:f81d4fae7dec11d0a76500a0c91e6bf6", "[" + string(canonical) + "]"} {
		add([]byte(s))
	}
	// compare preserves the public three-way ordering result.
	compare := func(a, b uuid.UUID) {
		out.Compare = append(out.Compare, compareCase{a.String(), b.String(), a.Compare(b)})
	}
	for i, u := range valid {
		compare(u, u)
		compare(uuid.Nil(), u)
		compare(u, uuid.Max())
		if i+1 < len(valid) {
			compare(u, valid[i+1])
			compare(valid[i+1], u)
		}
	}
	for i := 0; i < 16; i++ {
		var a, b uuid.UUID
		a[i] = 127
		b[i] = 128
		compare(a, b)
		compare(b, a)
	}
	for _, item := range []struct {
		name string
		fn   func() uuid.UUID
	}{{"default", uuid.New}, {"v4", uuid.NewV4}, {"v7", uuid.NewV7}} {
		for i := 0; i < 16; i++ {
			u := item.fn()
			out.Generated = append(out.Generated, generatedCase{item.name, u.String(), u[6] >> 4, u[8] >> 6})
		}
	}
	encoded, err := json.MarshalIndent(out, "", "  ")
	if err != nil {
		t.Fatal(err)
	}
	path := os.Getenv("CINCH_UUID_OUTPUT")
	if path == "" {
		t.Fatal("missing output")
	}
	if err := os.WriteFile(path, append(encoded, '\n'), 0600); err != nil {
		t.Fatal(err)
	}
	fmt.Printf("uuid vectors: %d parse, %d append, %d compare, %d V7 clocks, %d actual generations\n", len(out.Parse), len(out.Append), len(out.Compare), len(out.V7), len(out.Generated))
}
