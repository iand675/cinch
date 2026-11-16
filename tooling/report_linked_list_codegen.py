#!/usr/bin/env python3
"""Record optimized linked-list function bodies and their remaining allocation/ownership call operands."""
from __future__ import annotations

import argparse
import collections
import hashlib
import json
from pathlib import Path
import re
import subprocess

SELECTED = ['membership-error', 'same-token', 'allocate-node', 'copy-members', 'element-at',
            'collect-values', 'drain', 'move-handles', 'reuse-handles']


def digest(path: Path) -> str:
    """Identify the exact IR or assembly artifact inspected."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def decoded(name: str) -> str:
    """Present canonical source names while retaining exact emitted symbols separately."""
    return re.sub(r'\$([0-9a-f]{2})', lambda match: chr(int(match.group(1), 16)), name)


def main() -> None:
    """Inspect a common optimized LLVM stage without inferring work from source syntax alone."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--clang', default='clang')
    args = parser.parse_args()
    original = args.artifacts / 'bench.ll'
    optimized = args.artifacts / 'bench.optimized.ll'
    subprocess.run([args.clang, '-O2', '-S', '-emit-llvm', '-x', 'ir', str(original), '-o', str(optimized)], check=True, capture_output=True)
    functions = []
    for match in re.finditer(r'^define [^\n]+\{\n.*?^\}', optimized.read_text(), re.M | re.S):
        body = match.group(0)
        header = body.splitlines()[0]
        name_match = re.search(r'@("[^"]+"|[\w.$#-]+)\(', header)
        if not name_match:
            continue
        name = name_match.group(1).strip('"')
        source_name = decoded(name)
        if not any(part in source_name for part in SELECTED):
            continue
        calls = collections.Counter(re.findall(r'\bcall\b[^\n]*?(@"[^"]+"|@[\w.$]+|%[\w.$]+)\(', body))
        functions.append({'symbol': name, 'decoded_symbol': source_name, 'header': header,
                          'body_sha256': hashlib.sha256(body.encode()).hexdigest(),
                          'lines': len(body.splitlines()), 'call_operands': dict(sorted(calls.items())),
                          'indirect_calls': sum(count for operand, count in calls.items() if operand.startswith('%')),
                          'ownership_call_sites': sum(count for operand, count in calls.items() if any(key in operand for key in ['rc.', 'rc_', 'cinch_free'])),
                          'explicit_constructor_allocation_sites': sum(count for operand, count in calls.items() if any(key in operand for key in ['alloc_ctor', 'alloc_object', 'slots_new']))})
    report = {'description': __doc__, 'reporter_sha256': digest(Path(__file__)),
              'original_llvm_sha256': digest(original), 'optimized_llvm_sha256': digest(optimized),
              'assembly_sha256': digest(args.artifacts / 'bench.s'),
              'optimization': 'clang -O2 -S -emit-llvm from the exact consumed IR; function absence may reflect inlining, not absence of the underlying work.',
              'note': 'Static call sites describe code shape, not dynamic allocation counts. Ordinary process allocation probes provide independent space evidence.',
              'functions': functions}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
