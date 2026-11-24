#!/usr/bin/env python3
"""Generate the DES block and key-schedule kernels of Crypto.Des.

The block kernel is one straight-line LLVM body per endianness. The initial
and final permutations are grouped into one mask, shift and OR per distinct
bit displacement. Each round forms the eight 6-bit expansion windows as
top bits of funnel-shift rotations of the right half, XORs the round key's
six-bit chunks (loaded as an `<8 x i8>` vector in memory order), and looks up
the combined S-box and P-permutation (SP) tables addressed through the key
schedule buffer. The key-schedule kernel applies PC-1, rotates both 28-bit
halves by a cumulative amount and applies PC-2, spreading the eight chunks
into the eight bytes of a word. The SP tables are emitted as a hexadecimal
constant of little-endian words; big-endian bodies swap each loaded entry.

    python3 tooling/generate_des_kernels.py          # rewrite the generated suffix
    python3 tooling/generate_des_kernels.py --check  # fail when it is stale
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from des_reference import FP, IP, PC1, PC2, sp_entry  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT / "seed/prelude/Crypto/Des.cinch"
MARKER = "  -- Generated DES kernels: permutation networks, rounds and key-schedule bodies.\n"


def moves(pairs: list[tuple[int, int]], value: str, prefix: str, out: list[str]) -> str:
    """OR together masked shifts moving bit positions src -> dst of a 64-bit value; returns the result name."""
    groups: dict[int, int] = {}
    for src, dst in pairs:
        groups[dst - src] = groups.get(dst - src, 0) | (1 << src)
    acc = None
    for n, (delta, mask) in enumerate(sorted(groups.items())):
        signed = mask - (1 << 64) if mask >= 1 << 63 else mask
        out.append(f"      %{prefix}m{n} = and i64 {value}, {signed}")
        term = f"%{prefix}m{n}"
        if delta > 0:
            out.append(f"      %{prefix}s{n} = shl i64 %{prefix}m{n}, {delta}")
            term = f"%{prefix}s{n}"
        elif delta < 0:
            out.append(f"      %{prefix}s{n} = lshr i64 %{prefix}m{n}, {-delta}")
            term = f"%{prefix}s{n}"
        if acc is None:
            acc = term
        else:
            out.append(f"      %{prefix}o{n} = or i64 {acc}, {term}")
            acc = f"%{prefix}o{n}"
    return acc


def block_body(big: bool) -> list[str]:
    """One whole-block body: arguments are the SP table address, the round-key address and a little-endian block word."""
    out = ["      %b = call i64 @llvm.bswap.i64(i64 %2)"]
    ip = moves([(64 - IP[i], 64 - (i + 1)) for i in range(64)], "%b", "ip", out)
    out += [f"      %lh = lshr i64 {ip}, 32", "      %l0 = trunc i64 %lh to i32", f"      %r0 = trunc i64 {ip} to i32",
            "      %sp = inttoptr i64 %0 to ptr", "      %kb = inttoptr i64 %1 to ptr"]
    left, right = "%l0", "%r0"
    for r in range(16):
        out.append(f"      %ka{r} = getelementptr i8, ptr %kb, i64 {8 * r}")
        out.append(f"      %kv{r} = load <8 x i8>, ptr %ka{r}, align 1")
        terms = []
        for j in range(8):
            rot = (4 * j - 1) % 32
            t = f"r{r}j{j}"
            src = right
            if rot:
                out.append(f"      %{t}rot = call i32 @llvm.fshl.i32(i32 {right}, i32 {right}, i32 {rot})")
                src = f"%{t}rot"
            out += [f"      %{t}w = lshr i32 {src}, 26",
                    f"      %{t}k8 = extractelement <8 x i8> %kv{r}, i64 {j}",
                    f"      %{t}k = zext i8 %{t}k8 to i32",
                    f"      %{t}x = xor i32 %{t}w, %{t}k",
                    f"      %{t}x64 = zext i32 %{t}x to i64",
                    f"      %{t}o = shl i64 %{t}x64, 2",
                    f"      %{t}off = add i64 %{t}o, {256 * j}",
                    f"      %{t}p = getelementptr i8, ptr %sp, i64 %{t}off",
                    f"      %{t}e = load i32, ptr %{t}p, align 1"]
            entry = f"%{t}e"
            if big:
                out.append(f"      %{t}be = call i32 @llvm.bswap.i32(i32 %{t}e)")
                entry = f"%{t}be"
            terms.append(entry)
        acc = terms[0]
        for j in range(1, 8):
            out.append(f"      %r{r}f{j} = xor i32 {acc}, {terms[j]}")
            acc = f"%r{r}f{j}"
        out.append(f"      %r{r}n = xor i32 {left}, {acc}")
        left, right = right, f"%r{r}n"
    out += [f"      %ph = zext i32 {right} to i64", "      %pu = shl i64 %ph, 32", f"      %pl = zext i32 {left} to i64",
            "      %pre = or i64 %pu, %pl"]
    fp = moves([(64 - FP[i], 64 - (i + 1)) for i in range(64)], "%pre", "fp", out)
    out += [f"      %res = call i64 @llvm.bswap.i64(i64 {fp})", "      ret i64 %res"]
    return out


def key_body() -> list[str]:
    """Round key from a little-endian key word and a cumulative rotation: chunk j of PC-2's output in byte j."""
    out = ["      %b = call i64 @llvm.bswap.i64(i64 %0)"]
    cd = moves([(64 - PC1[i], 56 - (i + 1)) for i in range(56)], "%b", "pc", out)
    out += [f"      %c = lshr i64 {cd}, 28", f"      %d = and i64 {cd}, 268435455",
            "      %inv = sub i64 28, %1",
            "      %cl = shl i64 %c, %1", "      %cr = lshr i64 %c, %inv", "      %co = or i64 %cl, %cr", "      %cm = and i64 %co, 268435455",
            "      %dl = shl i64 %d, %1", "      %dr = lshr i64 %d, %inv", "      %do = or i64 %dl, %dr", "      %dm = and i64 %do, 268435455",
            "      %ch = shl i64 %cm, 28", "      %rot = or i64 %ch, %dm"]
    pairs = []
    for i, src in enumerate(PC2):
        j, bit = divmod(i, 6)
        pairs.append((56 - src, 8 * j + 5 - bit))
    k = moves(pairs, "%rot", "pk", out)
    out.append(f"      ret i64 {k}")
    return out


def sp_hex() -> str:
    """The eight SP tables as little-endian 32-bit words, table j at byte 256j."""
    return "".join(sp_entry(j, x).to_bytes(4, "little").hex() for j in range(8) for x in range(64))


def suffix() -> str:
    """The generated text that follows the marker."""
    lines = [MARKER.rstrip("\n"), "",
             "  -- | The combined S-box and P-permutation tables: eight tables of 64 little-endian words.",
             "  sp-tables-hex :: String",
             f'  sp-tables-hex = "{sp_hex()}"', "",
             "  -- | Encrypt one block (as a little-endian word of its bytes) with the round keys at an address, in their order;",
             "  -- the SP tables are at the other address. Reversed round keys decrypt.",
             "  prim des-rounds :: I64 -> I64 -> U64 -> U64 where",
             "    unsafe llvm {"] + block_body(False) + ["    } when endian little", "    unsafe llvm {"] + block_body(True) + ["    } when endian big", "",
             "  -- | The round key for a cumulative rotation of both key halves, its eight six-bit chunks in the eight bytes of a word.",
             "  prim des-round-key :: U64 -> U64 -> U64 where",
             "    llvm {"] + key_body() + ["    }"]
    return "\n".join(lines) + "\n"


def main() -> None:
    """Rewrite or check the generated suffix of Crypto.Des."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    existing = TARGET.read_text()
    if existing.count(MARKER) != 1:
        raise SystemExit(f"{TARGET}: expected exactly one generated marker")
    expected = existing.split(MARKER, 1)[0].rstrip() + "\n\n" + suffix()
    if args.check:
        if existing != expected:
            raise SystemExit(f"{TARGET} is stale; rerun tooling/generate_des_kernels.py")
        return
    TARGET.write_text(expected)


if __name__ == "__main__":
    main()
