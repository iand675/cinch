#!/usr/bin/env python3
"""Validate actual HKDF observations using independent scalar HMAC/HKDF and fixed RFC5869 data."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path


def digest(name: str, data: bytes, width: int) -> bytes:
    """Use an independent digest implementation or the exact test-only scalar streaming specification."""
    if name != 'toy':
        return hashlib.new(name, data).digest()
    state = 14695981039346656037
    for byte in data:
        state = ((state ^ byte)*1099511628211) % 18446744073709551616
    return bytes(((state >> ((index % 8)*8)) ^ (index*29)) & 255 for index in range(width))


def mac(name: str, width: int, block: int, key: bytes, data: bytes) -> bytes:
    """Apply exact pad/key-normalization rules without imposing a library's minimum block-size policy."""
    normalized = digest(name, key, width) if len(key) > block else key
    padded = (normalized+bytes(block))[:block]
    inner = bytes(value ^ 54 for value in padded)
    outer = bytes(value ^ 92 for value in padded)
    return digest(name, outer+digest(name, inner+data, width), width)


def expand(name: str, width: int, block: int, key: bytes, info: bytes, length: int) -> bytes:
    """Derive exact complete blocks with a one-based byte counter and the255-block upper bound."""
    if length < 0 or length > 255*width:
        raise ValueError('invalid independent expansion length')
    output = b''
    previous = b''
    for counter in range(1, 256):
        if len(output) == length:
            return output
        previous = mac(name, width, block, key, previous+info+bytes([counter]))
        output += previous[:length-len(output)]
    if len(output) != length:
        raise ValueError('independent counter bound exhausted')
    return output


def check_outcome(actual: dict, expected: bytes | None, failure: str | None) -> None:
    """Keep normal results, exact public length errors and recovered negative-length panics distinct."""
    if failure == 'negative':
        if actual['returned'] or not actual['panic'] or actual['output_hex'] or actual['error']:
            raise ValueError('negative SDK length did not retain its actual panic outcome')
    elif failure == 'length':
        if not actual['returned'] or not actual['nil'] or actual['output_hex'] or actual['panic'] or actual['error'] != 'hkdf: requested key length too large':
            raise ValueError('public too-large length error mismatch')
    elif not actual['returned'] or actual['error'] or actual['panic'] or actual['output_hex'] != expected.hex():
        raise ValueError('complete independent byte result mismatch')


def main() -> None:
    """Validate a baseline oracle run without compiling a reference or executing a candidate."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--observations', type=Path, required=True)
    parser.add_argument('--rfc', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--fips-only', action='store_true')
    args = parser.parse_args()
    raw = args.observations.read_bytes()
    data = json.loads(raw)
    if args.fips_only and (data['family'] != 'policy' or not data['fips_enabled']):
        raise ValueError('explicit policy-family FIPS-only process observation required')
    fixed = {row['id']: row for row in json.loads(args.rfc.read_text())['vectors']}
    checked = []
    for row in data['observations']:
        if row['retained_before'] != row['retained_after']:
            raise ValueError('actual SDK altered original input aliases')
        name, width, block = row['hash'], row['digest_size'], row['block_size']
        secret = bytes.fromhex(row['secret_hex'])
        salt = bytes(width) if row['salt_nil'] else bytes.fromhex(row['salt_hex'])
        key, info = bytes.fromhex(row['prk_hex']), bytes.fromhex(row['info_hex'])
        prk = mac(name, width, block, salt, secret)
        length = row['length']
        failure = 'length' if length > 255*width else 'negative' if length < 0 else None
        for operation, original in [('extract', secret), ('expand', key), ('key', secret)]:
            policy_error = None
            if args.fips_only:
                if len(original) < 14:
                    policy_error = 'crypto/hkdf: use of keys shorter than 112 bits is not allowed in FIPS 140-only mode'
                elif name not in ['sha256', 'sha512']:
                    policy_error = 'crypto/hkdf: use of hash functions other than SHA-2 or SHA-3 is not allowed in FIPS 140-only mode'
            actual = row[operation]
            if policy_error:
                if not actual['returned'] or not actual['nil'] or actual['output_hex'] or actual['panic'] or actual['error'] != policy_error:
                    raise ValueError('explicit original-key policy/length precedence mismatch')
            elif operation == 'extract':
                check_outcome(actual, prk, None)
            else:
                material = key if operation == 'expand' else prk
                check_outcome(actual, None if failure else expand(name, width, block, material, info, length), failure)
        if row['id'] in fixed:
            expected = fixed[row['id']]
            if row['extract']['output_hex'] != expected['prk_hex'] or row['expand']['output_hex'] != expected['output_hex'] or row['key']['output_hex'] != expected['output_hex']:
                raise ValueError('fixed RFC5869 observation mismatch')
        checked.append(row['id'])
    report = {'scope': __doc__, 'status': 'passed', 'oracle_sha256': hashlib.sha256(raw).hexdigest(),
              'rfc_sha256': hashlib.sha256(args.rfc.read_bytes()).hexdigest(),
              'runner_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
              'checked_ids': checked, 'checked_operations': len(checked)*3,
              'explicit_fips_only_policy': args.fips_only,
              'limits': 'Independent scalar/reference agreement only; no Cinch source/runtime or elapsed claim.'}
    args.output.write_text(json.dumps(report, indent=2)+'\n')
    print(len(checked)*3, 'complete independent HKDF observations agree')


if __name__ == '__main__':
    main()
