"""Prepare independent alphabet-contract fixtures; this generator has not been run."""
import json
import sys

ALPHABET = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567'


def record(name, values, scope):
    """Describe input and expected ASCII using independent alphabet indexing."""
    return {'case': name, 'scope': scope, 'input_hex': bytes(values).hex(),
            'expected': ''.join(ALPHABET[value % len(ALPHABET)] for value in values)}


def vectors():
    """Cover every byte, vector lane, SWAR lane and short/block/fringe length."""
    for value in range(256):
        yield record('repeat26-' + str(value), [value] * 26, 'model-pure-mapping')
    for length in range(41):
        yield record('length-' + str(length), [(index * 37 + length) % 256 for index in range(length)],
                     'model-pure-mapping')
    for lane in range(16):
        for value in range(256):
            values = [0] * 32
            values[lane] = value
            yield record('vector-lane-' + str(lane) + '-' + str(value), values, 'model-pure-mapping')
    for lane in range(8):
        for value in range(256):
            values = [0] * 8
            values[lane] = value
            yield record('swar-lane-' + str(lane) + '-' + str(value), values, 'model-pure-mapping')


def main():
    """Write functional model JSONL without replacing or claiming secure entropy."""
    if len(sys.argv) != 1:
        raise SystemExit('usage: token_vectors.py > vectors.jsonl')
    for item in vectors():
        print(json.dumps(item, separators=(',', ':')))


if __name__ == '__main__':
    main()
