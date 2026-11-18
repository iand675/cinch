#!/usr/bin/env python3
"""Reproduce scalar, SIMD and fused UTF8 scan experiments from the retained validated SWAR source."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]


def simd16(source: str) -> str:
    """Add a proved sixteen-byte read-only SIMD path without changing scalar decoding or mutation."""
    prim=' with an order-independent SIMD sign-bit mask.\n  prim ascii-sixteen :: I64 -> Bool where\n    unsafe llvm {\n      %p = inttoptr i64 %0 to ptr\n      %bytes = load <16 x i8>, ptr %p, align 1\n      %high = icmp slt <16 x i8> %bytes, zeroinitializer\n      %mask = bitcast <16 x i1> %high to i16\n      %yes = icmp eq i16 %mask, 0\n      %result = zext i1 %yes to i8\n      ret i8 %result\n    }\n\n'
    source=source.replace('  -- | Decode a low-byte-first','  -- | Recognize sixteen proved source bytes'+prim+'  -- | Decode a low-byte-first')
    source=source.replace('    if ge-u64 remaining 8 then let address = wrapping-add-i64 base (wrapping-u64-to-i64 at) in let block = load-eight address in\n      if ascii-eight block then invalid-from','    if ge-u64 remaining 16 then let address = wrapping-add-i64 base (wrapping-u64-to-i64 at) in\n      if ascii-sixteen address then invalid-from base size (wrapping-add-u64 at 16) else invalid-scalar base size at\n    else if ge-u64 remaining 8 then let address = wrapping-add-i64 base (wrapping-u64-to-i64 at) in let block = load-eight address in\n      if ascii-eight block then invalid-from')
    source=source.replace('    if ge-u64 remaining 8 then let address = wrapping-add-i64 base (wrapping-u64-to-i64 at) in let block = load-eight address in\n      if ascii-eight block then count-from','    if ge-u64 remaining 16 then let address = wrapping-add-i64 base (wrapping-u64-to-i64 at) in\n      if ascii-sixteen address then count-from base size (wrapping-add-u64 at 16) (wrapping-add-u64 count 16) else count-scalar base size at count\n    else if ge-u64 remaining 8 then let address = wrapping-add-i64 base (wrapping-u64-to-i64 at) in let block = load-eight address in\n      if ascii-eight block then count-from')
    return source


def fused(s: str) -> str:
    """Fuse private machine scan arithmetic to avoid imported helper calls."""
    helpers=''
    for name,op in [('lt','ult'),('ge','uge'),('gt','ugt'),('eq','eq')]:
     helpers+=f'''  -- | Compare private machine scan counters directly without an imported provider call.
  prim scan-{name} :: U64 -> U64 -> Bool where
    llvm {{
      %yes = icmp {op} i64 %0, %1
      %result = zext i1 %yes to i8
      ret i8 %result
    }}

'''
    for name,op in [('add','add'),('sub','sub')]:
     helpers+=f'''  -- | Advance or measure proved private machine scan positions with wrapping word arithmetic.
  prim scan-{name} :: U64 -> U64 -> U64 where
    llvm {{
      %result = {op} i64 %0, %1
      ret i64 %result
    }}

'''
    helpers+='''  -- | Form one private pointer from its retained source base and proved bounded byte offset.
  prim scan-address :: I64 -> U64 -> I64 where
    llvm {
      %result = add i64 %0, %1
      ret i64 %result
    }

'''
    for old,new in [('lt-u64','scan-lt'),('ge-u64','scan-ge'),('gt-u64','scan-gt'),('eq-u64','scan-eq'),('wrapping-add-u64','scan-add'),('wrapping-sub-u64','scan-sub')]:s=s.replace(old,new)
    s=s.replace('wrapping-add-i64 base (wrapping-u64-to-i64 at)','scan-address base at')
    s=s.replace('  -- | Read one proved source byte',helpers+'  -- | Read one proved source byte')
    a=s.index('  -- | Observe a packed consumed width')
    b=s.index('  -- | Convert one verified packed result',a)
    s=s[:a]+'''  -- | Observe a packed consumed width without intermediate provider shifts or conversions.
  prim packed-width :: U32 -> U64 where
    llvm {
      %shifted = lshr i32 %0, 21
      %width = and i32 %shifted, 7
      %result = zext i32 %width to i64
      ret i64 %result
    }

  -- | Recognize a packed valid status directly, excluding the independent incomplete-prefix flag.
  prim packed-valid :: U32 -> Bool where
    llvm {
      %status = and i32 %0, 50331648
      %valid = icmp eq i32 %status, 16777216
      %result = zext i1 %valid to i8
      ret i8 %result
    }

'''+s[b:]
    return s


def minimal(s: str) -> str:
    """Compute only bulk width/validity and classify obvious one-byte cases early."""
    prim='''  -- | Classify one leading byte without inspecting unrelated bytes; zero requests multibyte validation.
  prim scan-simple :: U8 -> U8 where
    llvm {
      %ascii = icmp ult i8 %0, 128
      %lower = icmp uge i8 %0, 194
      %upper = icmp ule i8 %0, 244
      %multi = and i1 %lower, %upper
      %nonascii = select i1 %multi, i8 0, i8 1
      %result = select i1 %ascii, i8 9, i8 %nonascii
      ret i8 %result
    }

  -- | Recognize the complete one-byte result from a simple lead classification.
  prim scan-ready :: U8 -> Bool where
    llvm {
      %yes = icmp ne i8 %0, 0
      %result = zext i1 %yes to i8
      ret i8 %result
    }

  -- | Validate a complete four-byte lookahead, returning only consumed width and validity.
  -- SWAR continuation masks avoid computing code points or incomplete-prefix metadata during bulk scans.
  prim scan-four :: U32 -> U8 where
    llvm {
      %first = and i32 %0, 255
      %second-shift = lshr i32 %0, 8
      %second = and i32 %second-shift, 255
      %lead-low = icmp uge i32 %first, 194
      %lead-high = icmp ule i32 %first, 244
      %lead = and i1 %lead-low, %lead-high
      %two = icmp ult i32 %first, 224
      %three = icmp ult i32 %first, 240
      %mask2 = and i32 %0, 49152
      %mask3 = and i32 %0, 12632064
      %mask4 = and i32 %0, 3233857536
      %cont2 = icmp eq i32 %mask2, 32768
      %cont3 = icmp eq i32 %mask3, 8421376
      %cont4 = icmp eq i32 %mask4, 2155905024
      %e0 = icmp eq i32 %first, 224
      %ed = icmp eq i32 %first, 237
      %f0 = icmp eq i32 %first, 240
      %f4 = icmp eq i32 %first, 244
      %atleast-a0 = icmp uge i32 %second, 160
      %below-a0 = icmp ult i32 %second, 160
      %atleast-90 = icmp uge i32 %second, 144
      %below-90 = icmp ult i32 %second, 144
      %e0valid = select i1 %e0, i1 %atleast-a0, i1 true
      %edvalid = select i1 %ed, i1 %below-a0, i1 true
      %f0valid = select i1 %f0, i1 %atleast-90, i1 true
      %f4valid = select i1 %f4, i1 %below-90, i1 true
      %special3 = and i1 %e0valid, %edvalid
      %special4 = and i1 %f0valid, %f4valid
      %valid3 = and i1 %cont3, %special3
      %valid4 = and i1 %cont4, %special4
      %wide-valid = select i1 %three, i1 %valid3, i1 %valid4
      %tail-valid = select i1 %two, i1 %cont2, i1 %wide-valid
      %valid = and i1 %lead, %tail-valid
      %wide-width = select i1 %three, i8 3, i8 4
      %width = select i1 %two, i8 2, i8 %wide-width
      %consumed = select i1 %valid, i8 %width, i8 1
      %flag = select i1 %valid, i8 8, i8 0
      %result = or i8 %consumed, %flag
      ret i8 %result
    }

  -- | Convert an exact tail decoder result to the same minimal scan representation.
  prim scan-from-packed :: U32 -> U8 where
    llvm {
      %shifted = lshr i32 %0, 21
      %width = and i32 %shifted, 7
      %status = and i32 %0, 50331648
      %valid = icmp eq i32 %status, 16777216
      %flag = select i1 %valid, i32 8, i32 0
      %combined = or i32 %width, %flag
      %result = trunc i32 %combined to i8
      ret i8 %result
    }

  -- | Observe the consumed scan width without imported arithmetic or conversion calls.
  prim scan-width :: U8 -> U64 where
    llvm {
      %width = and i8 %0, 7
      %result = zext i8 %width to i64
      ret i64 %result
    }

  -- | Recognize one valid scan result without constructing a decoded scalar.
  prim scan-valid :: U8 -> Bool where
    llvm {
      %flag = and i8 %0, 8
      %valid = icmp ne i8 %flag, 0
      %result = zext i1 %valid to i8
      ret i8 %result
    }

  -- | Scan one proved nonempty suffix with early ASCII/invalid-lead classification and exact lookahead.
  scan-raw :: I64 -> U64 -> U64 -> U8
  scan-raw base size at = let address = scan-address base at in let first = load-one address in
    let simple = scan-simple first in if scan-ready simple then simple else
      let remaining = scan-sub size at in
      if scan-ge remaining 4 then let word = load-four address in scan-four word
      else let word = tail-word address remaining in let available = wrapping-u64-to-u8 remaining in
        let packed = decode-word word available in scan-from-packed packed

'''
    s=s.replace('  -- | Observe a packed consumed width',prim+'  -- | Observe a packed consumed width')
    s=s.replace('invalid-scalar base size at = let packed = decode-raw base size at in\n    if packed-valid packed then invalid-from base size (scan-add at (packed-width packed)) else at','invalid-scalar base size at = let packed = scan-raw base size at in\n    if scan-valid packed then invalid-from base size (scan-add at (scan-width packed)) else at')
    s=s.replace('count-scalar base size at count = let packed = decode-raw base size at in\n    count-from base size (scan-add at (packed-width packed)) (scan-add count 1)','count-scalar base size at count = let packed = scan-raw base size at in\n    count-from base size (scan-add at (scan-width packed)) (scan-add count 1)')
    s=s.replace('''    let packed = decode-raw base size at in
    if packed-valid packed then at else invalid-end''','''    let packed = scan-raw base size at in
    if scan-valid packed then at else invalid-end''')
    assert 'invalid-scalar base size at = let packed = scan-raw' in s
    assert 'count-scalar base size at count = let packed = scan-raw' in s
    return s


def selected(source: str) -> str:
    """Clarify the SIMD/bulk-scan and plausible-lead contracts without changing executable tokens."""
    return source.replace('Complete ASCII blocks use SWAR;','Complete ASCII blocks use SIMD and SWAR;').replace('  -- | Validate a complete four-byte lookahead, returning only consumed width and validity.','  -- | Validate a plausible multibyte lead with complete four-byte lookahead, returning only width and validity.')


def main() -> None:
    """Keep exact source hashes for all experiment variants and their validated baseline epoch."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline',type=Path,default=ROOT/'tooling/data/utf8-swar-baseline.cinch')
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args();source=args.baseline.read_text()
    variants={'swar':source,'scalar':source.replace('if ge-u64 remaining 8 then','if False then')}
    variants['simd16']=simd16(source)
    variants['simd16-fused']=fused(variants['simd16'])
    variants['simd16-minimal']=minimal(variants['simd16-fused'])
    variants['simd16-selected']=selected(variants['simd16-minimal'])
    args.output.mkdir(parents=True,exist_ok=True)
    manifest={'baseline_commit':'be3fbdaf338bfeba507676d228931a0477702a9b','sources':{}}
    for name,text in variants.items():
        (args.output/(name+'.cinch')).write_text(text)
        manifest['sources'][name]=hashlib.sha256(text.encode()).hexdigest()
    (args.output/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')


if __name__=='__main__':main()
