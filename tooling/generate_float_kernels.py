#!/usr/bin/env python3
"""Generate the binary64 kernels of Data.Float.

Each kernel is written in the typed kernel language of ``float_dsl``: the classic algorithms (fdlibm-style
argument reduction and minimax polynomials for exp, log, log1p and expm1; Cephes-style rational
approximations for the trigonometric, inverse trigonometric and hyperbolic functions; Payne-Hanek reduction
with a 1216-bit 4/pi for huge trigonometric arguments; bit-level frexp, ldexp and modf). The LLVM backend lowers
each one without jumps (both arms of every test run and merge through select, so the special cases cost a few
compares rather than mispredicted branches); the Python backend runs the same kernels with their control flow as
executable models, which are checked against the reference vectors before the prims are written. Only the
loops of pow and mod stay in Cinch.

    python3 tooling/generate_float_kernels.py          # rewrite the generated suffix
    python3 tooling/generate_float_kernels.py --check  # fail when it is stale
"""
from __future__ import annotations

import argparse
import json
import math
import struct
import sys
from decimal import Decimal, getcontext
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from float_dsl import Kernel, Program, M64  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
TARGET = ROOT / "seed/prelude/Data/Float.cinch"
FIXTURE = ROOT / "seed/tests/fixtures/float-vectors.json"
MARKER = "  -- Generated binary64 kernels.\n"

getcontext().prec = 90
PI = Decimal("3.14159265358979323846264338327950288419716939937510582097494459")
LN2 = Decimal("0.693147180559945309417232121458176568075500134360255254120680009")
LN10 = Decimal("2.30258509299404568401799145468436420760110148862877297603332790")
SQRT2 = Decimal("1.41421356237309504880168872420969807856967187537694807317667974")


def bits(x: float) -> int:
    return struct.unpack("<Q", struct.pack("<d", x))[0]


CONSTS = {
    "INF": math.inf, "NAN": ("f", struct.unpack("<d", struct.pack("<Q", 0x7FF8000000000001))[0]),
    "PI": float(PI), "PI_2": float(PI / 2), "PI_4": float(PI / 4), "PI3_4": float(3 * PI / 4),
    "FOUR_PI": float(4 / PI), "LN2": float(LN2), "INV_LN2": float(1 / LN2), "INV_LN10": float(1 / LN10),
    "SQRT2_2": float(SQRT2 / 2),
    "LN2HI": 6.93147180369123816490e-01, "LN2LO": 1.90821492927058770002e-10,
    "SMALLEST_NORMAL": 2.2250738585072014e-308, "TWO52": 2.0**52, "TWO_M53": 2.0**-53, "TWO54": 2.0**54,
    "HALF_MAX": sys.float_info.max / 2, "TINY_REM": 4.45014771701440276618e-308,
    "REDUCE": 2.0**29, "PI4A": 7.85398125648498535156e-1, "PI4B": 3.77489470793079817668e-8,
    "PI4C": 2.69515142907905952645e-15,
    # exp
    "EXP_LOG2E": 1.44269504088896338700e+00, "EXP_OVER": 7.09782712893383973096e+02,
    "EXP_UNDER": -7.45133219101941108420e+02, "NEAR28": 2.0**-28,
    "P1": 1.66666666666666657415e-01, "P2": -2.77777777770155933842e-03, "P3": 6.61375632143793436117e-05,
    "P4": -1.65339022054652515390e-06, "P5": 4.13813679705723846039e-08,
    "EXP2_OVER": 1.0239999999999999e+03, "EXP2_UNDER": -1.0740e+03,
    # log and log1p
    "L1": 6.666666666666735130e-01, "L2": 3.999999999940941908e-01, "L3": 2.857142874366239149e-01,
    "L4": 2.222219843214978396e-01, "L5": 1.818357216161805012e-01, "L6": 1.531383769920937332e-01,
    "L7": 1.479819860511658591e-01, "SQRT2M1": 4.142135623730950488017e-01,
    "SQRT2HALFM1": -2.928932188134524755992e-01, "SMALL29": 2.0**-29, "TINY54": 2.0**-54, "TWO53": 2.0**53,
    # expm1
    "OTHRESHOLD": 7.09782712893383973096e+02, "LN2X56": 3.88162421113569373274e+01,
    "LN2HALFX3": 1.03972077083991796413e+00, "LN2HALF": 3.46573590279972654709e-01,
    "Q1": -3.33333333333331316428e-02, "Q2": 1.58730158725481460165e-03, "Q3": -7.93650757867487942473e-05,
    "Q4": 4.00821782732936239552e-06, "Q5": -2.01099218183624371326e-07,
    # cbrt
    "CB_C": 5.42857142857142815906e-01, "CB_D": -7.05306122448979611050e-01, "CB_E": 1.41428571428571436819e+00,
    "CB_F": 1.60714285714285720630e+00, "CB_G": 3.57142857142857150787e-01,
    # sin and cos
    "S0": 1.58962301576546568060e-10, "S1": -2.50507477628578072866e-8, "S2": 2.75573136213857245213e-6,
    "S3": -1.98412698295895385996e-4, "S4": 8.33333333332211858878e-3, "S5": -1.66666666666666307295e-1,
    "C0": -1.13585365213876817300e-11, "C1": 2.08757008419747316778e-9, "C2": -2.75573141792967388112e-7,
    "C3": 2.48015872888517045348e-5, "C4": -1.38888888888730564116e-3, "C5": 4.16666666666665929218e-2,
    # tan
    "TP0": -1.30936939181383777646e4, "TP1": 1.15351664838587416140e6, "TP2": -1.79565251976484877988e7,
    "TQ1": 1.36812963470692954678e4, "TQ2": -1.32089234440210967447e6, "TQ3": 2.50083801823357915839e7,
    "TQ4": -5.38695755929454629881e7,
    # atan
    "AP0": -8.750608600031904122785e-01, "AP1": -1.615753718733365076637e+01, "AP2": -7.500855792314704667340e+01,
    "AP3": -1.228866684490136173410e+02, "AP4": -6.485021904942025371773e+01, "AQ0": 2.485846490142306297962e+01,
    "AQ1": 1.650270098316988542046e+02, "AQ2": 4.328810604912902668951e+02, "AQ3": 4.853903996359136964868e+02,
    "AQ4": 1.945506571482613964425e+02, "MOREBITS": 6.123233995736765886130e-17, "TAN3PIO8": 2.41421356237309504880,
    # sinh, tanh
    "SH_P0": -0.6307673640497716991184787251e+6, "SH_P1": -0.8991272022039509355398013511e+5,
    "SH_P2": -0.2894211355989563807284660366e+4, "SH_P3": -0.2630563213397497062819489e+2,
    "SH_Q0": -0.6307673640497716991212077277e+6, "SH_Q1": 0.1521517378790019070696485176e+5,
    "SH_Q2": -0.173678953558233699533450911e+3,
    "TH_P0": -9.64399179425052238628e-1, "TH_P1": -9.92877231001918586564e1, "TH_P2": -1.61468768441708447952e3,
    "TH_Q0": 1.12811678491632931402e2, "TH_Q1": 2.23548839060100448583e3, "TH_Q2": 4.84406305325125486048e3,
    "TH_MAXLOG": 8.8029691931113054295988e+01,
    "ASINH_LN2": 6.93147180559945286227e-01, "LARGE28": 2.0**28,
}

TABLES = {
    "MPI4": [0x0000000000000001, 0x45F306DC9C882A53, 0xF84EAFA3EA69BB81, 0xB6C52B3278872083, 0xFCA2C757BD778AC3,
             0x6E48DC74849BA5C0, 0x0C925DD413A32439, 0xFC3BD63962534E7D, 0xD1046BEA5D768909, 0xD338E04D68BEFC82,
             0x7323AC7306A673E9, 0x3908BF177BF25076, 0x3FF12FFFBC0B301F, 0xDE5E2316B414DA3E, 0xDA6CFD9E4F96136E,
             0x9E8C7ECD3CBFD45A, 0xEA4F758FD7CBE2F6, 0x7A0E73EF14A525D4, 0xD7F6BF623F1ABA10, 0xAC06608DF8F6D757],
    "P10": [bits(float(f"1e{i}")) for i in range(32)],
    "P10POS32": [bits(float(f"1e{32 * i}")) for i in range(10)],
    "P10NEG32": [bits(float(f"1e-{32 * i}")) for i in range(11)],
}

K = []


def kernel(name: str, params: str, doc: str, source: str, entry: bool = True) -> None:
    """Register a kernel: Cinch name, parameter types (one letter each), documentation and source."""
    K.append(Kernel(name, list(params), source, doc, entry))


# --- Bits, scaling and splitting --------------------------------------------------------------------------

kernel("normalize", "f", "", '''
def normalize(x):
    if fabs(x) < SMALLEST_NORMAL:
        return x * TWO52, -52
    return x, 0
''', entry=False)

kernel("frexp-kernel", "f", "The fraction in [1/2, 1) (bits, low half) and binary exponent (high half) of a value.", '''
def frexp(f):
    if f == 0.0 or isnan(f) or fabs(f) == INF:
        return f, 0
    g, e = normalize(f)
    x = bits(g)
    e = e + i_((x >> 52) & 2047) - 1022
    x = (x & 0x800FFFFFFFFFFFFF) | (1022 << 52)
    return fbits(x), e
''')

kernel("ldexp", "fi", "A value times two to an integer power, rounding once into the subnormal range.", '''
def ldexp(frac, exp):
    if frac == 0.0 or isnan(frac) or fabs(frac) == INF:
        return frac
    g, e = normalize(frac)
    exp = exp + e
    x = bits(g)
    exp = exp + i_((x >> 52) & 2047) - 1023
    if exp < -1075:
        return copysign(0.0, g)
    if exp > 1023:
        if g < 0.0:
            return -INF
        return INF
    m = 1.0
    if exp < -1022:
        exp = exp + 53
        m = TWO_M53
    x = (x & 0x800FFFFFFFFFFFFF) | (u_(exp + 1023) << 52)
    return m * fbits(x)
''')

kernel("modf-ge1", "f", "", '''
def modf_ge1(f):
    x = bits(f)
    e = ((x >> 52) & 2047) - 1023
    if e < 52:
        x = x & ~((1 << (52 - e)) - 1)
    i = fbits(x)
    return i, f - i
''', entry=False)

kernel("modf-kernel", "f", "The integer part (low half) and fractional part (high half), both with the value's sign.", '''
def modf(f):
    if f < 1.0:
        if f < 0.0:
            g = -f
            if g < 1.0:
                return -0.0, f
            a, b = modf_ge1(g)
            return -a, -b
        if f == 0.0:
            return f, f
        return 0.0, f
    return modf_ge1(f)
''')

kernel("ilogb-kernel", "f", "The unbiased binary exponent of a finite nonzero value.", '''
def ilogb(x):
    y, e = normalize(x)
    return i_((bits(y) >> 52) & 2047) - 1023 + e
''')

kernel("cbrt-f64", "f", "The cube root, exact for perfect cubes of moderate size.", '''
def cbrt(x):
    if x == 0.0 or isnan(x) or fabs(x) == INF:
        return x
    sign = x < 0.0
    x = fabs(x)
    t = fbits((bits(x) / 3) + (715094163 << 32))
    if x < SMALLEST_NORMAL:
        t = TWO54 * x
        t = fbits((bits(t) / 3) + (696219795 << 32))
    r = t * t / x
    s = CB_C + r * t
    t = t * (CB_G + CB_F / (s + CB_E + CB_D / s))
    t = fbits((bits(t) & (0xFFFFFFFFC << 28)) + (1 << 30))
    s = t * t
    r = x / s
    w = t + t
    r = (r - t) / (w + r)
    t = t + t * r
    if sign:
        return -t
    return t
''')

kernel("hypot-f64", "ff", "The length of the hypotenuse, sqrt(p*p + q*q), without undue overflow or underflow.", '''
def hypot(p, q):
    p = fabs(p)
    q = fabs(q)
    if p == INF or q == INF:
        return INF
    if isnan(p) or isnan(q):
        return NAN
    if p < q:
        t = p
        p = q
        q = t
    if p == 0.0:
        return 0.0
    q = q / p
    return p * sqrt(1.0 + q * q)
''')

# --- Exponentials and logarithms --------------------------------------------------------------------------

kernel("scale", "fi", "", '''
def scale(y, k):
    """y * 2^k for a positive normal y and |k| below 2044: the first factor keeps the product normal (exact), the
    second rounds once, so the result is the correctly rounded product, as ldexp gives."""
    k1 = k >> 1
    k2 = k - k1
    return (y * fbits(u_(k1 + 1023) << 52)) * fbits(u_(k2 + 1023) << 52)
''', entry=False)

kernel("expmulti", "ffi", "", '''
def expmulti(hi, lo, k):
    r = hi - lo
    t = r * r
    c = r - t * (P1 + t * (P2 + t * (P3 + t * (P4 + t * P5))))
    y = 1.0 - ((lo - (r * c) / (2.0 - c)) - hi)
    return scale(y, k)
''', entry=False)

kernel("exp-f64", "f", "The natural exponential, e to the power x.", '''
def exp(x):
    if isnan(x) or x == INF:
        return x
    if x == -INF:
        return 0.0
    if x > EXP_OVER:
        return INF
    if x < EXP_UNDER:
        return 0.0
    if -NEAR28 < x and x < NEAR28:
        return 1.0 + x
    k = 0
    if x < 0.0:
        k = ftoi(EXP_LOG2E * x - 0.5)
    if x > 0.0:
        k = ftoi(EXP_LOG2E * x + 0.5)
    hi = x - itof(k) * LN2HI
    lo = itof(k) * LN2LO
    return expmulti(hi, lo, k)
''')

kernel("exp2-f64", "f", "Two to the power x.", '''
def exp2(x):
    if isnan(x) or x == INF:
        return x
    if x == -INF:
        return 0.0
    if x > EXP2_OVER:
        return INF
    if x < EXP2_UNDER:
        return 0.0
    k = 0
    if x > 0.0:
        k = ftoi(x + 0.5)
    if x < 0.0:
        k = ftoi(x - 0.5)
    t = x - itof(k)
    hi = t * LN2HI
    lo = -t * LN2LO
    return expmulti(hi, lo, k)
''')

kernel("expm1-f64", "f", "e to the power x, minus one, accurate when x is near zero.", '''
def expm1(x):
    if x == INF or isnan(x):
        return x
    if x == -INF:
        return -1.0
    absx = fabs(x)
    sign = x < 0.0
    if absx >= LN2X56:
        if sign:
            return -1.0
        if absx >= OTHRESHOLD:
            return INF
    c = 0.0
    k = 0
    if absx > LN2HALF:
        hi = 0.0
        lo = 0.0
        if absx < LN2HALFX3:
            if not sign:
                hi = x - LN2HI
                lo = LN2LO
                k = 1
            else:
                hi = x + LN2HI
                lo = -LN2LO
                k = -1
        else:
            if not sign:
                k = ftoi(EXP_LOG2E * x + 0.5)
            else:
                k = ftoi(EXP_LOG2E * x - 0.5)
            t = itof(k)
            hi = x - t * LN2HI
            lo = t * LN2LO
        x = hi - lo
        c = (hi - x) - lo
    else:
        if absx < TINY54:
            return x
    hfx = 0.5 * x
    hxs = x * hfx
    r1 = 1.0 + hxs * (Q1 + hxs * (Q2 + hxs * (Q3 + hxs * (Q4 + hxs * Q5))))
    t = 3.0 - r1 * hfx
    e = hxs * ((r1 - t) / (6.0 - x * t))
    if k == 0:
        return x - (x * e - hxs)
    e = x * (e - c) - c
    e = e - hxs
    if k == -1:
        return 0.5 * (x - e) - 0.5
    if k == 1:
        if x < -0.25:
            return -2.0 * (e - (x + 0.5))
        return 1.0 + 2.0 * (x - e)
    if k <= -2 or k > 56:
        y = 1.0 - (e - x)
        y = fbits(bits(y) + (u_(k) << 52))
        return y - 1.0
    if k < 20:
        t = fbits(0x3FF0000000000000 - (0x20000000000000 >> u_(k)))
        y = t - (e - x)
        return fbits(bits(y) + (u_(k) << 52))
    t = fbits(u_(1023 - k) << 52)
    y = x - (e + t)
    y = y + 1.0
    return fbits(bits(y) + (u_(k) << 52))
''')

kernel("log-f64", "f", "The natural logarithm.", '''
def log(x):
    if isnan(x) or x == INF:
        return x
    if x < 0.0:
        return NAN
    if x == 0.0:
        return -INF
    f1, ki = frexp(x)
    if f1 < SQRT2_2:
        f1 = f1 * 2.0
        ki = ki - 1
    f = f1 - 1.0
    k = itof(ki)
    s = f / (2.0 + f)
    s2 = s * s
    s4 = s2 * s2
    t1 = s2 * (L1 + s4 * (L3 + s4 * (L5 + s4 * L7)))
    t2 = s4 * (L2 + s4 * (L4 + s4 * L6))
    r = t1 + t2
    hfsq = 0.5 * f * f
    return k * LN2HI - ((hfsq - (s * (hfsq + r) + k * LN2LO)) - f)
''')

kernel("log10-f64", "f", "The decimal logarithm.", '''
def log10(x):
    return log(x) * INV_LN10
''')

kernel("log2-f64", "f", "The binary logarithm, exact for powers of two.", '''
def log2(x):
    frac, e = frexp(x)
    if frac == 0.5:
        return itof(e - 1)
    return log(frac) * INV_LN2 + itof(e)
''')

kernel("log1p-f64", "f", "The natural logarithm of 1 + x, accurate when x is near zero.", '''
def log1p(x):
    if x < -1.0 or isnan(x):
        return NAN
    if x == -1.0:
        return -INF
    if x == INF:
        return INF
    absx = fabs(x)
    f = 0.0
    iu: u = 0
    k = 1
    if absx < SQRT2M1:
        if absx < SMALL29:
            if absx < TINY54:
                return x
            return x - x * x * 0.5
        if x > SQRT2HALFM1:
            k = 0
            f = x
            iu = 1
    c = 0.0
    if k != 0:
        u = 0.0
        if absx < TWO53:
            u = 1.0 + x
            iu = bits(u)
            k = i_((iu >> 52) - 1023)
            if k > 0:
                c = 1.0 - (u - x)
            else:
                c = x - (u - 1.0)
            c = c / u
        else:
            u = x
            iu = bits(u)
            k = i_((iu >> 52) - 1023)
            c = 0.0
        iu = iu & 0x000FFFFFFFFFFFFF
        if iu < 0x0006A09E667F3BCD:
            u = fbits(iu | 0x3FF0000000000000)
        else:
            k = k + 1
            u = fbits(iu | 0x3FE0000000000000)
            iu = (0x0010000000000000 - iu) >> 2
        f = u - 1.0
    hfsq = 0.5 * f * f
    if iu == 0:
        if f == 0.0:
            if k == 0:
                return 0.0
            c = c + itof(k) * LN2LO
            return itof(k) * LN2HI + c
        r = hfsq * (1.0 - 0.66666666666666666 * f)
        if k == 0:
            return f - r
        return itof(k) * LN2HI - ((r - (itof(k) * LN2LO + c)) - f)
    s = f / (2.0 + f)
    z = s * s
    r = z * (L1 + z * (L2 + z * (L3 + z * (L4 + z * (L5 + z * (L6 + z * L7))))))
    if k == 0:
        return f - (hfsq - s * (hfsq + r))
    return itof(k) * LN2HI - ((hfsq - (s * (hfsq + r) + (itof(k) * LN2LO + c))) - f)
''')

kernel("pow-case", "ff", "Pow's special cases: the result (low half) and whether the general method must run (high half: 1 for the "
       "fractional-power start, 0 when the result is final).", '''
def pow_case(x, y):
    yi, yf = modf(fabs(y))
    if y == 0.0 or x == 1.0:
        return 1.0, u_(0)
    if y == 1.0:
        return x, u_(0)
    if isnan(x) or isnan(y):
        return NAN, u_(0)
    if x == 0.0:
        odd = fabs(y) < TWO53 and yf == 0.0 and (u_(ftoi(yi)) & 1) == 1 and y == y
        neg = (bits(x) >> 63) == 1
        if y < 0.0:
            if neg and odd:
                return -INF, u_(0)
            return INF, u_(0)
        if neg and odd:
            return x, u_(0)
        return 0.0, u_(0)
    if fabs(y) == INF:
        if x == -1.0:
            return 1.0, u_(0)
        if (fabs(x) < 1.0) == (y == INF):
            return 0.0, u_(0)
        return INF, u_(0)
    if fabs(x) == INF:
        if x == -INF:
            odd = fabs(y) < TWO53 and yf == 0.0 and (u_(ftoi(yi)) & 1) == 1
            if y < 0.0:
                if odd:
                    return -0.0, u_(0)
                return 0.0, u_(0)
            if odd:
                return -INF, u_(0)
            return INF, u_(0)
        if y < 0.0:
            return 0.0, u_(0)
        return INF, u_(0)
    if y == 0.5:
        return sqrt(x), u_(0)
    if y == -0.5:
        return 1.0 / sqrt(x), u_(0)
    if yf != 0.0 and x < 0.0:
        return NAN, u_(0)
    if yi >= 9223372036854775808.0:
        if x == -1.0:
            return 1.0, u_(0)
        if (fabs(x) < 1.0) == (y > 0.0):
            return 0.0, u_(0)
        return INF, u_(0)
    return 0.0, u_(1)
''')

kernel("pow-start", "ff", "The general method's start: the fractional power e^(yf log x) (low half) and the integer power still to "
       "apply (high half).", '''
def pow_start(x, y):
    yi, yf = modf(fabs(y))
    a1 = 1.0
    if yf != 0.0:
        if yf > 0.5:
            yf = yf - 1.0
            yi = yi + 1.0
        a1 = exp(yf * log(x))
    return a1, u_(ftoi(yi))
''')

kernel("pow10-f64", "i", "Ten to an integer power, from exact tables.", '''
def pow10(n):
    if 0 <= n and n <= 308:
        return fbits(pick(P10POS32, u_(n) / 32)) * fbits(pick(P10, u_(n) % 32))
    if -323 <= n and n <= 0:
        return fbits(pick(P10NEG32, u_(-n) / 32)) / fbits(pick(P10, u_(-n) % 32))
    if n > 0:
        return INF
    return 0.0
''')

kernel("mod-step", "fffi", "One step of the remainder: subtract the largest y * 2^k not above r.", '''
def mod_step(r, y, yfr, yexp):
    rfr, rexp = frexp(r)
    if rfr < yfr:
        rexp = rexp - 1
    return r - ldexp(y, rexp - yexp)
''')

kernel("remainder-fix", "fff", "The IEEE remainder's final correction of x mod 2y into [-y/2, y/2], with the sign of the dividend.", '''
def remainder_fix(x, y, orig):
    if y < TINY_REM:
        if x + x > y:
            x = x - y
            if x + x >= y:
                x = x - y
    else:
        yhalf = 0.5 * y
        if x > yhalf:
            x = x - y
            if x >= yhalf:
                x = x - y
    if orig < 0.0:
        return -x
    return x
''')

# --- Trigonometry -----------------------------------------------------------------------------------------

kernel("reduce-small", "f", "", '''
def reduce_small(x):
    j = ftou(x * FOUR_PI)
    y = utof(j)
    if (j & 1) == 1:
        j = j + 1
        y = y + 1.0
    j = j & 7
    z = ((x - y * PI4A) - y * PI4B) - y * PI4C
    return j, z
''', entry=False)

kernel("reduce-large", "f", "", '''
def reduce_large(x):
    if x < PI_4:
        return u_(0), x
    ix = bits(x)
    e = i_((ix >> 52) & 2047) - 1023 - 52
    ix = (ix & ~(2047 << 52)) | (1 << 52)
    digit = u_(e + 61) / 64
    shift = u_(e + 61) % 64
    z0 = (pick(MPI4, digit) << shift) | (pick(MPI4, digit + 1) >> (64 - shift))
    z1 = (pick(MPI4, digit + 1) << shift) | (pick(MPI4, digit + 2) >> (64 - shift))
    z2 = (pick(MPI4, digit + 2) << shift) | (pick(MPI4, digit + 3) >> (64 - shift))
    z2hi, z2lo = mul(z2, ix)
    z1hi, z1lo = mul(z1, ix)
    z0lo = z0 * ix
    lo = z1lo + z2hi
    carry = 1 if lo < z1lo else 0
    hi = z0lo + z1hi + u_(carry)
    j = hi >> 61
    hi = (hi << 3) | (lo >> 61)
    lz = clz(hi)
    ex = 1023 - (lz + 1)
    hi = (hi << (lz + 1)) | (lo >> (64 - (lz + 1)))
    hi = hi >> 12
    hi = hi | (ex << 52)
    z = fbits(hi)
    if (j & 1) == 1:
        j = (j + 1) & 7
        z = z - 1.0
    return j, z * PI_4
''', entry=False)

kernel("sin-poly", "f", "", '''
def sin_poly(z):
    zz = z * z
    return z + z * zz * ((((((S0 * zz) + S1) * zz + S2) * zz + S3) * zz + S4) * zz + S5)
''', entry=False)

kernel("cos-poly", "f", "", '''
def cos_poly(z):
    zz = z * z
    return 1.0 - 0.5 * zz + zz * zz * ((((((C0 * zz) + C1) * zz + C2) * zz + C3) * zz + C4) * zz + C5)
''', entry=False)

kernel("sin-octant", "uf", "", '''
def sin_octant(j, z):
    sign = False
    if j > 3:
        sign = True
        j = j - 4
    y = sin_poly(z)
    if j == 1 or j == 2:
        y = cos_poly(z)
    if sign:
        return -y
    return y
''', entry=False)

kernel("cos-octant", "uf", "", '''
def cos_octant(j, z):
    sign = False
    if j > 3:
        j = j - 4
        sign = not sign
    if j > 1:
        sign = not sign
    y = cos_poly(z)
    if j == 1 or j == 2:
        y = sin_poly(z)
    if sign:
        return -y
    return y
''', entry=False)

kernel("tan-octant", "uf", "", '''
def tan_octant(j, z):
    zz = z * z
    y = z
    if zz > 1e-14:
        y = z + z * (zz * (((TP0 * zz) + TP1) * zz + TP2) / ((((zz + TQ1) * zz + TQ2) * zz + TQ3) * zz + TQ4))
    if (j & 2) == 2:
        y = -1.0 / y
    return y
''', entry=False)

for fn, poly in (("sin", "sin"), ("tan", "tan")):
    for size in ("small", "large"):
        kernel(f"{fn}-{size}", "f", f"The {'sine' if fn == 'sin' else 'tangent'} for {'arguments below 2^29 in magnitude' if size == 'small' else 'any argument, by Payne-Hanek reduction'}.", f'''
def {fn}_{size}(x):
    if x == 0.0 or isnan(x):
        return x
    if fabs(x) == INF:
        return NAN
    sign = x < 0.0
    x = fabs(x)
    j, z = reduce_{size}(x)
    y = {poly}_octant(j, z)
    if sign:
        return -y
    return y
''')

for size in ("small", "large"):
    kernel(f"cos-{size}", "f", f"The cosine for {'arguments below 2^29 in magnitude' if size == 'small' else 'any argument, by Payne-Hanek reduction'}.", f'''
def cos_{size}(x):
    if isnan(x) or fabs(x) == INF:
        return NAN
    j, z = reduce_{size}(fabs(x))
    return cos_octant(j, z)
''')
    kernel(f"sincos-{size}", "f", f"The sine (low half) and cosine (high half) for {'arguments below 2^29 in magnitude' if size == 'small' else 'any argument'}.", f'''
def sincos_{size}(x):
    if x == 0.0:
        return x, 1.0
    if isnan(x) or fabs(x) == INF:
        return NAN, NAN
    sin_sign = x < 0.0
    j, z = reduce_{size}(fabs(x))
    cos_sign = False
    if j > 3:
        j = j - 4
        sin_sign = not sin_sign
        cos_sign = not cos_sign
    if j > 1:
        cos_sign = not cos_sign
    c = cos_poly(z)
    s = sin_poly(z)
    if j == 1 or j == 2:
        t = c
        c = s
        s = t
    if cos_sign:
        c = -c
    if sin_sign:
        s = -s
    return s, c
''')

kernel("xatan", "f", "", '''
def xatan(x):
    z = x * x
    z = z * ((((AP0 * z + AP1) * z + AP2) * z + AP3) * z + AP4) / (((((z + AQ0) * z + AQ1) * z + AQ2) * z + AQ3) * z + AQ4)
    return x * z + x
''', entry=False)

kernel("satan", "f", "", '''
def satan(x):
    small = x <= 0.66
    big = x > TAN3PIO8
    t = x
    if not small:
        num = x - 1.0
        den = x + 1.0
        if big:
            num = 1.0
            den = x
        t = num / den
    a = xatan(t)
    if small:
        return a
    if big:
        return PI_2 - a + MOREBITS
    return PI_4 + a + 0.5 * MOREBITS
''', entry=False)

kernel("atan-f64", "f", "The arctangent, in [-pi/2, pi/2].", '''
def atan(x):
    if x == 0.0:
        return x
    if x > 0.0:
        return satan(x)
    return -satan(-x)
''')

kernel("asin-f64", "f", "The arcsine, in [-pi/2, pi/2]; NaN outside [-1, 1].", '''
def asin(x):
    if x == 0.0:
        return x
    sign = x < 0.0
    x = fabs(x)
    if x > 1.0:
        return NAN
    temp = sqrt(1.0 - x * x)
    big = x > 0.7
    num = x
    den = temp
    if big:
        num = temp
        den = x
    a = satan(num / den)
    if big:
        a = PI_2 - a
    if sign:
        return -a
    return a
''')

kernel("acos-f64", "f", "The arccosine, in [0, pi]; NaN outside [-1, 1].", '''
def acos(x):
    return PI_2 - asin(x)
''')

kernel("atan2-f64", "ff", "The angle of the point (x, y) from the positive x axis, in [-pi, pi], with signed-zero and infinite "
       "cases resolved by quadrant.", '''
def atan2(y, x):
    if isnan(y) or isnan(x):
        return NAN
    if y == 0.0:
        if x >= 0.0 and (bits(x) >> 63) == 0:
            return copysign(0.0, y)
        return copysign(PI, y)
    if x == 0.0:
        return copysign(PI_2, y)
    if fabs(x) == INF:
        if x == INF:
            if fabs(y) == INF:
                return copysign(PI_4, y)
            return copysign(0.0, y)
        if fabs(y) == INF:
            return copysign(PI3_4, y)
        return copysign(PI, y)
    if fabs(y) == INF:
        return copysign(PI_2, y)
    q = atan(y / x)
    if x < 0.0:
        if q <= 0.0:
            return q + PI
        return q - PI
    return q
''')

# --- Hyperbolic functions ---------------------------------------------------------------------------------

kernel("sinh-f64", "f", "The hyperbolic sine.", '''
def sinh(x):
    sign = x < 0.0
    if sign:
        x = -x
    ex = exp(x)
    temp = ex * 0.5
    if x <= 21.0:
        temp = (ex - 1.0 / ex) * 0.5
    if x <= 0.5:
        sq = x * x
        temp = (((SH_P3 * sq + SH_P2) * sq + SH_P1) * sq + SH_P0) * x
        temp = temp / (((sq + SH_Q2) * sq + SH_Q1) * sq + SH_Q0)
    if sign:
        return -temp
    return temp
''')

kernel("cosh-f64", "f", "The hyperbolic cosine.", '''
def cosh(x):
    x = fabs(x)
    ex = exp(x)
    if x > 21.0:
        return ex * 0.5
    return (ex + 1.0 / ex) * 0.5
''')

kernel("tanh-f64", "f", "The hyperbolic tangent.", '''
def tanh(x):
    z = fabs(x)
    if z > 0.5 * TH_MAXLOG:
        if x < 0.0:
            return -1.0
        return 1.0
    if z >= 0.625:
        s = exp(2.0 * z)
        z = 1.0 - 2.0 / (s + 1.0)
        if x < 0.0:
            return -z
        return z
    if x == 0.0:
        return x
    s = x * x
    return x + x * s * ((TH_P0 * s + TH_P1) * s + TH_P2) / (((s + TH_Q0) * s + TH_Q1) * s + TH_Q2)
''')

kernel("asinh-f64", "f", "The inverse hyperbolic sine.", '''
def asinh(x):
    if isnan(x) or fabs(x) == INF:
        return x
    sign = x < 0.0
    if sign:
        x = -x
    large = x > LARGE28
    arg = 2.0 * x + 1.0 / (sqrt(x * x + 1.0) + x)
    if large:
        arg = x
    l = log(arg)
    temp = log1p(x + x * x / (1.0 + sqrt(1.0 + x * x)))
    if x < NEAR28:
        temp = x
    if x > 2.0:
        temp = l
    if large:
        temp = l + ASINH_LN2
    if sign:
        return -temp
    return temp
''')

kernel("acosh-f64", "f", "The inverse hyperbolic cosine; NaN below 1.", '''
def acosh(x):
    if x < 1.0 or isnan(x):
        return NAN
    if x == 1.0:
        return 0.0
    large = x >= LARGE28
    arg = 2.0 * x - 1.0 / (x + sqrt(x * x - 1.0))
    if large:
        arg = x
    l = log(arg)
    if large:
        return l + LN2
    if x > 2.0:
        return l
    t = x - 1.0
    return log1p(t + sqrt(2.0 * t + t * t))
''')

kernel("atanh-f64", "f", "The inverse hyperbolic tangent; NaN outside [-1, 1].", '''
def atanh(x):
    if x < -1.0 or x > 1.0 or isnan(x):
        return NAN
    if x == 1.0:
        return INF
    if x == -1.0:
        return -INF
    sign = x < 0.0
    if sign:
        x = -x
    small = x < 0.5
    temp = x + x
    arg = (x + x) / (1.0 - x)
    if small:
        arg = temp + temp * x / (1.0 - x)
    temp = 0.5 * log1p(arg)
    if x < NEAR28:
        temp = x
    if sign:
        return -temp
    return temp
''')

PROGRAM = Program(K, CONSTS, TABLES)


def render() -> str:
    """The generated suffix: one prim per entry kernel."""
    out = [MARKER]
    for k in K:
        if not k.entry:
            continue
        body, result = PROGRAM.llvm(k)
        sig = " -> ".join([{"f": "F64", "i": "I64", "u": "U64"}[t] for t in k.params] + [result])
        out.append(f"\n  -- | {k.doc}\n  prim {k.name} :: {sig} where\n    llvm {{\n")
        out.extend(f"      {line}\n" for line in body.splitlines())
        out.append("    }\n")
    return "".join(out)


# --- Checking the models against the reference vectors ----------------------------------------------------

def words(blob: str) -> list[int]:
    raw = bytes.fromhex(blob)
    return list(struct.unpack(f"<{len(raw) // 8}Q", raw))


def fb(u: int) -> float:
    return struct.unpack("<d", struct.pack("<Q", u))[0]


def same(got: float, want: int) -> bool:
    g = bits(got)
    return g == want or (got != got and (want & 0x7FF0000000000000) == 0x7FF0000000000000 and want & 0xFFFFFFFFFFFFF)


def models() -> dict:
    """Whole functions built from the Python kernel models, mirroring the Cinch wrappers."""
    m = PROGRAM.python()

    def mod(x, y):
        if y == 0 or math.isinf(x) or x != x or y != y:
            return math.nan
        y = abs(y)
        yfr, yexp = m["frexp"](y)
        r = -x if x < 0 else x
        while r >= y:
            r = m["mod_step"](r, y, yfr, yexp)
        return -r if x < 0 else r

    def remainder(x, y):
        if x != x or y != y or math.isinf(x) or y == 0:
            return math.nan
        if math.isinf(y):
            return x
        ax = -x if x < 0 else x
        ay = -y if y < 0 else y
        if ax == ay:
            return math.copysign(0.0, x)
        if ay <= CONSTS["HALF_MAX"]:
            ax = mod(ax, ay + ay)
        return m["remainder_fix"](ax, ay, x)

    def pow_(x, y):
        v, more = m["pow_case"](x, y)
        if not more:
            return v
        a1, i = m["pow_start"](x, y)
        ae = 0
        x1, xe = m["frexp"](x)
        while i != 0:
            if xe < -4096 or 4096 < xe:
                ae += xe
                break
            if i & 1:
                a1 *= x1
                ae += xe
            x1 *= x1
            xe <<= 1
            if x1 < 0.5:
                x1 += x1
                xe -= 1
            i >>= 1
        if y < 0:
            a1 = 1 / a1 if a1 != 0 else math.inf
            ae = -ae
        return m["ldexp"](a1, ae)

    def trig(name):
        return lambda x: m[f"{name}_small"](x) if abs(x) < CONSTS["REDUCE"] else m[f"{name}_large"](x)

    def logb(x):
        if x == 0:
            return -math.inf
        if math.isinf(x):
            return math.inf
        if x != x:
            return x
        return float(m["ilogb"](x))

    return {
        "cbrt": m["cbrt"], "exp": m["exp"], "exp2": m["exp2"], "expm1": m["expm1"], "log": m["log"],
        "log2": m["log2"], "log10": m["log10"], "log1p": m["log1p"], "sin": trig("sin"), "cos": trig("cos"),
        "tan": trig("tan"), "asin": m["asin"], "acos": m["acos"], "atan": m["atan"], "sinh": m["sinh"],
        "cosh": m["cosh"], "tanh": m["tanh"], "asinh": m["asinh"], "acosh": m["acosh"], "atanh": m["atanh"],
        "logb": logb, "pow": pow_, "atan2": m["atan2"], "hypot": m["hypot"], "mod": mod, "remainder": remainder,
        "frexp": m["frexp"], "modf": m["modf"], "ldexp": m["ldexp"], "pow10": m["pow10"],
        "sincos": lambda x: m["sincos_small"](x) if abs(x) < CONSTS["REDUCE"] else m["sincos_large"](x),
    }


def verify() -> None:
    """Every model matches every reference vector bit for bit (NaN payloads aside)."""
    vec = json.loads(FIXTURE.read_text())
    fns = models()
    bad = []
    for name, fn in fns.items():
        w = words(vec[name])
        if name in ("pow", "atan2", "hypot", "mod", "remainder"):
            for i in range(0, len(w), 3):
                if not same(fn(fb(w[i]), fb(w[i + 1])), w[i + 2]):
                    bad.append((name, fb(w[i]), fb(w[i + 1])))
        elif name in ("frexp", "modf", "sincos"):
            for i in range(0, len(w), 3):
                a, b = fn(fb(w[i]))
                second = (b & M64) if name == "frexp" else bits(b)
                if not (same(a, w[i + 1]) and (second == w[i + 2] or (name != "frexp" and same(b, w[i + 2])))):
                    bad.append((name, fb(w[i])))
        elif name == "ldexp":
            for i in range(0, len(w), 3):
                e = w[i + 1] - (1 << 64) if w[i + 1] >> 63 else w[i + 1]
                if not same(fn(fb(w[i]), e), w[i + 2]):
                    bad.append((name, fb(w[i]), e))
        elif name == "pow10":
            for i in range(0, len(w), 2):
                n = w[i] - (1 << 64) if w[i] >> 63 else w[i]
                if not same(fn(n), w[i + 1]):
                    bad.append((name, n))
        else:
            for i in range(0, len(w), 2):
                if not same(fn(fb(w[i])), w[i + 1]):
                    bad.append((name, fb(w[i])))
    if bad:
        raise AssertionError(f"{len(bad)} mismatches, first {bad[:8]}")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    verify()
    text = TARGET.read_text()
    head = text.split(MARKER)[0]
    new = head + render()
    if args.check:
        if new != text:
            sys.exit(f"{TARGET} is stale; run {Path(__file__).name}")
        return
    TARGET.write_text(new)


if __name__ == "__main__":
    main()
