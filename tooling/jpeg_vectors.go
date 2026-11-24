// Command jpeg_vectors writes independent JPEG vectors from the reference SDK's implementation.
//
// Decoding rows take the files of tooling/fixtures/jpeg-inputs.json (an independent encoder's output),
// the SDK's own JPEG test images, truncations and corruptions of them; each records the decoder's error
// or the decoded colour model, size and chroma ratio with SHA-256 digests of the visible samples (luma
// then chroma planes, or packed RGB, CMYK or gray rows) and of the 8-bit RGB conversion, plus the
// configuration reader's answer. Encoding rows give rasters of every pixel format and YCbCr planes of
// every chroma ratio with the digest and length of the bytes the reference encoder writes at a quality.
//
//	go run tooling/jpeg_vectors.go > seed/tests/fixtures/jpeg-vectors.json
package main

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"image"
	"image/color"
	"image/jpeg"
	"math/rand"
	"os"
	"path/filepath"
	"runtime"
	"sort"
)

type decodeRow struct {
	Name   string `json:"name"`
	Data   string `json:"data"`
	Error  string `json:"error"`
	Kind   string `json:"kind"`
	Width  int    `json:"width"`
	Height int    `json:"height"`
	Ratio  string `json:"ratio"`
	Planes string `json:"planes"`
	RGB    string `json:"rgb"`
	Config string `json:"config"`
}

type encodeRow struct {
	Kind    string `json:"kind"`
	Width   int    `json:"width"`
	Height  int    `json:"height"`
	Pix     string `json:"pix"`
	Palette string `json:"palette"`
	Ratio   string `json:"ratio"`
	Quality int    `json:"quality"`
	Digest  string `json:"digest"`
	Length  int    `json:"length"`
}

func digest(b []byte) string {
	s := sha256.Sum256(b)
	return hex.EncodeToString(s[:])
}

var ratioNames = map[image.YCbCrSubsampleRatio]string{
	image.YCbCrSubsampleRatio444: "444", image.YCbCrSubsampleRatio422: "422", image.YCbCrSubsampleRatio420: "420",
	image.YCbCrSubsampleRatio440: "440", image.YCbCrSubsampleRatio411: "411", image.YCbCrSubsampleRatio410: "410",
}

func chromaDiv(r image.YCbCrSubsampleRatio) (int, int) {
	switch r {
	case image.YCbCrSubsampleRatio422:
		return 2, 1
	case image.YCbCrSubsampleRatio420:
		return 2, 2
	case image.YCbCrSubsampleRatio440:
		return 1, 2
	case image.YCbCrSubsampleRatio411:
		return 4, 1
	case image.YCbCrSubsampleRatio410:
		return 4, 2
	}
	return 1, 1
}

func configName(data []byte) string {
	c, err := jpeg.DecodeConfig(bytes.NewReader(data))
	if err != nil {
		return "error: " + err.Error()
	}
	model := map[color.Model]string{color.GrayModel: "gray", color.YCbCrModel: "ycbcr", color.RGBAModel: "rgb", color.CMYKModel: "cmyk"}[c.ColorModel]
	return model + " " + itoa(c.Width) + " " + itoa(c.Height)
}

func itoa(n int) string {
	if n == 0 {
		return "0"
	}
	s := ""
	for n > 0 {
		s = string(rune('0'+n%10)) + s
		n /= 10
	}
	return s
}

func decodeRowOf(name string, data []byte) decodeRow {
	row := decodeRow{Name: name, Data: hex.EncodeToString(data), Config: configName(data)}
	img, err := jpeg.Decode(bytes.NewReader(data))
	if err != nil {
		row.Error = err.Error()
		return row
	}
	b := img.Bounds()
	row.Width, row.Height = b.Dx(), b.Dy()
	var planes, rgb []byte
	switch m := img.(type) {
	case *image.Gray:
		row.Kind = "gray"
		for y := 0; y < b.Dy(); y++ {
			planes = append(planes, m.Pix[y*m.Stride:y*m.Stride+b.Dx()]...)
		}
		rgb = planes
	case *image.YCbCr:
		row.Kind = "ycbcr"
		row.Ratio = ratioNames[m.SubsampleRatio]
		xd, yd := chromaDiv(m.SubsampleRatio)
		cw, ch := (b.Dx()+xd-1)/xd, (b.Dy()+yd-1)/yd
		for y := 0; y < b.Dy(); y++ {
			planes = append(planes, m.Y[y*m.YStride:y*m.YStride+b.Dx()]...)
		}
		for _, p := range [][]byte{m.Cb, m.Cr} {
			for y := 0; y < ch; y++ {
				planes = append(planes, p[y*m.CStride:y*m.CStride+cw]...)
			}
		}
		for y := 0; y < b.Dy(); y++ {
			for x := 0; x < b.Dx(); x++ {
				r, g, bl := color.YCbCrToRGB(m.Y[m.YOffset(x, y)], m.Cb[m.COffset(x, y)], m.Cr[m.COffset(x, y)])
				rgb = append(rgb, r, g, bl)
			}
		}
	case *image.RGBA:
		row.Kind = "rgb"
		for y := 0; y < b.Dy(); y++ {
			for x := 0; x < b.Dx(); x++ {
				o := y*m.Stride + 4*x
				planes = append(planes, m.Pix[o], m.Pix[o+1], m.Pix[o+2])
			}
		}
		rgb = planes
	case *image.CMYK:
		row.Kind = "cmyk"
		for y := 0; y < b.Dy(); y++ {
			planes = append(planes, m.Pix[y*m.Stride:y*m.Stride+4*b.Dx()]...)
			for x := 0; x < b.Dx(); x++ {
				o := y*m.Stride + 4*x
				r, g, bl := color.CMYKToRGB(m.Pix[o], m.Pix[o+1], m.Pix[o+2], m.Pix[o+3])
				rgb = append(rgb, r, g, bl)
			}
		}
	default:
		panic("unexpected image type")
	}
	row.Planes, row.RGB = digest(planes), digest(rgb)
	return row
}

func encodeRowOf(kind string, w, h int, pix, palette []byte, ratio string, quality int, img image.Image) encodeRow {
	var out bytes.Buffer
	if err := jpeg.Encode(&out, img, &jpeg.Options{Quality: quality}); err != nil {
		panic(err)
	}
	return encodeRow{Kind: kind, Width: w, Height: h, Pix: hex.EncodeToString(pix), Palette: hex.EncodeToString(palette), Ratio: ratio,
		Quality: quality, Digest: digest(out.Bytes()), Length: out.Len()}
}

// smooth is a deterministic picture channel with texture.
func smooth(rng *rand.Rand, x, y, w, h, c int) byte {
	v := (x*255/max(1, w-1))*(c+1)/3 + (y*255/max(1, h-1))*(3-c)/3 + rng.Intn(31) - 15
	if (x/3+y/4)%2 == 0 {
		v += 40
	}
	return byte(min(255, max(0, v)))
}

func rasters() []encodeRow {
	rng := rand.New(rand.NewSource(1995))
	var rows []encodeRow
	sizes := [][2]int{{1, 1}, {7, 5}, {16, 16}, {17, 33}, {40, 24}}
	qualities := []int{1, 25, 50, 75, 90, 100}
	n := 0
	for _, kind := range []string{"gray8", "rgb8", "rgba8", "gray16", "rgba16", "rgb16", "indexed", "grayalpha8", "grayalpha16"} {
		for si, sz := range sizes {
			w, h := sz[0], sz[1]
			q := qualities[(n+si)%len(qualities)]
			n++
			var pix, palette []byte
			var img image.Image
			switch kind {
			case "gray8":
				m := image.NewGray(image.Rect(0, 0, w, h))
				for y := 0; y < h; y++ {
					for x := 0; x < w; x++ {
						m.Pix[y*w+x] = smooth(rng, x, y, w, h, 1)
					}
				}
				pix, img = m.Pix, m
			case "rgb8":
				m := image.NewRGBA(image.Rect(0, 0, w, h))
				for y := 0; y < h; y++ {
					for x := 0; x < w; x++ {
						for c := 0; c < 3; c++ {
							v := smooth(rng, x, y, w, h, c)
							m.Pix[4*(y*w+x)+c] = v
							pix = append(pix, v)
						}
						m.Pix[4*(y*w+x)+3] = 255
					}
				}
				img = m
			case "rgba8", "grayalpha8":
				m := image.NewNRGBA(image.Rect(0, 0, w, h))
				for y := 0; y < h; y++ {
					for x := 0; x < w; x++ {
						a := byte(rng.Intn(256))
						if rng.Intn(3) == 0 {
							a = 255
						}
						if kind == "rgba8" {
							for c := 0; c < 3; c++ {
								m.Pix[4*(y*w+x)+c] = smooth(rng, x, y, w, h, c)
							}
							m.Pix[4*(y*w+x)+3] = a
							pix = append(pix, m.Pix[4*(y*w+x):4*(y*w+x)+4]...)
						} else {
							g := smooth(rng, x, y, w, h, 1)
							m.Pix[4*(y*w+x)], m.Pix[4*(y*w+x)+1], m.Pix[4*(y*w+x)+2], m.Pix[4*(y*w+x)+3] = g, g, g, a
							pix = append(pix, g, a)
						}
					}
				}
				img = m
			case "gray16":
				m := image.NewGray16(image.Rect(0, 0, w, h))
				for i := range m.Pix {
					m.Pix[i] = byte(rng.Intn(256))
				}
				pix, img = m.Pix, m
			case "rgba16", "rgb16", "grayalpha16":
				m := image.NewNRGBA64(image.Rect(0, 0, w, h))
				for y := 0; y < h; y++ {
					for x := 0; x < w; x++ {
						var v [4]uint16
						for c := 0; c < 4; c++ {
							v[c] = uint16(smooth(rng, x, y, w, h, c%3))<<8 | uint16(rng.Intn(256))
						}
						if kind == "rgb16" || rng.Intn(3) == 0 {
							v[3] = 65535
						}
						if kind == "grayalpha16" {
							v[1], v[2] = v[0], v[0]
						}
						m.SetNRGBA64(x, y, color.NRGBA64{v[0], v[1], v[2], v[3]})
						switch kind {
						case "rgba16":
							pix = append(pix, m.Pix[8*(y*w+x):8*(y*w+x)+8]...)
						case "rgb16":
							pix = append(pix, m.Pix[8*(y*w+x):8*(y*w+x)+6]...)
						default:
							pix = append(pix, m.Pix[8*(y*w+x):8*(y*w+x)+2]...)
							pix = append(pix, m.Pix[8*(y*w+x)+6:8*(y*w+x)+8]...)
						}
					}
				}
				img = m
			case "indexed":
				var pal color.Palette
				for i := 0; i < 37; i++ {
					c := color.NRGBA{byte(rng.Intn(256)), byte(rng.Intn(256)), byte(rng.Intn(256)), byte(rng.Intn(256))}
					if i%2 == 0 {
						c.A = 255
					}
					pal = append(pal, c)
					palette = append(palette, c.R, c.G, c.B, c.A)
				}
				m := image.NewPaletted(image.Rect(0, 0, w, h), pal)
				for i := range m.Pix {
					m.Pix[i] = byte(rng.Intn(len(pal)))
				}
				pix, img = m.Pix, m
			}
			rows = append(rows, encodeRowOf(kind, w, h, pix, palette, "", q, img))
		}
	}
	ratios := []image.YCbCrSubsampleRatio{image.YCbCrSubsampleRatio444, image.YCbCrSubsampleRatio422, image.YCbCrSubsampleRatio420,
		image.YCbCrSubsampleRatio440, image.YCbCrSubsampleRatio411, image.YCbCrSubsampleRatio410}
	for i, r := range ratios {
		for _, sz := range [][2]int{{13, 11}, {32, 16}} {
			w, h := sz[0], sz[1]
			m := image.NewYCbCr(image.Rect(0, 0, w, h), r)
			for j := range m.Y {
				m.Y[j] = byte(rng.Intn(256))
			}
			for j := range m.Cb {
				m.Cb[j], m.Cr[j] = byte(rng.Intn(256)), byte(rng.Intn(256))
			}
			pix := append(append(append([]byte{}, m.Y...), m.Cb...), m.Cr...)
			rows = append(rows, encodeRowOf("ycbcr", w, h, pix, nil, ratioNames[r], qualities[(i+w)%len(qualities)], m))
		}
	}
	return rows
}

func main() {
	var inputs []struct {
		Name string `json:"name"`
		Data string `json:"data"`
	}
	raw, err := os.ReadFile("tooling/fixtures/jpeg-inputs.json")
	if err != nil {
		panic(err)
	}
	if err := json.Unmarshal(raw, &inputs); err != nil {
		panic(err)
	}
	type file struct {
		name string
		data []byte
	}
	var files []file
	for _, in := range inputs {
		d, _ := hex.DecodeString(in.Data)
		files = append(files, file{in.Name, d})
	}
	sdk, _ := filepath.Glob(filepath.Join(runtime.GOROOT(), "src/image/testdata/*.jpeg"))
	sort.Strings(sdk)
	for _, p := range sdk {
		d, err := os.ReadFile(p)
		if err != nil {
			panic(err)
		}
		files = append(files, file{"sdk-" + filepath.Base(p), d})
	}
	var rows []decodeRow
	for _, f := range files {
		rows = append(rows, decodeRowOf(f.name, f.data))
	}
	// Truncations and corruptions of a few files.
	rng := rand.New(rand.NewSource(2024))
	var damaged []file
	damaged = append(damaged, files[:9]...)
	damaged = append(damaged, files[13:20]...)
	for _, f := range damaged {
		for _, cut := range []int{2, 4, 20, len(f.data) / 3, len(f.data) - 2, len(f.data) - 1} {
			rows = append(rows, decodeRowOf(f.name+"-cut", f.data[:cut]))
		}
		for k := 0; k < 6; k++ {
			d := append([]byte{}, f.data...)
			for j := 0; j < 1+k%3; j++ {
				d[rng.Intn(len(d))] = byte(rng.Intn(256))
			}
			rows = append(rows, decodeRowOf(f.name+"-corrupt", d))
		}
	}
	out := map[string]any{"decode": rows, "encode": rasters()}
	enc := json.NewEncoder(os.Stdout)
	if err := enc.Encode(out); err != nil {
		panic(err)
	}
}
