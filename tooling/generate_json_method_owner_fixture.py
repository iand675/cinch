#!/usr/bin/env python3
"""Combine unchanged directional and semantic-owner controls against one current source snapshot."""
from pathlib import Path
import hashlib
import re
root=Path(__file__).resolve().parents[1]
fixtures=root/'compiler/tests/fixtures'
category=fixtures/'json-codec-method-categories.cinch';metadata=fixtures/'json-codec-method-metadata.cinch'
def blocks(path):
    text=path.read_text();start=text.index('  -- |')
    return re.findall(r'  -- \|.*?(?=\n  -- \||\Z)',text[start:],re.S)
def declared(block):
    found=re.search(r'^  (?:prim )?([a-z][a-z0-9-]*)\s*::',block,re.M)
    assert found,block
    return found.group(1)
parts=['-- | Directional method categories and semantic byte ownership share one bounded real consumer.\nmodule Main where\n  import Text.Json.Codec\n  import Text.Json.Value\n  import Data.Machine\n\n']
seen=set()
for path,prefix in [(category,'categories'),(metadata,'metadata')]:
    for block in blocks(path):
        name=declared(block)
        if name in seen:
            assert name in {'runtime-size','input','same'}
            continue
        if name=='main':
            block=re.sub(r'(?m)^  main(?=\s*::|\s*=)',f'  {prefix}-status',block)
        else:seen.add(name)
        parts.append(block.rstrip()+'\n\n')
parts.append('  -- | Preserve each original nonzero group status, with metadata groups offset by one hundred.\n  main :: Int\n  main = let categories = categories-status in if categories == 0 then let metadata = metadata-status in if metadata == 0 then 0 else 100 + metadata else categories\n')
output=fixtures/'json-codec-method-owner-control.cinch';output.write_text(''.join(parts))
print(hashlib.sha256(output.read_bytes()).hexdigest())
