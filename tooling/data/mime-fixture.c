/** Independent callback/progress trace fixture and actual SIMD foreign-object oracle. */
#include <stdint.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>

/** Actual selected library classifier body, compiled as an independent LLVM object. */
extern uint32_t mime_fixture_classify16(uint64_t, uint64_t, uint8_t);
/** Actual pure packed-word lowercase body, compiled without a replacement implementation. */
extern uint64_t mime_fixture_lower8(uint64_t);

/** Serialized trace digest; process-local I/O state never retains a language pointer. */
static uint64_t trace_digest;
/** Next stable callback invocation identifier. */
static uint64_t event_count;
/** Observable short-writing sink storage. */
static unsigned char sink_data[1048576];
/** Exact number of output bytes accepted by the sink. */
static int64_t sink_size;
/** Negative accepts all bytes; otherwise the exact accepted output budget. */
static int64_t sink_budget;

/** Mix one byte using a stable wrapping digest independent of language representations. */
static void mix_byte(unsigned char value) { trace_digest = (trace_digest ^ value) * UINT64_C(1099511628211); }
/** Mix a length/count in explicit little-endian order. */
static void mix_word(uint64_t value) { for (unsigned i=0;i<8;i++) mix_byte((unsigned char)(value>>(8*i))); }
/** Copy an observed span into the trace digest before the borrowed pointer expires. */
static void mix_span(int64_t address,int64_t count) { const unsigned char *data=(const unsigned char *)(uintptr_t)address;for(int64_t i=0;i<count;i++)mix_byte(data[i]); }
/** Start one serialized trace; all language-owned pointers remain borrowed only for each call. */
int64_t mime_fixture_reset(int64_t budget) { trace_digest=UINT64_C(14695981039346656037);event_count=0;sink_size=0;sink_budget=budget;return 0; }
/** Observe a lowercase label and complete decoded callback input, returning its stable ordinal. */
int64_t mime_fixture_event(int64_t label,int64_t label_size,int64_t input,int64_t input_size) { uint64_t id=event_count++;mix_byte(1);mix_word(id);mix_word((uint64_t)label_size);mix_span(label,label_size);mix_word((uint64_t)input_size);mix_span(input,input_size);return (int64_t)id; }
/** Observe one positive-capacity pull result; status zero/more, one/EOF, two/failure. */
int64_t mime_fixture_read(int64_t id,int64_t request,int64_t data,int64_t count,int64_t status) { mix_byte(2);mix_word((uint64_t)id);mix_word((uint64_t)request);mix_word((uint64_t)count);mix_word((uint64_t)status);mix_span(data,count);return 0; }
/** Observe an optional whole-reader copy result with the same output/error vocabulary. */
int64_t mime_fixture_copy(int64_t id,int64_t data,int64_t count,int64_t status) { mix_byte(3);mix_word((uint64_t)id);mix_word((uint64_t)count);mix_word((uint64_t)status);mix_span(data,count);return 0; }
/** Read the complete trace digest for an independently frozen expected comparison. */
uint64_t mime_fixture_digest(int64_t ignored) { (void)ignored;return trace_digest; }
/** Accept an exact prefix, then report a later stable error when no positive progress remains. */
int64_t mime_fixture_write(int64_t address,int64_t count) { if(count<0 || count>(int64_t)sizeof sink_data-sink_size)return -2;int64_t n=count;if(sink_budget>=0 && n>sink_budget-sink_size)n=sink_budget-sink_size;if(n<0)n=0;if(count>0 && n==0)return -1;if(n>0)memcpy(sink_data+sink_size,(void *)(uintptr_t)address,(size_t)n);sink_size+=n;return n; }
/** Observe the exact accepted prefix length after success or failure. */
int64_t mime_fixture_sink_size(int64_t ignored) { (void)ignored;return sink_size; }
/** Observe an initialized accepted byte after checking the complete sink size. */
uint8_t mime_fixture_sink_byte(int64_t index) { return index>=0 && index<sink_size?sink_data[index]:0; }

/** Scalar transport-special predicate independent of the vector mask composition. */
static int special(unsigned byte) { const char *characters="()<>@,;:\\\"/[]?=";return byte<128 && strchr(characters,(int)byte)!=NULL && byte!=0; }
/** Scalar class predicate used only as the independent foreign-object oracle. */
static int good(unsigned byte,unsigned mode) { int token=byte>32 && byte<127 && !special(byte);if(mode==1)return byte==9 || (byte>=32 && byte<=126);if(mode==2)return byte>=33 && byte<=126 && byte!=61 && byte!=63 && byte!=95;if(mode==3)return token && byte!=42 && byte!=39 && byte!=37;if(mode==4)return byte!=34 && byte!=92 && byte!=13 && byte!=10;if(mode==5)return byte<128;return token; }
/** Execute every byte in every physical lane for all selected classes and packed-word transforms. */
int32_t mime_fixture_kernel_check(int64_t ignored) {
 (void)ignored;
 for(unsigned mode=0;mode<6;mode++)for(unsigned lane=0;lane<16;lane++)for(unsigned value=0;value<256;value++){
  unsigned char bytes[16];memset(bytes,'A',16);bytes[lane]=(unsigned char)value;uint64_t lo=0,hi=0;uint32_t expected=0;
  for(unsigned i=0;i<16;i++){if(!good(bytes[i],mode))expected|=UINT32_C(1)<<i;if(i<8)lo|=(uint64_t)bytes[i]<<(8*i);else hi|=(uint64_t)bytes[i]<<(8*(i-8));}
  if(mime_fixture_classify16(lo,hi,(uint8_t)mode)!=expected){fprintf(stderr,"MIME classifier mismatch mode=%u lane=%u byte=%u\n",mode,lane,value);return 1;}
 }
 for(unsigned lane=0;lane<8;lane++)for(unsigned value=0;value<256;value++){
  uint64_t input=0,expected=0;
  for(unsigned i=0;i<8;i++){unsigned byte=i==lane?value:(i*23+53)%256;input|=(uint64_t)byte<<(i*8);if(byte>='A' && byte<='Z')byte+=32;expected|=(uint64_t)byte<<(i*8);}
  if(mime_fixture_lower8(input)!=expected){fprintf(stderr,"MIME lowercase word mismatch lane=%u byte=%u\n",lane,value);return 2;}
 }
 return 0;
}
