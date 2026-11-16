#!/usr/bin/env python3
"""Generate a pinned SDK reference consumer for every exported Unicode range-table variable."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]


def generated() -> str:
    """Use factual public identifiers only; returned range fields are observed by the SDK itself."""
    data=json.loads((ROOT/'tooling/data/unicode17-tables.json').read_text())
    source='''// Generate independent current Unicode exported-table observations.
package main

import("encoding/json";"os";"runtime";"unicode")

// row preserves every public exported table's complete original metadata.
type row struct {Name string `json:"name"`;Latin int `json:"latin_offset"`;Small [][]uint32 `json:"r16"`;Wide [][]uint32 `json:"r32"`}

// observe reads the SDK's actual bounds, stride and Latin prefix metadata.
func observe(name string,table *unicode.RangeTable) row {
 small,wide:=[][]uint32{},[][]uint32{}
 for _,span:=range table.R16 {small=append(small,[]uint32{uint32(span.Lo),uint32(span.Hi),uint32(span.Stride)})}
 for _,span:=range table.R32 {wide=append(wide,[]uint32{span.Lo,span.Hi,span.Stride})}
 return row{name,table.LatinOffset,small,wide}
}

// main rejects edition drift and observes all current exported tables in sorted identifier order.
func main(){
 if runtime.Version()!="go1.27.4"||unicode.Version!="17.0.0" {panic("unexpected reference edition")}
 tables:=[]row{
'''
    source+=''.join(f'  observe({json.dumps(name)},unicode.{name}),\n' for name in sorted(data['exported_aliases']))
    source+=''' }
 encoder:=json.NewEncoder(os.Stdout);encoder.SetIndent("","  ")
 if err:=encoder.Encode(map[string]any{"sdk_version":runtime.Version(),"unicode_version":unicode.Version,"tables":tables});err!=nil{panic(err)}
}
'''
    return source


def main() -> None:
    """Write or check the reproducible public-variable reference consumer."""
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--check',action='store_true');args=parser.parse_args()
    path=ROOT/'tooling/go_unicode_catalog_vectors.go';source=generated()
    if args.check:
        if not path.exists() or path.read_text()!=source:raise SystemExit('stale Unicode exported-table reference')
    else:path.write_text(source)


if __name__=='__main__':main()
