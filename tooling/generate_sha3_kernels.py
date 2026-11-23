#!/usr/bin/env python3
"""Generate the pure Keccak-f[1600] permutation and rate-specific absorb steps.

Lane (x, y) is field x + 5y of the private 25-word constructor. Each round
computes the five column parities and the five theta words with tiny
primitives, then every output lane with one fused theta/rho/pi/chi primitive
over the three source lanes chi combines. The repeated rotations of a source
lane are identical pure expressions, so after inlining LLVM's value numbering
leaves one rotation per lane: the result is the standard register-resident
permutation. A single sponge is inherently sequential across rounds, so its
best single-message form is 64-bit scalar code with native rotates; the
fragments call only the funnel-shift intrinsic and perform no memory access.
Round constants come from the specification's linear feedback shift register.
"""
from __future__ import annotations

import argparse
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MARKER = "  -- | Generated Keccak-f[1600] permutation: twenty-four rounds over twenty-five private lanes.\n"
MASK64 = (1 << 64) - 1
RATES_IN_WORDS = [9, 13, 17, 18, 21]
# Three rounds (105 bindings) per function stay below the elaborator's 128-deep let limit.
PER_PHASE = 3
PHASES = "abcdefgh"
ROTATIONS = [[0, 36, 3, 41, 18], [1, 44, 10, 45, 2], [62, 6, 43, 15, 61], [28, 55, 25, 21, 56], [27, 20, 39, 8, 14]]


def round_constants() -> list[int]:
    """Derive the 24 iota constants from the degree-8 LFSR x^8 + x^6 + x^5 + x^4 + 1."""
    state = 1
    def bit() -> int:
        nonlocal state
        out = state & 1
        state = ((state << 1) ^ (0x71 if state & 0x80 else 0)) & 0xFF
        return out
    constants = []
    for _ in range(24):
        value = 0
        for j in range(7):
            if bit():
                value |= 1 << ((1 << j) - 1)
        constants.append(value)
    assert constants[0] == 1 and constants[23] == 0x8000000080008008
    return constants


def source(x: int, y: int) -> tuple[int, int]:
    """Return the pre-pi lane (x', y') that pi moves to position (x, y): x = y' and y = 2x' + 3y'."""
    yp = x
    xp = (3 * (y - 3 * x)) % 5
    assert (2 * xp + 3 * yp) % 5 == y
    return xp, yp


def rotl(value: int, amount: int) -> int:
    """Rotate a 64-bit lane left."""
    return ((value << amount) | (value >> (64 - amount))) & MASK64 if amount else value


def permute(lanes: list[int]) -> list[int]:
    """Reference permutation for self-checks."""
    a = lanes[:]
    for rc in round_constants():
        c = [a[x] ^ a[x + 5] ^ a[x + 10] ^ a[x + 15] ^ a[x + 20] for x in range(5)]
        d = [c[(x - 1) % 5] ^ rotl(c[(x + 1) % 5], 1) for x in range(5)]
        b = [0] * 25
        for x in range(5):
            for y in range(5):
                b[y + 5 * ((2 * x + 3 * y) % 5)] = rotl(a[x + 5 * y] ^ d[x], ROTATIONS[x][y])
        a = [b[x + 5 * y] ^ (~b[(x + 1) % 5 + 5 * y] & b[(x + 2) % 5 + 5 * y]) for y in range(5) for x in range(5)]
        a = [a[i] if i else a[0] ^ rc for i in range(25)]
    return a


def self_check() -> None:
    """Check the reference against the standard library's SHA3-256 of the empty string."""
    import hashlib
    lanes = [0] * 25
    lanes[0] ^= 0x06
    lanes[16] ^= 0x80 << 56
    out = b"".join(w.to_bytes(8, "little") for w in permute(lanes))[:32]
    assert out == hashlib.sha3_256(b"").digest()


def prim(name: str, types: list[str], doc: str, body: list[str]) -> str:
    """Render one documented private LLVM primitive."""
    text = "\n".join(f"      {line}" for line in body)
    return f"  -- | {doc}\n  prim {name} :: " + " -> ".join(types) + f" where\n    llvm {{\n{text}\n    }}\n"


def generated() -> str:
    """Generate the permutation in eight three-round phases, the absorb steps and private primitives."""
    lanes = " ".join(f"l{i}" for i in range(25))
    lines = [MARKER.rstrip(), "  -- Regenerate with python3 tooling/generate_sha3_kernels.py; do not edit this suffix.",
             "  keccak-permute :: KeccakLanes -> KeccakLanes",
             "  keccak-permute lanes = " + "".join(f"keccak-rounds-{letter} (" for letter in reversed(PHASES)) + "lanes" + ")" * len(PHASES), ""]
    constants = round_constants()
    for phase, letter in enumerate(PHASES):
        lines += [f"  -- | Apply rounds {phase * PER_PHASE} through {phase * PER_PHASE + PER_PHASE - 1} of the permutation.",
                  f"  keccak-rounds-{letter} :: KeccakLanes -> KeccakLanes",
                  f"  keccak-rounds-{letter} lanes = case lanes of",
                  f"    Lanes {lanes} ->"]
        current = [f"l{i}" for i in range(25)]
        for r in range(PER_PHASE):
            index = phase * PER_PHASE + r
            tag = f"r{r}"
            for x in range(5):
                lines.append(f"      let c{tag}x{x} = keccak-parity {' '.join(current[x + 5 * y] for y in range(5))} in")
            for x in range(5):
                lines.append(f"      let d{tag}x{x} = keccak-theta c{tag}x{(x - 1) % 5} c{tag}x{(x + 1) % 5} in")
            nxt = []
            for y in range(5):
                for x in range(5):
                    args = []
                    for offset in range(3):
                        sx, sy = source((x + offset) % 5, y)
                        args += [current[sx + 5 * sy], f"d{tag}x{sx}", str(ROTATIONS[sx][sy])]
                    name = f"a{tag}y{y}x{x}"
                    if x == 0 and y == 0:
                        lines.append(f"      let {name} = keccak-chi-iota {' '.join(args)} {constants[index]} in")
                    else:
                        lines.append(f"      let {name} = keccak-chi {' '.join(args)} in")
                    nxt.append(name)
            current = nxt
        lines += ["      Lanes " + " ".join(current), ""]
    for words in RATES_IN_WORDS:
        lines += [f"  -- | XOR one proved complete {words * 8}-byte block of little-endian words into the rate lanes, then permute.",
                  f"  keccak-absorb{words} :: KeccakLanes -> I64 -> KeccakLanes",
                  f"  keccak-absorb{words} lanes address = case lanes of",
                  f"    Lanes {lanes} ->",
                  "      keccak-permute (Lanes " + " ".join(f"(bit-xor-u64 l{i} (lane-word address {i * 8}))" if i < words else f"l{i}" for i in range(25)) + ")", ""]
    lines.append(prim("keccak-parity", ["U64"] * 6, "Column parity: the exclusive-or of the five lanes in one column.",
                      ["%x0 = xor i64 %0, %1", "%x1 = xor i64 %x0, %2", "%x2 = xor i64 %x1, %3", "%value = xor i64 %x2, %4", "ret i64 %value"]))
    lines.append(prim("keccak-theta", ["U64"] * 3, "Theta word for a column from the preceding column's parity and the following column's parity rotated by one.",
                      ["%r = call i64 @llvm.fshl.i64(i64 %1, i64 %1, i64 1)", "%value = xor i64 %0, %r", "ret i64 %value"]))
    chi = []
    for k in range(3):
        a, d, r = 3 * k, 3 * k + 1, 3 * k + 2
        chi += [f"%t{k} = xor i64 %{a}, %{d}", f"%b{k} = call i64 @llvm.fshl.i64(i64 %t{k}, i64 %t{k}, i64 %{r})"]
    chi += ["%n1 = xor i64 %b1, -1", "%m = and i64 %n1, %b2", "%chi = xor i64 %b0, %m"]
    lines.append(prim("keccak-chi", ["U64"] * 10,
                      "One output lane: theta, rho and pi applied to the three source lanes, combined by chi as b0 xor (not b1 and b2).",
                      chi + ["ret i64 %chi"]))
    lines.append(prim("keccak-chi-iota", ["U64"] * 11, "Output lane (0, 0), which additionally absorbs the round constant.",
                      chi + ["%value = xor i64 %chi, %9", "ret i64 %value"]))
    return "\n".join(lines)


def main() -> None:
    """Write or verify the generated suffix while preserving the handwritten checked public API."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--module", type=Path, default=ROOT / "seed/prelude/Text/Sha3.cinch")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    self_check()
    existing = args.module.read_text()
    if existing.count(MARKER) > 1:
        parser.error("the module contains duplicate generated-core markers")
    if MARKER not in existing and "  keccak-permute ::" in existing:
        parser.error("a permutation exists without its generation marker; refusing to append another")
    expected = existing.split(MARKER, 1)[0].rstrip() + "\n\n" + generated()
    if args.check:
        if existing != expected:
            parser.exit(1, f"Generated Keccak core is stale: {args.module}\n")
    elif existing != expected:
        args.module.write_text(expected)


if __name__ == "__main__":
    main()
