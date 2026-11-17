#!/usr/bin/env python3
"""Prepare or execute a bounded pinned-SDK suffix-index oracle; candidate runtime execution is separate."""
from __future__ import annotations
import argparse
import hashlib
import itertools
import json
import os
from pathlib import Path
import subprocess

HERE=Path(__file__).resolve().parent
SDK=Path('/tmp/cinch-go-current/go')


def cases() -> dict:
    """Exercise arbitrary bytes, repetitive prefixes, vector fringes and scalar replacement boundaries."""
    data={b'',b'banana',b'aaaaa',b'abababa',b'abc\x00abc',bytes(range(256)),b'\xff\x80a\xef\xbf\xbd\xc3\xa9\xf0\x9f\x98\x80'}
    for width in range(5):
        data.update(bytes(values) for values in itertools.product([0,97,255],repeat=width))
    for size in [7,8,15,16,17,31,32,33,63,64,65,257,1024]:
        data.add(b'a'*size)
        data.add(bytes((at*73+19)%256 for at in range(size)))
    queries=[b'',b'a',b'aa',b'aaa',b'ab',b'banana',b'na',b'\x00',b'\xff',b'\xff\x80',b'\x80',b'\xef\xbf\xbd',b'\xc3\xa9',b'not-present']
    return dict(data=[value.hex() for value in sorted(data)],queries=[value.hex() for value in queries],patterns=['','a','aa','a*','a?','(?:a|aa)','.','(?s:.)','^','$','\\x{FFFD}','(?:ab|b)+','[\\x00-\\x7f]+'],limits=[-1,0,1,2,5])


def digest(path: Path) -> str:
    """Hash complete immutable reference inputs before recording a generator receipt."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Keep preparation, SDK observations and future Cinch qualification as distinct evidence phases."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--prepare-only',action='store_true')
    parser.add_argument('--go',type=Path,default=SDK/'bin/go')
    parser.add_argument('--output',type=Path,default=HERE/'vectors.json')
    args=parser.parse_args();payload=cases()
    if args.prepare_only:
        args.output.write_text(json.dumps(payload,indent=2)+'\n');return
    pinned=json.loads((HERE/'surface.json').read_text())
    assert digest(args.go)==pinned['go_binary_sha256']
    assert digest(SDK/'src/index/suffixarray/suffixarray.go')==pinned['implementation_source_sha256']
    env={**os.environ,'GOROOT':str(SDK),'GOCACHE':'/tmp/cinch-go-surface-cache','GOTELEMETRY':'off','GOMAXPROCS':'1'}
    version=subprocess.check_output([str(args.go),'version'],env=env,text=True).strip()
    assert version=='go version go1.27.4 linux/amd64',version
    result=subprocess.run([str(args.go),'run',str(HERE/'oracle.go')],input=json.dumps(payload),capture_output=True,text=True,env=env,timeout=60)
    if result.returncode:
        args.output.with_suffix('.failure.txt').write_text(result.stdout+result.stderr)
        raise RuntimeError(result.stderr)
    observed=[json.loads(line) for line in result.stdout.splitlines()]
    assert len(observed)==len(payload['data'])
    encoded=json.dumps(dict(reference=version,go_binary_sha256=digest(args.go),oracle_sha256=digest(HERE/'oracle.go'),generator_sha256=digest(Path(__file__)),package_source_sha256=digest(SDK/'src/index/suffixarray/suffixarray.go'),cases=observed),indent=2)+'\n'
    assert len(encoded.encode())<4*1024*1024,'bounded fixture budget exceeded'
    args.output.write_text(encoded)


if __name__=='__main__':
    main()
