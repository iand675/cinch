// Command gif_vectors writes independent GIF vectors from the reference SDK's implementation.
// Encoding rows describe animations (global and local palettes, transparency, sub-rectangle
// frames, delays, disposal methods and loop counts) with the exact bytes the reference encoder
// writes; decoding rows give files (the encoded rows plus interlaced and GIF87a files read from
// tooling/fixtures) with the frames the reference decoder returns, or an error kind.
//
//	go run tooling/gif_vectors.go > seed/tests/fixtures/gif-vectors.json
package main

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"image"
	"image/color"
	"image/gif"
	"math/rand"
	"os"
	"strings"
)

type frame struct {
	Left     int    `json:"left"`
	Top      int    `json:"top"`
	Width    int    `json:"width"`
	Height   int    `json:"height"`
	Pix      string `json:"pix"`
	Palette  string `json:"palette"`
	Delay    int    `json:"delay"`
	Disposal int    `json:"disposal"`
}

type anim struct {
	Width      int     `json:"width"`
	Height     int     `json:"height"`
	Loop       int     `json:"loop"`
	Background int     `json:"background"`
	Global     string  `json:"global"`
	Frames     []frame `json:"frames"`
}

type row struct {
	Gif   *anim  `json:"gif"`
	Bytes string `json:"bytes"`
	Error string `json:"error"`
}

func paletteHex(p color.Palette) string {
	var b []byte
	for _, c := range p {
		r, g, bl, a := c.RGBA()
		b = append(b, byte(r>>8), byte(g>>8), byte(bl>>8), byte(a>>8))
	}
	return hex.EncodeToString(b)
}

func describe(g *gif.GIF) *anim {
	a := &anim{Width: g.Config.Width, Height: g.Config.Height, Loop: g.LoopCount, Background: int(g.BackgroundIndex), Frames: []frame{}}
	if p, ok := g.Config.ColorModel.(color.Palette); ok {
		a.Global = paletteHex(p)
	}
	for i, m := range g.Image {
		b := m.Bounds()
		pix := make([]byte, 0, b.Dx()*b.Dy())
		for y := b.Min.Y; y < b.Max.Y; y++ {
			pix = append(pix, m.Pix[m.PixOffset(b.Min.X, y):m.PixOffset(b.Min.X, y)+b.Dx()]...)
		}
		d := 0
		if g.Disposal != nil {
			d = int(g.Disposal[i])
		}
		a.Frames = append(a.Frames, frame{b.Min.X, b.Min.Y, b.Dx(), b.Dy(), hex.EncodeToString(pix), paletteHex(m.Palette), g.Delay[i], d})
	}
	return a
}

func kind(err error) string {
	m := err.Error()
	for _, k := range []struct{ p, n string }{
		{"gif: can't recognize format", "header"}, {"gif: reading header", "truncated"}, {"gif: reading color table", "truncated"},
		{"gif: reading frames", "truncated"}, {"gif: can't read image descriptor", "truncated"}, {"gif: reading extension", "truncated"},
		{"gif: can't read graphic control", "truncated"}, {"gif: unknown block type", "block"}, {"gif: unknown extension", "extension"},
		{"gif: invalid graphic control", "graphic-control"}, {"gif: frame bounds larger", "bounds"}, {"gif: no color table", "no-color-table"},
		{"gif: pixel size in decode out of range", "literal-width"}, {"gif: not enough image data", "not-enough"},
		{"gif: too much image data", "too-much"}, {"gif: invalid pixel value", "bad-pixel"}, {"gif: reading image data", "image-data"},
		{"gif: missing image data", "missing-image"},
	} {
		if strings.HasPrefix(m, k.p) {
			return k.n
		}
	}
	return "other:" + m
}

func randomPalette(r *rand.Rand, n int, transparent int) color.Palette {
	p := make(color.Palette, n)
	for i := range p {
		p[i] = color.RGBA{uint8(r.Intn(256)), uint8(r.Intn(256)), uint8(r.Intn(256)), 255}
	}
	if transparent >= 0 {
		p[transparent] = color.RGBA{}
	}
	return p
}

func paletted(r *rand.Rand, rect image.Rectangle, p color.Palette) *image.Paletted {
	m := image.NewPaletted(rect, p)
	for i := range m.Pix {
		if r.Intn(3) == 0 && i > 0 {
			m.Pix[i] = m.Pix[i-1]
		} else {
			m.Pix[i] = uint8(r.Intn(len(p)))
		}
	}
	return m
}

func main() {
	r := rand.New(rand.NewSource(89))
	var encodes, decodes []row
	var gifs []*gif.GIF
	for _, n := range []int{1, 2, 5, 16, 200, 256} {
		for _, size := range [][2]int{{1, 1}, {5, 3}, {37, 21}} {
			p := randomPalette(r, n, -1)
			m := paletted(r, image.Rect(0, 0, size[0], size[1]), p)
			gifs = append(gifs, &gif.GIF{Image: []*image.Paletted{m}, Delay: []int{0}, LoopCount: -1,
				Config: image.Config{ColorModel: p, Width: size[0], Height: size[1]}})
		}
	}
	for _, loop := range []int{0, -1, 3} {
		global := randomPalette(r, 12, -1)
		frames := []*image.Paletted{
			paletted(r, image.Rect(0, 0, 20, 10), global),
			paletted(r, image.Rect(3, 2, 11, 9), randomPalette(r, 7, 2)),
			paletted(r, image.Rect(5, 0, 20, 4), global),
			paletted(r, image.Rect(0, 5, 6, 10), randomPalette(r, 130, -1)),
		}
		gifs = append(gifs, &gif.GIF{Image: frames, Delay: []int{0, 10, 250, 7}, Disposal: []byte{0, 1, 2, 3}, LoopCount: loop,
			BackgroundIndex: 4, Config: image.Config{ColorModel: global, Width: 20, Height: 10}})
	}
	local := []*image.Paletted{paletted(r, image.Rect(0, 0, 9, 9), randomPalette(r, 3, 0)), paletted(r, image.Rect(0, 0, 9, 9), randomPalette(r, 64, -1))}
	gifs = append(gifs, &gif.GIF{Image: local, Delay: []int{5, 5}, LoopCount: 0, Config: image.Config{Width: 9, Height: 9}})
	big := randomPalette(r, 256, 17)
	gifs = append(gifs, &gif.GIF{Image: []*image.Paletted{paletted(r, image.Rect(0, 0, 300, 70), big)}, Delay: []int{0}, LoopCount: -1,
		Config: image.Config{ColorModel: big, Width: 300, Height: 70}})
	for _, g := range gifs {
		var buf bytes.Buffer
		if err := gif.EncodeAll(&buf, g); err != nil {
			panic(err)
		}
		encodes = append(encodes, row{Gif: describe(g), Bytes: hex.EncodeToString(buf.Bytes())})
		back, err := gif.DecodeAll(bytes.NewReader(buf.Bytes()))
		if err != nil {
			panic(err)
		}
		decodes = append(decodes, row{Gif: describe(back), Bytes: hex.EncodeToString(buf.Bytes())})
	}
	extra, _ := os.ReadFile("tooling/fixtures/gif-extra.json")
	var files []string
	json.Unmarshal(extra, &files)
	sample, _ := hex.DecodeString(encodes[4].Bytes)
	corrupt := func(f func([]byte) []byte) string { c := append([]byte(nil), sample...); return hex.EncodeToString(f(c)) }
	files = append(files,
		hex.EncodeToString([]byte("GIF88a")),
		hex.EncodeToString(sample[:10]),
		hex.EncodeToString(sample[:len(sample)-1]),
		hex.EncodeToString(sample[:len(sample)/2]),
		corrupt(func(c []byte) []byte { c[10] &^= 0x80; return c }),
		corrupt(func(c []byte) []byte { return append(c[:len(c)-1], 0x99) }),
		corrupt(func(c []byte) []byte { c[19] = 0x77; return c }),
		corrupt(func(c []byte) []byte { c[13+3*4+9] = 9; return c }),
		corrupt(func(c []byte) []byte { c[13+3*4+1] = 200; return c }),
		hex.EncodeToString(append(append([]byte("GIF89a"), 1, 0, 1, 0, 0x80, 0, 0, 0, 0, 0, 255, 255, 255), 0x3b)),
		hex.EncodeToString(append(append([]byte("GIF89a"), 1, 0, 1, 0, 0x80, 0, 0, 0, 0, 0, 255, 255, 255), 0x21, 0x42, 0x3b)),
	)
	for _, f := range files {
		data, _ := hex.DecodeString(f)
		g, err := gif.DecodeAll(bytes.NewReader(data))
		if err != nil {
			decodes = append(decodes, row{Bytes: f, Error: kind(err)})
		} else {
			decodes = append(decodes, row{Gif: describe(g), Bytes: f})
		}
	}
	enc := json.NewEncoder(os.Stdout)
	enc.Encode(map[string]any{"encode": encodes, "decode": decodes})
}
