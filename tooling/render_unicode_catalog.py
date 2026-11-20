#!/usr/bin/env python3
"""Render independent exact Unicode range observations and validated custom-table conformance checks."""
from __future__ import annotations
import json
from pathlib import Path
from generate_unicode_tables import suffix
ROOT=Path(__file__).resolve().parents[1]


def rendered() -> str:
    """Compare original stride fields and public catalog ordering with an independent SDK fixture."""
    fixture=json.loads((ROOT/'seed/tests/fixtures/unicode-vectors.json').read_text())
    source='''-- | Independent named Unicode catalogs, exact range observations and checked custom metadata.
module Main where
  import Data.Unicode
  import Data.Bool
  import Data.Char
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Process

  -- | Keep large catalog observations in runtime rather than constant evaluation.
  prim runtime-input :: U32 -> U32 where
    llvm {
      %result = or i32 %0, 0
      ret i32 %result
    }

  -- | Hash all four bytes of each independent numeric range field.
  hash-word :: U32 -> U32 -> U64 -> U64
  hash-word value shift hash = if lt-u32 shift 32 then
    let byte = u32-to-u64 (bit-and-u32 (wrapping-shr-u32 value shift) 255) in
    let next = wrapping-mul-u64 (bit-xor-u64 hash byte) 1099511628211 in
    hash-word value (wrapping-add-u32 shift 8) next else hash

  -- | Convert bounded observation counts without changing very large-index refusal policy.
  count-word :: Nat -> U32
  count-word value = if value <= 4294967295 then wrapping-i64-to-u32 (nat-to-i64 value) else 0

  -- | Hash one complete original low/high/stride triple.
  triple-hash :: U32 -> U32 -> U32 -> U64 -> U64
  triple-hash low high stride hash = let a = hash-word low 0 hash in let b = hash-word high 0 a in hash-word stride 0 b

  -- | Hash exact indexed16-bit records in original order.
  small-hash :: UnicodeRangeTable -> Nat -> U64 -> U64
  small-hash table at hash = case unicode-range16-at table at of
    None -> hash
    Some span -> small-hash table (at + 1) (triple-hash (u16-to-u32 (unicode-range16-low span)) (u16-to-u32 (unicode-range16-high span)) (u16-to-u32 (unicode-range16-stride span)) hash)

  -- | Hash exact indexed supplementary records in original order.
  wide-hash :: UnicodeRangeTable -> Nat -> U64 -> U64
  wide-hash table at hash = case unicode-range32-at table at of
    None -> hash
    Some span -> wide-hash table (at + 1) (triple-hash (unicode-range32-low span) (unicode-range32-high span) (unicode-range32-stride span) hash)

  -- | Observe counts, original Latin metadata and all preserved range fields.
  table-hash :: UnicodeRangeTable -> U64
  table-hash table = let a = hash-word (count-word (unicode-range16-count table)) 0 14695981039346656037 in
    let b = hash-word (count-word (unicode-range32-count table)) 0 a in let c = hash-word (count-word (unicode-latin-offset table)) 0 b in
    wide-hash table 0 (small-hash table 0 c)

  -- | Require original and copied queries to agree at raw numeric boundary inputs.
  agree :: UnicodeRangeTable -> UnicodeRangeTable -> U32 -> Bool
  agree first second cp = let raw = wrapping-u32-to-i32 cp in unicode-range-contains-code first raw == unicode-range-contains-code second raw

  -- | Cover endpoints, stride holes, surrounding gaps and the full numeric domain policy.
  span-check :: UnicodeRangeTable -> UnicodeRangeTable -> U32 -> U32 -> U32 -> Bool
  span-check first second low high stride = unicode-range-contains-code first (wrapping-u32-to-i32 low)
    && agree first second low && agree first second high && agree first second (wrapping-add-u32 low 1)
    && agree first second (wrapping-sub-u32 low 1) && agree first second (wrapping-add-u32 high 1)
    && agree first second (wrapping-add-u32 low stride)
    && (if gt-u32 stride 1 && lt-u32 low high then not (unicode-range-contains-code first (wrapping-u32-to-i32 (wrapping-add-u32 low 1))) else True)

  -- | Check every original16-bit range against its validated owned reconstruction.
  small-check :: UnicodeRangeTable -> UnicodeRangeTable -> Nat -> Bool
  small-check first second at = case unicode-range16-at first at of
    None -> True
    Some span -> span-check first second (u16-to-u32 (unicode-range16-low span)) (u16-to-u32 (unicode-range16-high span)) (u16-to-u32 (unicode-range16-stride span)) && small-check first second (at + 1)

  -- | Check every original supplementary range against its owned reconstruction.
  wide-check :: UnicodeRangeTable -> UnicodeRangeTable -> Nat -> Bool
  wide-check first second at = case unicode-range32-at first at of
    None -> True
    Some span -> span-check first second (unicode-range32-low span) (unicode-range32-high span) (unicode-range32-stride span) && wide-check first second (at + 1)

  -- | Resolve a current public catalog through a runtime selector.
  lookup :: U32 -> String -> Option UnicodeRangeTable
  lookup kind name = if eq-u32 kind 0 then unicode-category name else if eq-u32 kind 1 then unicode-script name
    else if eq-u32 kind 2 then unicode-property name else if eq-u32 kind 3 then unicode-fold-category name else if eq-u32 kind 4 then unicode-fold-script name else unicode-table name

  -- | Observe one key in the exact sorted catalog order.
  key-at :: U32 -> Nat -> Option String
  key-at kind at = if eq-u32 kind 0 then unicode-category-name at else if eq-u32 kind 1 then unicode-script-name at
    else if eq-u32 kind 2 then unicode-property-name at else if eq-u32 kind 3 then unicode-fold-category-name at else if eq-u32 kind 4 then unicode-fold-script-name at else unicode-table-name at

  -- | Check a source-independent catalog row and all exact public field observations.
  catalog-row :: U32 -> Nat -> String -> Nat -> Nat -> Nat -> U64 -> Bool
  catalog-row raw-kind index name n16 n32 latin expected = let kind = runtime-input raw-kind in case lookup kind name of
    None -> False
    Some table -> (case key-at kind index of None -> False; Some actual -> actual == name)
      && unicode-range16-count table == n16 && unicode-range32-count table == n32 && unicode-latin-offset table == latin
      && eq-u64 (table-hash table) expected && eq-u64 (table-hash (unicode-range-table-clone table)) expected
      && (case unicode-range-table-with-offset (wrapping-u32-to-i32 (count-word latin)) (unicode-range16-ranges table) (unicode-range32-ranges table) of
        Err problem -> False
        Ok copied -> eq-u64 (table-hash copied) expected && small-check table copied 0 && wide-check table copied 0
          && agree table copied 4294967295 && agree table copied 55296 && agree table copied 57343 && agree table copied 1114111 && agree table copied 1114112)
      && (case unicode-range16-at table 18446744073709551616 of None -> True; Some span -> False)
      && (case unicode-range32-at table n32 of None -> True; Some span -> False)

  -- | Fold a small fixture batch without a deeply nested constant expression.
  all-checks :: List Bool -> Bool
  all-checks checks = case checks of Nil -> True; Cons value rest -> value && all-checks rest

  -- | Confirm list catalogs have the exact indexed order and no extra or missing keys.
  names-check :: U32 -> Nat -> List String -> Bool
  names-check kind at values = case values of
    Nil -> case key-at kind at of None -> True; Some value -> False
    Cons value rest -> (case key-at kind at of None -> False; Some key -> key == value) && names-check kind (at + 1) rest

  -- | Confirm pair-list observations preserve sorted alias order and the full mapping.
  aliases-check :: Nat -> List (String, String) -> Bool
  aliases-check at values = case values of
    Nil -> case unicode-category-alias-at at of None -> True; Some value -> False
    Cons value rest -> (case unicode-category-alias-at at of None -> False; Some expected -> case expected of (a,b) -> case value of (x,y) -> a == x && b == y) && aliases-check (at + 1) rest

  -- | Hash all five exact standard or locale case fields through their list observations.
  cases-hash :: List UnicodeCaseRange -> U64 -> U64
  cases-hash values hash = case values of
    Nil -> hash
    Cons value rest -> let a = hash-word (unicode-case-range-low value) 0 hash in
      let b = hash-word (unicode-case-range-high value) 0 a in
      let c = hash-word (wrapping-i32-to-u32 (unicode-case-range-upper value)) 0 b in
      let d = hash-word (wrapping-i32-to-u32 (unicode-case-range-lower value)) 0 c in
      let e = hash-word (wrapping-i32-to-u32 (unicode-case-range-title value)) 0 d in cases-hash rest e

  -- | Refuse malformed caller bounds, zero stride, overlap and mismatched Latin metadata explicitly.
  contracts :: Unit -> Bool
  contracts unit = unicode-version == "17.0.0" && unicode-max-rune == '\\u{10FFFF}'
    && unicode-category-count == 38 && unicode-script-count == 174 && unicode-property-count == 39
    && unicode-fold-category-count == 6 && unicode-fold-script-count == 3 && unicode-table-count == 262
    && (case unicode-range-table [MkUnicodeRange16 10 9 1] Nil of Err (UnicodeRangeInvalidBounds False 0) -> True; other -> False)
    && (case unicode-range-table [MkUnicodeRange16 0 9 0] Nil of Err (UnicodeRangeZeroStride False 0) -> True; other -> False)
    && (case unicode-range-table [MkUnicodeRange16 0 9 1, MkUnicodeRange16 9 10 1] Nil of Err (UnicodeRangeInvalidOrder False 1) -> True; other -> False)
    && (case unicode-range-table Nil [MkUnicodeRange32 65535 65536 1] of Err (UnicodeRangeInvalidBounds True 0) -> True; other -> False)
    && (case unicode-range-table Nil [MkUnicodeRange32 65536 1114112 1] of Err (UnicodeRangeInvalidBounds True 0) -> True; other -> False)
    && (case unicode-range-table Nil [MkUnicodeRange32 65536 65537 0] of Err (UnicodeRangeZeroStride True 0) -> True; other -> False)
    && (case unicode-range-table-with-offset (-1) Nil Nil of Err (UnicodeRangeInvalidLatinOffset (-1)) -> True; other -> False)
    && (case unicode-range-table-with-offset 0 [MkUnicodeRange16 0 10 1] Nil of Err (UnicodeRangeInvalidLatinOffset 0) -> True; other -> False)
    && (case unicode-range-table [MkUnicodeRange16 55296 57343 1] [MkUnicodeRange32 65536 1114111 4294967295] of
      Err problem -> False
      Ok table -> unicode-range-contains-code table 55296 && unicode-range-contains-code table 65536 && not (unicode-range-contains-code table 65537)
        && not (unicode-range-contains-code table (-1)) && not (unicode-range-contains-code table 1114112))
    && (case unicode-category "invalid" of None -> True; Some table -> False)
    && (case unicode-script "greek" of None -> True; Some table -> False)
    && (case unicode-category-of-code 55296 of Some name -> name == "Cs"; None -> False)
    && (case unicode-category-of-code (-1) of None -> True; Some name -> False)
    && unicode-in '\\u{202F}' unicode-graphic-ranges && not (unicode-in '\\u{202F}' unicode-print-ranges)
    && unicode-is-print ' ' && not (unicode-in ' ' unicode-print-ranges)
    && unicode-is-one-of-code unicode-graphic-ranges 8239 && not (unicode-is-one-of unicode-print-ranges ' ')
    && names-check 0 0 (unicode-category-names ()) && names-check 1 0 (unicode-script-names ())
    && names-check 2 0 (unicode-property-names ()) && names-check 3 0 (unicode-fold-category-names ())
    && names-check 4 0 (unicode-fold-script-names ()) && names-check 5 0 (unicode-table-names ())
    && unicode-category-alias-count == 42 && aliases-check 0 (unicode-category-aliases ())
    && (case unicode-category-alias-at 18446744073709551616 of None -> True; Some value -> False)
    && (case unicode-table-name 18446744073709551616 of None -> True; Some value -> False)

'''
    counters={};checks=[]
    exported=json.loads((ROOT/'seed/tests/fixtures/unicode-catalog-vectors.json').read_text())
    rows=fixture['ranges']+[{**row,'catalog':'table'} for row in exported['tables']]
    for row in rows:
        cat=row['catalog'];index=counters.get(cat,0);counters[cat]=index+1
        kind={'category':0,'script':1,'property':2,'fold_category':3,'fold_script':4,'table':5}[cat]
        words=[len(row['r16']),len(row['r32']),row['latin_offset']]+[v for span in row['r16']+row['r32'] for v in span]
        h=14695981039346656037
        for value in words:
            for byte in value.to_bytes(4,'little'):h=((h^byte)*1099511628211)&((1<<64)-1)
        checks.append(f'catalog-row {kind} {index} {json.dumps(row["name"])} {len(row["r16"])} {len(row["r32"])} {row["latin_offset"]} {h}')
    for index,(alias,canonical) in enumerate(sorted(fixture['category_aliases'].items())):
        checks.append(f'(case unicode-category-alias-at {index} of None -> False; Some pair -> case pair of (key,value) -> key == {json.dumps(alias)} && value == {json.dumps(canonical)})')
        checks.append(f'(case unicode-category-canonical {json.dumps(alias)} of None -> False; Some actual -> actual == {json.dumps(canonical)})')
    for field,expression in [('case_ranges','unicode-case-ranges ()'),('turkish_ranges','unicode-special-case-ranges unicode-turkish-case'),('azeri_ranges','unicode-special-case-ranges unicode-azeri-case')]:
        h=14695981039346656037
        for row in fixture[field]:
            for value in row:
                for byte in (value & 0xFFFFFFFF).to_bytes(4,'little'):h=((h^byte)*1099511628211)&((1<<64)-1)
        checks.append(f'eq-u64 (cases-hash ({expression}) 14695981039346656037) {h}')
    calls=[]
    for index,start in enumerate(range(0,len(checks),16)):
        name='catalog-batch-'+suffix(index);calls.append(name+' ()')
        source+=f'  -- | Check a bounded batch of independent catalog/range/alias observations.\n  {name} :: Unit -> Bool\n  {name} unit = all-checks ['+', '.join(checks[start:start+16])+']\n\n'
    source+='''  -- | Complete exact current named-table observations and checked caller metadata.
  main :: Int
  main = if contracts () && all-checks ['''+', '.join(calls)+'] then 0 else 1\n'
    return source


if __name__=='__main__':
    print(rendered(),end='')
