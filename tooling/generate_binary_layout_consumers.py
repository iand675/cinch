#!/usr/bin/env python3
"""Generate concrete public-API consumers for every fixed binary arithmetic carrier.

The type table selects actual built-in layouts and exact bit constructors. Test
expectations remain in independent SDK fixtures, rather than in this generator.
"""
from __future__ import annotations

import argparse
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT / "seed/tests/fixtures/binary-layout-consumers.cinchpart"

# Field names and typed bit views keep every concrete layout exercised.
KINDS = [
    ("bool", "Bool", "(\\bits -> not (eq-u64 bits 0))", "(\\value -> if value then 1 else 0)"),
    ("u8", "U8", "wrapping-u64-to-u8", "wrapping-u8-to-u64"),
    ("i8", "I8", "(\\bits -> wrapping-u8-to-i8 (wrapping-u64-to-u8 bits))", "(\\value -> wrapping-u8-to-u64 (wrapping-i8-to-u8 value))"),
    ("u16", "U16", "wrapping-u64-to-u16", "wrapping-u16-to-u64"),
    ("i16", "I16", "(\\bits -> wrapping-u16-to-i16 (wrapping-u64-to-u16 bits))", "(\\value -> wrapping-u16-to-u64 (wrapping-i16-to-u16 value))"),
    ("u32", "U32", "wrapping-u64-to-u32", "wrapping-u32-to-u64"),
    ("i32", "I32", "(\\bits -> wrapping-u32-to-i32 (wrapping-u64-to-u32 bits))", "(\\value -> wrapping-u32-to-u64 (wrapping-i32-to-u32 value))"),
    ("u64", "U64", "(\\bits -> bits)", "(\\value -> value)"),
    ("i64", "I64", "wrapping-u64-to-i64", "wrapping-i64-to-u64"),
    ("f32", "F32", "(\\bits -> B.binary-f32-from-bits (wrapping-u64-to-u32 bits))", "(\\value -> wrapping-u32-to-u64 (B.binary-f32-bits value))"),
    ("f64", "F64", "(\\bits -> B.binary-f64-from-bits bits)", "(\\value -> B.binary-f64-bits value)"),
]

BASE = r'''-- | Concrete independent binary-layout fixture consumers.
module Main where
  import Text.Binary.Layout as B
  import Effect.Binary as E
  import Effect.Stream
  import Text.Hex
  import Data.Complex as Complex
  import Data.Bool
  import Data.Bytes
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  -- | Parse exact independently recorded bytes.
  input :: String -> Bytes
  input text = case hex-decode text of Ok bytes -> bytes; Err error -> bytes-new 0
  -- | Compare all bytes, including exact lengths.
  same :: Bytes -> Bytes -> Bool
  same left right = eq-i64 (bytes-compare left right) 0
  -- | Conjoin every runtime observation.
  all :: List Bool -> Bool
  all values = case values of Nil -> True; Cons first rest -> first && all rest
  -- | Keep large fixture construction outside the compiler's constant evaluator.
  prim runtime-size :: U32 -> U32 where
    llvm {
      %value = or i32 %0, 0
      ret i32 %value
    }
  -- | Preserve a bounded count as an exact opaque runtime value.
  runtime-count :: Nat -> Nat
  runtime-count n = if n <= 65536 then u32-to-nat (runtime-size (wrapping-i64-to-u32 (nat-to-i64 n))) else 0
  -- | Snapshot a checked half-open interval for independent observations.
  part :: Bytes -> Nat -> Nat -> Bytes
  part bytes start end = if start <= end && end <= bytes-len bytes then
    let out = bytes-new (end - start) in let copied = bytes-copy-range out bytes 0 start (end - start) in out
    else bytes-new 0
  -- | Join independent expected buffers without retaining aliases.
  joined :: Bytes -> Bytes -> Bytes
  joined first last = let n = bytes-len first in let out = bytes-new (n + bytes-len last) in
    if n <= bytes-len out then
      let a = bytes-copy-range out first 0 0 n in let b = bytes-copy-range out last n 0 (bytes-len last) in out
    else out
  -- | Select one of the three concrete public order values.
  order :: U8 -> B.BinaryOrder
  order tag = if eq-u8 tag 0 then B.BinaryLittleEndian else if eq-u8 tag 1 then B.BinaryBigEndian else B.BinaryNativeEndian
  -- | Select exact independent arithmetic patterns by runtime position, without floating arithmetic.
  pattern-bits :: U8 -> Nat -> U64
  pattern-bits kind i =
    if eq-u8 kind 9 || eq-u8 kind 11 then
      let n = nat-rem i 11 in
      if n == 0 then 0 else if n == 1 then 2147483648 else if n == 2 then 1 else if n == 3 then 2147483649 else if n == 4 then 1065353216 else if n == 5 then 2139095039 else if n == 6 then 2139095040 else if n == 7 then 4286578688 else if n == 8 then 2143289345 else if n == 9 then 4290847557 else 2139095041
    else if eq-u8 kind 10 || eq-u8 kind 12 then
      let n = nat-rem i 11 in
      if n == 0 then 0 else if n == 1 then 9223372036854775808 else if n == 2 then 1 else if n == 3 then 9223372036854775809 else if n == 4 then 4607182418800017408 else if n == 5 then 9218868437227405311 else if n == 6 then 9218868437227405312 else if n == 7 then 18442240474082181120 else if n == 8 then 9221120237041090561 else if n == 9 then 18444512289894210236 else 9218868437227405313
    else
      let n = nat-rem i 15 in
      if n == 0 then 0 else if n == 1 then 1 else if n == 2 then 127 else if n == 3 then 128 else if n == 4 then 255 else if n == 5 then 256 else if n == 6 then 32767 else if n == 7 then 32768 else if n == 8 then 65535 else if n == 9 then 2147483647 else if n == 10 then 2147483648 else if n == 11 then 4294967295 else if n == 12 then 9223372036854775807 else if n == 13 then 9223372036854775808 else 18446744073709551615
  -- | Normalize input bits to their actual fixed arithmetic carrier.
  carrier-bits :: U8 -> U64 -> U64
  carrier-bits kind bits = if eq-u8 kind 0 then (if eq-u64 bits 0 then 0 else 1)
    else if eq-u8 kind 1 || eq-u8 kind 2 then bit-and-u64 bits 255
    else if eq-u8 kind 3 || eq-u8 kind 4 then bit-and-u64 bits 65535
    else if eq-u8 kind 5 || eq-u8 kind 6 || eq-u8 kind 9 || eq-u8 kind 11 then bit-and-u64 bits 4294967295
    else bits
  -- | Build concrete typed list values in a bounded-stack accumulator.
  make-values :: forall a. (U64 -> a) -> U8 -> Nat -> Nat -> List a -> List a
  make-values from-bits kind remaining i reversed = if remaining == 0 then reverse reversed else
    let bits = pattern-bits kind i in let value = from-bits bits in let values = Cons value reversed in
    let left = remaining - 1 in let next = i + 1 in make-values from-bits kind left next values
  -- | Check every decoded element through exact raw-bit views rather than NaN equality.
  check-values :: forall a. (a -> U64) -> U8 -> List a -> Nat -> Nat -> Bool
  check-values bits kind values i count = case values of
    Nil -> i == count
    Cons first rest -> if i < count then
      let actual = bits first in let expected = carrier-bits kind (pattern-bits kind i) in
      if eq-u64 actual expected then let next = i + 1 in check-values bits kind rest next count else False
      else False
  -- | Uniform raw observations of a concretely typed scalar result.
  data DecodedBits where
    -- | Real bits, imaginary bits and consumed count.
    DecodedBits :: U64 -> U64 -> Nat -> DecodedBits
'''

TAIL = r'''
  -- | Compare every scalar encoder, fixed size, offset decoder and copied-prefix operation with exact independent bytes.
  scalar-case :: U8 -> U8 -> U64 -> U64 -> Nat -> String -> Bool
  scalar-case kind selected bits imaginary size expected =
    let wire-order = order selected in let raw = input expected in
    case encode-scalar kind wire-order bits imaginary of
      Err error -> False
      Ok encoded -> same encoded raw && scalar-size kind == size
        && (case decode-scalar kind wire-order (joined (input "aabb") (joined raw (input "ccdd"))) 2 of
          Err error -> False
          Ok (DecodedBits actual imag used) -> eq-u64 actual (carrier-bits kind bits)
            && eq-u64 imag (if eq-u8 kind 11 || eq-u8 kind 12 then carrier-bits kind imaginary else 0) && used == size)
        && scalar-buffer-case kind wire-order bits imaginary raw
  -- | Compare a homogeneous block plus its fringe with independent full bytes or a full-output digest.
  array-case :: U8 -> U8 -> Nat -> Nat -> String -> String -> Bool
  array-case kind selected count size expected hash =
    let n = runtime-count count in let wire-order = order selected in
    case encode-array kind wire-order n of
      Err error -> False
      Ok encoded -> bytes-len encoded == size && same (sha256-bytes encoded) (input hash)
        && (if count <= 65 then same encoded (input expected) else True)
        && decode-array kind wire-order n encoded size
'''


def call(function: str, argument: str) -> str:
    """Apply a typed conversion without relying on untyped numeric coercions."""
    return f"({function} {argument})"


def generated() -> str:
    """Render concrete scalar and homogeneous-array paths using actual public layouts."""
    out = BASE
    encodes, decodes, sizes, buffers, arrays, array_decodes = [], [], [], [], [], []
    for tag, (name, type_name, from_bits, to_bits) in enumerate(KINDS):
        layout = "B.binary-" + name
        value = call(from_bits, "bits")
        observed = call(to_bits, "value")
        prefix = "if" if tag == 0 else "else if"
        encodes.append(f"    {prefix} eq-u8 kind {tag} then B.binary-encode {layout} wire-order {value}")
        decodes.append(f"    {prefix} eq-u8 kind {tag} then (case B.binary-decode-at {layout} wire-order input start of Err error -> Err error; Ok pair -> case pair of (value, used) -> Ok (DecodedBits {observed} 0 used))")
        sizes.append(f"    {prefix} eq-u8 kind {tag} then B.binary-size {layout}")
        buffers.append(f"    {prefix} eq-u8 kind {tag} then check-buffer {layout} wire-order {value} expected")
        arrays.append(f"    {prefix} eq-u8 kind {tag} then let values = make-values {from_bits} kind count 0 Nil in B.binary-encode (B.binary-array count {layout}) wire-order values")
        array_decodes.append(f"    {prefix} eq-u8 kind {tag} then (case B.binary-decode (B.binary-array count {layout}) wire-order input of Err error -> False; Ok pair -> case pair of (values, used) -> used == size && check-values {to_bits} kind values 0 count)")
    for tag, name, component, constructor, observer in [(11, "complex64", "F32", "(\\word -> B.binary-f32-from-bits (wrapping-u64-to-u32 word))", "(\\value -> wrapping-u32-to-u64 (B.binary-f32-bits value))"), (12, "complex128", "F64", "B.binary-f64-from-bits", "B.binary-f64-bits")]:
        layout = "B.binary-" + name
        value = f"({call(constructor, 'bits')}, {call(constructor, 'imaginary')})"
        encodes.append(f"    else if eq-u8 kind {tag} then B.binary-encode {layout} wire-order {value}")
        decodes.append(f"    else if eq-u8 kind {tag} then (case B.binary-decode-at {layout} wire-order input start of Err error -> Err error; Ok pair -> case pair of (value, used) -> case value of (real, imag) -> Ok (DecodedBits {call(observer, 'real')} {call(observer, 'imag')} used))")
        sizes.append(f"    else if eq-u8 kind {tag} then B.binary-size {layout}")
        buffers.append(f"    else if eq-u8 kind {tag} then check-buffer {layout} wire-order {value} expected")
    sections = [
        ("Encode concrete scalar carriers without reflective type guessing.", "encode-scalar", "U8 -> B.BinaryOrder -> U64 -> U64 -> Result B.BinaryLayoutError Bytes", "kind wire-order bits imaginary", encodes, "    else Err (B.BinaryLengthMismatch 13 0)"),
        ("Observe concrete decoded scalar bits and exact consumed counts.", "decode-scalar", "U8 -> B.BinaryOrder -> Bytes -> Nat -> Result B.BinaryLayoutError DecodedBits", "kind wire-order input start", decodes, "    else Err (B.BinaryLengthMismatch 13 0)"),
        ("Observe actual descriptor size for each concrete arithmetic layout.", "scalar-size", "U8 -> Nat", "kind", sizes, "    else 0"),
        ("Exercise public buffer and append APIs for every concrete scalar layout.", "scalar-buffer-case", "U8 -> B.BinaryOrder -> U64 -> U64 -> Bytes -> Bool", "kind wire-order bits imaginary expected", buffers, "    else False"),
        ("Encode each concrete homogeneous list through its actual array descriptor.", "encode-array", "U8 -> B.BinaryOrder -> Nat -> Result B.BinaryLayoutError Bytes", "kind wire-order count", arrays, "    else Err (B.BinaryLengthMismatch 11 0)"),
        ("Check every concrete decoded array element including exact float payloads.", "decode-array", "U8 -> B.BinaryOrder -> Nat -> Bytes -> Nat -> Bool", "kind wire-order count input size", array_decodes, "    else False"),
    ]
    for doc, name, signature, args, lines, fallback in sections:
        out += f"\n  -- | {doc}\n  {name} :: {signature}\n  {name} {args} =\n" + "\n".join(lines + [fallback]) + "\n"
    out += r'''
  -- | Validate exact-size caller-buffer writes, untouched margins, append copies and short-buffer atomicity.
  check-buffer :: forall a. B.BinaryLayout a -> B.BinaryOrder -> a -> Bytes -> Bool
  check-buffer layout wire-order value expected =
    let size = B.binary-size layout in let prefix = input "aabb" in let destination = bytes-new (size + 4) in
    (case B.binary-append layout wire-order prefix value of Ok out -> same out (joined prefix expected); Err error -> False)
      && (case B.binary-encode-into layout wire-order destination 2 value of
        Ok used -> used == size && same (part destination 2 (2 + size)) expected && bytes-get destination 0 == 0 && bytes-get destination (size + 3) == 0
        Err error -> False)
      && (if size > 0 then let short = bytes-new (size - 1) in
        case B.binary-encode-into layout wire-order short 0 value of
          Err (B.BinaryShortDestination required available) -> required == size && available + 1 == size && same short (bytes-new (size - 1))
          other -> False
        else True)
'''
    return out + TAIL


def main() -> None:
    """Write the complete checked consumer fixture, or verify that committed generation is current."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    expected = generated()
    if args.check:
        if not TARGET.exists() or TARGET.read_text() != expected:
            parser.exit(1, f"Generated binary layout consumers are stale: {TARGET}\n")
    else:
        TARGET.write_text(expected)


if __name__ == "__main__":
    main()
