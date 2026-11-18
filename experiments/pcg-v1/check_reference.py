#!/usr/bin/env python3
"""Cross-check every reference output and snapshot with an independent integer model."""
from pathlib import Path
import argparse,hashlib,json


def main():
    """Reject any output mismatch without rewriting either reference or candidate input."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('vectors',type=Path)
    args=parser.parse_args()
    raw=args.vectors.read_bytes(); rows=json.loads(raw)
    mask=(1<<64)-1; modulus=1<<128
    multiplier=int('2360ed051fc65da44385df649fccf645',16)
    increment=(6364136223846793005<<64)|1442695040888963407
    output_multiplier=0xda942042e4dd58b5
    assert len(rows)==8
    for row in rows:
        state=(row['high']<<64)|row['low']
        snapshot=lambda value:(b'pcg:'+value.to_bytes(16,'big')).hex()
        assert snapshot(state)==row['before']
        assert len(row['values'])==256
        for index,value in enumerate(row['values']):
            state=(state*multiplier+increment)%modulus
            high,low=state>>64,state&mask
            high^=high>>32
            high=(high*output_multiplier)&mask
            high^=high>>48
            expected=(high*(low|1))&mask
            assert expected==value,(row['high'],row['low'],index,expected,value)
        assert snapshot(state)==row['after']
        assert (bytes([0,255,128,1])+bytes.fromhex(row['after'])).hex()==row['appended']
    print(json.dumps({'reference_sha256':hashlib.sha256(raw).hexdigest(),'seeds':len(rows),'outputs':sum(len(row['values']) for row in rows),'snapshots_and_appends':len(rows)*3,'independent_model':'unbounded Python integers modulo 2^128, scalar DXSM, explicit big-endian bytes'}))


if __name__=='__main__':main()
