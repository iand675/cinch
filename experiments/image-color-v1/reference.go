// Command reference captures independent concrete color and model observations.
package main

import (
	"encoding/json"
	"image/color"
	"os"
	"runtime"
)

// Spec records a source representation and its exact bounded raw fields.
type Spec struct {
	Kind string `json:"kind"`
	Channels []uint16 `json:"channels"`
}

// Observation records representation identity, original fields and canonical RGBA.
type Observation struct {
	Kind string `json:"kind"`
	Channels []uint16 `json:"channels"`
	RGBA [4]uint32 `json:"rgba"`
}

// Vector retains one concrete source and every builtin model result.
type Vector struct {
	Input Spec `json:"input"`
	Value Observation `json:"value"`
	Converted []Observation `json:"converted"`
}

// PaletteVector retains first-index and optional original selected-color observations.
type PaletteVector struct {
	Colors []Spec `json:"colors"`
	Query Spec `json:"query"`
	Index int `json:"index"`
	Converted *Observation `json:"converted"`
}

// RGBVector records both RGB conversion functions and an independent YCbCr input.
type RGBVector struct {
	Input [3]uint8 `json:"input"`
	YCbCr [3]uint8 `json:"ycbcr"`
	CMYK [4]uint8 `json:"cmyk"`
	FromYCbCr [3]uint8 `json:"from_ycbcr"`
}

// CMYKVector retains direct8-bit conversion separately from16-bit Color observation.
type CMYKVector struct {
	Input [4]uint8 `json:"input"`
	RGB [3]uint8 `json:"rgb"`
}

// CanonicalSource implements a distinct pure custom color capability.
type CanonicalSource struct { Channels [4]uint32 }

// RGBA returns all retained sixteen-bit channel observations without conversion.
func (c CanonicalSource) RGBA() (uint32, uint32, uint32, uint32) {
	return c.Channels[0], c.Channels[1], c.Channels[2], c.Channels[3]
}

// kinds fixes the shared representation order for independent fixture consumers.
var kinds = []string{"rgba8", "rgba16", "nrgba8", "nrgba16", "alpha8", "alpha16", "gray8", "gray16", "ycbcr", "ycbcra", "cmyk", "source"}

// models fixes the corresponding builtin conversion order without registry mutation.
var models = []color.Model{color.RGBAModel, color.RGBA64Model, color.NRGBAModel, color.NRGBA64Model, color.AlphaModel, color.Alpha16Model, color.GrayModel, color.Gray16Model, color.YCbCrModel, color.NYCbCrAModel, color.CMYKModel}

// build uses actual concrete SDK types, including malformed-but-representable premultiplied fields.
func build(s Spec) color.Color {
	c := append(append([]uint16{}, s.Channels...), 0, 0, 0, 0)
	switch s.Kind {
	case "rgba8": return color.RGBA{uint8(c[0]), uint8(c[1]), uint8(c[2]), uint8(c[3])}
	case "rgba16": return color.RGBA64{c[0], c[1], c[2], c[3]}
	case "nrgba8": return color.NRGBA{uint8(c[0]), uint8(c[1]), uint8(c[2]), uint8(c[3])}
	case "nrgba16": return color.NRGBA64{c[0], c[1], c[2], c[3]}
	case "alpha8": return color.Alpha{uint8(c[0])}
	case "alpha16": return color.Alpha16{c[0]}
	case "gray8": return color.Gray{uint8(c[0])}
	case "gray16": return color.Gray16{c[0]}
	case "ycbcr": return color.YCbCr{uint8(c[0]), uint8(c[1]), uint8(c[2])}
	case "ycbcra": return color.NYCbCrA{color.YCbCr{uint8(c[0]), uint8(c[1]), uint8(c[2])}, uint8(c[3])}
	case "cmyk": return color.CMYK{uint8(c[0]), uint8(c[1]), uint8(c[2]), uint8(c[3])}
	case "source": return CanonicalSource{[4]uint32{uint32(c[0]), uint32(c[1]), uint32(c[2]), uint32(c[3])}}
	default: panic("unrecognized fixture kind")
	}
}

// observe checks the actual dynamic representation instead of inferring it from RGBA.
func observe(c color.Color) Observation {
	r,g,b,a := c.RGBA()
	o := Observation{RGBA:[4]uint32{r,g,b,a}}
	switch c := c.(type) {
	case color.RGBA: o.Kind="rgba8"; o.Channels=[]uint16{uint16(c.R),uint16(c.G),uint16(c.B),uint16(c.A)}
	case color.RGBA64: o.Kind="rgba16"; o.Channels=[]uint16{c.R,c.G,c.B,c.A}
	case color.NRGBA: o.Kind="nrgba8"; o.Channels=[]uint16{uint16(c.R),uint16(c.G),uint16(c.B),uint16(c.A)}
	case color.NRGBA64: o.Kind="nrgba16"; o.Channels=[]uint16{c.R,c.G,c.B,c.A}
	case color.Alpha: o.Kind="alpha8"; o.Channels=[]uint16{uint16(c.A)}
	case color.Alpha16: o.Kind="alpha16"; o.Channels=[]uint16{c.A}
	case color.Gray: o.Kind="gray8"; o.Channels=[]uint16{uint16(c.Y)}
	case color.Gray16: o.Kind="gray16"; o.Channels=[]uint16{c.Y}
	case color.YCbCr: o.Kind="ycbcr"; o.Channels=[]uint16{uint16(c.Y),uint16(c.Cb),uint16(c.Cr)}
	case color.NYCbCrA: o.Kind="ycbcra"; o.Channels=[]uint16{uint16(c.Y),uint16(c.Cb),uint16(c.Cr),uint16(c.A)}
	case color.CMYK: o.Kind="cmyk"; o.Channels=[]uint16{uint16(c.C),uint16(c.M),uint16(c.Y),uint16(c.K)}
	case CanonicalSource: o.Kind="source"; o.Channels=[]uint16{uint16(r),uint16(g),uint16(b),uint16(a)}
	default: panic("unrecognized converted type")
	}
	return o
}

// next advances a fixed unsigned generator whose state and overflow policy are explicit.
func next(state *uint64) uint64 {
	*state ^= *state << 13
	*state ^= *state >> 7
	*state ^= *state << 17
	return *state
}

// inputs combines full byte-alpha/gray sweeps,16-bit edges and deterministic mixed channels.
func inputs() []Spec {
	result:=[]Spec{}
	edges:=[]uint16{0,1,2,127,128,254,255,256,257,32767,32768,65279,65280,65533,65534,65535}
	state:=uint64(0x913af592d77c3b21)
	for _,kind:=range kinds {
		count:=96
		if kind=="alpha8" || kind=="gray8" { count=256 }
		for i:=0;i<count;i++ {
			s:=Spec{Kind:kind,Channels:[]uint16{}}
			width:=4
			if kind=="alpha8" || kind=="alpha16" || kind=="gray8" || kind=="gray16" {width=1}
			if kind=="ycbcr" {width=3}
			for channel:=0;channel<width;channel++ {
				v:=uint16(next(&state))
				if i<32 {v=edges[(i+channel*5)%len(edges)]}
				if kind=="alpha8" || kind=="gray8" {v=uint16(i)}
				if kind!="rgba16" && kind!="nrgba16" && kind!="alpha16" && kind!="gray16" && kind!="source" {v=uint16(uint8(v))}
				s.Channels=append(s.Channels,v)
			}
			result=append(result,s)
		}
	}
	for _,kind:=range []string{"rgba8","rgba16","nrgba8","nrgba16","source"} {
		for _,a:=range edges {result=append(result,Spec{kind,[]uint16{65535,32768,1,a}})}
	}
	for y:=0;y<256;y++ {result=append(result,Spec{"ycbcr",[]uint16{uint16(y),128,128}})}
	return result
}

// paletteVectors captures empty behavior, alpha distance, first ties and truncated-distance equality.
func paletteVectors() []PaletteVector {
	zero:=Spec{"rgba16",[]uint16{0,0,0,0}}
	sets:=[][]Spec{{},{zero},{Spec{"rgba16",[]uint16{1,1,1,1}},zero},
		{Spec{"gray16",[]uint16{0}},Spec{"alpha16",[]uint16{0}}},
		{Spec{"rgba16",[]uint16{65535,0,0,65535}},Spec{"rgba16",[]uint16{0,65535,0,65535}}}}
	queries:=[]Spec{zero,Spec{"rgba16",[]uint16{32767,32767,0,65535}},Spec{"nrgba8",[]uint16{255,0,0,0}}}
	result:=[]PaletteVector{}
	for _,set:=range sets {for _,query:=range queries {
		palette:=color.Palette{}
		for _,spec:=range set {palette=append(palette,build(spec))}
		v:=PaletteVector{Colors:set,Query:query,Index:palette.Index(build(query))}
		if converted:=palette.Convert(build(query));converted!=nil {o:=observe(converted);v.Converted=&o}
		result=append(result,v)
	}}
	return result
}

// main emits actual package observations; no expected conversion arithmetic is reimplemented here.
func main() {
	vectors:=[]Vector{}
	for _,spec:=range inputs() {
		value:=build(spec)
		v:=Vector{Input:spec,Value:observe(value),Converted:[]Observation{}}
		for _,model:=range models {v.Converted=append(v.Converted,observe(model.Convert(value)))}
		vectors=append(vectors,v)
	}
	triples:=[]RGBVector{}
	edges:=[]uint8{0,1,16,127,128,254,255}
	for _,r:=range edges {for _,g:=range edges {for _,b:=range edges {
		y,cb,cr:=color.RGBToYCbCr(r,g,b)
		c,m,yellow,k:=color.RGBToCMYK(r,g,b)
		rr,gg,bb:=color.YCbCrToRGB(r,g,b)
		triples=append(triples,RGBVector{[3]uint8{r,g,b},[3]uint8{y,cb,cr},[4]uint8{c,m,yellow,k},[3]uint8{rr,gg,bb}})
	}}}
	cmyk:=[]CMYKVector{}
	for _,vector:=range vectors {if vector.Input.Kind=="cmyk" {
		v:=vector.Input.Channels;r,g,b:=color.CMYKToRGB(uint8(v[0]),uint8(v[1]),uint8(v[2]),uint8(v[3]))
		cmyk=append(cmyk,CMYKVector{[4]uint8{uint8(v[0]),uint8(v[1]),uint8(v[2]),uint8(v[3])},[3]uint8{r,g,b}})
	}}
	identity:=color.ModelFunc(func(c color.Color) color.Color {return c})
	absent:=color.ModelFunc(func(c color.Color) color.Color {return nil})
	custom:=build(Spec{"source",[]uint16{1,257,32768,65535}})
	constants:=[]Observation{observe(color.Black),observe(color.White),observe(color.Transparent),observe(color.Opaque)}
	out:=map[string]any{"sdk":runtime.Version(),"kinds":kinds,"model_kinds":kinds[:11],"vectors":vectors,"rgb_conversions":triples,"cmyk_conversions":cmyk,"palettes":paletteVectors(),"constants":constants,"custom_identity":observe(identity.Convert(custom)),"custom_absent":absent.Convert(custom)==nil}
	if err:=json.NewEncoder(os.Stdout).Encode(out);err!=nil {panic(err)}
}
