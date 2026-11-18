#!/usr/bin/env python3
"""Map every actual pinned PEM declaration to concrete byte-preserving public capabilities."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path
from audit_stdlib_surface import declarations

ROOT = Path(__file__).resolve().parents[1]


def digest(path: Path) -> str:
    """Identify independently generated observations and the exact reference implementation epoch."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Keep the package open until independent functional, portable and benchmark gates are reviewed."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--sdk', type=Path, default=Path('/tmp/cinch-go-current/go'))
    parser.add_argument('--output', type=Path, default=ROOT/'tooling/data/pem-capabilities.json')
    args = parser.parse_args()
    if (args.sdk/'VERSION').read_text().splitlines()[0] != 'go1.27.4': raise ValueError('unexpected reference SDK')
    mapping = {
        'func Decode([]uint8) (*Block, []uint8)': ['Text.Pem.pem-decode','Option PemBlock + fresh remainder Bytes','exact malformed candidate recovery and untouched original remainder on absence'],
        'func Encode(io.Writer, *Block) error': ['Effect.Pem.pem-write','Result PemError Unit + advanced ByteWriter','validate all keys before output; bounded payload chunks; positive short writes drained; failures retain continuation'],
        'func EncodeToMemory(*Block) []uint8': ['Text.Pem.pem-encode-bytes','Result PemError Bytes','fresh full encoding; colon-bearing key refusal returns no partial bytes'],
        'type Block struct': ['PemBlock / pem-block','opaque immutable owned byte metadata/payload; persistent updates preserve previous values'],
        'type Block struct, Bytes []uint8': ['pem-block-bytes','pem-with-bytes','fresh mutable observations and replacement snapshots; arbitrary payload bytes'],
        'type Block struct, Headers map[string]string': ['PemHeader / pem-header','pem-block-headers / pem-find-header','pem-with-header / pem-without-header / pem-with-headers','exact byte-key lookup, sorted unique observations, last-key-wins construction and persistent edits'],
        'type Block struct, Type string': ['pem-block-type','pem-with-type','owned arbitrary Bytes label; invalid UTF8 is preserved rather than silently discarded'],
    }
    actual = declarations(args.sdk)['encoding/pem']
    if set(actual) != set(mapping): raise ValueError('pinned PEM declarations changed: '+repr(set(actual)^set(mapping)))
    result = {'pinned_sdk':'go1.27.4','package':'encoding/pem','declaration_count':len(actual),'package_status':'implemented; standalone full-package review passed; parent integration gates pending','validation_receipt':'pem-selected-evidence.json','performance_review':'pem-review.md','reference_provenance':{'sdk_executable_sha256':digest(args.sdk/'bin/go'),'package_source_sha256':digest(args.sdk/'src/encoding/pem/pem.go'),'oracle_source_sha256':digest(ROOT/'tooling/go_pem_vectors.go'),'fixture_sha256':digest(ROOT/'compiler/tests/fixtures/pem-vectors.json')},'rows':[{'declaration':row,'capabilities':mapping[row],'status':'implemented'} for row in actual],'policies':[
        'Labels, header keys/values and payloads preserve arbitrary bytes; valid String conversion is optional outside this package.',
        'Block/header storage is immutable and private. Mutable byte observations and decoded remainders are fresh. Persistent updates share only private immutable fields.',
        'Headers use exact byte equality and lexical order, with the final duplicate value retained. Encoding writes exact Proc-Type first, then other keys in byte order.',
        'Empty and absent header collections have one logical empty representation; blocks are non-null opaque values.',
        'Decode uses first-END/final-BEGIN recovery, Unicode trimming only for header key/value edges, and loose standard padded Base64 after removing spaces/tabs.',
        'Only colon-bearing header keys are refused by the encoder; arbitrary label/value bytes, including line breaks, are retained exactly.',
        'Writer chunk boundaries are unspecified. The stream protocol drains positive short writes and rejects zero progress/impossible counts; borrowed sinks are neither flushed nor closed.',
    ]}
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps(result,indent=2)+'\n')
    print('Mapped',len(actual),'actual PEM declarations')


if __name__ == '__main__': main()
