#!/usr/bin/env python3
"""Qualify the actual production byte-mask LLVM bodies independently and compile their foreign-target objects."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess

ROOT=Path(__file__).resolve().parents[1]
SOURCE=ROOT/'seed/prelude/Effect/Buffered/Internal/Scan.cinch'
C_SOURCE=r'''/** Independent numeric-byte expectations for the production SIMD masks. */
#include <stdint.h>
#include <stdio.h>
extern uint32_t byte_mask(uint64_t,uint8_t);
extern uint32_t nonspace_mask(uint64_t);
extern uint32_t word_end_mask(uint64_t);
/** The complete ASCII whitespace set, independent of vector comparisons. */
static int white(unsigned byte) { return byte==32 || (byte>=9 && byte<=13); }
/** Compute exact lane bits by scalar numeric shifts rather than compiler vector layout. */
__attribute__((optnone,noinline))
static uint32_t expected(uint64_t word,unsigned needle,unsigned kind) {
    uint32_t mask=256;
    for(unsigned lane=0;lane<8;lane++) {
        unsigned byte=(word>>(lane*8))&255;
        int hit=kind==0 ? byte==needle : kind==1 ? !white(byte) : white(byte)||byte>=128;
        if(hit)mask|=1u<<lane;
    }
    return mask;
}
/** Preserve the complete failing word and lane metadata with a normalized nonzero status. */
static int failure(unsigned kind,unsigned lane,unsigned byte,unsigned needle,uint64_t word,uint32_t actual,uint32_t wanted) {
    fprintf(stderr,"kind=%u lane=%u byte=%u needle=%u word=%016llx actual=%u expected=%u\n",kind,lane,byte,needle,(unsigned long long)word,actual,wanted);
    return 1;
}
/** Every byte value in every lane, every byte needle and all ASCII/non-ASCII boundaries. */
int main(void) {
    unsigned checks=0;
    for(unsigned lane=0;lane<8;lane++) for(unsigned byte=0;byte<256;byte++) {
        uint64_t word=(UINT64_C(0x514109200d7f8051)&~(UINT64_C(255)<<(8*lane)))|((uint64_t)byte<<(8*lane));
        uint32_t actual=nonspace_mask(word),wanted=expected(word,0,1);
        if(actual!=wanted)return failure(1,lane,byte,0,word,actual,wanted);
        actual=word_end_mask(word);wanted=expected(word,0,2);
        if(actual!=wanted)return failure(2,lane,byte,0,word,actual,wanted);
        for(unsigned needle=0;needle<256;needle++) {
            actual=byte_mask(word,needle);wanted=expected(word,needle,0);
            if(actual!=wanted)return failure(0,lane,byte,needle,word,actual,wanted);
            checks++;
        }
    }
    for(unsigned bits=0;bits<256;bits++) {
        uint64_t word=0;
        for(unsigned lane=0;lane<8;lane++)word|=(uint64_t)((bits>>lane)&1 ? 32:65)<<(lane*8);
        uint32_t actual=nonspace_mask(word),wanted=256u|(~bits&255u);
        if(actual!=wanted)return failure(3,bits,0,0,word,actual,wanted);
        actual=word_end_mask(word);wanted=256u|bits;
        if(actual!=wanted)return failure(4,bits,0,0,word,actual,wanted);
    }
    printf("%u byte-mask cases, 4096 whitespace/word boundaries, 256 complete lane masks\n",checks);
    return 0;
}
'''


def digest(path:Path)->str:
    """Identify the exact source, emitted body, executable and object bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def invoke(command:list[str],artifacts:Path,name:str)->subprocess.CompletedProcess:
    """Preserve every diagnostic and stop immediately on a compiler or functional failure."""
    result=subprocess.run(command,capture_output=True,text=True)
    (artifacts/(name+'.stdout')).write_text(result.stdout)
    (artifacts/(name+'.stderr')).write_text(result.stderr)
    if result.returncode:raise RuntimeError(str(command)+'\n'+result.stdout+result.stderr)
    return result


def main()->None:
    """Compile only source-extracted production arithmetic; this is not a full frontend or foreign execution claim."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--cpu',type=int,default=2)
    args=parser.parse_args();os.sched_setaffinity(0,{args.cpu})
    artifacts=args.artifacts.resolve();artifacts.mkdir(parents=True,exist_ok=True)
    text=SOURCE.read_text();matches=dict(re.findall(r'^  prim ([\w-]+) ::[^\n]+where\n    llvm \{(.*?)^    \}',text,re.M|re.S))
    signatures={'byte-mask':'i64 %0, i8 %1','nonspace-mask':'i64 %0','word-end-mask':'i64 %0'}
    if set(matches)!=set(signatures):raise ValueError('production primitive set changed')
    llvm=artifacts/'production.ll';llvm.write_text('\n'.join('define i32 @'+name.replace('-','_')+'('+signatures[name]+') {\n'+matches[name]+'\n}\n' for name in signatures))
    source=artifacts/'expected.c';source.write_text(C_SOURCE)
    rows=[]
    for opt in ['0','2']:
        executable=artifacts/('masks-o'+opt)
        command=['clang','-O'+opt,str(source),str(llvm),'-o',str(executable)]
        invoke(command,artifacts,'build-o'+opt);result=invoke([str(executable)],artifacts,'run-o'+opt)
        rows.append({'kind':'host functional math','opt_level':opt,'command':command,'executable_sha256':digest(executable),'stdout':result.stdout})
    for target in ['wasm32-unknown-wasi','aarch64-unknown-linux-gnu','powerpc64-unknown-linux-gnu']:
        output=artifacts/(target+'.o');command=['clang','--target='+target,'-O2','-c',str(llvm),'-o',str(output)]
        invoke(command,artifacts,'object-'+target)
        rows.append({'kind':'foreign object compilation only','target':target,'command':command,'object_sha256':digest(output)})
    clang=Path(shutil.which('clang')).resolve()
    version=invoke([str(clang),'--version'],artifacts,'clang-version').stdout
    report={'source_sha256':digest(SOURCE),'llvm_sha256':digest(llvm),'independent_c_sha256':digest(source),'runner_sha256':digest(Path(__file__)),'clang_path':str(clang),'clang_sha256':digest(clang),'clang_version':version,'cpu':args.cpu,'rows':rows,'all_passed':True,'scope':'Actual three production primitive bodies. Scalar expectations are deliberately unoptimized. Checked trailing-zero routing, public owner bounds/alias holds, complete frontend ABI and foreign execution are separate gates. No runtime or Cinc leak-check behavior is changed.'}
    (artifacts/'receipt.json').write_text(json.dumps(report,indent=2)+'\n')
    print('Production masks: host O0/O2 and wasm/ARM/PPC objects passed')


if __name__=='__main__':main()
