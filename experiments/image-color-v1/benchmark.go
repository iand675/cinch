// Command benchmark measures complete color pipelines with exact output and owner observations.
package main

import (
	"fmt"
	"image/color"
	"os"
	"strconv"
	"time"
)

// sourceColor retains a real custom canonical-channel capability.
type sourceColor struct{ channels [4]uint16 }

// RGBA observes all immutable captured channel fields when invoked.
func (c sourceColor) RGBA() (uint32, uint32, uint32, uint32) {
	return uint32(c.channels[0]), uint32(c.channels[1]), uint32(c.channels[2]), uint32(c.channels[3])
}

// modes fixes the public pipeline selection outside the measured interval.
var modes = []string{"rgba-control", "premultiply", "straight-model", "gray-model", "ycbcr16", "rgb-ycbcr", "ycbcr-rgb", "rgb-cmyk", "cmyk-rgb", "custom-source", "custom-model", "alpha-model", "palette-index-empty", "palette-index-small", "palette-index-256", "palette-index-ties", "palette-convert-empty", "palette-convert-small", "palette-convert-256", "palette-convert-ties", "palette-model-256"}

// word consumes every bounded scalar through the same dependent scalar checksum.
func word(value, scalar uint64) uint64 { return (value*131 + scalar) & 2147483647 }

// fields consumes a complete four-slot representation, padding smaller types with zero.
func fields(value uint64, r, g, b, a uint32) uint64 {
	for _, v := range [4]uint32{r, g, b, a} {
		value = word(value, uint64(v))
	}
	return value
}

// rawHash observes the real returned type and every raw field before canonical conversion.
func rawHash(value color.Color, initial uint64) uint64 {
	switch c := value.(type) {
	case color.RGBA:
		return fields(word(initial, 0), uint32(c.R), uint32(c.G), uint32(c.B), uint32(c.A))
	case color.RGBA64:
		return fields(word(initial, 1), uint32(c.R), uint32(c.G), uint32(c.B), uint32(c.A))
	case color.NRGBA:
		return fields(word(initial, 2), uint32(c.R), uint32(c.G), uint32(c.B), uint32(c.A))
	case color.NRGBA64:
		return fields(word(initial, 3), uint32(c.R), uint32(c.G), uint32(c.B), uint32(c.A))
	case color.Alpha:
		return fields(word(initial, 4), uint32(c.A), 0, 0, 0)
	case color.Alpha16:
		return fields(word(initial, 5), uint32(c.A), 0, 0, 0)
	case color.Gray:
		return fields(word(initial, 6), uint32(c.Y), 0, 0, 0)
	case color.Gray16:
		return fields(word(initial, 7), uint32(c.Y), 0, 0, 0)
	case color.YCbCr:
		return fields(word(initial, 8), uint32(c.Y), uint32(c.Cb), uint32(c.Cr), 0)
	case color.NYCbCrA:
		return fields(word(initial, 9), uint32(c.Y), uint32(c.Cb), uint32(c.Cr), uint32(c.A))
	case color.CMYK:
		return fields(word(initial, 10), uint32(c.C), uint32(c.M), uint32(c.Y), uint32(c.K))
	case sourceColor:
		r, g, b, a := c.RGBA()
		return fields(word(initial, 11), r, g, b, a)
	default:
		panic("unexpected benchmark color identity")
	}
}

// colorHash consumes all source fields and all sixteen-bit canonical channels.
func colorHash(value color.Color, initial uint64) uint64 {
	raw := rawHash(value, initial)
	r, g, b, a := value.RGBA()
	return fields(raw, r, g, b, a)
}

// optionHash preserves nil conversions as an explicit discriminant.
func optionHash(value color.Color, initial uint64) uint64 {
	if value == nil {
		return word(initial, 97)
	}
	return colorHash(value, word(initial, 98))
}

// components generates matching fixed-width raw fields and zero/partial/opaque alpha patterns.
func components(index uint64) (uint16, uint16, uint16, uint16) {
	r := (index*1103515245 + 12345) & 65535
	g := (r*97 + 53) & 65535
	b := (r*251 + 101) & 65535
	a := [4]uint16{0, 257, 32768, 65535}[index&3]
	return uint16(r), uint16(g), uint16(b), a
}

// canonical stores exact premultiplied input channels before the start clock.
func canonical(value color.Color) color.RGBA64 {
	r, g, b, a := value.RGBA()
	return color.RGBA64{uint16(r), uint16(g), uint16(b), uint16(a)}
}

// pixel prepares equivalent concrete source types outside the measured operation.
func pixel(mode int, index uint64) color.Color {
	r, g, b, a := components(index)
	ordinary := color.NRGBA64{r, g, b, a}
	switch mode {
	case 1, 10:
		return ordinary
	case 4:
		return color.NYCbCrA{color.YCbCr{uint8(r >> 8), uint8(g >> 8), uint8(b >> 8)}, uint8(a >> 8)}
	case 5, 7:
		return color.RGBA{uint8(r >> 8), uint8(g >> 8), uint8(b >> 8), 255}
	case 6:
		return color.YCbCr{uint8(r >> 8), uint8(g >> 8), uint8(b >> 8)}
	case 8:
		return color.CMYK{uint8(r >> 8), uint8(g >> 8), uint8(b >> 8), uint8(index * 17)}
	case 9:
		c := canonical(ordinary)
		return sourceColor{[4]uint16{c.R, c.G, c.B, c.A}}
	case 15, 19:
		return color.RGBA64{}
	default:
		return canonical(ordinary)
	}
}

// paletteFor preserves emptiness, first truncated-distance ties and original prepared order.
func paletteFor(mode int) color.Palette {
	if mode < 12 || mode == 12 || mode == 16 {
		return color.Palette{}
	}
	if mode == 15 || mode == 19 {
		return color.Palette{color.RGBA64{1, 1, 1, 1}, color.RGBA64{}}
	}
	size := 256
	if mode == 13 || mode == 17 {
		size = 4
	}
	palette := make(color.Palette, size)
	for i := range palette {
		palette[i] = pixel(0, uint64(i))
	}
	return palette
}

// customModel retains a runtime-selected transform and returns nil for transparent inputs.
func customModel(rotate bool) color.Model {
	return color.ModelFunc(func(value color.Color) color.Color {
		r, g, b, a := value.RGBA()
		if a == 0 {
			return nil
		}
		if rotate {
			return color.RGBA64{uint16(g), uint16(b), uint16(r), uint16(a)}
		}
		return color.RGBA64{uint16(b), uint16(r), uint16(g), uint16(a)}
	})
}

// operation executes actual public model/helper/palette calls and consumes complete outputs.
func operation(mode int, palette color.Palette, model color.Model, input color.Color, initial uint64) uint64 {
	switch mode {
	case 2:
		return optionHash(color.NRGBAModel.Convert(input), initial)
	case 3:
		return optionHash(color.Gray16Model.Convert(input), initial)
	case 5:
		c := input.(color.RGBA)
		y, cb, cr := color.RGBToYCbCr(c.R, c.G, c.B)
		return colorHash(color.YCbCr{y, cb, cr}, initial)
	case 6:
		c := input.(color.YCbCr)
		r, g, b := color.YCbCrToRGB(c.Y, c.Cb, c.Cr)
		return colorHash(color.RGBA{r, g, b, 255}, initial)
	case 7:
		c := input.(color.RGBA)
		cc, m, y, k := color.RGBToCMYK(c.R, c.G, c.B)
		return colorHash(color.CMYK{cc, m, y, k}, initial)
	case 8:
		c := input.(color.CMYK)
		r, g, b := color.CMYKToRGB(c.C, c.M, c.Y, c.K)
		return colorHash(color.RGBA{r, g, b, 255}, initial)
	case 10:
		return optionHash(model.Convert(input), initial)
	case 11:
		return optionHash(color.Alpha16Model.Convert(input), initial)
	case 12, 13, 14, 15:
		return word(initial, uint64(palette.Index(input)))
	case 16, 17, 18, 19:
		return optionHash(palette.Convert(input), initial)
	case 20:
		return optionHash(model.Convert(input), initial)
	default:
		return colorHash(input, initial)
	}
}

// main excludes setup and retained-owner traversal from the exact public-pipeline clock.
func main() {
	if len(os.Args) != 4 {
		os.Exit(2)
	}
	mode := -1
	for i, name := range modes {
		if os.Args[1] == name {
			mode = i
		}
	}
	size, err := strconv.Atoi(os.Args[2])
	if err != nil || size < 0 || size > 65535 {
		os.Exit(2)
	}
	count, err := strconv.Atoi(os.Args[3])
	if err != nil || count < 0 || mode < 0 {
		os.Exit(2)
	}
	input := make([]color.Color, size)
	for i := range input {
		input[i] = pixel(mode, uint64(i))
	}
	palette := paletteFor(mode)
	model := customModel(size%2 == 0)
	if mode == 20 {
		model = color.Model(palette)
	}
	result := uint64(0)
	start := time.Now()
	for i := 0; i < count; i++ {
		for _, value := range input {
			result = operation(mode, palette, model, value, result)
		}
	}
	elapsed := time.Since(start).Nanoseconds()
	retained := uint64(0)
	for _, value := range input {
		retained = colorHash(value, retained)
	}
	for _, value := range palette {
		retained = colorHash(value, retained)
	}
	retained = optionHash(model.Convert(color.White), retained)
	fmt.Printf("%d %d %d\n", elapsed, result, retained)
}
