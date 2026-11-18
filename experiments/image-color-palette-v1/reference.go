// Command reference captures the actual ordered fixed tables and nearest-color observations.
package main

import (
	"encoding/json"
	"image/color"
	"image/color/palette"
	"os"
	"runtime"
)

// rgba records every canonical channel without reducing sixteen-bit precision.
func rgba(value color.Color) [4]uint32 {
	r, g, b, a := value.RGBA()
	return [4]uint32{r, g, b, a}
}

// raw requires the actual table representation rather than manufacturing a converted value.
func raw(value color.Color) [4]uint8 {
	c, ok := value.(color.RGBA)
	if !ok {
		panic("fixed palette entry is not RGBA8")
	}
	return [4]uint8{c.R, c.G, c.B, c.A}
}

// queries covers every gray byte and deterministic mixed channels with alpha boundaries.
func queries() [][4]uint16 {
	result := make([][4]uint16, 0, 512)
	for value := uint16(0); value < 256; value++ {
		result = append(result, [4]uint16{value * 257, value * 257, value * 257, 65535})
	}
	state := uint32(0x5eeda11c)
	alpha := [...]uint16{0, 1, 255, 256, 32768, 65534, 65535}
	for index := 0; index < 256; index++ {
		var channels [4]uint16
		for lane := 0; lane < 3; lane++ {
			state = state*1664525 + 1013904223
			channels[lane] = uint16(state >> 16)
		}
		channels[3] = alpha[index%len(alpha)]
		result = append(result, channels)
	}
	return result
}

// observe retains original fields, exact canonical values and real model/index results.
func observe(name string, values []color.Color, inputs [][4]uint16) map[string]any {
	p := color.Palette(values)
	var model color.Model = p
	entries := make([]map[string]any, 0, len(values))
	for index, value := range values {
		entries = append(entries, map[string]any{"raw": raw(value), "rgba": rgba(value), "index": p.Index(value), "converted": raw(p.Convert(value)), "modeled": raw(model.Convert(value))})
		if p.Index(value) != index {
			panic("unexpected duplicate or exact-distance tie in the actual table")
		}
	}
	uses := make([]map[string]any, 0, len(inputs))
	for _, channels := range inputs {
		query := color.RGBA64{channels[0], channels[1], channels[2], channels[3]}
		uses = append(uses, map[string]any{"query": channels, "index": p.Index(query), "converted": raw(p.Convert(query)), "modeled": raw(model.Convert(query))})
	}
	return map[string]any{"name": name, "entries": entries, "queries": uses}
}

// main snapshots the actual tables without changing either process-global reference value.
func main() {
	inputs := queries()
	result := map[string]any{"sdk": runtime.Version(), "query_count": len(inputs), "palettes": []any{
		observe("Plan9", append([]color.Color(nil), palette.Plan9...), inputs),
		observe("WebSafe", append([]color.Color(nil), palette.WebSafe...), inputs),
	}}
	encoder := json.NewEncoder(os.Stdout)
	encoder.SetIndent("", "  ")
	if err := encoder.Encode(result); err != nil {
		panic(err)
	}
}
