#!/usr/bin/env python3
"""Map every actual pinned Unicode/UTF8 API declaration to implemented or pending concrete Cinch capabilities."""
from __future__ import annotations
import argparse
import json
from pathlib import Path
import re
from audit_stdlib_surface import declarations
ROOT=Path(__file__).resolve().parents[1]


def unicode_capability(declaration: str, aliases: dict) -> list[str]:
    """Preserve every current returned table capability while identifying historical version records."""
    name=declaration.split()[1]
    if declaration.startswith('const '):
        names={'LowerCase':'UnicodeLower','UpperCase':'UnicodeUpper','TitleCase':'UnicodeTitle','MaxCase':'unicode-case-count','MaxASCII':'unicode-max-ascii','MaxLatin1':'unicode-max-latin1','MaxRune':'unicode-max-codepoint / unicode-max-rune','ReplacementChar':'unicode-replacement','UpperLower':'unicode-upper-lower','Version':'unicode-version (17.0.0 current edition only)'}
        return [names[name]]
    if declaration.startswith('func '):
        name=name.split('(')[0]
        specials={'In':'unicode-in-code','Is':'unicode-range-contains-code','IsOneOf':'unicode-is-one-of-code','SimpleFold':'unicode-simple-fold-code','To':'unicode-to-code','ToLower':'unicode-to-lower-code','ToUpper':'unicode-to-upper-code','ToTitle':'unicode-to-title-code'}
        return [specials[name] if name in specials else 'unicode-is-'+name[2:].lower()+'-code']
    if declaration.startswith('method '):
        name=declaration.split()[2].split('(')[0]
        return ['unicode-special-to-'+name[2:].lower()+'-code']
    if declaration.startswith('var '):
        if name in aliases:return [f'unicode-table "{name}"','unicode-table-count/name/names','unicode-range16/32-count/at/ranges','unicode-latin-offset','unicode-range-contains-code']
        names={'Categories':'unicode-category','Scripts':'unicode-script','Properties':'unicode-property','FoldCategory':'unicode-fold-category','FoldScript':'unicode-fold-script','CategoryAliases':'unicode-category-canonical / unicode-category-alias-at / unicode-category-aliases','CaseRanges':'unicode-case-range-at / unicode-case-ranges','TurkishCase':'unicode-turkish-case','AzeriCase':'unicode-azeri-case','GraphicRanges':'unicode-graphic-ranges','PrintRanges':'unicode-print-ranges'}
        return [names[name]]
    if declaration.startswith('type CaseRange'):return ['UnicodeCaseRange / MkUnicodeCaseRange','unicode-case-range-low/high/upper/lower/title','unicode-special-case checked construction']
    if declaration.startswith('type Range16'):return ['UnicodeRange16 / MkUnicodeRange16','unicode-range16-low/high/stride']
    if declaration.startswith('type Range32'):return ['UnicodeRange32 / MkUnicodeRange32','unicode-range32-low/high/stride']
    if declaration.startswith('type RangeTable'):return ['UnicodeRangeTable','unicode-range-table / unicode-range-table-with-offset','unicode-range16/32-ranges / count / at','unicode-latin-offset']
    if declaration.startswith('type SpecialCase'):return ['UnicodeSpecialCase','unicode-special-case / unicode-special-case-count / unicode-special-case-range-at','unicode-special-to-upper/lower/title-code']
    raise ValueError('unmapped Unicode declaration: '+declaration)


def utf8_capability(declaration: str) -> tuple[list[str],str]:
    """Keep arbitrary invalid strings in byte APIs and isolate the pending valid-String facade epoch."""
    name=declaration.split()[1].split('(')[0]
    constants={'MaxRune':'utf8-max-rune','RuneError':'utf8-rune-error','RuneSelf':'utf8-rune-self','UTFMax':'utf8-max-width'}
    if declaration.startswith('const '):return [constants[name]],'implemented'
    methods={'AppendRune':'utf8-append-rune','DecodeLastRune':'utf8-last','DecodeRune':'utf8-first / utf8-decode-at','EncodeRune':'utf8-encode-into / utf8-encode-rune','FullRune':'utf8-full-rune','RuneCount':'utf8-count','RuneLen':'utf8-rune-length','RuneStart':'utf8-rune-start','Valid':'utf8-valid','ValidRune':'utf8-valid-rune'}
    if name in methods:return [methods[name]],'implemented'
    strings={'DecodeLastRuneInString':'Data.String.string-utf8-last / planned Text.Utf8.utf8-last-text','DecodeRuneInString':'Data.String.string-utf8-first / planned Text.Utf8.utf8-first-text','FullRuneInString':'planned Text.Utf8.utf8-full-rune-text; valid String nonempty test','RuneCountInString':'Data.String.string-length / planned Text.Utf8.utf8-count-text','ValidString':'String valid-UTF8 invariant / planned Text.Utf8.utf8-valid-text'}
    return [strings[name]],'pending-string-provider-epoch'


def main() -> None:
    """Audit cumulative actual SDK declarations without editing the shared stdlib inventory."""
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('--sdk',type=Path,default=Path('/tmp/cinch-go-current/go'));parser.add_argument('--output',type=Path,default=ROOT/'tooling/data/unicode-capabilities.json');args=parser.parse_args()
    if args.sdk.joinpath('VERSION').read_text().splitlines()[0]!='go1.27.4':raise ValueError('unexpected pinned SDK')
    public=declarations(args.sdk);data=json.loads((ROOT/'tooling/data/unicode17-tables.json').read_text());packages=[]
    for package in ['unicode','unicode/utf8']:
        rows=[]
        for declaration in public[package]:
            capabilities,status=(unicode_capability(declaration,data['exported_aliases']),'implemented') if package=='unicode' else utf8_capability(declaration)
            rows.append({'declaration':declaration,'capabilities':capabilities,'status':status})
        packages.append({'package':package,'declaration_count':len(rows),'rows':rows})
    output={'pinned_sdk':'go1.27.4','unicode_version':'17.0.0','packages':packages,'policies':['Current Unicode edition is versioned; cumulative historical Version constant records do not create obsolete table sets.','Char is scalar-only; numeric range membership includes surrogate code points.','Built-in tables and catalogs are immutable; custom metadata is checked before private owned storage is allocated.','Simple mappings preserve one scalar; no normalization, expansion or full case folding is supplied by this package.','Raw byte APIs retain malformed/non-UTF8 input; valid String adapters use the private backing under a separately registered provider epoch.','Invalid decoding consumes one byte as U+FFFD; invalid numeric encoding emits U+FFFD; rune-length reports None for invalid numbers.','Append and repair outputs are unconditionally fresh; encode-into capacity refusal writes nothing.']}
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(output,indent=2)+'\n')
    print([(row['package'],row['declaration_count']) for row in packages])


if __name__=='__main__':main()
