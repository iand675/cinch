/** Independent scalar controls for the actual packed color arithmetic kernels. */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

/** Actual checked byte-expansion body extracted from the selected source. */
extern uint64_t color_expand8(uint64_t);
/** Actual checked premultiplication body extracted from the selected source. */
extern uint64_t color_premultiply(uint64_t);
/** Actual checked inverse-alpha body extracted from the selected source. */
extern uint64_t color_unpremultiply(uint64_t);
/** Actual checked grayscale accumulator extracted from the selected source. */
extern uint32_t color_gray_sum(uint64_t);
/** Actual checked full-precision chroma decoder extracted from the selected source. */
extern uint64_t color_ycbcr16(uint64_t);
/** Actual checked fixed-point chroma encoder extracted from the selected source. */
extern uint64_t color_rgb_ycbcr(uint64_t);
/** Actual checked RGB-to-ink body extracted from the selected source. */
extern uint64_t color_rgb_cmyk(uint64_t);
/** Actual checked full-precision ink decoder extracted from the selected source. */
extern uint64_t color_cmyk16(uint64_t);
/** Actual checked four-channel palette distance extracted from the selected source. */
extern uint32_t color_distance4(uint64_t,uint64_t);

/** Pack bounded channels using scalar arithmetic independent of vector lane order. */
static uint64_t pack16(uint64_t r,uint64_t g,uint64_t b,uint64_t a) {
 return (r&65535) | ((g&65535)<<16) | ((b&65535)<<32) | ((a&65535)<<48);
}
/** Observe one logical sixteen-bit channel without accessing memory. */
static uint64_t lane(uint64_t word,unsigned index) { return (word>>(16*index))&65535; }
/** Clip signed full-precision arithmetic to a channel width. */
static uint64_t clip(int64_t value,int64_t maximum) { return value<0?0:(value>maximum?(uint64_t)maximum:(uint64_t)value); }
/** Divide with mathematical floor rather than implementation-defined signed shifts. */
static int64_t floor_div(int64_t value,int64_t divisor) { return value>=0?value/divisor:-((-value+divisor-1)/divisor); }
/** Stop at the first exact kernel discrepancy, retaining the specific input and values. */
static void equal(const char *name,uint64_t input,uint64_t actual,uint64_t expected) {
 if(actual!=expected) {fprintf(stderr,"color-kernel %s input=%llu actual=%llu expected=%llu\n",name,(unsigned long long)input,(unsigned long long)actual,(unsigned long long)expected);exit(1);}
}
/** Independently interpret all packed channels with wide unsigned products. */
static void packed_controls(uint64_t word,uint64_t other) {
 uint64_t a=lane(word,3),r=lane(word,0),g=lane(word,1),b=lane(word,2);
 uint64_t expanded=pack16((word&255)*257,((word>>8)&255)*257,((word>>16)&255)*257,((word>>24)&255)*257);
 equal("expand8",word,color_expand8(word),expanded);
 equal("premultiply",word,color_premultiply(word),pack16(r*a/65535,g*a/65535,b*a/65535,a));
 uint64_t inverse=a?pack16(r*65535/a,g*65535/a,b*65535/a,a):word;
 equal("unpremultiply",word,color_unpremultiply(word),inverse);
 equal("gray",word,color_gray_sum(word),19595*r+38470*g+7471*b+32768);
 uint64_t sum=0;
 for(unsigned i=0;i<4;i++) {int64_t difference=(int64_t)lane(word,i)-(int64_t)lane(other,i);sum+=(uint64_t)(difference*difference)/4;}
 equal("distance4",word,color_distance4(word,other),sum);
}
/** Independently calculate chroma conversions with signed wide arithmetic and saturation. */
static void chroma_controls(uint64_t word) {
 int64_t r=word&255,g=(word>>8)&255,b=(word>>16)&255;
 uint64_t y=clip(floor_div(19595*r+38470*g+7471*b+32768,65536),255);
 uint64_t cb=clip(floor_div(-11056*r-21712*g+32768*b+8421376,65536),255);
 uint64_t cr=clip(floor_div(32768*r-27440*g-5328*b+8421376,65536),255);
 equal("rgb_ycbcr",word,color_rgb_ycbcr(word),y+(cb<<8)+(cr<<16));
 int64_t yy=r*65793,blue=g-128,red=b-128;
 uint64_t rr=clip(floor_div(yy+91881*red,256),65535);
 uint64_t gg=clip(floor_div(yy-22554*blue-46802*red,256),65535);
 uint64_t bb=clip(floor_div(yy+116130*blue,256),65535);
 equal("ycbcr16",word,color_ycbcr16(word),pack16(rr,gg,bb,65535));
 uint64_t white=(uint64_t)r;if(white<(uint64_t)g)white=g;if(white<(uint64_t)b)white=b;
 uint64_t inks=white?((white-r)*255/white)+(((white-g)*255/white)<<8)+(((white-b)*255/white)<<16)+((255-white)<<24):UINT64_C(4278190080);
 equal("rgb_cmyk",word,color_rgb_cmyk(word),inks);
}
/** Independently calculate full-precision CMYK output before any byte truncation. */
static void ink_controls(uint64_t word) {
 uint64_t c=word&255,m=(word>>8)&255,y=(word>>16)&255,k=(word>>24)&255;
 uint64_t white=65535-k*257;
 equal("cmyk16",word,color_cmyk16(word),pack16((65535-c*257)*white/65535,(65535-m*257)*white/65535,(65535-y*257)*white/65535,65535));
}
/** Exercise all sixteen-bit alpha coordinates, all byte triples and a full sixteen-level ink cube. */
int main(void) {
 for(uint64_t i=0;i<65536;i++) {
  uint64_t word=pack16(i,(i*997)&65535,65535-i,(i*53)&65535);
  uint64_t other=pack16(65535-i,i,(i*13)&65535,65535);
  packed_controls(word,other);
  packed_controls(pack16(65535,i,1,i),pack16(0,65535,65535,0));
 }
 for(uint64_t value=0;value<UINT64_C(16777216);value++)chroma_controls(value);
 for(uint64_t i=0;i<65536;i++) {
  uint64_t word=((i&15)*17)+((((i>>4)&15)*17)<<8)+((((i>>8)&15)*17)<<16)+((((i>>12)&15)*17)<<24);
  ink_controls(word);
 }
 puts("color-kernels packed=131072 chroma-triples=16777216 ink-cube=65536 passed");
 return 0;
}
