// Command live-reference captures live callback and selected-owner behavior independently.
package main

import (
	"encoding/json"
	"image/color"
	"os"
)

// LiveSource observes a caller-owned byte sequence whenever canonical channels are requested.
type LiveSource struct { Bytes []byte }

// RGBA expands the current owner contents, retaining live alias behavior between calls.
func (c *LiveSource) RGBA() (uint32,uint32,uint32,uint32) {
	return uint32(c.Bytes[0])*257,uint32(c.Bytes[1])*257,uint32(c.Bytes[2])*257,uint32(c.Bytes[3])*257
}

// rgba records a complete canonical observation without replacing the original capability.
func rgba(c color.Color) [4]uint32 { r,g,b,a:=c.RGBA();return [4]uint32{r,g,b,a} }

// makeOwners returns source/model/palette aliases after their construction scope ends.
func makeOwners(input []byte) (*LiveSource,color.Model,color.Palette,color.Color) {
	source:=&LiveSource{input}
	model:=color.ModelFunc(func(unused color.Color) color.Color {
		r,g,b,a:=source.RGBA()
		return color.RGBA64{uint16(r),uint16(g),uint16(b),uint16(a)}
	})
	palette:=color.Palette{source,color.Gray{128}}
	return source,model,palette,palette.Convert(color.Black)
}

// main captures observations before and after serialized mutation of the shared owner.
func main() {
	input:=[]byte{0,0,0,255}
	source,model,palette,selected:=makeOwners(input)
	before:=rgba(source)
	beforeIndex:=palette.Index(color.Black)
	input[0],input[1],input[2]=255,255,255
	modeled:=model.Convert(color.Black)
	_,modelIsRGBA16:=modeled.(color.RGBA64)
	result:=map[string]any{"before":before,"before_index":beforeIndex,"after":rgba(source),
		"after_index":palette.Index(color.Black),"selected_after":rgba(selected),
		"selected_identity_retained":selected==source,"modeled_after":rgba(modeled),"model_is_rgba16":modelIsRGBA16}
	if err:=json.NewEncoder(os.Stdout).Encode(result);err!=nil {panic(err)}
}
