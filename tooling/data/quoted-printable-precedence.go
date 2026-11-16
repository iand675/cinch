// Independent supplemental fixtures combine inline, transport and physical-buffer failures.
package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"io"
	"mime/quotedprintable"
	"os"
	"strings"
)

// encode obtains complete output through only the public writer API.
func encode(input []byte, binary bool) string {
	var buffer bytes.Buffer
	writer := quotedprintable.NewWriter(&buffer)
	writer.Binary = binary
	if _, err := writer.Write(input); err != nil {
		panic(err)
	}
	if err := writer.Close(); err != nil {
		panic(err)
	}
	return hex.EncodeToString(buffer.Bytes())
}

// main freezes simultaneous failure precedence without importing implementation helpers.
func main() {
	inputs := []string{"a\x00=\r\r\n", "ab\x00=\r\n", strings.Repeat("a", 4093) + "\x00==", strings.Repeat("a", 4094) + "\x00=", strings.Repeat("a", 4095) + "=", strings.Repeat("a", 4094) + "=\r", "=\r\n\x00\n", "\x00= \t"}
	values := []map[string]string{}
	for _, input := range inputs {
		data := []byte(input)
		decoded, err := io.ReadAll(quotedprintable.NewReader(bytes.NewReader(data)))
		message := ""
		if err != nil {
			message = err.Error()
		}
		values = append(values, map[string]string{"input": hex.EncodeToString(data), "text": encode(data, false), "binary": encode(data, true), "decoded": hex.EncodeToString(decoded), "error": message})
	}
	json.NewEncoder(os.Stdout).Encode(map[string]any{"values": values, "readers": []any{}, "writers": []any{}})
}
