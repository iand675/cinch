#!/usr/bin/env python3
"""Log-log exponent fitting for the complexity sweep.

An axis is a family of programs parameterised by a size `n`; the compiler's
block count over that family is `a + b*n^k`. Two numbers matter:

  delta      `blocks(n) - blocks(n0)` at the smallest point. This removes the
             fixed floor every compile pays (reading the manifest, the base
             interfaces, the runtime ABI) and leaves the term that grows.
  exponent   the least-squares slope of `ln delta` against `ln n`. A slope of
             1.0 is linear and fine; 1.2 needs a look; 2.0 or worse is a
             defect to name.

The absolute-blocks slope is reported beside it because a term that is
quadratic but small still fits close to linear once the floor dominates; the
gap between the two slopes is itself the diagnostic.
"""

import math


def loglog(xs, ys):
    """Least squares of `ln y = k ln x + c`; returns the fit and its residuals.

    `residuals` are in log space: a point at +0.1 is `e**0.1` times the fitted
    curve, about 10% high. Fewer than two positive points cannot be fitted.
    """
    points = [(x, y) for x, y in zip(xs, ys) if y is not None and x > 0 and y > 0]
    if len(points) < 2:
        return {"exponent": None, "intercept": None, "r2": None,
                "residuals": [], "points": len(points)}
    lx = [math.log(x) for x, _ in points]
    ly = [math.log(y) for _, y in points]
    n = len(lx)
    mean_x = sum(lx) / n
    mean_y = sum(ly) / n
    sxx = sum((x - mean_x) ** 2 for x in lx)
    if sxx == 0:
        return {"exponent": None, "intercept": None, "r2": None,
                "residuals": [], "points": n}
    sxy = sum((x - mean_x) * (y - mean_y) for x, y in zip(lx, ly))
    slope = sxy / sxx
    intercept = mean_y - slope * mean_x
    residuals = [y - (slope * x + intercept) for x, y in zip(lx, ly)]
    ss_res = sum(r * r for r in residuals)
    ss_tot = sum((y - mean_y) ** 2 for y in ly)
    r2 = 1.0 - ss_res / ss_tot if ss_tot else 1.0
    return {"exponent": slope, "intercept": intercept, "r2": r2,
            "residuals": residuals, "points": n}


def local_exponents(sizes, values):
    """The exponent between each adjacent pair: `ln(v2/v1) / ln(n2/n1)`.

    A single exponent hides the shape. A curve that is linear below 32 and
    quadratic above it shows as a rising column here, and a defect that only
    appears past a threshold is visible without fitting anything.
    """
    out = []
    for (n1, v1), (n2, v2) in zip(zip(sizes, values), zip(sizes[1:], values[1:])):
        if n1 > 0 and n2 > n1 and v1 > 0 and v2 > 0:
            out.append(math.log(v2 / v1) / math.log(n2 / n1))
        else:
            out.append(None)
    return out


def floor_fit(sizes, values, steps=400, refine=8):
    """Fit `blocks(n) = floor + a * n^k` by a grid over `k` alone.

    Two reasons this is not the delta fit and not a log-space fit:

    Subtracting `blocks(n_min)` assumes the smallest point holds none of the
    growing term. It always holds some, so the delta grows faster than the
    real term near `n_min` and the delta exponent reads high. Fitting the
    floor is how that bias goes away.

    The residual has to be measured in LINEAR units, not log units. A term
    worth 2% of the total is invisible in log space (it moves `ln y` by
    0.02), so a log-space objective prefers "no floor, no growth" and reports
    an exponent of 0 for exactly the axes whose growth is small but real. In
    linear units that 2% is a real residual and the fit finds it.

    For a fixed `k` the model is linear in `(floor, a)`, so those two come
    from the 2x2 normal equations exactly; only `k` is searched. The reported
    `r2` is linear-space, and `residuals` are relative to the mean, so 0.01
    means a 1% misfit.
    """
    points = [(x, y) for x, y in zip(sizes, values) if x > 0 and y > 0]
    if len(points) < 3:
        return {"exponent": None, "floor": None, "r2": None, "residuals": []}
    xs = [x for x, _ in points]
    ys = [y for _, y in points]
    mean_y = sum(ys) / len(ys)
    ss_tot = sum((y - mean_y) ** 2 for y in ys)

    def solve(exponent):
        powers = [x ** exponent for x in xs]
        n = len(powers)
        sum_p = sum(powers)
        sum_pp = sum(p * p for p in powers)
        sum_y = sum(ys)
        sum_py = sum(p * y for p, y in zip(powers, ys))
        det = n * sum_pp - sum_p * sum_p
        if det == 0:
            return None
        floor = (sum_y * sum_pp - sum_p * sum_py) / det
        scale = (n * sum_py - sum_p * sum_y) / det
        # A fixed cost cannot be negative and a growing term cannot shrink.
        if floor < 0 or scale < 0:
            return None
        fitted = [floor + scale * p for p in powers]
        ss_res = sum((y - f) ** 2 for y, f in zip(ys, fitted))
        return ss_res, floor, scale, fitted

    best = None
    low, high = 0.0, 4.0
    for _ in range(refine):
        for index in range(steps + 1):
            exponent = low + (high - low) * index / steps
            found = solve(exponent)
            if found is None:
                continue
            if best is None or found[0] < best[0]:
                best = (found[0], exponent, found)
        if best is None:
            break
        span = (high - low) / steps
        low = max(0.0, best[1] - span)
        high = min(8.0, best[1] + span)
    if best is None:
        return {"exponent": None, "floor": None, "r2": None, "residuals": []}
    ss_res, exponent, (_, floor, scale, fitted) = best
    r2 = 1.0 - ss_res / ss_tot if ss_tot else 1.0
    residuals = [(y - f) / mean_y for y, f in zip(ys, fitted)]
    return {"exponent": exponent, "floor": floor, "scale": scale, "r2": r2,
            "residuals": residuals}


def fit_axis(sizes, blocks, bytes_allocated=None, ir_lines=None, wall=None):
    """Every number the table reports for one axis, from its raw points.

    Three exponents, because each answers a different question:

      delta      subtract the smallest point and fit. This is the marginal
                 cost of the axis, and it is biased low when the fixed floor
                 is a large share of the smallest point.
      floor      fit `floor + a * n^k` with the floor free. This is the
                 exponent to trust when the deltas are small, because it does
                 not assume the smallest point has no growth in it.
      absolute   fit the raw totals. This is what a user sees, and it is
                 dominated by the floor, so it is only ever a sanity column.
    """
    base = blocks[0]
    delta = [b - base for b in blocks]
    # A zero delta at the first point would poison the log fit; the fit skips
    # non-positive points rather than inventing an offset.
    delta_fit = loglog(sizes, delta)
    absolute_fit = loglog(sizes, blocks)
    floored = floor_fit(sizes, blocks)
    return {
        "sizes": list(sizes),
        "blocks": list(blocks),
        "delta_blocks": delta,
        "bytes": list(bytes_allocated) if bytes_allocated is not None else None,
        "ir_lines": list(ir_lines) if ir_lines is not None else None,
        "wall_s": list(wall) if wall is not None else None,
        "delta_exponent": delta_fit["exponent"],
        "delta_r2": delta_fit["r2"],
        "delta_residuals": delta_fit["residuals"],
        "absolute_exponent": absolute_fit["exponent"],
        "absolute_r2": absolute_fit["r2"],
        "floor_exponent": floored["exponent"],
        "floor_value": floored["floor"],
        "floor_r2": floored["r2"],
        "floor_residuals": floored["residuals"],
        "local": local_exponents(sizes, blocks),
        "delta_local": local_exponents(sizes, delta),
        "bytes_exponent": loglog(sizes, bytes_allocated)["exponent"]
                         if bytes_allocated else None,
        "ir_exponent": loglog(sizes, ir_lines)["exponent"] if ir_lines else None,
    }
