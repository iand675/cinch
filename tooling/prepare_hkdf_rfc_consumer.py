#!/usr/bin/env python3
"""Render SDK-free public HKDF checks from unchanged fixed RFC5869 expectations."""
from __future__ import annotations
import argparse
import json
from pathlib import Path

#: Public source caller compares complete outputs, never a truncated fingerprint.
PRELUDE = '''-- | Execute fixed public HKDF expectations and preserve complete output comparisons.
module Main where
  import Crypto.Hkdf
  import Data.Bool
  import Data.Bytes
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Text.Digest
  import Text.Hex

  -- | Compare every output octet under explicit checked byte-buffer bounds.
  same-from :: Bytes -> Bytes -> Nat -> Bool
  same-from left right index =
    if index < bytes-len left && index < bytes-len right then
      bytes-get left index == bytes-get right index && same-from left right (index + 1)
    else index == bytes-len left && index == bytes-len right

  -- | Reject all provider errors and compare the complete independently expected byte string.
  result-is :: Result (HkdfError Unit) Bytes -> Bytes -> Bool
  result-is actual expected = case actual of
    Err error -> False
    Ok output -> same-from output expected 0

  -- | Preserve absent and explicitly empty fixed salts without an unchecked parse fallback.
  decode-salt :: Option String -> Result HexError (Option Bytes)
  decode-salt salt = case salt of
    None -> Ok None
    Some input -> case hex-decode input of
      Err error -> Err error
      Ok output -> Ok (Some output)

  -- | Adapt the real pure digest to the generic callback contract.
  generic-hash :: Digest -> Bytes -> Result Unit Bytes
  generic-hash selector input = Ok (digest-bytes selector input)

  -- | Adapt the real dedicated HMAC primitive to the direct provider contract.
  direct-hmac :: Bytes -> Bytes -> Result Unit Bytes
  direct-hmac key input = Ok (hmac-sha256 key input)

  -- | Check all three direct-provider operations only for their actual SHA256 implementation.
  direct-row :: Digest -> Bytes -> Option Bytes -> Bytes -> Bytes -> Bytes -> Nat -> Bool
  direct-row selector secret salt info prk output count = case selector of
    DigestSha256 ->
      result-is (hkdf-extract-hmac (hkdf-digest-size selector) direct-hmac secret salt) prk &&
      result-is (hkdf-expand-hmac (hkdf-digest-size selector) direct-hmac prk info count) output &&
      result-is (hkdf-key-hmac (hkdf-digest-size selector) direct-hmac secret salt info count) output
    _ -> True

  -- | Compare native conveniences, generic hashing and applicable direct HMAC against fixed complete keys.
  decoded-row :: Digest -> Bytes -> Option Bytes -> Bytes -> Bytes -> Bytes -> Nat -> Bool
  decoded-row selector secret salt info prk output count =
    result-is (hkdf-extract selector secret salt) prk &&
    result-is (hkdf-expand selector prk info count) output &&
    result-is (hkdf-key selector secret salt info count) output &&
    result-is (hkdf-extract-with (hkdf-digest-hash selector) (generic-hash selector) secret salt) prk &&
    result-is (hkdf-expand-with (hkdf-digest-hash selector) (generic-hash selector) prk info count) output &&
    result-is (hkdf-key-with (hkdf-digest-hash selector) (generic-hash selector) secret salt info count) output &&
    direct-row selector secret salt info prk output count

  -- | Refuse a malformed fixed fixture before invoking any cryptographic operation.
  row :: Digest -> String -> Option String -> String -> String -> String -> Nat -> Bool
  row selector secret salt info prk output count = case hex-decode secret of
    Err error -> False
    Ok input -> case decode-salt salt of
      Err error -> False
      Ok key -> case hex-decode info of
        Err error -> False
        Ok context -> case hex-decode prk of
          Err error -> False
          Ok extracted -> case hex-decode output of
            Err error -> False
            Ok expected -> decoded-row selector input key context extracted expected count

'''


def main() -> None:
    """Copy fixed vectors into a transparent caller without executing or altering expectations."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--rfc', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    rows = json.loads(args.rfc.read_text())['vectors']
    assert len(rows) == 8
    source = PRELUDE
    for index, row in enumerate(rows, 1):
        selector = {'sha256': 'DigestSha256', 'sha1': 'DigestSha1'}[row['hash']]
        salt = 'None' if row['salt_hex'] is None else '(Some ' + json.dumps(row['salt_hex']) + ')'
        source += f"  -- | Check unchanged fixed vector {row['id']} through all applicable public routes.\n"
        source += f"  rfc{index} :: Unit -> Bool\n"
        source += f"  rfc{index} unit = row {selector} {json.dumps(row['secret_hex'])} {salt} {json.dumps(row['info_hex'])} {json.dumps(row['prk_hex'])} {json.dumps(row['output_hex'])} {row['length']}\n\n"
    source += '  -- | Return the first failing fixed-vector index; zero means all sixty complete output observations agree.\n  main :: Int\n  main =\n'
    source += '\n'.join(f"    {'if' if index == 1 else 'else if'} not (rfc{index} ()) then {index}" for index in range(1, 9))
    source += '\n    else 0\n'
    args.out.write_text(source)


if __name__ == '__main__':
    main()
