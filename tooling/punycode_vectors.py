#!/usr/bin/env python3
"""Write Punycode vectors from Python's bootstring codec (RFC 3492).

Rows encode labels (the RFC's sample strings, mixed-case ASCII, empty and all-ASCII labels, and seeded
random labels across the Basic Multilingual Plane and astral planes), decode the encodings back and decode
malformed inputs, which must be refused. Host rows convert each dot-separated label holding non-ASCII text
to "xn--" plus its encoding and keep ASCII labels as they are.

    python3 tooling/punycode_vectors.py > seed/tests/fixtures/punycode-vectors.json
"""
import json
import random

SAMPLES = [
    "ليهمابتكلموشعربي؟",
    "他们为什么不说中文",
    "Pročprostěnemluvíčesky",
    "למההםפשוטלאמדבריםעברית",
    "यहलोगहिन्दीक्यों",
    "なぜみんな日本語を話してくれないのか",
    "세계의모든사람들이한국어를이해한다면얼마나좋을까",
    "почемужеонинеговорятпорусски",
    "PorquénopuedensimplementehablarenEspañol",
    "TạisaohọkhôngthểchỉnóitiếngViệt",
    "3年B組金八先生",
    "安室奈美恵-with-SUPER-MONKEYS",
    "Hello-Another-Way-それぞれの場所",
    "ひとつ屋根の下2",
    "MajiでKoiする5秒前",
    "パフィーdeルンバ",
    "そのスピードで",
    "-> $1.00 <-",
    "bücher", "münchen", "BÜcher", "日本語", "abc", "", "-", "a-b-", "\U0001f600",
    "x\U0001f600y\U00010348z", "\U0010fffd", "éééé", "￿\u0080",
]

BAD = ["ab$c", "-!", "abc-9999999999", "zzzzzzzzzzzzzzzzzz", "a-é", "99999999999999999999999", "foo-b,ar"]

HOSTS = ["www.bücher.example", "München.test", "plain.example", "", ".", "a..b", "日本.中国", "trailing.", "x\U0001f600.y"]


def codepoints(rng: random.Random) -> str:
    """A random label of ASCII letters, digits and hyphens mixed with BMP and astral code points."""
    out = []
    for _ in range(rng.randrange(1, 24)):
        kind = rng.randrange(4)
        if kind == 0:
            out.append(rng.choice("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-"))
        elif kind == 1:
            out.append(chr(rng.randrange(0x80, 0x800)))
        elif kind == 2:
            c = rng.randrange(0x800, 0x10000)
            out.append(chr(c if not 0xD800 <= c < 0xE000 else c - 0x1000))
        else:
            out.append(chr(rng.randrange(0x10000, 0x110000)))
    return "".join(out)


def to_ascii(host: str) -> str:
    """Per-label conversion: non-ASCII labels become "xn--" plus their encoding."""
    return ".".join(l if l.isascii() else "xn--" + l.encode("punycode").decode() for l in host.split("."))


def main() -> None:
    rng = random.Random(3492)
    labels = SAMPLES + [codepoints(rng) for _ in range(40)]
    encode = [[s, s.encode("punycode").decode()] for s in labels]
    decode = [[e, s] for s, e in encode]
    for b in BAD:
        try:
            got = b.encode("utf-8").decode("punycode")
            assert False, f"{b!r} decoded to {got!r}"
        except (UnicodeError, ValueError, OverflowError):
            decode.append([b, None])
    hosts = [[h, to_ascii(h)] for h in HOSTS]
    print(json.dumps({"encode": encode, "decode": decode, "hosts": hosts}, ensure_ascii=False))


if __name__ == "__main__":
    main()
