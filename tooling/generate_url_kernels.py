#!/usr/bin/env python3
"""Generate scalar/SIMD URL byte-policy classifiers and a percent-spelling table.

The policies distinguish complete paths, path segments, hosts, zones, userinfo,
query components and fragments. Encoded-path/fragment hints additionally accept
the pchar spellings that may be preserved verbatim. No generated primitive reads
or writes memory; checked source code owns all buffer access and mutation.
"""
from __future__ import annotations

import argparse
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MARKER = "  -- | Generated URL byte-policy kernels; regenerate with tooling/generate_url_kernels.py.\n"


def allowed(byte: int, mode: int) -> bool:
    """Return the exact ASCII policy used by one context, independently of LLVM emission."""
    basic = 65 <= byte <= 90 or 97 <= byte <= 122 or 48 <= byte <= 57 or byte in b"-_.~"
    sets = {
        1: b"$&+,/:;=@",
        2: b"$&+:=@",
        3: b"!\"$&'()*+,:;=[]<>",
        4: b"!\"$&'()*+,:;=[]<>",
        5: b"$&+,;=",
        6: b"",
        7: b"$&+,/:;=?@!()*",
        8: b"$&+,/:;=@!'()*[]%",
        9: b"$&+,/:;=?@!'()*[]%",
    }
    return basic or byte in sets.get(mode, b"")


class Instructions:
    """Build uniquely named scalar or eight-byte SIMD policy instructions."""

    def __init__(self, vector: bool) -> None:
        """Select scalar/vector byte and predicate types for one fragment."""
        self.vector = vector
        self.byte = "<8 x i8>" if vector else "i8"
        self.boolean = "<8 x i1>" if vector else "i1"
        self.lines: list[str] = []
        self.next = 0

    def emit(self, expression: str) -> str:
        """Append one named expression and return its SSA reference."""
        name = f"%v{self.next}"
        self.next += 1
        self.lines.append(f"      {name} = {expression}")
        return name

    def byte_literal(self, value: int) -> str:
        """Render one unsigned byte or a complete constant splat."""
        return "<" + ", ".join(f"i8 {value}" for _ in range(8)) + ">" if self.vector else str(value)

    def eq(self, byte: str, value: int) -> str:
        """Compare every byte independently with one ASCII constant."""
        return self.emit(f"icmp eq {self.byte} {byte}, {self.byte_literal(value)}")

    def any(self, values: list[str]) -> str:
        """Combine nonempty scalar/vector predicates without changing lane identity."""
        result = values[0]
        for value in values[1:]:
            result = self.emit(f"or {self.boolean} {result}, {value}")
        return result

    def policy(self) -> tuple[str, str, str, str]:
        """Return input bytes, selected allowed mask, query-space mask and query-plus mask."""
        byte = self.emit("bitcast i64 %0 to <8 x i8>") if self.vector else "%0"
        lower = self.emit(f"or {self.byte} {byte}, {self.byte_literal(32)}")
        lo = self.emit(f"icmp uge {self.byte} {lower}, {self.byte_literal(97)}")
        hi = self.emit(f"icmp ule {self.byte} {lower}, {self.byte_literal(122)}")
        alpha = self.emit(f"and {self.boolean} {lo}, {hi}")
        lo = self.emit(f"icmp uge {self.byte} {byte}, {self.byte_literal(48)}")
        hi = self.emit(f"icmp ule {self.byte} {byte}, {self.byte_literal(57)}")
        digit = self.emit(f"and {self.boolean} {lo}, {hi}")
        comparisons = {value: self.eq(byte, value) for value in sorted(set(b"-_.~$&+,/:;=?@!\"'()*[]<>% "))}
        basic = self.any([alpha, digit] + [comparisons[value] for value in b"-_.~"])
        sets = {1: b"$&+,/:;=@", 2: b"$&+:=@", 3: b"!\"$&'()*+,:;=[]<>", 4: b"!\"$&'()*+,:;=[]<>", 5: b"$&+,;=", 6: b"", 7: b"$&+,/:;=?@!()*", 8: b"$&+,/:;=@!'()*[]%", 9: b"$&+,/:;=?@!'()*[]%"}
        selected = basic
        for mode, extra in sets.items():
            mask = self.any([basic] + [comparisons[value] for value in extra])
            condition = self.emit(f"icmp eq i8 %1, {mode}")
            selected = self.emit(f"select i1 {condition}, {self.boolean} {mask}, {self.boolean} {selected}")
        query = self.emit("icmp eq i8 %1, 6")
        false = "zeroinitializer" if self.vector else "false"
        space = self.emit(f"select i1 {query}, {self.boolean} {comparisons[32]}, {self.boolean} {false}")
        plus = self.emit(f"select i1 {query}, {self.boolean} {comparisons[43]}, {self.boolean} {false}")
        return byte, selected, space, plus

    def body(self) -> str:
        """Render one complete pure LLVM fragment."""
        return "    llvm {\n" + "\n".join(self.lines) + "\n    }\n"


def classifier(vector: bool, decoding: bool = False) -> str:
    """Emit scalar action, packed encoder counts or decoder-special byte masks."""
    ir = Instructions(vector)
    byte, permitted, spaces, plus = ir.policy()
    true = "<" + ", ".join("i1 true" for _ in range(8)) + ">" if vector else "true"
    denied = ir.emit(f"xor {ir.boolean} {permitted}, {true}")
    if decoding:
        percent = ir.eq(byte, 37)
        ascii = ir.emit(f"icmp ult {ir.byte} {byte}, {ir.byte_literal(128)}")
        illegal = ir.emit(f"and {ir.boolean} {ascii}, {denied}")
        host = ir.emit("icmp eq i8 %1, 3")
        zone = ir.emit("icmp eq i8 %1, 4")
        context = ir.emit(f"or i1 {host}, {zone}")
        illegal = ir.emit(f"select i1 {context}, {ir.boolean} {illegal}, {ir.boolean} zeroinitializer")
        special = ir.any([percent, plus, illegal])
        value = ir.emit(f"bitcast <8 x i1> {special} to i8")
        ir.lines.append(f"      ret i8 {value}")
    elif not vector:
        percent = ir.emit(f"select i1 {denied}, i8 2, i8 0")
        action = ir.emit(f"select i1 {spaces}, i8 1, i8 {percent}")
        ir.lines.append(f"      ret i8 {action}")
    else:
        no_space = ir.emit(f"xor <8 x i1> {spaces}, {true}")
        percent = ir.emit(f"and <8 x i1> {denied}, {no_space}")
        mask = ir.emit(f"bitcast <8 x i1> {percent} to i8")
        word = ir.emit(f"zext i8 {mask} to i64")
        shifted = ir.emit(f"lshr i64 {word}, 1")
        part = ir.emit(f"and i64 {shifted}, 85")
        pairs = ir.emit(f"sub i64 {word}, {part}")
        left = ir.emit(f"and i64 {pairs}, 51")
        shifted = ir.emit(f"lshr i64 {pairs}, 2")
        right = ir.emit(f"and i64 {shifted}, 51")
        fours = ir.emit(f"add i64 {left}, {right}")
        shifted = ir.emit(f"lshr i64 {fours}, 4")
        total = ir.emit(f"add i64 {fours}, {shifted}")
        count = ir.emit(f"and i64 {total}, 15")
        space_mask = ir.emit(f"bitcast <8 x i1> {spaces} to i8")
        any_space = ir.emit(f"icmp ne i8 {space_mask}, 0")
        flag = ir.emit(f"select i1 {any_space}, i64 9223372036854775808, i64 0")
        value = ir.emit(f"or i64 {count}, {flag}")
        ir.lines.append(f"      ret i64 {value}")
    return ir.body()


def generated() -> str:
    """Build all pure classifiers and exact uppercase percent triplets."""
    table = "".join(f"%{byte:02X}" for byte in range(256))
    return (MARKER + "  -- Scalar actions are 0 for ordinary bytes, 1 for query spaces and 2 for percent escaping.\n  prim classify-one :: U8 -> U8 -> U8 where\n" + classifier(False)
            + "\n  -- | Count percent expansions in eight independent SIMD lanes and flag query spaces in bit 63.\n  prim classify-eight :: U64 -> U8 -> U64 where\n" + classifier(True)
            + "\n  -- | Locate percent signs, query plus signs and invalid raw ASCII host bytes in eight SIMD lanes.\n  prim decode-flags-eight :: U64 -> U8 -> U8 where\n" + classifier(True, True)
            + "\n  -- | Prepare all uppercase percent triplets once for an operation that requires escaping.\n  percent-triplets :: Unit -> Bytes\n  percent-triplets unit = string-to-bytes \"" + table + "\"\n")


def main() -> None:
    """Replace or verify only the generated codec suffix, retaining the handwritten checked boundary."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--module", type=Path, default=ROOT / "seed/prelude/Text/Url/Codec.cinch")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    source = args.module.read_text()
    if source.count(MARKER) > 1:
        parser.error("duplicate generated URL kernel markers")
    expected = source.split(MARKER, 1)[0].rstrip() + "\n\n" + generated()
    if args.check:
        if source != expected:
            parser.exit(1, f"Generated URL codec is stale: {args.module}\n")
    elif source != expected:
        args.module.write_text(expected)


if __name__ == "__main__":
    main()
