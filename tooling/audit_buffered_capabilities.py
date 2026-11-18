#!/usr/bin/env python3
"""Audit every pinned buffered-I/O declaration and its concrete planned operation route."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
import re
from audit_stdlib_surface import declarations

ROOT=Path(__file__).resolve().parents[1]
READER={
    'Buffered':['buffered-reader-buffered'], 'Discard':['buffered-discard'],
    'Peek':['buffered-peek'], 'Read':['buffered-read','buffered-read-into'],
    'ReadByte':['buffered-read-byte'], 'ReadBytes':['buffered-read-delimited'],
    'ReadLine':['buffered-read-line'], 'ReadRune':['buffered-read-rune'],
    'ReadSlice':['buffered-read-slice'],
    'ReadString':['buffered-read-delimited','buffer-read-text'],
    'Reset':['buffered-reader-reset','buffered-reader-reset-self'],
    'Size':['buffered-reader-size'], 'UnreadByte':['buffered-unread-byte'],
    'UnreadRune':['buffered-unread-rune'], 'WriteTo':['buffered-write-to'],
}
WRITER={
    'Available':['buffered-writer-available'],
    'AvailableBuffer':['buffered-writer-space','buffer-space-append','buffer-space-bytes','buffered-write-space'],
    'Buffered':['buffered-writer-buffered'], 'Flush':['buffered-flush'],
    'ReadFrom':['buffered-read-from'], 'Reset':['buffered-writer-reset','buffered-writer-reset-self'],
    'Size':['buffered-writer-size'], 'Write':['buffered-write'],
    'WriteByte':['buffered-write-byte'], 'WriteRune':['buffered-write-rune','buffered-write-char'],
    'WriteString':['buffered-write-text','buffered-write'],
}
SCANNER={
    'Buffer':['scanner-buffer','scanner-buffer-size'], 'Bytes':['scanner-bytes','scanner-token'],
    'Err':['scanner-error'], 'Scan':['scanner-next'], 'Split':['scanner-split'],
    'Text':['scanner-text','scanner-bytes'],
}
ERRORS={
    'ErrAdvanceTooFar':'Effect.Scanner.Split.ScanAdvanceTooFar',
    'ErrBadReadCount':'Effect.Scanner.Split.ScanBadReadCount',
    'ErrBufferFull':'Effect.Buffered.Core.BufferFull',
    'ErrFinalToken':'Effect.Scanner.Split.ScanFinalToken',
    'ErrInvalidUnreadByte':'Effect.Buffered.Core.BufferInvalidUnreadByte',
    'ErrInvalidUnreadRune':'Effect.Buffered.Core.BufferInvalidUnreadRune',
    'ErrNegativeAdvance':'Effect.Scanner.Split.ScanNegativeAdvance',
    'ErrNegativeCount':'Effect.Buffered.Core.BufferNegativeCount',
    'ErrTooLong':'Effect.Scanner.Split.ScanTooLong',
}


def qualified(home: str, names: list[str]) -> list[str]:
    """Keep each operation's actual home explicit instead of counting a module as implementation."""
    return [home+'.'+name for name in names]


def route(row: str) -> dict:
    """Map promoted methods through a real component observation/update route, preserving both handles."""
    match=re.match(r'method \((\*?)(Reader|Writer|Scanner|ReadWriter)\) (\w+)\(',row)
    if match:
        receiver,name=match[2],match[3]
        if receiver=='Scanner':return {'capabilities':qualified('Effect.Scanner',SCANNER[name])}
        table,home,component=(READER,'Effect.Buffered.Reader','reader') if receiver=='Reader' or (receiver=='ReadWriter' and name in READER and name not in WRITER) else (WRITER,'Effect.Buffered.Writer','writer')
        operations=qualified(home,table[name])
        if receiver=='ReadWriter':operations=['Effect.Buffered.buffered-read-writer-'+component]+operations+['Effect.Buffered.buffered-read-writer-with-'+component]
        return {'capabilities':operations}
    match=re.match(r'func (\w+)\(',row)
    if match:
        name=match[1]
        factories={
            'NewReadWriter':['Effect.Buffered.buffered-read-writer'],
            'NewReader':['Effect.Buffered.Reader.buffered-reader','Effect.Buffered.Reader.buffered-reader-from-stream'],
            'NewReaderSize':['Effect.Buffered.Reader.buffered-reader-sized','Effect.Buffered.Reader.buffered-reader-from-stream-sized','Effect.Buffered.Reader.buffered-reader-rebuffer'],
            'NewScanner':['Effect.Scanner.scanner','Effect.Scanner.scanner-from-stream'],
            'NewWriter':['Effect.Buffered.Writer.buffered-writer','Effect.Buffered.Writer.buffered-writer-from-stream'],
            'NewWriterSize':['Effect.Buffered.Writer.buffered-writer-sized','Effect.Buffered.Writer.buffered-writer-from-stream-sized','Effect.Buffered.Writer.buffered-writer-rebuffer'],
            'ScanBytes':['Effect.Scanner.Split.scan-bytes'], 'ScanLines':['Effect.Scanner.Split.scan-lines'],
            'ScanRunes':['Effect.Scanner.Split.scan-runes'], 'ScanWords':['Effect.Scanner.Split.scan-words'],
        }
        return {'capabilities':factories[name]}
    match=re.match(r'var (\w+) error',row)
    if match:return {'capabilities':[ERRORS[match[1]]]}
    if row.startswith('const MaxScanTokenSize '):return {'capabilities':['Effect.Scanner.scanner-max-token-size']}
    if row=='type ReadWriter struct, embedded *Reader':return {'capabilities':['Effect.Buffered.buffered-read-writer-reader','Effect.Buffered.buffered-read-writer-with-reader']}
    if row=='type ReadWriter struct, embedded *Writer':return {'capabilities':['Effect.Buffered.buffered-read-writer-writer','Effect.Buffered.buffered-read-writer-with-writer']}
    types={
        'type ReadWriter struct':['Effect.Buffered.BufferedReadWriter'],
        'type Reader struct':['Effect.Buffered.Reader.BufferedReader'],
        'type Writer struct':['Effect.Buffered.Writer.BufferedWriter'],
        'type Scanner struct':['Effect.Scanner.Scanner'],
        'type SplitFunc func([]uint8, bool) (int, []uint8, error)':['Effect.Scanner.Split.ScanSplit','Effect.Scanner.Split.ScanStep'],
    }
    if row not in types:raise ValueError('unmapped pinned declaration '+row)
    return {'capabilities':types[row]}


def implemented(symbol: str) -> bool:
    """Distinguish a planned operation from a declaration that is actually present in owned source."""
    home,name=symbol.rsplit('.',1);path=ROOT/'compiler/prelude'/(home.replace('.','/')+'.cinch')
    if not path.exists():return False
    return bool(re.search(r'^\s+(?:(?:data|prim)\s+)?'+re.escape(name)+r'(?:\s+[^\n:=]+)?\s*(?:::|where)',path.read_text(),re.M))


def main() -> None:
    """Require exact surface completeness while reporting implementation and validation progress honestly."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--sdk',type=Path,default=Path('/tmp/cinch-go-current/go'))
    parser.add_argument('--output',type=Path,default=ROOT/'tooling/data/buffered-capabilities.json')
    args=parser.parse_args()
    if (args.sdk/'VERSION').read_text().splitlines()[0]!='go1.27.4':raise ValueError('unexpected reference SDK')
    actual=declarations(args.sdk)['bufio']
    if len(actual)!=80:raise ValueError('pinned declaration count changed')
    rows=[]
    for row in actual:
        mapped=route(row);present={symbol:implemented(symbol) for symbol in mapped['capabilities']}
        rows.append({'declaration':row,**mapped,'source_declarations_present':present,'status':'implemented; validation pending' if all(present.values()) else 'in progress'})
    result={'pinned_sdk':'go1.27.4','package':'bufio','declaration_count':len(rows),'status':'full80 capability implementation in progress; no exclusions','rows':rows,'policies':[
        'Raw source/sink events preserve signed counts, byte prefixes and simultaneous error; existing stream adapters retain their narrower Result contracts.',
        'Optional direct transfer callbacks preserve dispatch and advanced handles through private managed IO continuation cells; no negative recursive data types or checker exceptions.',
        'Buffered storage is private. Externally mutable observations are fresh; callback input snapshots cannot mutate retained state.',
        'ReadWriter promoted operations route through concrete component getters and setters, preserving the other component.',
        'AvailableBuffer is an owned bounded append builder/reservation; it exposes capacity and a fresh used prefix without private backing aliases.',
        'Byte data is primary. Text conversions explicitly validate UTF8 or replace invalid bytes; invalid input is never silently converted to an empty String.',
        'Scanner distinguishes absent and empty tokens, final-token termination, signed advance errors, source count errors and checked nonprogress/configuration refusals.',
    ],'reference_sources':{name:hashlib.sha256((args.sdk/'src/bufio'/name).read_bytes()).hexdigest() for name in ['bufio.go','scan.go']}}
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(result,indent=2)+'\n')
    print('Mapped all',len(rows),'pinned buffered-I/O records; implemented-source progress remains explicit')


if __name__=='__main__':main()
