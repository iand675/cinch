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
from float_dsl import Kernel, Program, M64, _fdiv  # noqa: E402

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


# Coefficient tables of the special functions (binary64 words, read with fpick).
def _words(*groups):
    return [bits(float(v)) for g in groups for v in g]


TABLES.update({
    "PZ_R": _words([0.0, -0.07031249999999004, -8.081670412753498, -257.06310567970485, -2485.216410094288, -5253.043804907295], [-1.141254646918945e-11, -0.07031249408735993, -4.159610644705878, -67.67476522651673, -331.23129964917297, -346.4333883656049], [-2.547046017719519e-09, -0.07031196163814817, -2.409032215495296, -21.96597747348831, -58.07917047017376, -31.44794705948885], [-8.875343330325264e-08, -0.07030309954836247, -1.4507384678095299, -7.635696138235278, -11.193166886035675, -3.2336457935133534]),
    "PZ_S": _words([116.53436461966818, 3833.7447536412183, 40597.857264847255, 116752.97256437592, 47627.728414673096], [60.753938269230034, 1051.2523059570458, 5978.970943338558, 9625.445143577745, 2406.058159229391], [35.85603380552097, 361.51398305030386, 1193.6078379211153, 1127.9967985690741, 173.58093081333575], [22.22029975320888, 136.2067942182152, 270.4702786580835, 153.87539420832033, 14.65761769482562]),
    "QZ_R": _words([0.0, 0.0732421874999935, 11.76820646822527, 557.6733802564019, 8859.197207564686, 37014.62677768878], [1.8408596359451553e-11, 0.07324217666126848, 5.8356350896205695, 135.11157728644983, 1027.243765961641, 1989.9778586460538], [4.377410140897386e-09, 0.07324111800429114, 3.344231375161707, 42.621844074541265, 170.8080913405656, 166.73394869665117], [1.5044444488698327e-07, 0.07322342659630793, 1.99819174093816, 14.495602934788574, 31.666231750478154, 16.252707571092927]),
    "QZ_S": _words([163.77602689568982, 8098.344946564498, 142538.29141912048, 803309.2571195144, 840501.5798190605, -343899.2935378666], [82.77661022365378, 2077.81416421393, 18847.28877857181, 56751.11228949473, 35976.75384251145, -5354.342756019448], [48.75887297245872, 709.689221056606, 3704.1482262011136, 6460.425167525689, 2516.3336892036896, -149.2474518361564], [30.36558483552192, 269.34811860804984, 844.7837575953201, 882.9358451124886, 212.66638851179883, -5.3109549388266695]),
    "PO_R": _words([0.0, 0.11718749999998865, 13.239480659307358, 412.05185430737856, 3874.7453891396053, 7914.479540318917], [1.3199051955624352e-11, 0.1171874931906141, 6.802751278684329, 108.30818299018911, 517.6361395331998, 528.7152013633375], [3.025039161373736e-09, 0.11718686556725359, 3.9329775003331564, 35.11940355916369, 91.05501107507813, 48.55906851973649], [1.0771083010687374e-07, 0.11717621946268335, 2.368514966676088, 12.242610914826123, 17.693971127168773, 5.073523125888185]),
    "PO_S": _words([114.20737037567841, 3650.9308342085346, 36956.206026903346, 97602.79359349508, 30804.27206278888], [59.28059872211313, 991.4014187336144, 5353.26695291488, 7844.690317495512, 1504.0468881036106], [34.79130950012515, 336.76245874782575, 1046.8713997577513, 890.8113463982564, 103.78793243963928], [21.43648593638214, 125.29022716840275, 232.2764690571628, 117.6793732871471, 8.364638933716183]),
    "QO_R": _words([0.0, -0.10253906249999271, -16.271753454459, -759.6017225139501, -11849.806670242959, -48438.512428575035], [-2.089799311417641e-11, -0.10253905024137543, -8.05644828123936, -183.66960747488838, -1373.1937606550816, -2612.4444045321566], [-5.078312264617666e-09, -0.10253782982083709, -4.610115811394734, -57.847221656278364, -228.2445407376317, -219.21012847890933], [-1.7838172751095887e-07, -0.10251704260798555, -2.7522056827818746, -19.663616264370372, -42.32531333728305, -21.371921170370406]),
    "QO_S": _words([161.3953697007229, 7825.385999233485, 133875.33628724958, 719657.7236832409, 666601.2326177764, -294490.26430383464], [81.27655013843358, 1991.7987346048596, 17468.48519249089, 49851.42709103523, 27948.075163891812, -4719.183547951285], [47.66515503237295, 673.8651126766997, 3380.1528667952634, 5547.729097207228, 1903.119193388108, -135.20119144430734], [29.533362906052385, 252.98154998219053, 757.5028348686454, 739.3932053204672, 155.94900333666612, -4.959498988226282]),
    "GAM_P": _words([0.00016011952247675185, 0.0011913514700658638, 0.010421379756176158, 0.04763678004571372, 0.20744822764843598, 0.4942148268014971, 1.0]),
    "GAM_Q": _words([-2.3158187332412014e-05, 0.0005396055804933034, -0.004456419138517973, 0.011813978522206043, 0.035823639860549865, -0.23459179571824335, 0.0714304917030273, 1.0]),
    "GAM_S": _words([0.0007873113957930937, -0.00022954996161337813, -0.0026813261780578124, 0.0034722222160545866, 0.08333333333334822]),
    "LG_A": _words([0.07721566490153287, 0.3224670334241136, 0.06735230105312927, 0.020580808432516733, 0.007385550860814029, 0.0028905138367341563, 0.0011927076318336207, 0.0005100697921535113, 0.00022086279071390839, 0.00010801156724758394, 2.5214456545125733e-05, 4.4864094961891516e-05]),
    "LG_R": _words([1.0, 1.3920053346762105, 0.7219355475671381, 0.17193386563280308, 0.01864591917156529, 0.0007779424963818936, 7.326684307446256e-06]),
    "LG_S": _words([-0.07721566490153287, 0.21498241596060885, 0.325778796408931, 0.14635047265246445, 0.02664227030336386, 0.0018402845140733772, 3.194753265841009e-05]),
    "LG_T": _words([0.48383612272381005, -0.1475877229945939, 0.06462494023913339, -0.032788541075985965, 0.01797067508118204, -0.010314224129834144, 0.006100538702462913, -0.0036845201678113826, 0.0022596478090061247, -0.0014034646998923284, 0.000881081882437654, -0.0005385953053567405, 0.00031563207090362595, -0.00031275416837512086, 0.0003355291926355191]),
    "LG_U": _words([-0.07721566490153287, 0.6328270640250934, 1.4549225013723477, 0.9777175279633727, 0.22896372806469245, 0.013381091853678766]),
    "LG_V": _words([1.0, 2.4559779371304113, 2.128489763798934, 0.7692851504566728, 0.10422264559336913, 0.003217092422824239]),
    "LG_W": _words([0.4189385332046727, 0.08333333333333297, -0.0027777777772877554, 0.0007936505586430196, -0.00059518755745034, 0.0008363399189962821, -0.0016309293409657527]),
})

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

# --- Special functions ------------------------------------------------------------------------------------

SQRTPI = Decimal("1.77245385090551602729816748334114518279754945612238712821380779")
YMIN = Decimal("1.461632144968362245")
TC = Decimal("1.46163214496836224576")
CONSTS.update({
    "ERF_INV035": float(Decimal(1) / Decimal("0.35")), "VERYTINY": 2.848094538889218e-306,
    "ERX": 8.45062911510467529297e-01, "EFX": 1.28379167095512586316e-01, "EFX8": 1.02703333676410069053e+00,
    "TINY56": 2.0**-56, "TINY70": 2.0**-70, "TWO58": 2.0**58, "TWO_M27": 2.0**-27, "TWO_M13": 2.0**-13,
    "TWO129": 2.0**129, "INV_SQRTPI": float(1 / SQRTPI), "TWO_OVER_PI": float(2 / PI),
    "LG_A1": float(YMIN - 1 + Decimal("0.27")), "LG_A2": float(YMIN - 1 - Decimal("0.27")),
    "LG_B1": float(YMIN + Decimal("0.27")), "LG_B2": float(YMIN - Decimal("0.27")),
    "LG_TC": float(TC), "LG_TC1": float(TC - 1), "LG_TF": -1.21486290535849611461e-01,
    "LG_TT": -3.63867699703950536541e-18, "EULER": float(Decimal("0.57721566490153286060651209008240243104215933593992")),
    "SQRT_TWO_PI": 2.506628274631000502417,
})
ERF = {
    "PP": [1.28379167095512558561e-01, -3.25042107247001499370e-01, -2.84817495755985104766e-02, -5.77027029648944159157e-03, -2.37630166566501626084e-05],
    "QQ": [0.0, 3.97917223959155352819e-01, 6.50222499887672944485e-02, 5.08130628187576562776e-03, 1.32494738004321644526e-04, -3.96022827877536812320e-06],
    "PA": [-2.36211856075265944077e-03, 4.14856118683748331666e-01, -3.72207876035701323847e-01, 3.18346619901161753674e-01, -1.10894694282396677476e-01, 3.54783043256182359371e-02, -2.16637559486879084300e-03],
    "QA": [0.0, 1.06420880400844228286e-01, 5.40397917702171048937e-01, 7.18286544141962662868e-02, 1.26171219808761642112e-01, 1.36370839120290507362e-02, 1.19844998467991074170e-02],
    "RA": [-9.86494403484714822705e-03, -6.93858572707181764372e-01, -1.05586262253232909814e+01, -6.23753324503260060396e+01, -1.62396669462573470355e+02, -1.84605092906711035994e+02, -8.12874355063065934246e+01, -9.81432934416914548592e+00],
    "SA": [0.0, 1.96512716674392571292e+01, 1.37657754143519042600e+02, 4.34565877475229228821e+02, 6.45387271733267880336e+02, 4.29008140027567833386e+02, 1.08635005541779435134e+02, 6.57024977031928170135e+00, -6.04244152148580987438e-02],
    "RB": [-9.86494292470009928597e-03, -7.99283237680523006574e-01, -1.77579549177547519889e+01, -1.60636384855821916062e+02, -6.37566443368389627722e+02, -1.02509513161107724954e+03, -4.83519191608651397019e+02],
    "SB": [0.0, 3.03380607434824582924e+01, 3.25792512996573918826e+02, 1.53672958608443695994e+03, 3.19985821950859553908e+03, 2.55305040643316442583e+03, 4.74528541206955367215e+02, -2.24409524465858183362e+01],
    "EIA": [1.1975323115670912564578e0, 4.7072688112383978012285e1, 6.9706266534389598238465e2, 4.8548868893843886794648e3, 1.6235862515167575384252e4, 2.3782041382114385731252e4, 1.1819493347062294404278e4, 8.8709406962545514830200e2],
    "EIB": [1.0000000000000000000e0, 4.2313330701600911252e1, 6.8718700749205790830e2, 5.3941960214247511077e3, 2.1213794301586595867e4, 3.9307895800092710610e4, 2.8729085735721942674e4, 5.2264952788528545610e3],
    "EIC": [1.42343711074968357734e0, 4.63033784615654529590e0, 5.76949722146069140550e0, 3.64784832476320460504e0, 1.27045825245236838258e0, 2.41780725177450611770e-1, 2.27238449892691845833e-2, 7.74545014278341407640e-4],
    "EID": [1.4142135623730950488016887e0, 2.9036514445419946173133295e0, 2.3707661626024532365971225e0, 9.7547832001787427186894837e-1, 2.0945065210512749128288442e-1, 2.1494160384252876777097297e-2, 7.7441459065157709165577218e-4, 1.4859850019840355905497876e-9],
    "EIE": [6.65790464350110377720e0, 5.46378491116411436990e0, 1.78482653991729133580e0, 2.96560571828504891230e-1, 2.65321895265761230930e-2, 1.24266094738807843860e-3, 2.71155556874348757815e-5, 2.01033439929228813265e-7],
    "EIF": [1.414213562373095048801689e0, 8.482908416595164588112026e-1, 1.936480946950659106176712e-1, 2.103693768272068968719679e-2, 1.112800997078859844711555e-3, 2.611088405080593625138020e-5, 2.010321207683943062279931e-7, 2.891024605872965461538222e-15],
    "J0R": [0.0, 0.0, 1.56249999999999947958e-02, -1.89979294238854721751e-04, 1.82954049532700665670e-06, -4.61832688532103189199e-09],
    "J0S": [0.0, 1.56191029464890010492e-02, 1.16926784663337450260e-04, 5.13546550207318111446e-07, 1.16614003333790000205e-09],
    "Y0U": [-7.38042951086872317523e-02, 1.76666452509181115538e-01, -1.38185671945596898896e-02, 3.47453432093683650238e-04, -3.81407053724364161125e-06, 1.95590137035022920206e-08, -3.98205194132103398453e-11],
    "Y0V": [0.0, 1.27304834834123699328e-02, 7.60068627350353253702e-05, 2.59150851840457805467e-07, 4.41110311332675467403e-10],
    "J1R": [-6.25000000000000000000e-02, 1.40705666955189706048e-03, -1.59955631084035597520e-05, 4.96727999609584448412e-08],
    "J1S": [0.0, 1.91537599538363460805e-02, 1.85946785588630915560e-04, 1.17718464042623683263e-06, 5.04636257076217042715e-09, 1.23542274426137913908e-11],
    "Y1U": [-1.96057090646238940668e-01, 5.04438716639811282616e-02, -1.91256895875763547298e-03, 2.35252600561610495928e-05, -9.19099158039878874504e-08],
    "Y1V": [1.99167318236649903973e-02, 2.02552581025135171496e-04, 1.35608801097516229404e-06, 6.22741452364621501295e-09, 1.66559246207992079114e-11],
}
TABLES.update({k: [bits(v) for v in vs] for k, vs in ERF.items()})


def c(table: str, i: int) -> str:
    """A coefficient reference."""
    return f"fpick({table}, {i})"


def nested(var: str, terms: list[str]) -> str:
    """t0 + v*(t1 + v*(t2 + ...)), the association the reference evaluates."""
    acc = terms[-1]
    for term in reversed(terms[:-1]):
        acc = f"{term} + {var} * ({acc})"
    return acc


def horner(var: str, terms: list[str]) -> str:
    """((t0*v + t1)*v + t2)*v + ..., highest coefficient first."""
    acc = terms[0]
    for term in terms[1:]:
        acc = f"({acc}) * {var} + {term}"
    return acc


def span(table: str, lo: int, hi: int) -> list[str]:
    return [c(table, i) for i in range(lo, hi)]


kernel("erf-tail", "f", "", f'''
def erf_tail(x):
    s = 1.0 / (x * x)
    r = {nested("s", span("RB", 0, 7))}
    q = 1.0 + s * ({nested("s", span("SB", 1, 8))})
    if x < ERF_INV035:
        r = {nested("s", span("RA", 0, 8))}
        q = 1.0 + s * ({nested("s", span("SA", 1, 9))})
    z = fbits(bits(x) & 0xFFFFFFFF00000000)
    return exp(-z * z - 0.5625) * exp((z - x) * (z + x) + r / q)
''', entry=False)

kernel("erf-small", "f", "", f'''
def erf_small(x):
    z = x * x
    r = {nested("z", span("PP", 0, 5))}
    s = 1.0 + z * ({nested("z", span("QQ", 1, 6))})
    return r / s
''', entry=False)

kernel("erf-mid", "f", "", f'''
def erf_mid(x):
    s = x - 1.0
    p = {nested("s", span("PA", 0, 7))}
    q = 1.0 + s * ({nested("s", span("QA", 1, 7))})
    return p / q
''', entry=False)

kernel("erf-f64", "f", "The error function, 2/sqrt(pi) times the integral of exp(-t^2) from 0 to x.", '''
def erf(x):
    if isnan(x):
        return NAN
    if x == INF:
        return 1.0
    if x == -INF:
        return -1.0
    sign = x < 0.0
    if sign:
        x = -x
    if x < 0.84375:
        temp = x + x * erf_small(x)
        if x < NEAR28:
            temp = x + EFX * x
            if x < VERYTINY:
                temp = 0.125 * (8.0 * x + EFX8 * x)
        if sign:
            return -temp
        return temp
    if x < 1.25:
        if sign:
            return -ERX - erf_mid(x)
        return ERX + erf_mid(x)
    if x >= 6.0:
        if sign:
            return -1.0
        return 1.0
    r = erf_tail(x)
    if sign:
        return r / x - 1.0
    return 1.0 - r / x
''')

kernel("erfc-f64", "f", "The complementary error function, 1 - erf(x), accurate where erf(x) is near 1.", '''
def erfc(x):
    if isnan(x):
        return NAN
    if x == INF:
        return 0.0
    if x == -INF:
        return 2.0
    sign = x < 0.0
    if sign:
        x = -x
    if x < 0.84375:
        y = erf_small(x)
        temp = 0.5 + (x * y + (x - 0.5))
        if x < 0.25:
            temp = x + x * y
        if x < TINY56:
            temp = x
        if sign:
            return 1.0 + temp
        return 1.0 - temp
    if x < 1.25:
        if sign:
            return 1.0 + ERX + erf_mid(x)
        return 1.0 - ERX - erf_mid(x)
    if x < 28.0:
        if sign and x > 6.0 and x >= ERF_INV035:
            return 2.0
        r = erf_tail(x)
        if sign:
            return 2.0 - r / x
        return r / x
    if sign:
        return 2.0
    return 0.0
''')

kernel("erfinv-f64", "f", "The inverse error function: the y with erf(y) = x, for x in (-1, 1).", f'''
def erfinv(x):
    if isnan(x) or x <= -1.0 or x >= 1.0:
        if x == -1.0:
            return -INF
        if x == 1.0:
            return INF
        return NAN
    sign = x < 0.0
    if sign:
        x = -x
    r = 0.180625 - 0.25 * x * x
    ans = (x * ({horner("r", [c("EIA", i) for i in range(7, -1, -1)])})) / ({horner("r", [c("EIB", i) for i in range(7, -1, -1)])})
    if x > 0.85:
        t = sqrt(LN2 - log(1.0 - x))
        u = t - 1.6
        z1 = {horner("u", [c("EIC", i) for i in range(7, -1, -1)])}
        z2 = {horner("u", [c("EID", i) for i in range(7, -1, -1)])}
        if t > 5.0:
            u = t - 5.0
            z1 = {horner("u", [c("EIE", i) for i in range(7, -1, -1)])}
            z2 = {horner("u", [c("EIF", i) for i in range(7, -1, -1)])}
        ans = z1 / z2
    if sign:
        return -ans
    return ans
''')

kernel("erfcinv-f64", "f", "The inverse complementary error function: erfinv(1 - x).", '''
def erfcinv(x):
    return erfinv(1.0 - x)
''')

kernel("sin-pi", "f", "", '''
def sin_pi(x):
    z = floor(x)
    y = x
    n = 0
    if not (z == x):
        y = x - 2.0 * floor(x * 0.5)
        n = ftoi(y * 4.0)
    else:
        if x >= TWO53:
            y = 0.0
            n = 0
        else:
            zz = z
            if x < TWO52:
                zz = x + TWO52
            n = i_(1 & bits(zz))
            y = itof(n)
            n = n << 2
    v = sin_small(PI * (y - 2.0))
    if n == 0:
        v = sin_small(PI * y)
    if n == 1 or n == 2:
        v = cos_small(PI * (0.5 - y))
    if n == 3 or n == 4:
        v = sin_small(PI * (1.0 - y))
    if n == 5 or n == 6:
        v = -cos_small(PI * (y - 1.5))
    if x < 0.25:
        return -sin_small(PI * x)
    return -v
''', entry=False)

kernel("lgamma-kernel", "f", "The natural logarithm of |Gamma(x)| (low half) and the sign of Gamma(x) (high half: 1 or -1).", f'''
def lgamma(x):
    sign = 1
    if isnan(x) or fabs(x) == INF:
        return x, sign
    if x == 0.0:
        return INF, sign
    neg = x < 0.0
    if neg:
        x = -x
    if x < TINY70:
        if neg:
            sign = -1
        return -log(x), sign
    nadj = 0.0
    if neg:
        if x >= TWO52:
            return INF, sign
        t = sin_pi(x)
        if t == 0.0:
            return INF, sign
        nadj = log(PI / fabs(t * x))
        if t < 0.0:
            sign = -1
    if x == 1.0 or x == 2.0:
        return 0.0, sign
    lg = 0.0
    if x < 2.0:
        y = 0.0
        i = 0
        if x <= 0.9:
            lg = -log(x)
            y = x
            i = 2
            if x >= LG_A2:
                y = x - LG_TC1
                i = 1
            if x >= LG_A1:
                y = 1.0 - x
                i = 0
        else:
            y = x - 1.0
            i = 2
            if x >= LG_B2:
                y = x - LG_TC
                i = 1
            if x >= LG_B1:
                y = 2.0 - x
                i = 0
        z = y * y
        p1 = {nested("z", [c("LG_A", k) for k in (0, 2, 4, 6, 8, 10)])}
        p2 = z * ({nested("z", [c("LG_A", k) for k in (1, 3, 5, 7, 9, 11)])})
        d0 = (y * p1 + p2) - 0.5 * y
        w = z * y
        t1 = {nested("w", [c("LG_T", k) for k in (0, 3, 6, 9, 12)])}
        t2 = {nested("w", [c("LG_T", k) for k in (1, 4, 7, 10, 13)])}
        t3 = {nested("w", [c("LG_T", k) for k in (2, 5, 8, 11, 14)])}
        d1 = LG_TF + (z * t1 - (LG_TT - w * (t2 + y * t3)))
        u1 = y * ({nested("y", span("LG_U", 0, 6))})
        u2 = 1.0 + y * ({nested("y", span("LG_V", 1, 6))})
        d2 = -0.5 * y + u1 / u2
        d = d2
        if i == 0:
            d = d0
        if i == 1:
            d = d1
        lg = lg + d
    else:
        if x < 8.0:
            k = ftoi(x)
            y = x - itof(k)
            p = y * ({nested("y", span("LG_S", 0, 7))})
            q = 1.0 + y * ({nested("y", span("LG_R", 1, 7))})
            lg = 0.5 * y + p / q
            z = 1.0
            if k == 7:
                z = z * (y + 6.0)
            if k >= 6:
                z = z * (y + 5.0)
            if k >= 5:
                z = z * (y + 4.0)
            if k >= 4:
                z = z * (y + 3.0)
            if k >= 3:
                z = z * (y + 2.0)
                lg = lg + log(z)
        else:
            if x < TWO58:
                t = log(x)
                z = 1.0 / x
                y = z * z
                w = {c("LG_W", 0)} + z * ({nested("y", span("LG_W", 1, 7))})
                lg = (x - 0.5) * (t - 1.0) + w
            else:
                lg = x * (log(x) - 1.0)
    if neg:
        lg = nadj - lg
    return lg, sign
''')

kernel("gamma-poly", "ff", "Gamma's rational approximation on [2, 3): z * P(x - 2) / Q(x - 2).", f'''
def gamma_poly(x, z):
    x = x - 2.0
    p = {horner("x", span("GAM_P", 0, 7))}
    q = {horner("x", span("GAM_Q", 0, 8))}
    return z * p / q
''')

kernel("gamma-tiny", "ff", "Gamma near zero: z / ((1 + Euler x) x), infinite at zero.", '''
def gamma_tiny(x, z):
    if x == 0.0:
        return INF
    return z / ((1.0 + EULER * x) * x)
''')

kernel("stirling-w", "f", "The Stirling series correction 1 + w(1/x).", f'''
def stirling_w(x):
    w = 1.0 / x
    return 1.0 + w * ({horner("w", span("GAM_S", 0, 5))})
''')


def bessel_pq(name: str, rt: str, st: str, ns: int, lead: str, finish: str) -> None:
    kernel(name.replace("_", "-"), "f", "", f'''
def {name}(x):
    i: u = 3
    if x >= 2.8571:
        i = 2
    if x >= 4.5454:
        i = 1
    if x >= 8.0:
        i = 0
    z = 1.0 / (x * x)
    r = {nested("z", [f"fpick({rt}, i * 6 + {k})" for k in range(6)])}
    s = 1.0 + z * ({nested("z", [f"fpick({st}, i * {ns} + {k})" for k in range(ns)])})
    return {finish}
''', entry=False)


bessel_pq("pzero", "PZ_R", "PZ_S", 5, "", "1.0 + r / s")
bessel_pq("qzero", "QZ_R", "QZ_S", 6, "", "(-0.125 + r / s) / x")
bessel_pq("pone", "PO_R", "PO_S", 5, "", "1.0 + r / s")
bessel_pq("qone", "QO_R", "QO_S", 6, "", "(0.375 + r / s) / x")

kernel("j0-poly", "f", "", f'''
def j0_poly(x):
    if x < TWO_M13:
        if x < TWO_M27:
            return 1.0
        return 1.0 - 0.25 * x * x
    z = x * x
    r = z * ({nested("z", span("J0R", 2, 6))})
    s = 1.0 + z * ({nested("z", span("J0S", 1, 5))})
    if x < 1.0:
        return 1.0 + z * (-0.25 + (r / s))
    u = 0.5 * x
    return (1.0 + u) * (1.0 - u) + z * (r / s)
''', entry=False)

kernel("j1-poly", "f", "", f'''
def j1_poly(x):
    if x < TWO_M27:
        return 0.5 * x
    z = x * x
    r = z * ({nested("z", span("J1R", 0, 4))})
    s = 1.0 + z * ({nested("z", span("J1S", 1, 6))})
    r = r * x
    return 0.5 * x + r / s
''', entry=False)

for size in ("small", "large"):
    trig = (f"s, c = sincos_{size}(x)", f"cos_{size}(x + x)") if size == "small" else ("s, c = sincos_any(x)", "cos_any(x + x)")
    common = f'''
        {trig[0]}
        ss = SSE
        cc = CCE
        if x < HALF_MAX:
            z = ZSIGN{trig[1]}
            if s * c CMP 0.0:
                cc = z / ss
            else:
                ss = z / cc
'''
    j0 = common.replace("SSE", "s - c").replace("CCE", "s + c").replace("ZSIGN", "-").replace("CMP", "<")
    j1 = common.replace("SSE", "-s - c").replace("CCE", "s - c").replace("ZSIGN", "").replace("CMP", ">")
    where = "arguments below 2^28" if size == "small" else "any argument"
    kernel(f"j0-{size}", "f", f"The Bessel function of the first kind of order zero, for {where}.", f'''
def j0_{size}(x):
    if isnan(x):
        return x
    if fabs(x) == INF:
        return 0.0
    if x == 0.0:
        return 1.0
    x = fabs(x)
    if x >= 2.0:{j0}
        z = INV_SQRTPI * cc / sqrt(x)
        if x <= TWO129:
            z = INV_SQRTPI * (pzero(x) * cc - qzero(x) * ss) / sqrt(x)
        return z
    return j0_poly(x)
''')
    kernel(f"y0-{size}", "f", f"The Bessel function of the second kind of order zero, for {where}.", f'''
def y0_{size}(x):
    if x < 0.0 or isnan(x):
        return NAN
    if x == INF:
        return 0.0
    if x == 0.0:
        return -INF
    if x >= 2.0:{j0}
        z = INV_SQRTPI * ss / sqrt(x)
        if x <= TWO129:
            z = INV_SQRTPI * (pzero(x) * ss + qzero(x) * cc) / sqrt(x)
        return z
    if x <= TWO_M27:
        return {c("Y0U", 0)} + TWO_OVER_PI * log(x)
    z = x * x
    u = {nested("z", span("Y0U", 0, 7))}
    v = 1.0 + z * ({nested("z", span("Y0V", 1, 5))})
    return u / v + TWO_OVER_PI * j0_poly(x) * log(x)
''')
    kernel(f"j1-{size}", "f", f"The Bessel function of the first kind of order one, for {where}.", f'''
def j1_{size}(x):
    if isnan(x):
        return x
    if fabs(x) == INF or x == 0.0:
        return 0.0
    sign = x < 0.0
    if sign:
        x = -x
    if x >= 2.0:{j1}
        z = INV_SQRTPI * cc / sqrt(x)
        if x <= TWO129:
            z = INV_SQRTPI * (pone(x) * cc - qone(x) * ss) / sqrt(x)
        if sign:
            return -z
        return z
    z = j1_poly(x)
    if sign:
        return -z
    return z
''')
    kernel(f"y1-{size}", "f", f"The Bessel function of the second kind of order one, for {where}.", f'''
def y1_{size}(x):
    if x < 0.0 or isnan(x):
        return NAN
    if x == INF:
        return 0.0
    if x == 0.0:
        return -INF
    if x >= 2.0:{j1}
        z = INV_SQRTPI * ss / sqrt(x)
        if x <= TWO129:
            z = INV_SQRTPI * (pone(x) * ss + qone(x) * cc) / sqrt(x)
        return z
    if x <= TINY54:
        return -TWO_OVER_PI / x
    z = x * x
    u = {nested("z", span("Y1U", 0, 5))}
    v = 1.0 + z * ({nested("z", span("Y1V", 0, 5))})
    return x * (u / v) + TWO_OVER_PI * (j1_poly(x) * log(x) - 1.0 / x)
''')

kernel("sincos-any", "f", "", '''
def sincos_any(x):
    a, b = sincos_large(x)
    if fabs(x) < REDUCE:
        a, b = sincos_small(x)
    return a, b
''', entry=False)

kernel("cos-any", "f", "", '''
def cos_any(x):
    if fabs(x) < REDUCE:
        return cos_small(x)
    return cos_large(x)
''', entry=False)

kernel("bessel-asymptotic", "if", "The large-argument form of the first kind: sin and cos combined by n mod 4, over sqrt(pi x).", '''
def bessel_asym(n, x):
    s, c = sincos_any(x)
    q = n & 3
    t = c - s
    if q == 0:
        t = c + s
    if q == 1:
        t = -c + s
    if q == 2:
        t = -c - s
    return INV_SQRTPI * t / sqrt(x)
''')

kernel("bessel-asymptotic-second", "if", "The large-argument form of the second kind: sin and cos combined by n mod 4, over sqrt(pi x).", '''
def bessel_asym2(n, x):
    s, c = sincos_any(x)
    q = n & 3
    t = s + c
    if q == 0:
        t = s - c
    if q == 1:
        t = -s - c
    if q == 2:
        t = -s + c
    return INV_SQRTPI * t / sqrt(x)
''')

kernel("fma-f64", "fff", "x * y + z computed exactly and rounded once (round to nearest, ties to even).", '''
def fma(x, y, z):
    bx = bits(x)
    by = bits(y)
    bz = bits(z)
    if x == 0.0 or y == 0.0 or z == 0.0 or (bx & 0x7FF0000000000000) == 0x7FF0000000000000 or (by & 0x7FF0000000000000) == 0x7FF0000000000000:
        return x * y + z
    if (bz & 0x7FF0000000000000) == 0x7FF0000000000000:
        return z
    xs, xe, xm = fma_split(bx)
    ys, ye, ym = fma_split(by)
    zs, ze, zm = fma_split(bz)
    pe = xe + ye - 1023 + 1
    pm1, pm2 = mul(xm << 10, ym << 11)
    zm1: u = zm << 10
    zm2: u = 0
    ps = xs ^ ys
    is62zero = ((~pm1) >> 62) & 1
    pm1, pm2 = fma_shl(pm1, pm2, is62zero)
    pe = pe - i_(is62zero)
    if pe < ze or (pe == ze and pm1 < zm1):
        ps, pe, pm1, pm2, zs, ze, zm1, zm2 = zs, ze, zm1, zm2, ps, pe, pm1, pm2
    if not (ps == zs) and pe == ze and pm1 == zm1 and pm2 == zm2:
        return 0.0
    zm1, zm2 = fma_shrcompress(zm1, zm2, u_(pe - ze))
    m: u = 0
    if ps == zs:
        lo = pm2 + zm2
        carry: u = 0
        if lo < pm2:
            carry = 1
        pm1 = pm1 + zm1 + carry
        pm2 = lo
        pe = pe - i_((~pm1) >> 63)
        pm1, m = fma_shrcompress(pm1, pm2, 64 + (pm1 >> 63))
    else:
        lo = pm2 - zm2
        borrow: u = 0
        if pm2 < zm2:
            borrow = 1
        pm1 = pm1 - zm1 - borrow
        pm2 = lo
        nz = fma_lz(pm1, pm2)
        pe = pe - nz
        m, pm2 = fma_shl(pm1, pm2, u_(nz - 1))
        m = m | fma_nonzero(pm2)
    if pe > 1022 + 1023 or (pe == 1022 + 1023 and ((m + (1 << 9)) >> 63) == 1):
        return fbits((ps << 63) | 0x7FF0000000000000)
    if pe < 0:
        n = u_(-pe)
        m = (m >> n) | fma_nonzero(m & ((1 << n) - 1))
        pe = 0
    m = ((m + (1 << 9)) >> 10) & ~fma_zero((m & ((1 << 10) - 1)) ^ (1 << 9))
    pe = pe & (-i_(fma_nonzero(m)))
    return fbits((ps << 63) + (u_(pe) << 52) + m)
''')

kernel("fma-zero", "u", "", '''
def fma_zero(x):
    if x == 0:
        return u_(1)
    return u_(0)
''', entry=False)

kernel("fma-nonzero", "u", "", '''
def fma_nonzero(x):
    if x == 0:
        return u_(0)
    return u_(1)
''', entry=False)

kernel("fma-shl", "uuu", "", '''
def fma_shl(u1, u2, n):
    return (u1 << n) | (u2 >> (64 - n)) | (u2 << (n - 64)), u2 << n
''', entry=False)

kernel("fma-shr", "uuu", "", '''
def fma_shr(u1, u2, n):
    return u1 >> n, (u2 >> n) | (u1 << (64 - n)) | (u1 >> (n - 64))
''', entry=False)

kernel("fma-shrcompress", "uuu", "", '''
def fma_shrcompress(u1, u2, n):
    r1, r2 = fma_shr(u1, u2, n)
    sticky = fma_nonzero((u1 & ((1 << (n - 64)) - 1)) | u2)
    if n < 64:
        sticky = fma_nonzero(u2 & ((1 << n) - 1))
    r2 = r2 | sticky
    if n >= 128:
        r1 = u_(0)
        r2 = fma_nonzero(u1 | u2)
    if n == 64:
        r1 = u_(0)
        r2 = u1 | fma_nonzero(u2)
    if n == 0:
        r1 = u1
        r2 = u2
    return r1, r2
''', entry=False)

kernel("fma-lz", "uu", "", '''
def fma_lz(u1, u2):
    l = i_(clz(u1))
    if l == 64:
        l = l + i_(clz(u2))
    return l
''', entry=False)

kernel("fma-split", "u", "", '''
def fma_split(b):
    sign = b >> 63
    e = i_((b >> 52) & 2047)
    m = b & 0x000FFFFFFFFFFFFF
    if e == 0:
        shift = clz(m) - 11
        m = m << shift
        e = 1 - i_(shift)
    else:
        m = m | (1 << 52)
    return sign, e, m
''', entry=False)

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

    def dispatch(name, limit):
        return lambda x: m[f"{name}_small"](x) if abs(x) < limit else m[f"{name}_large"](x)

    j0, j1, y0, y1 = (dispatch(n, 2.0**28) for n in ("j0", "j1", "y0", "y1"))

    def stirling(x):
        if x > 200:
            return math.inf, 1.0
        w = m["stirling_w"](x)
        y1, y2 = m["exp"](x), 1.0
        if x > 143.01608:
            v = pow_(x, 0.5 * x - 0.25)
            y1, y2 = v, _fdiv(v, y1)
        else:
            y1 = _fdiv(pow_(x, x - 0.5), y1)
        return y1, CONSTS["SQRT_TWO_PI"] * w * y2

    def gamma(x):
        if (x < 0 and m["modf"](x)[1] == 0) or x == -math.inf or x != x:
            return math.nan
        if x == math.inf:
            return math.inf
        if x == 0:
            return math.copysign(math.inf, x)
        q = abs(x)
        p = float(math.floor(q))
        if q > 33:
            if x >= 0:
                a, b = stirling(x)
                return a * b
            sg = -1 if int(p) & 1 == 0 else 1
            z = q - p
            if z > 0.5:
                p = p + 1
                z = q - p
            z = q * trig("sin")(CONSTS["PI"] * z)
            if z == 0:
                return math.copysign(math.inf, sg)
            sq1, sq2 = stirling(q)
            absz = abs(z)
            d = absz * sq1 * sq2
            z = _fdiv(_fdiv(_fdiv(CONSTS["PI"], absz), sq1), sq2) if math.isinf(d) else _fdiv(CONSTS["PI"], d)
            return sg * z
        z = 1.0
        while x >= 3:
            x = x - 1
            z = z * x
        while x < 0:
            if x > -1e-09:
                return m["gamma_tiny"](x, z)
            z = z / x
            x = x + 1
        while x < 2:
            if x < 1e-09:
                return m["gamma_tiny"](x, z)
            z = z / x
            x = x + 1
        if x == 2:
            return z
        return m["gamma_poly"](x, z)

    def jn(n, x):
        if x != x:
            return x
        if math.isinf(x):
            return 0.0
        if n == 0:
            return j0(x)
        if x == 0:
            return 0.0
        if n < 0:
            n, x = -n, -x
        if n == 1:
            return j1(x)
        sign = False
        if x < 0:
            x = -x
            sign = n & 1 == 1
        if float(n) <= x:
            if x >= 2.0**302:
                b = m["bessel_asym"](n, x)
            else:
                a, b = j0(x), j1(x)
                for i in range(1, n):
                    a, b = b, b * (float(i + i) / x) - a
        elif x < 2.0**-29:
            if n > 33:
                b = 0.0
            else:
                temp = x * 0.5
                b, a = temp, 1.0
                for i in range(2, n + 1):
                    a *= float(i)
                    b *= temp
                b /= a
        else:
            w = float(n + n) / x
            h = 2 / x
            q0, z = w, w + h
            q1 = w * z - 1
            k = 1
            while q1 < 1e9:
                k += 1
                z += h
                q0, q1 = q1, z * q1 - q0
            t = 0.0
            i = 2 * (n + k)
            while i >= n + n:
                t = 1 / (float(i) / x - t)
                i -= 2
            a, b = t, 1.0
            tmp = float(n)
            tmp = tmp * m["log"](abs(2 / x * tmp))
            for i in range(n - 1, 0, -1):
                di = float(i + i)
                a, b = b, b * di / x - a
                if tmp >= 7.09782712893383973096e+02 and b > 1e100:
                    a /= b
                    t /= b
                    b = 1.0
            b = t * j0(x) / b
        return -b if sign else b

    def yn(n, x):
        if x < 0 or x != x:
            return math.nan
        if x == math.inf:
            return 0.0
        if n == 0:
            return y0(x)
        if x == 0:
            return math.inf if n < 0 and n & 1 == 1 else -math.inf
        sign = False
        if n < 0:
            n = -n
            sign = n & 1 == 1
        if n == 1:
            return -y1(x) if sign else y1(x)
        if x >= 2.0**302:
            b = m["bessel_asym2"](n, x)
        else:
            a, b = y0(x), y1(x)
            i = 1
            while i < n and b != -math.inf:
                a, b = b, (float(i + i) / x) * b - a
                i += 1
        return -b if sign else b

    return {
        "erf": m["erf"], "erfc": m["erfc"], "erfinv": m["erfinv"], "erfcinv": m["erfcinv"], "gamma": gamma,
        "j0": j0, "j1": j1, "y0": y0, "y1": y1, "lgamma": m["lgamma"], "jn": jn, "yn": yn, "fma": m["fma"],
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
        elif name == "lgamma":
            for i in range(0, len(w), 3):
                v, s = fn(fb(w[i]))
                if not (same(v, w[i + 1]) and (s & M64) == w[i + 2]):
                    bad.append((name, fb(w[i])))
        elif name in ("jn", "yn"):
            for i in range(0, len(w), 3):
                n = w[i] - (1 << 64) if w[i] >> 63 else w[i]
                if not same(fn(n, fb(w[i + 1])), w[i + 2]):
                    bad.append((name, n, fb(w[i + 1])))
        elif name == "fma":
            for i in range(0, len(w), 4):
                if not same(fn(fb(w[i]), fb(w[i + 1]), fb(w[i + 2])), w[i + 3]):
                    bad.append((name, fb(w[i]), fb(w[i + 1]), fb(w[i + 2])))
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
