#!/usr/bin/env python3
"""Build an isolated source-local U32 table-routing candidate without changing any factual Unicode words."""
from __future__ import annotations
import argparse
import hashlib
import json
import re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]


def local_words(source: str, class_only: bool = False) -> str:
    """Provide six exact private word operations backed by always-inlined Data.Machine primitives."""
    if '  table-add ::' in source:return source
    definitions=''
    for name,fn in [('add','wrapping-add-u32'),('sub','wrapping-sub-u32'),('and','bit-and-u32')]:
        definitions+=f'''  -- | Apply exact private 32-bit {name} semantics.
  table-{name} :: U32 -> U32 -> U32
  table-{name} a b = {fn} a b

'''
    definitions+='''  -- | Shift a private 32-bit table word with wrapping machine shift semantics.
  table-shr :: U32 -> U32 -> U32
  table-shr a b = wrapping-shr-u32 a b

'''
    for name,fn in [('lt','lt-u32'),('le','le-u32')]:
        definitions+=f'''  -- | Compare private unsigned table positions without allocating.
  table-{name} :: U32 -> U32 -> Bool
  table-{name} a b = {fn} a b

'''
    replacements=[('wrapping-add-u32','table-add'),('wrapping-sub-u32','table-sub'),('wrapping-shr-u32','table-shr'),('bit-and-u32','table-and'),('lt-u32','table-lt'),('le-u32','table-le')]
    chunks=re.split(r'(?=^  -- \|)',source,flags=re.MULTILINE) if class_only else [source]
    result=[]
    for chunk in chunks:
        name=re.search(r'^  ([a-z][a-z0-9-]+) ::',chunk,re.MULTILINE)
        if not class_only or name and (name[1]=='class-search' or name[1].startswith('unicode-data-class')):
            for old,new in replacements:chunk=chunk.replace(old,new)
        result.append(chunk)
    source=''.join(result)
    source=source.replace('let middle = table-shr (table-add low high) 1 in', 'let total = table-add low high in let middle = table-shr total 1 in')
    source=source.replace('else class-search (table-add middle 1) high byte', 'else let next = table-add middle 1 in class-search next high byte')
    source=source.replace('let index = class-search start (table-add start count) low in', 'let limit = table-add start count in let index = class-search start limit low in')
    at=source.index('  -- | Number of complete preserved simple-case range records.') if '  -- | Number of complete preserved simple-case range records.' in source else source.index('  -- | Number of complete preserved simple-case')
    return source[:at]+definitions+source[at:]


def main() -> None:
    """Retain exact baseline/candidate identities for source-level codegen comparison."""
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--output',type=Path,required=True);parser.add_argument('--baseline',type=Path);parser.add_argument('--class-only',action='store_true');args=parser.parse_args()
    if args.baseline:
        source=args.baseline.read_text()
    else:
        from generate_unicode_tables import generated,model
        data=json.loads((ROOT/'tooling/data/unicode17-tables.json').read_text())
        source=generated(model(data),inline_class_ops=False)
    candidate=local_words(source,args.class_only)
    args.output.mkdir(parents=True,exist_ok=True);target=args.output/'Data.Unicode.Tables.cinch';target.write_text(candidate)
    manifest={'baseline_sha256':hashlib.sha256(source.encode()).hexdigest(),'candidate_sha256':hashlib.sha256(candidate.encode()).hexdigest(),'factual_words_changed':False,'pure_scalar_operations':6,'class_only':args.class_only}
    (args.output/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')


if __name__=='__main__':main()
