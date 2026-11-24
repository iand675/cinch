// Command draw_vectors writes independent compositing vectors from the reference SDK's draw
// package: Src and Over, with and without masks, uniform sources, clipped rectangles, and every
// destination kind the rasters model (NRGBA, NRGBA64, Gray, Gray16 and Paletted, the last also
// with Floyd-Steinberg error diffusion), over NRGBA, NRGBA64, Gray and Paletted sources. Each row
// gives the inputs and the destination's pixels afterwards.
//
//	go run tooling/draw_vectors.go > seed/tests/fixtures/draw-vectors.json
package main

import (
	"encoding/hex"
	"encoding/json"
	"image"
	"image/color"
	"image/draw"
	"math/rand"
	"os"
)

type img struct {
	Kind    string `json:"kind"`
	Width   int    `json:"width"`
	Height  int    `json:"height"`
	Pix     string `json:"pix"`
	Palette string `json:"palette"`
}

type row struct {
	Dst     img    `json:"dst"`
	Src     img    `json:"src"`
	Mask    *img   `json:"mask"`
	Uniform []int  `json:"uniform"`
	Rect    [4]int `json:"rect"`
	Sp      [2]int `json:"sp"`
	Mp      [2]int `json:"mp"`
	Op      string `json:"op"`
	Dither  bool   `json:"dither"`
	Out     string `json:"out"`
}

var r = rand.New(rand.NewSource(7))

func palette(n int) color.Palette {
	p := make(color.Palette, n)
	for i := range p {
		p[i] = color.NRGBA{uint8(r.Intn(256)), uint8(r.Intn(256)), uint8(r.Intn(256)), 255}
	}
	if n > 2 {
		p[1] = color.NRGBA{}
	}
	return p
}

func paletteHex(p color.Palette) string {
	var b []byte
	for _, c := range p {
		n := c.(color.NRGBA)
		b = append(b, n.R, n.G, n.B, n.A)
	}
	return hex.EncodeToString(b)
}

func alphaByte() uint8 {
	switch r.Intn(4) {
	case 0:
		return 0
	case 1:
		return 255
	}
	return uint8(r.Intn(256))
}

func newImage(kind string, w, h int, p color.Palette) (draw.Image, img) {
	rect := image.Rect(0, 0, w, h)
	switch kind {
	case "nrgba":
		m := image.NewNRGBA(rect)
		for i := 0; i < len(m.Pix); i += 4 {
			m.Pix[i], m.Pix[i+1], m.Pix[i+2], m.Pix[i+3] = uint8(r.Intn(256)), uint8(r.Intn(256)), uint8(r.Intn(256)), alphaByte()
		}
		return m, img{kind, w, h, "", ""}
	case "nrgba64":
		m := image.NewNRGBA64(rect)
		for i := range m.Pix {
			m.Pix[i] = uint8(r.Intn(256))
		}
		for i := 6; i < len(m.Pix); i += 8 {
			if r.Intn(3) == 0 {
				m.Pix[i], m.Pix[i+1] = 255, 255
			}
		}
		return m, img{kind, w, h, "", ""}
	case "gray":
		m := image.NewGray(rect)
		for i := range m.Pix {
			m.Pix[i] = uint8(r.Intn(256))
		}
		return m, img{kind, w, h, "", ""}
	case "gray16":
		m := image.NewGray16(rect)
		for i := range m.Pix {
			m.Pix[i] = uint8(r.Intn(256))
		}
		return m, img{kind, w, h, "", ""}
	}
	m := image.NewPaletted(rect, p)
	for i := range m.Pix {
		m.Pix[i] = uint8(r.Intn(len(p)))
	}
	return m, img{kind, w, h, "", paletteHex(p)}
}

func pix(m draw.Image) string {
	switch v := m.(type) {
	case *image.NRGBA:
		return hex.EncodeToString(v.Pix)
	case *image.NRGBA64:
		return hex.EncodeToString(v.Pix)
	case *image.Gray:
		return hex.EncodeToString(v.Pix)
	case *image.Gray16:
		return hex.EncodeToString(v.Pix)
	case *image.Paletted:
		return hex.EncodeToString(v.Pix)
	}
	panic("kind")
}

func main() {
	var rows []row
	kinds := []string{"nrgba", "nrgba64", "gray", "gray16", "paletted"}
	srcs := []string{"nrgba", "nrgba64", "gray", "paletted"}
	rects := [][4]int{{0, 0, 9, 7}, {2, 1, 7, 6}, {-3, -2, 5, 4}, {4, 3, 20, 20}}
	points := [][2]int{{0, 0}, {1, 2}, {-1, 0}, {3, 1}}
	n := 0
	for _, dk := range kinds {
		for _, sk := range srcs {
			for _, op := range []draw.Op{draw.Src, draw.Over} {
				for masked := 0; masked < 3; masked++ {
					p := palette(1 + r.Intn(20))
					dst, dimg := newImage(dk, 9, 7, p)
					src, simg := newImage(sk, 8, 6, palette(1+r.Intn(30)))
					dimg.Pix, simg.Pix = pix(dst), pix(src)
					rc, sp := rects[n%4], points[(n/4)%4]
					mp := points[(n+1)%4]
					n++
					rowv := row{Dst: dimg, Src: simg, Rect: rc, Sp: sp, Mp: mp, Op: map[draw.Op]string{draw.Src: "src", draw.Over: "over"}[op]}
					rect := image.Rect(rc[0], rc[1], rc[2], rc[3])
					switch masked {
					case 0:
						draw.Draw(dst, rect, src, image.Pt(sp[0], sp[1]), op)
					case 1:
						mask, mimg := newImage("nrgba", 7, 8, nil)
						mimg.Pix = pix(mask)
						rowv.Mask = &mimg
						draw.DrawMask(dst, rect, src, image.Pt(sp[0], sp[1]), mask, image.Pt(mp[0], mp[1]), op)
					case 2:
						c := color.NRGBA{uint8(r.Intn(256)), uint8(r.Intn(256)), uint8(r.Intn(256)), alphaByte()}
						rowv.Uniform = []int{int(c.R), int(c.G), int(c.B), int(c.A)}
						draw.Draw(dst, rect, image.NewUniform(c), image.Point{}, op)
					}
					rowv.Out = pix(dst)
					rows = append(rows, rowv)
				}
			}
		}
	}
	for _, sk := range srcs {
		for k := 0; k < 3; k++ {
			p := palette(2 + r.Intn(40))
			dst, dimg := newImage("paletted", 13, 9, p)
			src, simg := newImage(sk, 12, 10, palette(50))
			dimg.Pix, simg.Pix = pix(dst), pix(src)
			rc := rects[k%4]
			draw.FloydSteinberg.Draw(dst, image.Rect(rc[0], rc[1], rc[2], rc[3]), src, image.Pt(k, 0))
			rows = append(rows, row{Dst: dimg, Src: simg, Rect: rc, Sp: [2]int{k, 0}, Op: "src", Dither: true, Out: pix(dst)})
		}
	}
	json.NewEncoder(os.Stdout).Encode(rows)
}
