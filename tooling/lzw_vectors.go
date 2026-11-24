// Command lzw_vectors writes independent LZW vectors from the reference SDK's implementation:
// compressed bytes for both bit orders and literal widths, and decodings of hand-built streams.
//
//	go run tooling/lzw_vectors.go > seed/tests/fixtures/lzw-vectors.json
package main

import (
	"bytes"
	"compress/lzw"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"io"
	"os"
)

// vector records the corpus by generator (kind and size) so consumers rebuild it, the full
// encoding for small cases, and the SHA-256 digest and length of every encoding.
type vector struct {
	Order   int    `json:"order"`
	Width   int    `json:"width"`
	Kind    int    `json:"kind"`
	Size    int    `json:"size"`
	Encoded string `json:"encoded"`
	Digest  string `json:"digest"`
	Length  int    `json:"length"`
}

func corpus(n int, width int, kind int) []byte {
	out := make([]byte, n)
	state := uint32(12345)
	for i := range out {
		switch kind {
		case 0:
			out[i] = byte((i*37 + 11) % (1 << width))
		case 1:
			out[i] = byte(i % 3 % (1 << width))
		default:
			state = state*1103515245 + 12345
			out[i] = byte(state>>16) & byte(1<<width-1)
		}
	}
	return out
}

func main() {
	var vectors []vector
	for _, order := range []lzw.Order{lzw.LSB, lzw.MSB} {
		for _, width := range []int{2, 3, 5, 7, 8} {
			for _, size := range []int{0, 1, 2, 3, 10, 100, 1000, 5000, 20000} {
				for kind := 0; kind < 3; kind++ {
					in := corpus(size, width, kind)
					var buf bytes.Buffer
					w := lzw.NewWriter(&buf, order, width)
					w.Write(in)
					w.Close()
					r := lzw.NewReader(bytes.NewReader(buf.Bytes()), order, width)
					back, err := io.ReadAll(r)
					if err != nil || !bytes.Equal(back, in) {
						panic("reference round trip failed")
					}
					digest := sha256.Sum256(buf.Bytes())
					v := vector{int(order), width, kind, size, "", hex.EncodeToString(digest[:]), buf.Len()}
					if size <= 1000 {
						v.Encoded = hex.EncodeToString(buf.Bytes())
					}
					vectors = append(vectors, v)
				}
			}
		}
	}
	enc := json.NewEncoder(os.Stdout)
	enc.SetIndent("", " ")
	enc.Encode(vectors)
}
