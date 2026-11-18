"""Independent exact-arithmetic models and executable invariants, prepared but not executed during the static phase."""
from __future__ import annotations
import math


def multiply_binary(first: int, second: int, modulus: int) -> int:
    """Check the residue invariant while multiplying without a full product in the modeled update steps."""
    assert first >= 0 and second >= 0 and modulus > 0
    base, remaining, accumulated = first % modulus, second % modulus, 0
    expected = first * second % modulus
    while remaining:
        assert 0 <= base < modulus and 0 <= accumulated < modulus
        assert (accumulated + base * remaining) % modulus == expected
        before = remaining
        if remaining % 2:
            total = accumulated + base
            assert total < 2 * modulus
            accumulated = total - modulus if total >= modulus else total
        remaining //= 2
        assert remaining < before
        if remaining:
            doubled = base + base
            assert doubled < 2 * modulus
            base = doubled - modulus if doubled >= modulus else doubled
    assert accumulated == expected
    return accumulated


def power_binary(base: int, exponent: int, modulus: int) -> int:
    """Check each binary-power state against an independent built-in modular exponentiation."""
    assert base >= 0 and exponent >= 0 and modulus > 0
    expected = pow(base, exponent, modulus)
    base %= modulus
    accumulated = 1 % modulus
    while exponent:
        assert 0 <= base < modulus and 0 <= accumulated < modulus
        assert accumulated * pow(base, exponent, modulus) % modulus == expected
        before = exponent
        if exponent % 2:
            accumulated = multiply_binary(accumulated, base, modulus)
        exponent //= 2
        assert exponent < before
        if exponent:
            base = multiply_binary(base, base, modulus)
    assert accumulated == expected
    return accumulated


def bezout(first: int, second: int) -> tuple[int, int, int]:
    """Prove both linear-combination recurrences and strict remainder progress for Euclid's algorithm."""
    assert first >= 0 and second >= 0
    if first == 0:
        return second, 0, int(second != 0)
    if second == 0:
        return first, 1, 0
    a, b, x0, x1, y0, y1 = first, second, 1, 0, 0, 1
    while b:
        assert a == first * x0 + second * y0
        assert b == first * x1 + second * y1
        quotient, remainder = divmod(a, b)
        assert 0 <= remainder < b
        a, b, x0, x1, y0, y1 = b, remainder, x1, x0 - quotient * x1, y1, y0 - quotient * y1
    assert a == math.gcd(first, second) == first * x0 + second * y0
    return a, x0, y0


def inverse(value: int, modulus: int) -> tuple[str, int]:
    """Separate zero modulus, a non-coprime gcd and the unique canonical inverse residue."""
    assert modulus >= 0
    if modulus == 0:
        return 'zero-modulus', 0
    divisor, first, second = bezout(value % modulus, modulus)
    if divisor != 1:
        return 'not-coprime', divisor
    result = first % modulus
    assert value * result % modulus == 1 % modulus
    assert result == pow(value, -1, modulus)
    return 'ok', result


def signed_power(base: int, exponent: int, modulus: int) -> tuple[str, int]:
    """Model negative exponents through exact inversion without silently using an absent inverse."""
    assert modulus >= 0
    if modulus == 0:
        return 'zero-modulus', 0
    if exponent < 0:
        status, value = inverse(base, modulus)
        if status != 'ok':
            return status, value
        result = power_binary(value, -exponent, modulus)
    else:
        result = power_binary(base % modulus, exponent, modulus)
    assert result == pow(base, exponent, modulus)
    return 'ok', result


def word_product(first: int, second: int, modulus: int) -> int:
    """Check the exact 128-bit product and nonzero divisor premises for the planned machine kernel."""
    assert 0 <= first < 2**64 and 0 <= second < 2**64 and 0 < modulus < 2**64
    product = first * second
    assert 0 <= product < 2**128
    result = product % modulus
    assert 0 <= result < modulus < 2**64
    return result


def word_roundtrip(value: int) -> int | None:
    """Check both signed-safe shortcuts and the full unsigned upper-half reconstruction."""
    assert value >= 0
    if value >= 2**64:
        return None
    word = value if value < 2**63 else (value % 2**32) | ((value // 2**32) << 32)
    signed = word if word < 2**63 else word - 2**64
    result = signed if signed >= 0 else (word & (2**32-1)) + (word >> 32) * 2**32
    assert result == value
    return result


def planned_checks() -> None:
    """Prepare exhaustive small-domain and word/bignum fringe checks for a later authorized model run."""
    for modulus in range(1, 33):
        for first in range(33):
            for second in range(33):
                assert multiply_binary(first, second, modulus) == first * second % modulus
                assert word_product(first, second, modulus) == first * second % modulus
            for exponent in range(10):
                assert power_binary(first, exponent, modulus) == pow(first, exponent, modulus)
            for signed in [first, -first]:
                inverse(signed, modulus)
                for exponent in [-5, -1, 0, 1, 5]:
                    signed_power(signed, exponent, modulus)
    boundaries = [0, 1, 2, 2**32-1, 2**32, 2**63-1, 2**63, 2**64-1, 2**64, 2**127+1, 2**256-189]
    for first in boundaries:
        assert word_roundtrip(first) == (first if first < 2**64 else None)
        for second in boundaries:
            divisor, x, y = bezout(first, second)
            assert divisor == first * x + second * y == math.gcd(first, second)
            for modulus in [1, 2**32-5, 2**63, 2**64-59, 2**64, 2**127-1]:
                multiply_binary(first, second, modulus)
    assert inverse(3, 0) == ('zero-modulus', 0)
    assert inverse(-6, 15) == ('not-coprime', 3)
    assert signed_power(0, -1, 1) == ('ok', 0)


if __name__ == '__main__':
    planned_checks()
    print('All independent exact-arithmetic model checks passed.')
