#!/usr/bin/env python3
"""Cross-check captured color observations with independent unbounded scalar arithmetic."""
from __future__ import annotations
import argparse
import hashlib
import json
from pathlib import Path


#: Standard source identities whose raw channels have sixteen rather than eight bits.
WIDE = {'rgba16', 'nrgba16', 'alpha16', 'gray16', 'source'}


def clip(value: int, maximum: int) -> int:
    """Saturate signed mathematical arithmetic without machine mask tricks."""
    return max(0, min(maximum, value))


def rgb_ycbcr(r: int, g: int, b: int) -> list[int]:
    """Apply fixed rational coefficients and exact per-channel output rounding."""
    return [clip((19595*r + 38470*g + 7471*b + 32768) // 65536, 255),
            clip((-11056*r - 21712*g + 32768*b + 8421376) // 65536, 255),
            clip((32768*r - 27440*g - 5328*b + 8421376) // 65536, 255)]


def ycbcr_rgb(y: int, cb: int, cr: int, divisor: int, maximum: int) -> list[int]:
    """Evaluate direct fixed-point output at the selected eight/sixteen-bit precision."""
    adjusted = y * 65793
    return [clip((adjusted + 91881*(cr-128)) // divisor, maximum),
            clip((adjusted - 22554*(cb-128) - 46802*(cr-128)) // divisor, maximum),
            clip((adjusted + 116130*(cb-128)) // divisor, maximum)]


def rgb_cmyk(r: int, g: int, b: int) -> list[int]:
    """Normalize by the largest channel with an explicit zero-white singularity."""
    white = max(r, g, b)
    return [0, 0, 0, 255] if white == 0 else [
        (white-r)*255 // white, (white-g)*255 // white,
        (white-b)*255 // white, 255-white]


def cmyk_rgb(c: int, m: int, y: int, k: int) -> list[int]:
    """Keep full intermediate precision before the final byte truncation."""
    return [((255-channel)*(255-k)*257 // 255) // 256 for channel in (c, m, y)]


def observe(spec: dict) -> dict:
    """Interpret each source representation separately, including raw over-alpha channels."""
    kind = spec['kind']
    fields = [value & (65535 if kind in WIDE else 255) for value in spec['channels']]
    if kind == 'rgba8':
        rgba = [value*257 for value in fields]
    elif kind in ('rgba16', 'source'):
        rgba = fields[:]
    elif kind == 'nrgba8':
        rgba = [value*257*fields[3] // 255 for value in fields[:3]] + [fields[3]*257]
    elif kind == 'nrgba16':
        rgba = [value*fields[3] // 65535 for value in fields[:3]] + [fields[3]]
    elif kind in ('alpha8', 'alpha16'):
        rgba = [fields[0]*(257 if kind == 'alpha8' else 1)]*4
    elif kind in ('gray8', 'gray16'):
        rgba = [fields[0]*(257 if kind == 'gray8' else 1)]*3 + [65535]
    elif kind == 'ycbcr':
        rgba = ycbcr_rgb(*fields, 256, 65535) + [65535]
    elif kind == 'ycbcra':
        alpha = fields[3]*257
        rgba = [value*alpha // 65535 for value in ycbcr_rgb(*fields[:3], 256, 65535)] + [alpha]
    elif kind == 'cmyk':
        white = 65535-fields[3]*257
        rgba = [(65535-value*257)*white // 65535 for value in fields[:3]] + [65535]
    else:
        raise ValueError(kind)
    return {'kind': kind, 'channels': fields, 'rgba': rgba}


def convert(kind: str, original: dict) -> dict:
    """Model conversion independently of tagged machine-word storage and vector code."""
    if kind == original['kind']:
        return original
    r, g, b, alpha = original['rgba']
    if kind == 'rgba8':
        channels = [value // 256 for value in (r, g, b, alpha)]
    elif kind == 'rgba16':
        channels = [r, g, b, alpha]
    elif kind in ('nrgba8', 'nrgba16'):
        straight = [value*65535 // alpha for value in (r, g, b)] if alpha else [0, 0, 0]
        channels = [value // 256 & 255 for value in straight] + [alpha // 256] if kind == 'nrgba8' else [value & 65535 for value in straight] + [alpha]
    elif kind in ('alpha8', 'alpha16'):
        channels = [alpha // 256 if kind == 'alpha8' else alpha]
    elif kind in ('gray8', 'gray16'):
        weighted = 19595*r + 38470*g + 7471*b + 32768
        channels = [weighted // (16777216 if kind == 'gray8' else 65536)]
    elif kind == 'ycbcr':
        channels = rgb_ycbcr(r // 256, g // 256, b // 256)
    elif kind == 'ycbcra':
        if original['kind'] == 'ycbcr':
            channels = original['channels'] + [255]
        else:
            straight = [value*65535 // alpha for value in (r, g, b)] if alpha else [r, g, b]
            channels = rgb_ycbcr(*(value // 256 & 255 for value in straight)) + [alpha // 256]
    elif kind == 'cmyk':
        channels = rgb_cmyk(r // 256, g // 256, b // 256)
    else:
        raise ValueError(kind)
    return observe({'kind': kind, 'channels': channels})


def check(actual: object, expected: object, context: str) -> None:
    """Stop on the first exact independent discrepancy and retain both observations."""
    if actual != expected:
        raise AssertionError(f'{context}: captured={actual!r}; scalar={expected!r}')


def main() -> None:
    """Validate all captured source/model/helper/palette observations without compiling."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--vectors', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    data = json.loads(args.vectors.read_text())
    observations = 0
    for index, vector in enumerate(data['vectors']):
        original = observe(vector['input'])
        check(vector['value'], original, f'original{index}')
        for kind, captured in zip(data['model_kinds'], vector['converted'], strict=True):
            check(captured, convert(kind, original), f'model{index}/{kind}')
            observations += 1
        observations += 1
    for index, vector in enumerate(data['rgb_conversions']):
        rgb = vector['input']
        check(vector['ycbcr'], rgb_ycbcr(*rgb), f'RGBToYCbCr{index}')
        check(vector['cmyk'], rgb_cmyk(*rgb), f'RGBToCMYK{index}')
        check(vector['from_ycbcr'], ycbcr_rgb(*rgb, 65536, 255), f'YCbCrToRGB{index}')
        observations += 3
    for index, vector in enumerate(data['cmyk_conversions']):
        check(vector['rgb'], cmyk_rgb(*vector['input']), f'CMYKToRGB{index}')
        observations += 1
    for index, vector in enumerate(data['palettes']):
        query = observe(vector['query'])['rgba']
        colors = [observe(spec) for spec in vector['colors']]
        distances = [sum((a-b)**2 // 4 for a, b in zip(query, color['rgba'])) for color in colors]
        nearest = min(range(len(colors)), key=distances.__getitem__) if colors else 0
        check(vector['index'], nearest, f'paletteIndex{index}')
        check(vector['converted'], colors[nearest] if colors else None, f'paletteConvert{index}')
        observations += 2
    expected_constants = [observe({'kind': kind, 'channels': [value]}) for kind, value in
                          [('gray16', 0), ('gray16', 65535), ('alpha16', 0), ('alpha16', 65535)]]
    check(data['constants'], expected_constants, 'constants')
    check(data['custom_identity'], observe({'kind': 'source', 'channels': [1, 257, 32768, 65535]}), 'customIdentity')
    check(data['custom_absent'], True, 'customAbsent')
    observations += 6
    report = {'scope': __doc__, 'vectors_sha256': hashlib.sha256(args.vectors.read_bytes()).hexdigest(),
              'runner_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
              'sdk': data['sdk'], 'independent_exact_observations': observations,
              'source_colors': len(data['vectors']), 'all_passed': True,
              'limits': 'Independent scalar reference validation only; no Cinch runtime or kernel claim.'}
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(observations, 'independent exact scalar observations passed')


if __name__ == '__main__':
    main()
