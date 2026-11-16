#!/usr/bin/env python3
"""Emit with one sealed legacy Native compiler/current-base tuple without selecting fallback artifacts."""
from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys


def digest(path: Path) -> str:
    """Hash tuple evidence with bounded memory before invoking the selected executable."""
    state = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            state.update(block)
    return state.hexdigest()


def emit_with_pair(receipt: Path, expected: str, arguments: list[str]) -> int:
    """Accept only explicit IR emission against the exact admitted manifest and object configuration."""
    if digest(receipt) != expected:
        raise ValueError('legacy provider tuple receipt changed')
    pair = json.loads(receipt.read_text())
    if pair.get('schema') != 1 or pair.get('kind') != 'legacy-native-current-base-pair':
        raise ValueError('unsupported legacy provider tuple')
    for item in pair['verified_files']:
        if digest(Path(item['path'])) != item['sha256']:
            raise ValueError('legacy provider tuple file changed: ' + item['path'])
    host, consumer = pair['compiler_host_runtime'], pair['consumer_runtime']
    if host['runtime_abi'] != consumer['runtime_abi'] or host['source_sha256'] != consumer['source_sha256']:
        raise ValueError('compiler-host and consumer runtime source/ABI compatibility changed')
    if arguments[:1] != ['--emit-ir'] or len(arguments) < 7:
        raise ValueError('legacy provider adapter requires --emit-ir with explicit --interfaces and --objects')
    options, paths = arguments[1:-2], arguments[-2:]
    selected = {}
    debug = []
    while options:
        flag, options = options[0], options[1:]
        if flag == '--rc-debug' and not debug:
            debug = [flag]
        elif flag in ['--interfaces', '--objects'] and flag not in selected and options:
            selected[flag], options = options[0], options[1:]
        else:
            raise ValueError('unsupported or repeated legacy provider option: ' + flag)
    for flag, name in [('--interfaces', 'interfaces'), ('--objects', 'objects')]:
        if flag not in selected or digest(Path(selected[flag])) != pair[name]['sha256']:
            raise ValueError('legacy provider adapter requires its exact admitted ' + name)
    command = [pair['compiler']['path'], '--emit-ir', *debug, '--interfaces', selected['--interfaces'],
               '--objects', selected['--objects'], *paths]
    return subprocess.call(command, env={**os.environ, 'CINCH_RC_DEBUG': '0', 'CINCH_LEAK_CHECK': '1'})


if __name__ == '__main__':
    raise SystemExit('Use the sealed frontend.py beside its explicit tuple receipt.')
