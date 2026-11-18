#!/usr/bin/env python3
"""Append test-only overwrite canaries to a temporary Ring source without changing its public module."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def main() -> None:
    """Keep baseline and pinned-loop canaries identical apart from the private loop's container argument."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--pinned', action='store_true')
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    source = args.source.read_text()
    hook = (ROOT / 'compiler/tests/fixtures/ring-storage-canary.cinchfrag').read_text()
    hook = hook.replace('CANARY-CONTAINER', 'dense' if args.pinned else 'arena')
    marker = '  export ring-for-each, ring-to-list\n'
    assert source.count(marker) == 1
    temporary = source.replace(marker, marker + '  export ring-transfer-canary\n') + '\n' + hook
    args.output.mkdir(parents=True, exist_ok=True)
    module = args.output / 'Data.Mutable.Ring.cinch'
    module.write_text(temporary)
    consumer = args.output / 'Main.cinch'
    consumer.write_text('-- | Require private transfer-loop canaries without exporting them from the library.\n'
                        'module Main where\n  import Data.Mutable.Ring (ring-transfer-canary)\n'
                        '  -- | Return the complete private canary verdict.\n'
                        '  main :: Unit ->{Mut r} Int\n  main unit = if ring-transfer-canary () then 0 else 1\n')
    sha = lambda text: hashlib.sha256(text.encode()).hexdigest()
    (args.output / 'inputs.json').write_text(json.dumps({'description': __doc__, 'source_sha256': sha(source),
        'temporary_test_module_sha256': sha(temporary), 'consumer_sha256': sha(consumer.read_text()),
        'private_hook_sha256': sha(hook), 'pinned': args.pinned, 'production_exports_modified': False}, indent=2) + '\n')


if __name__ == '__main__':
    main()
