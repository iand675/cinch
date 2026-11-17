#!/usr/bin/env python3
"""Prepare exact modular inputs or, only after a later execution grant, crosscheck actual SDK observations with independent Python arithmetic."""
from __future__ import annotations
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import random
import subprocess

HERE = Path(__file__).resolve().parent


def digest(path: Path) -> str:
    """Identify complete reference sources and binaries without using their version label as identity."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def inputs() -> list[dict[str, str]]:
    """Prepare zero/one, parity, full unsigned, large exponent and arbitrary-precision fringes deterministically."""
    values = [0, 1, 2, 3, 6, 15, 2**32-1, 2**32, 2**63-1, 2**63, 2**64-1, 2**64, 2**64+13, 2**127-1, 2**128+1, 2**256-189, 2**521-1]
    cases = []
    for number, value in enumerate(values):
        for modulus in [0, 1, 2, 15, 2**63, 2**64-59, 2**64, 2**127-1]:
            for exponent in [0, 1, -1, 17]:
                cases.append(dict(First=str(value), Second=str(values[(number+3) % len(values)]),
                                  Base=str(-value if number % 2 else value), Exponent=str(exponent), Modulus=str(modulus)))
    rng = random.Random(0x6D6F64756C6172)
    for bits in [7, 31, 32, 33, 62, 63, 64, 65, 127, 128, 129, 255, 256, 257, 1024, 2048, 4096]:
        for turn in range(4):
            first = rng.getrandbits(bits)
            second = rng.getrandbits(bits)
            modulus = rng.getrandbits(bits) | 1
            exponent = (2**64 + 1) if turn == 0 else rng.getrandbits(17)
            cases.append(dict(First=str(first), Second=str(second), Base=str(-first if turn & 1 else first),
                              Exponent=str(-exponent if turn & 2 else exponent), Modulus=str(modulus)))
    return cases


def optional_power(base: int, exponent: int, modulus: int) -> str | None:
    """Preserve the mathematical absence of a negative modular power instead of inventing a residue."""
    try:
        return str(pow(base, exponent, modulus))
    except ValueError:
        return None


def verify(observations: list[dict], cases: list[dict]) -> None:
    """Check every arithmetic result independently and accept any valid exact Bezout coefficient pair."""
    assert len(observations) == len(cases)
    for observed, supplied in zip(observations, cases):
        assert observed['input'] == supplied
        first, second, base, exponent, modulus = (int(supplied[key]) for key in ['First', 'Second', 'Base', 'Exponent', 'Modulus'])
        divisor = math.gcd(first, second)
        assert int(observed['GCD']) == divisor
        assert first * int(observed['FirstCoefficient']) + second * int(observed['SecondCoefficient']) == divisor
        if modulus == 0:
            assert observed['PolicyRefusal'] and observed['ProductResidue'] is None and observed['PowerResidue'] is None
            continue
        assert observed['ProductResidue'] == str(first * second % modulus)
        assert observed['PowerResidue'] == optional_power(base, exponent, modulus)
        assert observed['InverseResidue'] == optional_power(base, -1, modulus)
        assert int(observed['InverseGCD']) == math.gcd(base, modulus)


def main() -> None:
    """Keep reference generation explicit, bounded and separate from any candidate source qualification."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--inputs-only', action='store_true')
    parser.add_argument('--go', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    cases = inputs()
    if args.inputs_only:
        args.output.write_text(json.dumps(cases, separators=(',', ':')) + '\n')
        return
    assert args.go is not None
    pins = json.loads((HERE / 'surface.json').read_text())['reference_pins']
    sdk = args.go.resolve().parent.parent
    assert digest(args.go) == pins['binary_sha256']
    for relative, expected in pins['sources'].items():
        assert digest(sdk / relative) == expected
    env = {**os.environ, 'GOROOT': str(sdk), 'GOCACHE': '/tmp/cinch-go-surface-cache', 'GOMAXPROCS': '1', 'GOTELEMETRY': 'off'}
    version = subprocess.run([str(args.go), 'version'], env=env, check=True, capture_output=True, text=True).stdout.strip()
    result = subprocess.run([str(args.go), 'run', str(HERE / 'oracle.go')], input=json.dumps(cases), env=env,
                            check=True, capture_output=True, text=True, timeout=120)
    observations = json.loads(result.stdout)
    verify(observations, cases)
    output = dict(scope='Actual SDK arithmetic independently crosschecked; candidate library remains separately unqualified',
                  reference_version=version, reference_binary_sha256=digest(args.go), reference_sources=pins['sources'],
                  oracle_sha256=digest(HERE / 'oracle.go'), generator_sha256=digest(Path(__file__)), cases=observations)
    raw = json.dumps(output, separators=(',', ':')) + '\n'
    assert len(raw.encode()) <= 4 * 1024 * 1024
    args.output.write_text(raw)


if __name__ == '__main__':
    main()
