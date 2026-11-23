"""ML-KEM (FIPS 203) written from the standard with plain modular arithmetic, for test vectors.

Parameter sets ML-KEM-768 and ML-KEM-1024. Keys are derived from a 64-byte seed
(d || z); encapsulation takes its 32-byte randomness m explicitly.
"""
from __future__ import annotations

import hashlib

Q = 3329
N = 256
PARAMS = {768: dict(k=3, eta1=2, eta2=2, du=10, dv=4), 1024: dict(k=4, eta1=2, eta2=2, du=11, dv=5)}


def bitrev7(i: int) -> int:
    """Reverse the low seven bits."""
    return int(f"{i:07b}"[::-1], 2)


ZETAS = [pow(17, bitrev7(i), Q) for i in range(128)]
GAMMAS = [pow(17, 2 * bitrev7(i) + 1, Q) for i in range(128)]


def G(data: bytes) -> tuple[bytes, bytes]:
    """SHA3-512 split in halves."""
    h = hashlib.sha3_512(data).digest()
    return h[:32], h[32:]


def H(data: bytes) -> bytes:
    """SHA3-256."""
    return hashlib.sha3_256(data).digest()


def J(data: bytes) -> bytes:
    """SHAKE256 with 32 output bytes."""
    return hashlib.shake_256(data).digest(32)


def prf(eta: int, s: bytes, b: int) -> bytes:
    """SHAKE256(s || b) with 64 * eta output bytes."""
    return hashlib.shake_256(s + bytes([b])).digest(64 * eta)


def sample_ntt(seed: bytes) -> list[int]:
    """Rejection-sample a polynomial in the NTT domain from SHAKE128(seed)."""
    stream = hashlib.shake_128(seed).digest(168 * 16)
    out, i = [], 0
    while len(out) < N:
        d1 = stream[i] | ((stream[i + 1] & 15) << 8)
        d2 = (stream[i + 1] >> 4) | (stream[i + 2] << 4)
        if d1 < Q:
            out.append(d1)
        if d2 < Q and len(out) < N:
            out.append(d2)
        i += 3
    return out


def cbd(eta: int, data: bytes) -> list[int]:
    """Centered binomial sampling."""
    bits = [(data[i // 8] >> (i % 8)) & 1 for i in range(len(data) * 8)]
    return [(sum(bits[2 * i * eta + j] for j in range(eta)) - sum(bits[2 * i * eta + eta + j] for j in range(eta))) % Q for i in range(N)]


def ntt(f: list[int]) -> list[int]:
    """Number-theoretic transform (Algorithm 9)."""
    f = list(f)
    i, length = 1, 128
    while length >= 2:
        for start in range(0, N, 2 * length):
            zeta = ZETAS[i]
            i += 1
            for j in range(start, start + length):
                t = zeta * f[j + length] % Q
                f[j + length] = (f[j] - t) % Q
                f[j] = (f[j] + t) % Q
        length //= 2
    return f


def ntt_inverse(f: list[int]) -> list[int]:
    """Inverse transform (Algorithm 10)."""
    f = list(f)
    i, length = 127, 2
    while length <= 128:
        for start in range(0, N, 2 * length):
            zeta = ZETAS[i]
            i -= 1
            for j in range(start, start + length):
                t = f[j]
                f[j] = (t + f[j + length]) % Q
                f[j + length] = zeta * (f[j + length] - t) % Q
        length *= 2
    return [x * 3303 % Q for x in f]


def multiply(f: list[int], g: list[int]) -> list[int]:
    """Product in the NTT domain (Algorithms 11 and 12)."""
    h = [0] * N
    for i in range(128):
        a0, a1, b0, b1 = f[2 * i], f[2 * i + 1], g[2 * i], g[2 * i + 1]
        h[2 * i] = (a0 * b0 + a1 * b1 * GAMMAS[i]) % Q
        h[2 * i + 1] = (a0 * b1 + a1 * b0) % Q
    return h


def add(f: list[int], g: list[int]) -> list[int]:
    """Coefficient-wise sum."""
    return [(a + b) % Q for a, b in zip(f, g)]


def encode(d: int, f: list[int]) -> bytes:
    """ByteEncode_d."""
    bits = [(x >> j) & 1 for x in f for j in range(d)]
    return bytes(sum(bits[8 * i + j] << j for j in range(8)) for i in range(len(bits) // 8))


def decode(d: int, data: bytes) -> list[int]:
    """ByteDecode_d (reducing modulo q for d = 12)."""
    bits = [(data[i // 8] >> (i % 8)) & 1 for i in range(len(data) * 8)]
    m = Q if d == 12 else 1 << d
    return [sum(bits[i * d + j] << j for j in range(d)) % m for i in range(N)]


def compress(d: int, f: list[int]) -> list[int]:
    """Compress_d with rounding."""
    return [((x << d) + Q // 2) // Q % (1 << d) for x in f]


def decompress(d: int, f: list[int]) -> list[int]:
    """Decompress_d with rounding."""
    return [(x * Q + (1 << (d - 1))) >> d for x in f]


def matrix(rho: bytes, k: int) -> list[list[list[int]]]:
    """A-hat[i][j] = SampleNTT(rho || j || i)."""
    return [[sample_ntt(rho + bytes([j, i])) for j in range(k)] for i in range(k)]


def keygen(size: int, seed: bytes) -> tuple[bytes, bytes]:
    """(encapsulation key, expanded decapsulation key) from d || z."""
    p = PARAMS[size]
    k, d, z = p["k"], seed[:32], seed[32:]
    rho, sigma = G(d + bytes([k]))
    a = matrix(rho, k)
    s = [ntt(cbd(p["eta1"], prf(p["eta1"], sigma, i))) for i in range(k)]
    e = [ntt(cbd(p["eta1"], prf(p["eta1"], sigma, k + i))) for i in range(k)]
    t = [e[i] for i in range(k)]
    for i in range(k):
        for j in range(k):
            t[i] = add(t[i], multiply(a[i][j], s[j]))
    ek = b"".join(encode(12, x) for x in t) + rho
    dk = b"".join(encode(12, x) for x in s)
    return ek, dk + ek + H(ek) + z


def encrypt(size: int, ek: bytes, m: bytes, r: bytes) -> bytes:
    """K-PKE.Encrypt."""
    p = PARAMS[size]
    k = p["k"]
    t = [decode(12, ek[384 * i:384 * (i + 1)]) for i in range(k)]
    rho = ek[384 * k:]
    a = matrix(rho, k)
    y = [ntt(cbd(p["eta1"], prf(p["eta1"], r, i))) for i in range(k)]
    e1 = [cbd(p["eta2"], prf(p["eta2"], r, k + i)) for i in range(k)]
    e2 = cbd(p["eta2"], prf(p["eta2"], r, 2 * k))
    u = []
    for i in range(k):
        acc = [0] * N
        for j in range(k):
            acc = add(acc, multiply(a[j][i], y[j]))
        u.append(add(ntt_inverse(acc), e1[i]))
    acc = [0] * N
    for j in range(k):
        acc = add(acc, multiply(t[j], y[j]))
    mu = decompress(1, decode(1, m))
    v = add(add(ntt_inverse(acc), e2), mu)
    return b"".join(encode(p["du"], compress(p["du"], x)) for x in u) + encode(p["dv"], compress(p["dv"], v))


def encapsulate(size: int, ek: bytes, m: bytes) -> tuple[bytes, bytes]:
    """(shared key, ciphertext) for explicit randomness m."""
    key, r = G(m + H(ek))
    return key, encrypt(size, ek, m, r)


def decapsulate(size: int, dk: bytes, c: bytes) -> bytes:
    """The shared key, or the implicit-rejection key when c does not re-encrypt."""
    p = PARAMS[size]
    k, du, dv = p["k"], p["du"], p["dv"]
    dk_pke, ek = dk[:384 * k], dk[384 * k:768 * k + 32]
    h, z = dk[768 * k + 32:768 * k + 64], dk[768 * k + 64:]
    u = [decompress(du, decode(du, c[32 * du * i:32 * du * (i + 1)])) for i in range(k)]
    v = decompress(dv, decode(dv, c[32 * du * k:]))
    s = [decode(12, dk_pke[384 * i:384 * (i + 1)]) for i in range(k)]
    acc = [0] * N
    for i in range(k):
        acc = add(acc, multiply(s[i], ntt(u[i])))
    w = [(a - b) % Q for a, b in zip(v, ntt_inverse(acc))]
    m = encode(1, compress(1, w))
    key, r = G(m + h)
    rejected = J(z + c)
    return key if encrypt(size, ek, m, r) == c else rejected
