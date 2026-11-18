#!/usr/bin/env python3
"""Prepare bounded independent tabwriter requests without deriving any formatted expectation from the candidate."""
from pathlib import Path
import argparse
import json


def config(flags=0, minimum=0, tab=4, padding=1, pad=32):
    """Keep all public initializer arguments explicit, including unusual byte and zero-tab configurations."""
    return {'min_width': minimum, 'tab_width': tab, 'padding': padding, 'pad': pad, 'flags': flags}


def write(data, alias=False):
    """Encode arbitrary input bytes without asking JSON text decoding to repair them."""
    return {'operation': 'write', 'data': data.hex(), 'alias': alias}


def case(name, data, cfg=None, sink=None, actions=None):
    """Construct one fresh output/formatter epoch; only the SDK will supply expected outcomes."""
    return {'id': name, 'initial': cfg or config(),
            'sinks': [sink or {'id': 'a'}, {'id': 'b'}],
            'actions': actions or [write(data), {'operation': 'flush'}]}


def requests(full):
    """Cover flags, byte/UTF8 fringes, boundaries and exact failure/replay/alias protocols with bounded sinks."""
    rows = []
    combined = (b'a\tb\nlong\tz\n\t\vX\n<tag>\v&odd;\tY\f' +
                b'\xffa\tb\n\f\xff\tlast\n')
    for flags in range(64):
        rows.append(case('flags-' + str(flags), combined, config(flags=flags)))
    for tab in (0, 1, 4, 8):
        for pad in (32, 9, 46, 0, 128, 255):
            for padding in (0, 1, 2):
                name = f'config-{tab}-{pad}-{padding}'
                rows.append(case(name, b'a\tX\nlong\tY\n', config(tab=tab, pad=pad, padding=padding)))
    for minimum in (1, 4, 8, 16):
        for pad in (32, 9):
            rows.append(case(f'minimum-{minimum}-{pad}', b'a\tX\nb\tY\n', config(minimum=minimum, pad=pad)))
    for flags in (64, 1 << 63, 63 | (1 << 40)):
        rows.append(case('unknown-flags-' + str(flags), b'a\tX\nlong\tY\n', config(flags=flags)))
    rows.append(case('tab-padding-overrides-right', b'a\tX\nlong\tY\n', config(flags=4, pad=9)))
    rows.append(case('tab-indent-custom-pad', b'\t\tA\n\t\tBC\n', config(flags=16, pad=46)))
    patterns = [b'', b'a\t', b'a\t\n', b'\t', b'\n', b'\f', b'a\v\vX\n\v\vY\n',
                b'<bad\t\n>\tA\n&unfinished', b'\xff\t\n\f\xff\tX\n',
                'é\t界\ne\u0301\t🙂\n'.encode(), b'\xe2\x82\tX\n\xed\xa0\x80\tY\n',
                b'\x00\x80\xfe\tX\n']
    for number, data in enumerate(patterns):
        for flags in (0, 1, 2, 3, 8, 16, 32, 63):
            rows.append(case(f'boundary-{number}-{flags}', data, config(flags=flags)))
    fragments = ['é\t界\n'.encode(), b'<b>x</b>\t&amp;\n', b'\xffa\tb\xff\tX\n', b'\xe2\x82\xac\t\xf0\x9f\x99\x82\n']
    for number, data in enumerate(fragments):
        for cut in range(len(data) + 1):
            rows.append(case(f'split-{number}-{cut}', data, config(flags=3),
                             actions=[write(data[:cut]), write(data[cut:]), {'operation': 'flush'}]))
        rows.append(case(f'byte-chunks-{number}', data, config(flags=3),
                         actions=[write(bytes([value])) for value in data] + [{'operation': 'flush'}]))
    replay = [write(b'a\tb\ncc\tdd\fTAIL'), write(b'E\tF\n'), {'operation': 'flush'},
              write(b'R\tS\n'), {'operation': 'flush'}, {'operation': 'flush'}]
    for flags in (0, 4, 32, 36):
        for call in range(1, 9):
            rows.append(case(f'auto-error-{flags}-{call}', b'', config(flags=flags),
                             {'id': 'a', 'fail_at': call, 'fail_prefix': 1, 'error_code': f'failure-{call}'}, replay))
    for call in range(1, 9):
        rows.append(case(f'flush-error-{call}', b'', sink={'id': 'a', 'fail_at': call, 'fail_prefix': 1, 'error_code': f'failure-{call}'},
                         actions=[write(b'a\tb\ncc\tdd\n'), {'operation': 'flush'}, {'operation': 'flush'}, write(b'X\tY\n'), {'operation': 'flush'}]))
    for maximum in (0, 1, 2):
        rows.append(case(f'short-{maximum}', b'', sink={'id': 'a', 'max_per_call': maximum}, actions=replay))
    rows.append(case('empty-output-error', b'', sink={'id': 'a', 'fail_empty': True, 'error_code': 'empty-offer'}, actions=replay))
    for reported in (-1, 999):
        for failing in (False, True):
            sink = {'id': 'a', 'report_at': 1, 'reported': reported}
            if failing:
                sink.update(fail_at=1, fail_prefix=1, error_code='original-with-invalid-count')
            rows.append(case(f'invalid-count-{reported}-{failing}', b'', sink=sink, actions=replay))
    for call in (1, 2, 3):
        rows.append(case(f'sink-panic-{call}', b'', sink={'id': 'a', 'panic_at': call}, actions=replay))
    for dimension in ('min_width', 'tab_width', 'padding'):
        invalid = config(); invalid[dimension] = -1
        rows.append(case('negative-' + dimension, b'', invalid))
        rows.append(case('init-refusal-' + dimension, b'', actions=[write(b'A\tB'),
                         {'operation': 'init', 'sink': 'b', 'config': invalid}, write(b'X\n', True), {'operation': 'flush'}]))
    rows.append(case('init-alias', b'', actions=[write(b'A\tB'), {'operation': 'init', 'sink': 'b', 'config': config(flags=32)},
                     write(b'X\tY\n', True), {'operation': 'flush', 'alias': True}]))
    rows.append(case('new-preserves-old-alias', b'', actions=[write(b'A\tB'), {'operation': 'new', 'sink': 'b', 'config': config()},
                     write(b'X\tY\n'), {'operation': 'flush'}, {'operation': 'flush', 'alias': True}]))
    rows.append({'id': 'zero-required-init', 'zero': True, 'sinks': [{'id': 'a'}],
                 'actions': [write(b''), {'operation': 'flush'}, {'operation': 'init', 'sink': 'a', 'config': config()},
                             write(b'A\tB\n'), {'operation': 'flush'}]})
    for field in ('min_width', 'tab_width', 'padding'):
        cfg = config(); cfg[field] = (1 << 63) - 1
        rows.append(case('maximum-' + field, b'A\tB\f', cfg,
                         {'id': 'a', 'offer_limit': 16, 'byte_limit': 64}))
    rows.append(case('maximum-tab-padding', b'A\tB\f', config(tab=(1 << 63) - 1, pad=9),
                     {'id': 'a', 'offer_limit': 16, 'byte_limit': 64}))
    if full:
        for pad in range(256):
            rows.append(case('all-pad-' + str(pad), b'A\tX\nlong\tY\n', config(pad=pad)))
    assert len({row['id'] for row in rows}) == len(rows)
    return rows


def main():
    """Emit requests only; preserve expected bytes/errors as an independent later SDK observation."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--full', action='store_true')
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    assert not args.output.exists(), 'refuse to overwrite a reference request epoch'
    rows = requests(args.full)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(rows, separators=(',', ':')) + '\n')
    print('Prepared', len(rows), 'independent SDK requests; no formatted expectation generated')


if __name__ == '__main__':
    main()
